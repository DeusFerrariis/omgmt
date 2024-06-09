use crate::service::product::ProductService;
use axum::{extract::State, http::StatusCode, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDetails {
    sku: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductRecord {
    id: i64,
    sku: String,
    description: String,
}

pub async fn create_product<T: ProductService>(
    State(mut service): State<T>,
    Json(payload): Json<ProductDetails>,
) -> Result<(StatusCode, Json<ProductRecord>), StatusCode> {
    let result = service
        .create_product(&payload.sku, &payload.description)
        .await;

    match result {
        Ok(i) => Ok((
            StatusCode::CREATED,
            Json(ProductRecord {
                id: i,
                sku: payload.sku,
                description: payload.description,
            }),
        )),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
