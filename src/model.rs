use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductDetails {
    pub sku: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record<T> {
    pub id: i64,
    #[serde(flatten)]
    pub data: T,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FulfillmentDetails {
    pub fulfillment_type: FulfillmentType,
    pub status: FulfillmentStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FulfillmentType {
    StockPickUp,
    StockDelivery,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FulfillmentStatus {
    New,
    Initialized,
    InProgress,
    Fulfilled,
}

impl<T> Record<T> {
    fn new(id: i64, data: T) -> Self {
        Record { id: id, data: data }
    }
}

pub trait ToRecord: Sized + Clone {
    fn to_record(&self, id: i64) -> Record<Self> {
        Record {
            id,
            data: self.clone(),
        }
    }
}

impl ToRecord for ProductDetails {}
impl ToRecord for FulfillmentDetails {}

impl From<FulfillmentType> for String {
    fn from(value: FulfillmentType) -> Self {
        format!("{:?}", value)
    }
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
