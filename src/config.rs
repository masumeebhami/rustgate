use config::{Config, File};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub services: HashMap<String, String>,
}

pub fn load() -> Settings {
    Config::builder()
        .add_source(File::with_name("config/config.toml").required(true))
        .build()
        .expect("Failed to build configuration")
        .try_deserialize()
        .expect("Failed to deserialize configuration")
}