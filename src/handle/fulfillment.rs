use crate::model::{
    self, FulfillmentDetails, FulfillmentStatus, FulfillmentType, ProductDetails, ToRecord,
};
use crate::service;
use crate::service::fulfillment::FulfillmentService;
use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json, Router};
use log::error;
use serde::{Deserialize, Serialize};

pub struct FulfillmentHandler;

type JsonResult<T> = Result<(StatusCode, Json<T>), StatusCode>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewFulfillmentRequest {
    fulfillment_type: FulfillmentType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewFulfillmentStatusRequest {
    fulfillment_status: FulfillmentStatus,
}

impl FulfillmentHandler {
    pub async fn create_fulfillment<T: FulfillmentService>(
        State(mut service): State<T>,
        Json(payload): Json<NewFulfillmentRequest>,
    ) -> JsonResult<model::Record<model::FulfillmentDetails>> {
        match service
            .create_fulfillment(payload.fulfillment_type.clone())
            .await
        {
            Ok(id) => Ok((
                StatusCode::CREATED,
                Json(
                    model::FulfillmentDetails {
                        fulfillment_type: payload.fulfillment_type,
                        status: model::FulfillmentStatus::New,
                    }
                    .to_record(id),
                ),
            )),
            // FIXME: handle abstract error type 400 errors not covered
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    pub async fn update_fulfillment_status<T: FulfillmentService>(
        State(mut service): State<T>,
        Path(fulfillment_id): Path<i64>,
        Json(payload): Json<NewFulfillmentStatusRequest>,
    ) -> Result<StatusCode, StatusCode> {
        println!("aaaaa");
        match service
            .set_fulfillment_status(&fulfillment_id, payload.fulfillment_status)
            .await
        {
            Ok(_) => Ok(StatusCode::ACCEPTED),
            // FIXME: handle abstract error type 400 errors not covered
            Err(e) => {
                println!("service error setting fulfillment status {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
