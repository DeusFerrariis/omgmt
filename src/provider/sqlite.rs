use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone, Debug)]
pub struct SqliteProvider {
    pub connection: Pool<Sqlite>,
}

impl SqliteProvider {
    pub async fn new_memory() -> Result<Self, sqlx::Error> {
        let conn = SqlitePool::connect("sqlite::memory:").await?;
        Ok(Self { connection: conn })
    }

    // pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
    //     let conn = SqlitePool::connect(url).await?;
    //     Ok(Self { connection: conn })
    // }
}
