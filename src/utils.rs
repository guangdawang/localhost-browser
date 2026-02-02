use crate::BrowserConfig;

pub async fn check_port_available(port: u16) -> bool {
    use tokio::net::TcpListener;
    
    match TcpListener::bind(("127.0.0.1", port)).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn open_in_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }
    
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
    
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(url)
            .spawn();
    }
}

pub fn generate_example_config() -> String {
    let config = BrowserConfig::default();
    serde_json::to_string_pretty(&config).unwrap()
}

#[derive(Debug)]
pub struct PerfTimer {
    start: std::time::Instant,
    label: String,
}

impl PerfTimer {
    pub fn new(label: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            label: label.to_string(),
        }
    }
    
    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        println!("⏱️  {}: {:.2?}", self.label, elapsed);
    }
}
