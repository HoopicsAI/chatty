use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct ChattyServiceConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChattyConfig {
    pub chatty_service: ChattyServiceConfig,
}

lazy_static! {
    pub static ref CHATTY_CONFIG: ChattyConfig =
        load_config("config.toml").expect("Failed to load config");
}

// 从文件加载配置
fn load_config(filename: &str) -> Result<ChattyConfig, Box<dyn std::error::Error>> {
    // 获取项目根目录
    let mut path = PathBuf::from(std::env::current_dir()?);

    // 将配置文件名添加到路径中
    path.push(filename);

    let contents = fs::read_to_string(path)?;
    let config: ChattyConfig = toml::from_str(&contents)?;
    Ok(config)
}
