# =============================================================================
# Plog CMS Docker Configuration
# 
# 纯 Rust 架构 - 使用预构建二进制
# =============================================================================

# 使用 Alpine 作为基础镜像（更小更快）
FROM alpine:3.19

WORKDIR /app

# 安装运行时依赖
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# 复制预构建的 Linux 二进制文件
COPY plog-rs/target/release/plog-api.exe /app/plog-api

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
