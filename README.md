# 高性能端口浏览器

一个轻量的本地端口启动器，帮助你一键打开 `http://localhost:<port>`，并提供常用端口与最近记录。

## 功能

- 一键打开本地端口地址
- 端口输入校验
- 常用端口与最近端口列表
- 自动启动与启动后自动关闭
- 简洁的桌面 GUI

## 环境要求

- Rust 1.70+（建议最新稳定版）
- Windows / macOS / Linux

## 构建与运行

```bash
# 开发版
cargo build
cargo run

# 发布版
cargo build --release
```

发布版可执行文件位于：

```bash
./target/release/port-browser
```

Windows 下为 `port-browser.exe`。

## 配置

应用会在用户配置目录中保存配置（默认端口、窗口尺寸等）。

## 许可证

MIT
