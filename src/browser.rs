use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    webview::WebViewBuilder,
};
use crate::{config::BrowserConfig, security::SecurityFilter};

pub struct Browser {
    event_loop: EventLoop<()>,
    window: Window,
    config: BrowserConfig,
}

impl Browser {
    pub fn new(config: BrowserConfig) -> Result<Self, wry::Error> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&config.window_title)
            .with_inner_size(wry::application::dpi::LogicalSize::new(
                config.window_size.0,
                config.window_size.1,
            ))
            .build(&event_loop)?;

        Ok(Self {
            event_loop,
            window,
            config,
        })
    }

    pub async fn run(self) -> Result<(), wry::Error> {
        let url = format!("http://localhost:{}", self.config.port);
        println!("🚀 启动浏览器，访问: {}", url);

        let security_filter = SecurityFilter::new(&self.config.security_policy);
        
        let _webview = WebViewBuilder::new(self.window)?
            .with_url(&url)?
            .with_ipc_handler(move |req: String| {
                if !security_filter.is_allowed(&req) {
                    println!("🚫 拦截请求: {}", req);
                    return "".to_string();
                }
                req
            })
            .with_initialization_script(&self.get_init_script())
            .with_devtools(self.config.enable_devtools)
            .with_transparent(false)
            .build()?;

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("👋 关闭浏览器");
                    *control_flow = ControlFlow::Exit;
                }
                Event::MainEventsCleared => {
                }
                _ => {}
            }
        });
    }

    fn get_init_script(&self) -> String {
        format!(
            r#"
            window.__LOCALHOST_BROWSER = {{
                version: "1.0",
                port: {},
                securityLevel: "strict"
            }};
            
            (function() {{
                const originalLocation = window.location;
                Object.defineProperty(window, 'location', {{
                    set: function(url) {{
                        if (!url.includes('localhost') && !url.includes('127.0.0.1')) {{
                            console.error('安全阻止: 不允许重定向到外部地址');
                            return;
                        }}
                        originalLocation.href = url;
                    }},
                    get: function() {{
                        return originalLocation;
                    }}
                }});
            }})();
            
            console.log('🔒 本地浏览器安全模式已启用');
            "#,
            self.config.port
        )
    }
}
