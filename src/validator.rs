//! 输入验证模块
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("端口号不能为空")]
    Empty,
    #[error("端口号必须是数字")]
    NotNumber,
    #[error("端口号必须在 1-65535 之间 (当前: {0})")]
    OutOfRange(u32),
    #[error("端口号 0 是无效的")]
    ZeroNotAllowed,
}

/// 验证端口字符串
pub fn validate_port_str(port_str: &str) -> Result<u16, ValidationError> {
    let trimmed = port_str.trim();

    // 检查是否为空
    if trimmed.is_empty() {
        return Err(ValidationError::Empty);
    }

    // 解析为数字
    let port_num: u32 = trimmed.parse().map_err(|_| ValidationError::NotNumber)?;

    // 检查范围
    if port_num == 0 {
        return Err(ValidationError::ZeroNotAllowed);
    }

    if port_num > 65535 {
        return Err(ValidationError::OutOfRange(port_num));
    }

    Ok(port_num as u16)
}

/// 验证并格式化URL
#[allow(dead_code)]
pub fn format_url(port: u16, use_https: bool) -> String {
    let protocol = if use_https { "https" } else { "http" };
    format!("{}://localhost:{}", protocol, port)
}

/// 检查端口是否被占用
#[cfg(not(target_os = "windows"))]
pub fn is_port_available(port: u16) -> bool {
    use std::net::TcpListener;
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

#[cfg(target_os = "windows")]
pub fn is_port_available(port: u16) -> bool {
    // Windows上需要特殊处理
    use std::net::TcpStream;
    TcpStream::connect(("127.0.0.1", port)).is_err()
}

/// 智能端口建议
#[allow(dead_code)]
pub fn suggest_port(start_port: u16) -> u16 {
    let end_port = start_port.saturating_add(100);
    for port in start_port..end_port {
        if is_port_available(port) {
            return port;
        }
    }
    start_port
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_port_str() {
        assert!(validate_port_str("").is_err());
        assert!(validate_port_str("abc").is_err());
        assert!(validate_port_str("0").is_err());
        assert!(validate_port_str("70000").is_err());
        assert_eq!(validate_port_str("8080").unwrap(), 8080);
        assert_eq!(validate_port_str(" 3000 ").unwrap(), 3000);
    }

    #[test]
    fn test_format_url() {
        assert_eq!(format_url(8080, false), "http://localhost:8080");
        assert_eq!(format_url(443, true), "https://localhost:443");
    }
}
