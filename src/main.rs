mod api_server;
mod config;
mod comfyui_client;

use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "comfyui_api_proxy=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::new().expect("Failed to load configuration");

    // Create ComfyUI client
    let comfyui_client = comfyui_client::ComfyUIClient::new(config.comfyui_url.clone());

    // Start the API server
    api_server::start(config, comfyui_client).await;
}