use axum::{
    extract::{Query, State},
    routing::{get, post},
    Router, Json,
};
use serde_json::Value;
use std::sync::Arc;
use crate::{config::Config, prompt_constructor};
use crate::comfyui_client::ComfyUIClient;
use crate::prompt_constructor::PromptConstructor;
use crate::workflow_manager::WorkflowManager;

pub struct AppState {
    config: Config,
    comfyui_client: ComfyUIClient,
    prompt_constructor: PromptConstructor,
    workflow_manager: WorkflowManager,
}

pub async fn start(config: Config, comfyui_client: ComfyUIClient) {
    let state = Arc::new(AppState {
        config,
        comfyui_client,
        prompt_constructor: PromptConstructor::new(),
        workflow_manager: WorkflowManager::new(),
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/queue_prompt", post(queue_prompt))
        .route("/get_image", get(get_image))
        .route("/history", get(get_history))
        .route("/add_workflow", post(add_workflow))
        .route("/get_node_info", get(get_node_info))
        .route("/construct_prompt", post(construct_prompt))
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
        .map(Json)
        .map_err(|e| e.to_string())
}

async fn get_image(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Vec<u8>, String> {
    let filename = params.get("filename").ok_or("Filename is required")?;
    state.comfyui_client.get_image(filename)
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

async fn add_workflow(
    State(state): State<Arc<AppState>>,
    Json(workflow): Json<Value>,
) -> Result<Json<Value>, String> {
    let state = state.clone();
    let mut workflow_manager = state.workflow_manager.clone();
    workflow_manager.add_workflow(workflow)
        .map(|_| Json(serde_json::json!({"status": "success"})))
        .map_err(|e| e.to_string())
}

async fn get_node_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Value>, String> {
    let node_type = params.get("node_type").ok_or("Node type is required")?;
    state.workflow_manager.get_node_info(node_type)
        .map(Json)
        .ok_or_else(|| "Node type not found".to_string())
}

async fn construct_prompt(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, String> {
    let template = payload.get("template").ok_or("Template is required")?;
    let inputs = payload.get("inputs").ok_or("Inputs are required")?;
    let state = state.clone();
    let mut prompt_constructor = state.prompt_constructor.clone();
    println!("Constructing prompt with template: {}", template);
    println!("Inputs: {}", inputs);
    prompt_constructor.construct_prompt(template, inputs)
        .map(Json)
        .map_err(|e| e.to_string())
}