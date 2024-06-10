use axum::{
    routing::{get, post, put},
    Router,
};
use provider::SqliteProvider;
use service::{fulfillment::FulfillmentService, product::ProductService};

mod handle;
mod model;
mod provider;
mod service;

use handle::{fulfillment, product};

#[tokio::main]
async fn main() {
    let mut sqlite_provider = provider::SqliteProvider::new_memory().await.unwrap();

    // Init provider for each backend

    FulfillmentService::init_provider(&mut sqlite_provider)
        .await
        .unwrap();
    ProductService::init_provider(&mut sqlite_provider)
        .await
        .unwrap();

    let app: Router<()> = Router::new()
        .route(
            "/product",
            post(product::ProductHandler::create_product::<SqliteProvider>),
        )
        .route(
            "/product/:product_id",
            get(product::ProductHandler::get_product::<SqliteProvider>),
        )
        .route(
            "/fulfillment",
            post(fulfillment::FulfillmentHandler::create_fulfillment::<SqliteProvider>),
        )
        .route(
            "/fulfillment/:fulfillment_id/status",
            put(fulfillment::FulfillmentHandler::update_fulfillment_status::<SqliteProvider>),
        )
        .with_state(sqlite_provider);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
