//! 高性能端口浏览器启动器
//! 允许用户输入端口号，默认使用localhost

mod browser;
mod config;
mod validator;

use log::{error, info};
use parking_lot::RwLock;
use slint::SharedString;
use std::sync::Arc;

// 导入Slint生成的UI代码
slint::include_modules!();

/// 应用状态
struct AppState {
    recent_ports: Vec<u16>,
    config: Arc<RwLock<config::AppConfig>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            recent_ports: Vec::new(),
            config: Arc::new(RwLock::new(config::AppConfig::load_default())),
        }
    }

    fn add_recent_port(&mut self, port: u16) {
        // 移除已存在的端口
        self.recent_ports.retain(|&p| p != port);
        // 添加到开头
        self.recent_ports.insert(0, port);
        // 限制历史记录数量
        if self.recent_ports.len() > 10 {
            self.recent_ports.truncate(10);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    simple_logger::init_with_level(log::Level::Info)?;
    info!("启动端口浏览器应用");

    // 加载配置
    let config = config::AppConfig::load();
    info!("加载配置: {:?}", config);

    // 创建UI
    let ui = MainWindow::new()?;

    // 设置窗口属性
    ui.set_window_title("高性能端口浏览器".into());
    ui.set_window_width(config.window_width as i32);
    ui.set_window_height(config.window_height as i32);

    // 设置默认值
    ui.set_default_port(config.default_port as i32);
    ui.set_use_default_port(config.use_default_port);
    ui.set_auto_launch(config.auto_launch);
    ui.set_auto_close(config.auto_close);

    // 初始化状态
    let app_state = Arc::new(RwLock::new(AppState::new()));

    // ========== 1. 端口输入验证 ==========
    {
        let ui_weak = ui.as_weak();
        ui.on_validate_port_input(move |input: SharedString| {
            info!("验证端口输入: {}", input);

            let validation_result = validator::validate_port_str(&input);
            let is_valid = validation_result.is_ok();
            let message = match validation_result {
                Ok(_) => String::new(),
                Err(e) => e.to_string(),
            };

            // 更新UI
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_port_validation_message(message.into());
                ui.set_is_port_valid(is_valid);
            }
        });
    }

    // ========== 2. 启动浏览器 ==========
    {
        let ui_weak = ui.as_weak();
        let state_clone = app_state.clone();
        ui.on_launch_browser(
            move |port_input: SharedString, use_default: bool, auto_close: bool| {
                info!(
                    "启动浏览器请求: port={}, use_default={}",
                    port_input, use_default
                );

                let state = state_clone.clone();
                let ui_weak = ui_weak.clone();

                // 在后台线程执行，避免阻塞UI
                std::thread::spawn(move || {
                    let result = if use_default {
                        // 使用默认端口
                        let default_port = {
                            let state = state.read();
                            let config = state.config.read();
                            config.default_port
                        };
                        browser::launch_with_port(default_port)
                    } else {
                        // 验证用户输入的端口
                        match validator::validate_port_str(&port_input) {
                            Ok(port) => {
                                // 添加到历史记录
                                {
                                    let mut state = state.write();
                                    state.add_recent_port(port);
                                }
                                browser::launch_with_port(port)
                            }
                            Err(e) => Err(browser::BrowserError::InvalidPort(e.to_string())),
                        }
                    };

                    // 更新UI状态
                    slint::invoke_from_event_loop(move || {
                        if let Some(ui) = ui_weak.upgrade() {
                            match result {
                                Ok(url) => {
                                    ui.set_status_message(format!("✅ 成功打开: {}", url).into());
                                    info!("浏览器已启动: {}", url);

                                    // 如果启用自动关闭，延迟关闭窗口
                                    if auto_close {
                                        let ui_weak2 = ui.as_weak();
                                        std::thread::spawn(move || {
                                            std::thread::sleep(std::time::Duration::from_secs(1));
                                            slint::invoke_from_event_loop(move || {
                                                if let Some(ui) = ui_weak2.upgrade() {
                                                    let _ = ui.hide();
                                                }
                                            })
                                            .ok();
                                        });
                                    }
                                }
                                Err(e) => {
                                    let error_msg = format!("❌ 打开失败: {}", e);
                                    ui.set_status_message(error_msg.into());
                                    error!("浏览器启动失败: {}", e);
                                }
                            }
                        }
                    })
                    .unwrap_or_else(|e| error!("更新UI失败: {}", e));
                });
            },
        );
    }

    // ========== 3. 快速端口按钮 ==========
    {
        let ui_weak = ui.as_weak();
        let state_clone = app_state.clone();
        ui.on_quick_port_selected(move |port_str: SharedString| {
            info!("快速选择端口: {}", port_str);

            let ui_weak = ui_weak.clone();
            let state = state_clone.clone();

            let port = match port_str.parse::<u16>() {
                Ok(p) => p,
                Err(_) => return,
            };

            std::thread::spawn(move || {
                match browser::launch_with_port(port) {
                    Ok(url) => {
                        // 添加到历史记录
                        {
                            let mut state = state.write();
                            state.add_recent_port(port);
                        }

                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_weak.upgrade() {
                                ui.set_status_message(format!("✅ 快速打开: {}", url).into());
                            }
                        })
                        .ok();
                    }
                    Err(e) => {
                        slint::invoke_from_event_loop(move || {
                            if let Some(ui) = ui_weak.upgrade() {
                                ui.set_status_message(format!("❌ 快速打开失败: {}", e).into());
                            }
                        })
                        .ok();
                    }
                }
            });
        });
    }

    // ========== 4. 保存配置 ==========
    {
        let state_clone = app_state.clone();
        ui.on_save_config(move || {
            info!("保存配置");

            let state = state_clone.read();
            let config = state.config.read().clone();
            drop(state);

            if let Err(e) = config.save() {
                error!("保存配置失败: {}", e);
            }
        });
    }

    // ========== 5. 窗口事件处理 ==========
    {
        let state_clone = app_state.clone();
        ui.on_window_resized(move |width, height| {
            let state = state_clone.write();
            let mut config = state.config.write();
            config.window_width = width as u32;
            config.window_height = height as u32;
        });
    }

    // ========== 6. 加载历史记录 ==========
    let recent_ports = config.quick_ports();
    let recent_ports: Vec<SharedString> = recent_ports
        .into_iter()
        .map(|p| SharedString::from(p.to_string()))
        .collect();
    ui.set_recent_ports(recent_ports.as_slice().into());

    // 显示窗口
    ui.show()?;

    // 运行事件循环
    info!("进入主事件循环");
    ui.run()?;

    info!("应用退出");
    Ok(())
}
