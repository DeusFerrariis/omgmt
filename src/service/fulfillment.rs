use std::fmt::Display;

use sqlx::query;

use crate::{model, provider::SqliteProvider};

pub trait FulfillmentService {
    type Error: Display;

    async fn init_provider(&mut self) -> Result<(), Self::Error>;
    async fn create_fulfillment(
        &mut self,
        fulfillment_type: model::FulfillmentType,
    ) -> Result<i64, Self::Error>;
    async fn set_fulfillment_status(
        &mut self,
        fulfillment_id: &i64,
        fulfillment_status: model::FulfillmentStatus,
    ) -> Result<(), Self::Error>;
}

pub const CREATE_FULFILLMENT_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS fulfillments (
        id INTEGER NOT NULL UNIQUE PRIMARY KEY,
        fulfillmentStatus TEXT NOT NULL,
        fulfillmentType TEXT NOT NULL
    );
"#;

impl FulfillmentService for SqliteProvider {
    type Error = super::Error;

    async fn init_provider(&mut self) -> Result<(), Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let _result = sqlx::query(CREATE_FULFILLMENT_TABLE_SQL)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }

    async fn create_fulfillment(
        &mut self,
        fulfillment_type: model::FulfillmentType,
    ) -> Result<i64, Self::Error> {
        let mut conn = self.connection.acquire().await?;

        let result = query(
            r#"
            INSERT INTO fulfillments VALUES( NULL, ?1, ?2 );
        "#,
        )
        .bind(String::from(model::FulfillmentStatus::New))
        .bind(String::from(fulfillment_type))
        .execute(&mut *conn)
        .await?;

        Ok(result.last_insert_rowid())
    }

    async fn set_fulfillment_status(
        &mut self,
        fulfillment_id: &i64,
        fulfillment_status: model::FulfillmentStatus,
    ) -> Result<(), Self::Error> {
        let mut conn = self.connection.acquire().await?;
        let allowed_priors_strings = fulfillment_status
            .allowed_priors()
            .iter()
            .map(|s| format!("'{}'", String::from(s.clone())))
            .collect::<Vec<String>>()
            .join(", ");

        println!("{}", allowed_priors_strings);

        let query_str = format!(
            r#"
                UPDATE fulfillments
                SET fulfillmentStatus = $1
                WHERE id = $2
                AND fulfillmentStatus IN ( {} );
            "#,
            allowed_priors_strings,
        );

        let result = query(&query_str)
            .bind(String::from(fulfillment_status))
            .bind(fulfillment_id.to_owned())
            .execute(&mut *conn)
            .await?;

        match result.rows_affected() {
            1 => Ok(()),
            0 => Err(super::Error::BadInput(
                "bad fulfillment status transition".to_string(),
            )),
            n => Err(super::Error::ProviderFailure(format!(
                "{} rows affected, expected 1 or 0",
                n
            ))),
        }
    }
}
