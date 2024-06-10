// use crate::provider::SqliteProvider;

// pub trait OrderService {
//     type Error;

//     async fn init_provider(&mut self) -> Result<(), Self::Error>;
//     async fn create_order(&mut self) -> Result<(), Self::Error>;
// }

// const CREATE_ORDER_TABLE_SQL: &str = r#"
//     CREATE TABLE IF NOT EXISTS orders (
//         id INTEGER NOT NULL UNIQUE PRIMARY KEY,
//     );
// "#;

// impl OrderService for SqliteProvider {
//     type Error = super::Error;

//     async fn init_provider(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }

//     async fn create_order(&mut self) -> Result<(), Self::Error> {
//         todo!()
//     }
// }
