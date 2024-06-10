use log::warn;
use sqlx::Row;

use crate::{
    model::{self, ToRecord},
    provider::SqliteProvider,
};

pub trait ProductService {
    type Error;
    async fn init_provider(&mut self) -> Result<(), Self::Error>;
    async fn create_product(
        &mut self,
        sku: &String,
        description: &String,
    ) -> Result<i64, Self::Error>;
    async fn get_product(
        &mut self,
        id: &i64,
    ) -> Result<model::Record<model::ProductDetails>, Self::Error>;
}

impl ProductService for SqliteProvider {
    type Error = super::Error;
    async fn init_provider(&mut self) -> Result<(), Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let _result = sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS products (
                    id INTEGER NOT NULL UNIQUE PRIMARY KEY,
                    sku TEXT NOT NULL,
                    description TEXT NOT NULL  
                );
            "#,
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
    async fn create_product(
        &mut self,
        sku: &String,
        description: &String,
    ) -> Result<i64, Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let result = sqlx::query(
            r#"
                INSERT INTO products VALUES( NULL, ?1, ?2 );
            "#,
        )
        .bind(sku)
        .bind(description)
        .execute(&mut *conn)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn get_product(
        &mut self,
        id: &i64,
    ) -> Result<model::Record<model::ProductDetails>, Self::Error> {
        let mut conn = self.connection.acquire().await?;

        let result = sqlx::query(
            r#"
                SELECT * FROM products WHERE id=?1;
            "#,
        )
        .bind(id.to_owned())
        .fetch_optional(&mut *conn)
        .await?;

        let Some(row) = result else {
            warn!("Sql row not found");
            return Err(super::Error::ProductNotFound(String::new()));
        };

        let record = model::ProductDetails {
            sku: row.try_get("sku")?,
            description: row.try_get("description")?,
        }
        .to_record(id.to_owned());

        Ok(record)
    }
}
