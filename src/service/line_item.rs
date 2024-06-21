use crate::model::{LineItemDetails, ToRecord};

use std::fmt::Display;

use axum::http::StatusCode;
use futures::TryStreamExt;
use log::warn;
use sqlx::Row;

use crate::{model, provider::SqliteProvider};

pub trait LineItemService {
    type Error: Display + Into<StatusCode>;

    async fn init_provider(&mut self) -> Result<(), Self::Error>;

    async fn create_line_item(
        &mut self,
        fulfillment_id: i64,
        product_id: i64,
        quantity: i64,
    ) -> Result<i64, Self::Error>;

    async fn get_line_item(
        &mut self,
        line_item_id: i64,
    ) -> Result<Option<model::Record<model::LineItemDetails>>, Self::Error>;

    async fn get_line_items_by_fulfillment_id(
        &mut self,
        fulfillment_id: i64,
    ) -> Result<Vec<model::Record<model::LineItemDetails>>, Self::Error>;
}

impl LineItemService for SqliteProvider {
    type Error = super::Error;

    async fn init_provider(&mut self) -> Result<(), Self::Error> {
        let pool_connection = self.connection.acquire().await?;
        let mut conn = pool_connection;

        let _result = sqlx::query(sql_stmt::CREATE_TABLE)
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

        let result = sqlx::query(sql_stmt::INSERT_LINE_ITEM)
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

    async fn get_line_item(
        &mut self,
        line_item_id: i64,
    ) -> Result<Option<model::Record<model::LineItemDetails>>, Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let result = sqlx::query(sql_stmt::SELECT_LINE_ITEM)
            .bind(line_item_id)
            .fetch_optional(&mut *conn)
            .await?;

        let Some(row) = result else {
            warn!("Sql row not found");
            return Ok(None);
        };

        let record = model::LineItemDetails {
            product_id: row.try_get("productId")?,
            fulfillment_id: row.try_get("fulfillmentId")?,
            quantity: row.try_get("quantity")?,
            quantity_fulfilled: 0,
        }
        .to_record(line_item_id.to_owned());

        Ok(Some(record))
    }

    async fn get_line_items_by_fulfillment_id(
        &mut self,
        fulfillment_id: i64,
    ) -> Result<Vec<model::Record<model::LineItemDetails>>, Self::Error> {
        let mut conn = self.connection.acquire().await?;

        let mut rows = sqlx::query(sql_stmt::SELECT_BY_FULFILLMENT_ID)
            .bind(fulfillment_id)
            .fetch(&mut *conn);

        let mut records: Vec<model::Record<model::LineItemDetails>> = Vec::new();

        while let Some(row) = rows.try_next().await? {
            records.push(
                LineItemDetails {
                    product_id: row.try_get("productId")?,
                    fulfillment_id: row.try_get("fulfillmentId")?,
                    quantity: row.try_get("quantity")?,
                    quantity_fulfilled: 0,
                }
                .to_record(row.try_get("id")?),
            );
        }

        Ok(records)
    }
}

mod sql_stmt {
    pub const SELECT_LINE_ITEM: &str = r#"
        SELECT id, fulfillmentId, productId, quantity FROM lineItems WHERE id=$1;
    "#;

    pub const CREATE_TABLE: &str = r#"
        CREATE TABLE IF NOT EXISTS lineItems (
            id INTEGER NOT NULL UNIQUE PRIMARY KEY,
            fulfillmentId INTEGER NOT NULL,
            productId INTEGER NOT NULL,
            quantity INTEGER NOT NULL
        );
    "#;

    pub const INSERT_LINE_ITEM: &str = r#"
        INSERT INTO lineItems (fulfillmentId, productId, quantity)
        SELECT $1, $2, $3
        WHERE EXISTS (
            SELECT 1
            FROM fulfillments
            WHERE id = $1 AND fulfillmentStatus = 'New'
        );
    "#;

    pub const SELECT_BY_FULFILLMENT_ID: &str = r#"
        SELECT id, fulfillmentId, productId, quantity FROM lineItems WHERE fulfillmentId=$1;
    "#;
}

