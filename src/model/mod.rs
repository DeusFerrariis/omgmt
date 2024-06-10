mod fulfillment;
mod line_item;
mod order;
mod product;

pub use fulfillment::*;
pub use line_item::*;
pub use product::*;

use serde::{Deserialize, Serialize};

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
