use api_server::start;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod api;
mod api_server;
mod comfyui_client;
mod config;
mod prompt_constructor;
mod static_drive_poller;
mod workflow_manager;

use crate::comfyui_client::ComfyUIClient;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::Config::new().expect("Failed to load configuration");
    config::Config::dotenv_load();
    config::Config::print_env_vars();
    // Create ComfyUI client
    let comfyui_client = comfyui_client::ComfyUIClient::new(config.comfyui_url.clone());

    // Build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "ComfyUI API Proxy" }))
        .route("/queue_prompt", post(api::queue_prompt))
        .route("/get_image", get(api::get_image))
        .route("/get_history", get(api::get_history))
        .layer(CorsLayer::permissive())
        .with_state(comfyui_client);

    // Run our application
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    let comfyui_client = ComfyUIClient::new(config.comfyui_url.clone());

    start(config, comfyui_client).await

}