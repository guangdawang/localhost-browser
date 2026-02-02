use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::security::SecurityPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserConfig {
    pub port: u16,
    pub enable_devtools: bool,
    pub window_title: String,
    pub window_size: (u32, u32),
    pub security_policy: SecurityPolicy,
    #[serde(skip)]
    pub config_path: Option<PathBuf>,
}

impl BrowserConfig {
    pub fn from_file(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&path)?;
        let mut config: Self = serde_json::from_str(&content)?;
        config.config_path = Some(path);
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.config_path {
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    pub fn get_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            enable_devtools: false,
            window_title: "Localhost Browser".to_string(),
            window_size: (1024, 768),
            security_policy: SecurityPolicy::default(),
            config_path: None,
        }
    }
}
