# =============================================================================
# Plog CMS Docker Configuration
# 
# 纯 Rust 架构 - 多阶段构建
# =============================================================================

# Build stage
FROM rust:1.77 as builder

WORKDIR /app

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制源代码
COPY plog-rs/ .

# 构建
RUN cargo build --release --bin plog-api

# Runtime stage - 使用 Debian slim
FROM debian:bookworm-slim

WORKDIR /app

# 复制构建产物
COPY --from=builder /app/target/release/plog-api /app/plog-api

# 复制配置文件
COPY plog-rs/config/ /app/config/

# 创建必要目录
RUN mkdir -p /app/content/{uploads,plugins,themes,cache,logs}

# 设置可执行权限
RUN chmod +x /app/plog-api

# 暴露端口
EXPOSE 8080

# 启动命令
CMD ["/app/plog-api"]
