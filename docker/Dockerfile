# =============================================================================
# Plog CMS Docker Configuration
# 
# 纯 Rust 架构 - 多阶段构建
# =============================================================================

# -----------------------------------------------------------------------------
# 阶段 1: 构建阶段
# -----------------------------------------------------------------------------
FROM rust:1.75-alpine AS builder

WORKDIR /app

# 配置 Alpine 镜像源（加速）
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 安装构建依赖
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    mariadb-connector-c-dev

# 配置 Rust crates 镜像源（加速）
RUN mkdir -p /root/.cargo && \
    echo '[source.crates-io]' > /root/.cargo/config.toml && \
    echo 'replace-with = "ustc"' >> /root/.cargo/config.toml && \
    echo '[source.ustc]' >> /root/.cargo/config.toml && \
    echo 'registry = "https://mirrors.ustc.edu.cn/crates.io-index"' >> /root/.cargo/config.toml

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
# 阶段 2: 运行阶段
# -----------------------------------------------------------------------------
FROM alpine:3.19

WORKDIR /app

# 配置 Alpine 镜像源（加速）
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    mariadb-connector-c

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
