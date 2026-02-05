//! 应用配置管理
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON解析错误: {0}")]
    Json(#[from] serde_json::Error),
    #[error("配置文件路径错误")]
    PathError,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 默认端口号
    pub default_port: u16,
    /// 是否使用默认端口
    pub use_default_port: bool,
    /// 自动启动浏览器
    pub auto_launch: bool,
    /// 自动关闭窗口
    pub auto_close: bool,
    /// 窗口宽度
    pub window_width: u32,
    /// 窗口高度
    pub window_height: u32,
    /// 最近使用的端口
    pub recent_ports: Vec<u16>,
    /// 主题设置
    pub theme: Theme,
}

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_port: 8080,
            use_default_port: true,
            auto_launch: false,
            auto_close: false,
            window_width: 400,
            window_height: 500,
            recent_ports: vec![3000, 8080, 5173, 3001, 4200],
            theme: Theme::System,
        }
    }
}

impl AppConfig {
    /// 获取配置文件路径
    fn config_path() -> Result<PathBuf, ConfigError> {
        let mut path = dirs::config_dir().ok_or(ConfigError::PathError)?;
        path.push("port-browser");
        fs::create_dir_all(&path)?;
        path.push("config.json");
        Ok(path)
    }

    /// 加载配置
    pub fn load() -> Self {
        match Self::config_path() {
            Ok(path) if path.exists() => match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(config) => {
                        log::info!("从 {} 加载配置", path.display());
                        config
                    }
                    Err(e) => {
                        log::warn!("配置文件损坏: {}, 使用默认配置", e);
                        Self::default()
                    }
                },
                Err(e) => {
                    log::warn!("读取配置文件失败: {}, 使用默认配置", e);
                    Self::default()
                }
            },
            _ => {
                log::info!("配置文件不存在，使用默认配置");
                Self::default()
            }
        }
    }

    /// 保存配置
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        log::info!("配置已保存");
        Ok(())
    }

    /// 加载默认配置（不读取文件）
    pub fn load_default() -> Self {
        Self::default()
    }

    /// 添加快捷端口
    #[allow(dead_code)]
    pub fn add_quick_port(&mut self, port: u16) {
        if !self.recent_ports.contains(&port) {
            self.recent_ports.push(port);
            // 限制数量
            if self.recent_ports.len() > 10 {
                self.recent_ports.remove(0);
            }
        }
    }

    /// 获取快捷端口列表
    pub fn quick_ports(&self) -> Vec<u16> {
        let mut ports = self.recent_ports.clone();
        // 添加常见开发端口
        let common_ports = [3000, 8080, 5173, 3001, 4200, 8000, 9000, 5500];
        for port in common_ports {
            if !ports.contains(&port) && ports.len() < 15 {
                ports.push(port);
            }
        }
        ports.into_iter().take(15).collect()
    }
}
