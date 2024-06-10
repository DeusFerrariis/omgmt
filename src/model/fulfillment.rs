use serde::{Deserialize, Serialize};

use super::ToRecord;

impl ToRecord for FulfillmentDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FulfillmentDetails {
    pub fulfillment_type: FulfillmentType,
    pub status: FulfillmentStatus,
}

impl From<FulfillmentType> for String {
    fn from(value: FulfillmentType) -> Self {
        format!("{:?}", value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FulfillmentType {
    StockPickUp,
    StockDelivery,
}

impl From<FulfillmentStatus> for String {
    fn from(value: FulfillmentStatus) -> Self {
        format!("{:?}", value)
    }
}

impl FulfillmentStatus {
    pub fn allowed_priors(&self) -> Vec<Self> {
        match self {
            Self::Initialized => vec![Self::New],
            Self::InProgress => vec![Self::Initialized],
            Self::Fulfilled => vec![Self::InProgress],
            _ => vec![],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FulfillmentStatus {
    New,
    Initialized,
    InProgress,
    Fulfilled,
}
