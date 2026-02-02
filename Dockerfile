# 构建阶段
FROM rust:1.75-slim AS builder

WORKDIR /app

# 安装必要的构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

# 复制Cargo配置
COPY Cargo.toml ./
COPY src ./src
COPY build.rs ./

# 构建项目
RUN cargo build --release

# 运行阶段（Linux 运行时，匹配 WSL/Compose）
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# 运行时依赖（wry / webkit2gtk）
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-37 \
    libgtk-3-0 \
    libglib2.0-0 \
    libgdk-pixbuf2.0-0 \
    libpango-1.0-0 \
    libcairo2 \
    libatk1.0-0 \
    libx11-6 \
    libxext6 \
    libxrender1 \
    libxrandr2 \
    libxi6 \
    && rm -rf /var/lib/apt/lists/*

# 复制构建的二进制文件
COPY --from=builder /app/target/release/localhost-browser ./

# 设置入口点
ENTRYPOINT ["./localhost-browser"]
