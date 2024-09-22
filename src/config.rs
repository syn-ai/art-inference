use std::env;

pub struct Config {
    pub comfyui_url: String,
    pub static_drive_path: String,
}

impl Config {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(Config {
            comfyui_url: env::var("COMFYUI_URL")?,
            static_drive_path: env::var("STATIC_DRIVE_PATH")?,
        })
    }
}