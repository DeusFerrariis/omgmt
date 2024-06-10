use serde::{Deserialize, Serialize};

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
