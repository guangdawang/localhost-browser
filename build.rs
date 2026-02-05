// build.rs
use slint_build::compile_with_config;
use slint_build::CompilerConfiguration;

fn main() {
    // 配置Slint编译器
    let config = CompilerConfiguration::new()
        .with_style("fluent".into()) // 使用Fluent风格
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer);

    // 编译UI文件
    compile_with_config("src/ui.slint", config).unwrap();

    // 复制资源文件
    println!("cargo:rerun-if-changed=src/ui.slint");
    println!("cargo:rerun-if-changed=assets/");

    // Windows特定设置
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // 设置Windows子系统
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        // 设置入口点（如果使用WinMain）
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    }
}
