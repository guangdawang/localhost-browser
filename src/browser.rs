//! 浏览器操作模块
use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("无效的端口号: {0}")]
    InvalidPort(String),
    #[error("浏览器启动失败: {0}")]
    LaunchFailed(String),
    #[allow(dead_code)]
    #[error("系统不支持")]
    UnsupportedSystem,
    #[allow(dead_code)]
    #[error("未知错误")]
    Unknown,
}

impl From<Box<dyn std::error::Error + Send + Sync>> for BrowserError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        BrowserError::LaunchFailed(e.to_string())
    }
}

/// 启动浏览器访问指定端口
pub fn launch_with_port(port: u16) -> Result<String, BrowserError> {
    // 验证端口
    if port == 0 {
        return Err(BrowserError::InvalidPort(format!("端口 {} 无效", port)));
    }

    // 构建URL
    let url = format!("http://localhost:{}", port);

    // 记录日志
    info!("尝试打开浏览器: {}", url);

    // 打开浏览器
    webbrowser::open(&url)
        .map(|_| {
            info!("浏览器启动成功: {}", url);
            url
        })
        .map_err(|e| BrowserError::LaunchFailed(e.to_string()))
}

/// 使用HTTPS启动
#[allow(dead_code)]
pub fn launch_with_https(port: u16) -> Result<String, BrowserError> {
    let url = format!("https://localhost:{}", port);
    info!("尝试打开浏览器(HTTPS): {}", url);

    webbrowser::open(&url)
        .map(|_| {
            info!("浏览器启动成功: {}", url);
            url
        })
        .map_err(|e| BrowserError::LaunchFailed(e.to_string()))
}

/// 批量打开多个端口
#[allow(dead_code)]
pub fn launch_multiple_ports(ports: &[u16]) -> Vec<Result<String, BrowserError>> {
    ports.iter().map(|&port| launch_with_port(port)).collect()
}

/// 使用特定浏览器打开
#[allow(dead_code)]
pub fn launch_with_browser(
    port: u16,
    browser: webbrowser::Browser,
) -> Result<String, BrowserError> {
    let url = format!("http://localhost:{}", port);
    info!("使用特定浏览器打开: {} -> {:?}", url, browser);

    webbrowser::open_browser(browser, &url)
        .map(|_| {
            info!("特定浏览器启动成功: {}", url);
            url
        })
        .map_err(|e| BrowserError::LaunchFailed(e.to_string()))
}

/// 检查默认浏览器
#[allow(dead_code)]
pub fn get_default_browser() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\Shell\Associations\UrlAssociations\http\UserChoice";

        if let Ok(key) = hkcu.open_subkey(path) {
            if let Ok(prog_id) = key.get_value::<String, _>("ProgId") {
                return Some(prog_id);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("defaults")
            .args(&[
                "read",
                "com.apple.LaunchServices/com.apple.launchservices.secure",
                "LSHandlers",
            ])
            .output();

        if let Ok(output) = output {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return Some(stdout);
            }
        }
    }

    None
}
