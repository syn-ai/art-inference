use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use comfyui_api_proxy::{
    api::routes,
    config::Config,
    comfyui::client::ComfyUIClient,
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn test_root_endpoint() {
    let config = Config::new().expect("Failed to load configuration");
    let comfyui_client = ComfyUIClient::new(config.comfyui_url.clone());
    let app = routes::setup_routes(comfyui_client).await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"ComfyUI API Proxy");
}

#[tokio::test]
async fn test_queue_prompt() {
    let config = Config::new().expect("Failed to load configuration");
    let comfyui_client = ComfyUIClient::new(config.comfyui_url.clone());
    let app = setup_routes(config, comfyui_client);

    let test_prompt = json!({
        "prompt": "test prompt",
        "workflow": {}
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/queue_prompt")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&test_prompt).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    // You might want to add more assertions here based on the expected response
}