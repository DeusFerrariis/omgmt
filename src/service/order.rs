use crate::provider::SqliteProvider;

pub trait OrderService {
    type Error;

    async fn init_provider(&mut self) -> Result<(), Self::Error>;
    async fn create_order(&mut self) -> Result<i64, Self::Error>;
}

const CREATE_ORDER_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS orders (
        id INTEGER NOT NULL UNIQUE PRIMARY KEY
    );
"#;

pub mod sql_stmt {
    pub const NEW_ORDER: &str = r#"INSERT INTO orders VALUES( null );"#;
}

impl OrderService for SqliteProvider {
    type Error = super::Error;

    async fn init_provider(&mut self) -> Result<(), Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let _ = sqlx::query(CREATE_ORDER_TABLE_SQL)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }

    async fn create_order(&mut self) -> Result<i64, Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let result = sqlx::query(sql_stmt::NEW_ORDER).execute(&mut *conn).await?;
        Ok(result.last_insert_rowid())
    }
}

#[cfg(test)]
mod test {
    use crate::{provider::SqliteProvider, service::order::OrderService};

    #[tokio::test]
    async fn test_create_order() {
        let mut provider = SqliteProvider::new_memory().await.unwrap();
        OrderService::init_provider(&mut provider).await.unwrap();
        let result = provider.create_order().await.unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_cancel_order() {
        // TODO: adds a canceled tag to the order
        // NOTE: does not in effect do anything other than act as a
        //       marker for the end user
        todo!()
    }
}
