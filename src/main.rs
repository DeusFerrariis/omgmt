use axum::{
    routing::{get, post, put},
    Router,
};
use log::info;
use provider::SqliteProvider;
use service::{
    fulfillment::FulfillmentService, line_item::LineItemService, product::ProductService,
};

mod handle;
mod model;
mod provider;
mod service;

use handle::{fulfillment, line_item, product};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut sqlite_provider = provider::SqliteProvider::new_memory().await.unwrap();

    // Init provider for each backend

    FulfillmentService::init_provider(&mut sqlite_provider)
        .await
        .unwrap();
    ProductService::init_provider(&mut sqlite_provider)
        .await
        .unwrap();
    LineItemService::init_provider(&mut sqlite_provider)
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
        .route(
            "/lineItem",
            post(line_item::LineItemHandler::create_line_item::<SqliteProvider>),
        )
        .with_state(sqlite_provider)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on :3000...error");
    axum::serve(listener, app).await.unwrap();
}
