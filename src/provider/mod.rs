mod sqlite;

pub use sqlite::SqliteProvider;

pub trait Provider {
    type Error;
}
