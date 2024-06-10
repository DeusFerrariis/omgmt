use serde::{Deserialize, Serialize};

use super::ToRecord;

impl ToRecord for ProductDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductDetails {
    pub sku: String,
    pub description: String,
}
