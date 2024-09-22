use axum::{
    extract::State,
    routing::{get, post},
    Router, Json,
};
use serde_json::Value;
use std::sync::Arc;
use crate::config::Config;
use crate::comfyui_client::ComfyUIClient;

pub struct AppState {
    config: Config,
    comfyui_client: ComfyUIClient,
}

pub async fn start(config: Config, comfyui_client: ComfyUIClient) {
    let state = Arc::new(AppState {
        config,
        comfyui_client,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/queue_prompt", post(queue_prompt))
        .route("/get_image", get(get_image))
        .route("/history", get(get_history))
        .with_state(state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "ComfyUI API Proxy"
}

async fn queue_prompt(
    State(state): State<Arc<AppState>>,
    Json(workflow): Json<Value>,
) -> Result<Json<Value>, String> {
    state.comfyui_client.queue_prompt(workflow)
        .await
        .map(|prompt_id| Json(serde_json::json!({"prompt_id": prompt_id})))
        .map_err(|e| e.to_string())
}

async fn get_image(
    State(state): State<Arc<AppState>>,
    filename: String,
) -> Result<Vec<u8>, String> {
    state.comfyui_client.get_image(&filename)
        .await
        .map_err(|e| e.to_string())
}

async fn get_history(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, String> {
    state.comfyui_client.get_history()
        .await
        .map(Json)
        .map_err(|e| e.to_string())
}