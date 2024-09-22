use reqwest::Client;
use serde_json::Value;
use std::error::Error;

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

    pub async fn queue_prompt(&self, workflow: Value) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/prompt", self.base_url);
        let response = self.client.post(&url).json(&workflow).send().await?;
        
        if response.status().is_success() {
            let json: Value = response.json().await?;
            Ok(json["prompt_id"].as_str().unwrap_or("").to_string())
        } else {
            Err(format!("Failed to queue prompt: {:?}", response.status()).into())
        }
    }

    pub async fn get_image(&self, filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let url = format!("{}/view", self.base_url);
        let response = self.client.get(&url).query(&[("filename", filename)]).send().await?;
        
        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            Err(format!("Failed to get image: {:?}", response.status()).into())
        }
    }

    pub async fn get_history(&self) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/history", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(format!("Failed to get history: {:?}", response.status()).into())
        }
    }
}