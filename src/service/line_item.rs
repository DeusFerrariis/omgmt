use std::fmt::Display;

use axum::http::StatusCode;
use log::{error, warn};

use crate::provider::SqliteProvider;

pub trait LineItemService {
    type Error: Display + Into<StatusCode>;

    async fn init_provider(&mut self) -> Result<(), Self::Error>;
    async fn create_line_item(
        &mut self,
        fulfillment_id: i64,
        product_id: i64,
        quantity: i64,
    ) -> Result<i64, Self::Error>;
}

impl LineItemService for SqliteProvider {
    type Error = super::Error;

    async fn init_provider(&mut self) -> Result<(), Self::Error> {
        let mut conn = self.connection.acquire().await?;

        let _result = sqlx::query(CREATE_LINE_ITEM_TABLE_SQL)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn create_line_item(
        &mut self,
        fulfillment_id: i64,
        product_id: i64,
        quantity: i64,
    ) -> Result<i64, Self::Error> {
        let mut conn = self.connection.acquire().await?;

        let result = sqlx::query(INSERT_LINE_ITEM_SQL)
            .bind(fulfillment_id)
            .bind(product_id)
            .bind(quantity)
            .execute(&mut *conn)
            .await?;

        if result.rows_affected() == 0 {
            warn!("rows affected was 0 expected 1");
            return Err(super::Error::BadInput(format!(
                "can't add line item to fulfillment {}",
                fulfillment_id
            )));
        }

        Ok(result.last_insert_rowid())
    }
}

// SQL Statements

const CREATE_LINE_ITEM_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS lineItems (
        id INTEGER NOT NULL UNIQUE PRIMARY KEY,
        fulfillmentId INTEGER NOT NULL,
        productId INTEGER NOT NULL,
        quantity INTEGER NOT NULL
    );
"#;

const INSERT_LINE_ITEM_SQL: &str = r#"
    INSERT INTO lineItems (fulfillmentId, productId, quantity)
    SELECT $1, $2, $3
    WHERE EXISTS (
        SELECT 1
        FROM fulfillments
        WHERE id = $1 AND fulfillmentStatus = 'New'
    );
"#;
