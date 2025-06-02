use serde::{Deserialize, Serialize};
use crate::localwasmtime::WasmProduct;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_id")]
    pub _id: Option<ObjectId>,
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub image: String,
}

#[derive(Deserialize)]
pub struct ProductInfo {
    pub product_id: i32,
}

// Convert Product (Mongo model) into WasmProduct (no _id)
impl Into<WasmProduct> for Product {
    fn into(self) -> WasmProduct {
        WasmProduct {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price,
            image: self.image,
        }
    }
}

// Convert WasmProduct into Product (with _id set to None)
impl From<WasmProduct> for Product {
    fn from(product: WasmProduct) -> Self {
        Self {
            _id: None,
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            image: product.image,
        }
    }
}