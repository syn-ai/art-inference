use reqwest::Client;
use serde_json::Value;
use crate::error::{AppResult, AppError};

#[derive(Clone)]
pub struct ComfyUIClient {
    client: Client,
    base_url: String,
}

impl ComfyUIClient {
    pub fn new(base_url: String) -> Self {
        ComfyUIClient {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn queue_prompt(&self, prompt: Value) -> AppResult<Value> {
        let url = format!("{}/prompt", self.base_url);
        tracing::info!("Sending prompt to ComfyUI at URL: {}", url);
        tracing::debug!("Prompt payload: {:?}", prompt);
    
        let response = self.client.post(&url)
            .json(&prompt)
            .send()
            .await
            .map_err(AppError::HttpClient)?;
    
        if response.status().is_success() {
            let json = response.json().await.map_err(AppError::HttpClient)?;
            tracing::info!("Successfully queued prompt. Response: {:?}", json);
            Ok(json)
        } else {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error body".to_string());
            let error_message = format!("Failed to queue prompt. Status: {}, Body: {}", status, error_body);
            tracing::error!("{}", error_message);
            Err(AppError::ComfyUI(error_message))
        }
    }

    pub async fn get_image(&self, filename: &str) -> AppResult<Vec<u8>> {
        let url = format!("{}/view", self.base_url);
        let response = self.client.get(&url)
            .query(&[("filename", filename)])
            .send()
            .await
            .map_err(AppError::HttpClient)?;

        if response.status().is_success() {
            response.bytes().await.map(|b| b.to_vec()).map_err(AppError::HttpClient)
        } else {
            Err(AppError::ComfyUI(format!("Failed to get image: {:?}", response.status())))
        }
    }

    pub async fn get_history(&self) -> AppResult<Value> {
        let url = format!("{}/history", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(AppError::HttpClient)?;

        if response.status().is_success() {
            response.json().await.map_err(AppError::HttpClient)
        } else {
            Err(AppError::ComfyUI(format!("Failed to get history: {:?}", response.status())))
        }
    }

    // Add more methods for other ComfyUI API endpoints here
}