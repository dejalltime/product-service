use actix_web::{web, HttpResponse, error};
use crate::model::Product;
use crate::startup::AppState;
use crate::localwasmtime::validate_product;
use futures_util::StreamExt;

pub async fn add_product(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > data.settings.max_size {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let product = serde_json::from_slice::<Product>(&body)?;

    // Rules engine validation
    match validate_product(&data.settings, &product) {
        Ok(validated_product) => {
            // generate ID by counting existing docs
            let count = data.product_collection.count_documents(None, None).await.unwrap_or(0);
            let mut validated_product = validated_product; // make it mutable
            validated_product.id = (count + 1) as i32;

            // insert to DB
            data.product_collection
                .insert_one(&validated_product, None)
                .await
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

            Ok(HttpResponse::Ok().json(validated_product))
        }
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}