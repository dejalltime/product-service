use actix_web::{error, web, Error, HttpResponse};
use crate::model::Product;
use crate::startup::AppState;
use futures_util::StreamExt;
use crate::localwasmtime::validate_product;
use mongodb::bson::{doc, to_document};

pub async fn update_product(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > data.settings.max_size {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let product = serde_json::from_slice::<Product>(&body)?;

    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            let filter = doc! { "id": validated_product.id };
            let update = doc! {
                "$set": to_document(&validated_product)
                    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
            };

            let result = data.product_collection
                .update_one(filter, update, None)
                .await
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

            if result.matched_count == 1 {
                Ok(HttpResponse::Ok().json(validated_product))
            } else {
                Ok(HttpResponse::NotFound().body("Product not found"))
            }
        }
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}
