use axum::{extract::State, http::StatusCode, Json};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::{
    model::{self, ToRecord},
    service::line_item::LineItemService,
};

type JsonResult<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub struct LineItemHandler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLineItem {
    pub fulfillment_id: i64,
    pub product_id: i64,
    pub quantity: i64,
}

impl LineItemHandler {
    pub async fn create_line_item<T: LineItemService>(
        State(mut service): State<T>,
        Json(payload): Json<CreateLineItem>,
    ) -> JsonResult<model::Record<model::LineItemDetails>> {
        match service
            .create_line_item(payload.fulfillment_id, payload.product_id, payload.quantity)
            .await
        {
            Ok(id) => Ok((
                StatusCode::CREATED,
                Json(
                    model::LineItemDetails {
                        product_id: payload.product_id,
                        quantity: payload.quantity,
                        quantity_fulfilled: 0,
                    }
                    .to_record(id),
                ),
            )),
            Err(e) => {
                warn!("{}", e);
                warn!("error creating line item");
                Err(e.into())
            }
        }
    }
}
