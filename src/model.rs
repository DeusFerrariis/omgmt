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
