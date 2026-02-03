use localhost_browser::{Browser, BrowserConfig, SecurityPolicy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let matches = clap::Command::new("localhost-browser")
        .version("1.0")
        .about("仅支持Localhost的高性能浏览器")
        .arg(
            clap::arg!([PORT] "Localhost端口")
                .default_value("3000")
                .validator(validate_port)
        )
        .arg(
            clap::arg!(-d --dev "启用开发者工具")
                .takes_value(false)
        )
        .get_matches();

    // 创建配置
    let port = matches.value_of("PORT").unwrap().parse::<u16>()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let config = BrowserConfig {
        port,
        enable_devtools: matches.is_present("dev"),
        window_title: format!("Localhost:{}", port),
        window_size: (1024, 768),
        security_policy: SecurityPolicy::default(),
        config_path: None,
    };

    // 启动浏览器
    let browser = Browser::new(config)?;
    browser.run().await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn validate_port(port: &str) -> Result<(), String> {
    match port.parse::<u16>() {
        Ok(p) if p > 0 => Ok(()),
        _ => Err("端口必须在1-65535之间".into()),
    }
}
