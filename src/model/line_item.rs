use serde::{Deserialize, Serialize};

use super::ToRecord;

impl ToRecord for LineItemDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemDetails {
    pub product_id: i64,
    pub quantity: i64,
    pub quantity_fulfilled: i64,
}
