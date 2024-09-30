use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub providers: HashMap<String, ProviderConfig>,
    pub plugin_dir: Option<String>,
    pub tracked_accounts: Vec<String>,
    pub tracked_programs: Vec<ProgramConfig>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProgramConfig {
    pub address: String,
    pub idl_path: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProviderConfig {
    pub url: String,
    pub provider_type: String,
    pub priority: u8,
    // Add any other provider-specific configurations here
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}