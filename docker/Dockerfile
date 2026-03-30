# =============================================================================
# Plog CMS Docker Configuration
# 
# 纯 Rust 架构 - 多阶段构建
# =============================================================================

# -----------------------------------------------------------------------------
# 阶段 1: 构建阶段 (使用 Debian 基础镜像)
# -----------------------------------------------------------------------------
FROM rust:latest AS builder

# 构建参数 - 代理设置
ARG HTTP_PROXY
ARG HTTPS_PROXY

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libmariadb-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件以利用 Docker 缓存
COPY plog-rs/Cargo.toml plog-rs/Cargo.lock ./
COPY plog-rs/crates ./crates
COPY plog-rs/migrations ./migrations

# 创建空的 src 目录并构建依赖（利用缓存）
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制实际源代码
COPY plog-rs/src ./src

# 构建应用
RUN cargo build --release

# -----------------------------------------------------------------------------
# 阶段 2: 运行阶段 (使用 Debian slim)
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libmariadb3 \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/plog-api /app/plog-api

# 复制配置文件
COPY plog-rs/config/ /app/config/

# 创建必要目录
RUN mkdir -p /app/content/uploads /app/content/plugins /app/content/themes /app/content/cache /app/content/logs

# 设置可执行权限
RUN chmod +x /app/plog-api

# 暴露端口
EXPOSE 8080

# 启动命令
CMD ["/app/plog-api"]
