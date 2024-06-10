use core::fmt;

pub mod product;

#[derive(Debug)]
pub enum Error {
    BadInput(String),
    ProductNotFound(String),
    ProviderError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::BadInput(s) => format!("Bad input: {}", s),
            Self::ProductNotFound(s) => format!("Product not found: {}", s),
            Self::ProviderError(s) => format!("Provider error: {}", s),
        };

        write!(f, "{}", message)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::BadInput("sql row not found".to_string()),
            v => Self::ProviderError(format!("{}", v)),
        }
    }
}
