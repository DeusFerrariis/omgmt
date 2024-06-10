use core::fmt;

use axum::http::StatusCode;

pub mod fulfillment;
pub mod line_item;
pub mod order;
pub mod product;

#[derive(Debug)]
pub enum Error {
    BadInput(String),
    ProductNotFound(String),
    ProviderFailure(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::BadInput(s) => format!("Bad input - {}", s),
            Self::ProductNotFound(s) => format!("Product not found - {}", s),
            Self::ProviderFailure(s) => format!("Provider error - {}", s),
        };

        write!(f, "{}", message)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::BadInput("sql row not found".to_string()),
            v => Self::ProviderFailure(format!("{}", v)),
        }
    }
}

impl Into<StatusCode> for Error {
    fn into(self) -> StatusCode {
        match self {
            Error::BadInput(_) => StatusCode::BAD_REQUEST,
            Error::ProductNotFound(_) => StatusCode::NOT_FOUND,
            Error::ProviderFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// TODO: set up error for (StatusCode, String or Json(ErrorMessage))
