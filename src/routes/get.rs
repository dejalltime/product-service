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
        .map_err(|e| {
            eprintln!("❌ Error querying single product: {:?}", e);
            error::ErrorInternalServerError("Failed to fetch product")
        })?;

    match product {
        Some(mut product) => {
            product._id = None; // remove Mongo _id to ensure serializability
            Ok(HttpResponse::Ok().json(product))
        },
        None => Ok(HttpResponse::NotFound().body("Product not found")),
    }
}

pub async fn get_products(
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut cursor = match data.product_collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            eprintln!("❌ Failed to query MongoDB: {:?}", e);
            return Err(error::ErrorInternalServerError("Failed to query products"));
        }
    };

    let mut products = Vec::new();
    while let Ok(Some(mut product)) = cursor.try_next().await {
        product._id = None; // clear _id to prevent JSON issues
        products.push(product);
    }

    println!("✅ Successfully fetched {} products", products.len());
    Ok(HttpResponse::Ok().json(products))
}