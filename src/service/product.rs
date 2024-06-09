use crate::provider::SqliteProvider;

pub trait ProductService {
    async fn init_provider(&mut self) -> Result<(), ()>;
    async fn create_product(&mut self, sku: &String, description: &String) -> Result<i64, ()>;
}

impl ProductService for SqliteProvider {
    async fn init_provider(&mut self) -> Result<(), ()> {
        let Ok(mut conn) = self.connection.acquire().await else {
            return Err(());
        };

        let result = sqlx::query(
            r#"
                CREATE TABLE IF NOT EXISTS products (
                    id INTEGER NOT NULL UNIQUE PRIMARY KEY,
                    sku TEXT NOT NULL,
                    description TEXT NOT NULL  
                );
            "#,
        )
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(())
            }
        }
    }
    async fn create_product(&mut self, sku: &String, description: &String) -> Result<i64, ()> {
        let Ok(mut conn) = self.connection.acquire().await else {
            return Err(());
        };

        let result = sqlx::query(
            r#"
                INSERT INTO products VALUES( NULL, ?1, ?2 );
            "#,
        )
        .bind(sku)
        .bind(description)
        .execute(&mut *conn)
        .await;

        match result {
            Ok(resp) => Ok(resp.last_insert_rowid()),
            Err(_) => Err(()),
        }
    }
}
