use crate::model::{self, ToRecord};
use crate::service::product::ProductService;
use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode};

pub struct ProductHandler;

type JsonResult<T> = Result<(StatusCode, Json<T>), StatusCode>;

impl ProductHandler {
    pub async fn create_product<T: ProductService>(
        State(mut service): State<T>,
        Json(payload): Json<model::ProductDetails>,
    ) -> JsonResult<model::Record<model::ProductDetails>> {
        match service
            .create_product(&payload.sku, &payload.description)
            .await
        {
            Ok(id) => Ok((StatusCode::CREATED, Json(payload.to_record(id)))),
            // FIXME: handle abstract error type 400 errors not covered
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_product<T: ProductService>(
        State(mut service): State<T>,
        Path(product_id): Path<u32>,
    ) -> JsonResult<model::Record<model::ProductDetails>> {
        // FIXME: Dirty conversion here fix
        match service.get_product(&product_id.into()).await {
            Ok(record) => Ok((StatusCode::OK, Json(record))),
            // FIXME: handle abstract error type 400 errors not covered
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
