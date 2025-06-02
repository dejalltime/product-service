use actix_web::{web, HttpResponse, Error, error};
use mongodb::bson::doc;
use crate::startup::AppState;
use crate::model::ProductInfo;
use futures_util::TryStreamExt;

pub async fn get_product(
    data: web::Data<AppState>,
    path: web::Path<ProductInfo>,
) -> Result<HttpResponse, Error> {
    let filter = doc! { "id": path.product_id };

    let product = data.product_collection
        .find_one(filter, None)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    match product {
        Some(product) => Ok(HttpResponse::Ok().json(product)),
        None => Ok(HttpResponse::NotFound().body("Product not found")),
    }
}

pub async fn get_products(
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut cursor = data.product_collection
        .find(None, None)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let mut products = Vec::new();
    while let Some(product) = cursor
        .try_next()
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
    {
        products.push(product);
    }

    Ok(HttpResponse::Ok().json(products))
}