#[cfg(test)]
mod test_create_line_item {
    use test_log::test;

    use crate::{
        model,
        provider::SqliteProvider,
        service::{fulfillment::FulfillmentService, line_item::LineItemService},
    };

    #[test(tokio::test)]
    async fn test_no_tables() {
        // Line items should require both fulfillment & line item tables
        let mut provider = SqliteProvider::new_memory().await.unwrap();

        assert!(provider.create_line_item(1, 1, 1).await.is_err());
    }

    #[test(tokio::test)]
    async fn test_fulfillment_dependency() {
        // Line items should require a valid fulfillment to relate to
        let mut provider = SqliteProvider::new_memory().await.unwrap();

        FulfillmentService::init_provider(&mut provider)
            .await
            .unwrap();
        LineItemService::init_provider(&mut provider).await.unwrap();

        assert!(provider.create_line_item(1, 1, 1).await.is_err());
    }

    #[test(tokio::test)]
    async fn test_expected_use() {
        let mut provider = SqliteProvider::new_memory().await.unwrap();

        FulfillmentService::init_provider(&mut provider)
            .await
            .unwrap();
        LineItemService::init_provider(&mut provider).await.unwrap();

        provider
            .create_fulfillment(model::FulfillmentType::StockPickUp)
            .await
            .unwrap();

        match provider.create_line_item(1, 1, 1).await {
            Ok(id) => assert_eq!(id, 1),
            Err(e) => panic!("Expected Ok got {:?}", e),
        };
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;

    use sqlx::Acquire;

    use crate::{
        model,
        provider::{self, SqliteProvider},
        service::line_item::LineItemService,
    };

    async fn create_tables(provider: &mut SqliteProvider) {
        let mut conn = provider.connection.acquire().await.unwrap();
        sqlx::query(crate::service::fulfillment::CREATE_FULFILLMENT_TABLE_SQL)
            .execute(&mut *conn)
            .await
            .unwrap();
        sqlx::query(super::sql_stmt::CREATE_TABLE)
            .execute(&mut *conn)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_line_item() {
        let mut provider = SqliteProvider::new_memory().await.unwrap();
        let _ = create_tables(&mut provider).await;

        let _db_init: () = {
            let mut conn = provider.connection.acquire().await.unwrap();

            sqlx::query(r#"INSERT INTO fulfillments VALUES( NULL, ?1, ?2 );"#)
                .bind(String::from(model::FulfillmentStatus::New))
                .bind(String::from(model::FulfillmentType::StockPickUp))
                .execute(&mut *conn)
                .await
                .unwrap();
            sqlx::query(super::sql_stmt::INSERT_LINE_ITEM)
                .bind(1)
                .bind(1)
                .bind(1)
                .execute(&mut *conn)
                .await
                .unwrap();
        };

        match provider.get_line_item(1).await {
            Ok(_model) => {}
            Err(e) => {
                panic!("expected ok got {:?}", e);
            }
        };
    }

    #[tokio::test]
    async fn test_get_line_items_by_fullfillment_id() {
        let mut provider = SqliteProvider::new_memory().await.unwrap();
        let _ = create_tables(&mut provider).await;

        let _db_init: () = {
            let mut conn = provider.connection.acquire().await.unwrap();

            sqlx::query(r#"INSERT INTO fulfillments VALUES ( NULL, ?1, ?2 );"#)
                .bind(String::from(model::FulfillmentStatus::New))
                .bind(String::from(model::FulfillmentType::StockPickUp))
                .execute(&mut *conn)
                .await
                .unwrap();

            for _ in 0..2 {
                sqlx::query(super::sql_stmt::INSERT_LINE_ITEM)
                    .bind(1)
                    .bind(1)
                    .bind(1)
                    .execute(&mut *conn)
                    .await
                    .unwrap();
            }
        };

        match provider.get_line_items_by_fulfillment_id(1).await {
            Ok(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].id, 1);
                assert_eq!(records[1].id, 2);
            }
            Err(e) => panic!("expected ok got {:?}", e),
        };
    }
}
