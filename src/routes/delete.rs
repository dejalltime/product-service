use crate::model::ProductInfo;
use crate::startup::AppState;
use actix_web::{web, Error, HttpResponse, error};
use mongodb::bson::doc;

pub async fn delete_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let filter = doc! { "id": path.product_id };

    let delete_result = data.product_collection
        .delete_one(filter, None)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    if delete_result.deleted_count == 1 {
        Ok(HttpResponse::Ok().body("Product deleted"))
    } else {
        Ok(HttpResponse::NotFound().body("Product not found"))
    }
}
