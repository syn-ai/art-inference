use reqwest::Client;
use serde_json::Value;
use std::error::Error;


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

    pub async fn queue_prompt(&self, prompt: Value) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/prompt", self.base_url);
        tracing::info!("Sending prompt to ComfyUI at URL: {}", url);
        tracing::debug!("Prompt payload: {:?}", prompt);

        let response = self.client.post(&url).json(&prompt).send().await?;
        
        if response.status().is_success() {
            let json = response.json().await?;
            tracing::info!("Successfully queued prompt. Response: {:?}", json);
            Ok(json)
        } else {
            let error_message = format!("Failed to queue prompt: {:?}", response.status());
            tracing::error!("{}", error_message);
            Err(error_message.into())
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