use axum::Router;
use dotenv::dotenv;
use envy::from_env;
use std::io;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use backend::db::Mongo;
use backend::routes::API;
use backend::ENV;

#[tokio::main]
async fn main() -> io::Result<()> {
    // env
    dotenv().ok();
    let _envs: ENV = from_env().unwrap();

    // tracing and logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // middlewares / layers
    let cors = CorsLayer::new().allow_origin(Any);
    let compression = CompressionLayer::new().gzip(true);
    let trace = TraceLayer::new_for_http();

    let middlewares = ServiceBuilder::new()
        .layer(compression)
        .layer(trace)
        .layer(cors);

    // DB connection
    Mongo::establish_conn().await;

    let app = Router::new()
        .nest("/api", API::generate_route())
        .layer(middlewares);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
