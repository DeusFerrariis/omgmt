use axum::{routing::post, Router};
use provider::SqliteProvider;
use service::product::ProductService;

mod handle;
mod provider;
mod service;

#[tokio::main]
async fn main() {
    let mut sqlite_provider = provider::SqliteProvider::new_memory().await.unwrap();
    sqlite_provider.init_provider().await.unwrap();

    let app: Router<()> = Router::new()
        .route("/product", post(handle::create_product::<SqliteProvider>))
        .with_state(sqlite_provider);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
