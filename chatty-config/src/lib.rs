use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct ChattyServiceConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FictionXServiceConfig {
    pub base_endpoint: String,
    pub path_recommend: String,
    pub path_login: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChattyConfig {
    pub chatty_service: ChattyServiceConfig,
    pub fictionx_service: FictionXServiceConfig,
}

lazy_static! {
    pub static ref CHATTY_CONFIG: ChattyConfig =
        load_config("config.toml").expect("Failed to load config");
}

fn load_config(filename: &str) -> Result<ChattyConfig, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(std::env::current_dir()?);
    path.push(filename);

    let contents = fs::read_to_string(path)?;
    let config: ChattyConfig = toml::from_str(&contents)?;
    Ok(config)
}
