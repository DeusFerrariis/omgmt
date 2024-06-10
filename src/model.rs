use serde::{Deserialize, Serialize};

// TODO: break this up by service

// Helpers / Utils

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record<T> {
    pub id: i64,
    #[serde(flatten)]
    pub data: T,
}

pub trait ToRecord: Sized + Clone {
    fn to_record(&self, id: i64) -> Record<Self> {
        Record {
            id,
            data: self.clone(),
        }
    }
}

// PRODUCT

impl ToRecord for ProductDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductDetails {
    pub sku: String,
    pub description: String,
}

// ORDER

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderDetails {
    status: OrderStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrderStatus {
    // NOTE: this may be expounded on or converted to an emergent property
    Quote,
    Sold,
    Done,
}

// FULFILLMENT

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

// LINE ITEM

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItem {
    product_id: i64,
    quantity: i64,
}
