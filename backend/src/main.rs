mod analytics;
mod api;
mod bitcoin;
mod blocks;
mod config;
mod database;
mod events;
mod graph;
mod mempool;
mod rpc;
mod scripts;
mod transactions;
mod utxos;

use config::AppConfig;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,bitcoin_utxo_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Bitcoin UTXO Visualizer backend engine...");

    // 2. Load configuration from environment / .env
    let config = AppConfig::from_env();
    tracing::info!(
        "Loaded configuration for network: {}",
        config.bitcoin_network
    );

    // 3. Connect to PostgreSQL and run migrations
    tracing::info!(
        "Connecting to PostgreSQL database at {}...",
        config.database_url
    );
    let pool = database::create_pool(&config.database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to PostgreSQL: {}", e);
            e
        })?;

    tracing::info!("Running database schema migrations...");
    database::run_migrations(&pool).await.map_err(|e| {
        tracing::error!("Database migration failed: {}", e);
        e
    })?;
    tracing::info!("Database schema up-to-date.");

    // 4. Configure CORS layer for frontend communication
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 5. Construct API router with middleware
    let app = api::create_router(pool)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // 6. Bind and serve HTTP server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("Bitcoin UTXO Visualizer API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
