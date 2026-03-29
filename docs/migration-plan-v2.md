# Plog CMS 全量 Rust 替换 - 任务规划

> 基于 `docs/full-rust-replacement-blueprint-v2.md` 制定

## 当前状态

已完成：
- Phase 1-4: Rust 微服务 + PHP 兼容层 + 前端迁移

待完成：
- Phase 0-7: 完全替换 PHP，实现纯 Rust 架构

---

## Phase 0: 冻结协议与清点资产

**目标**: 完整输出 contract 规范，清点 PHP 遗留点

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 创建 contracts crate | 定义 API 请求/响应格式 | P0 |
| 定义权限模型 | RBAC 权限系统规范 | P0 |
| 定义插件 manifest | plugin.toml 规范 | P0 |
| 定义主题 manifest | theme.toml 规范 | P0 |
| 清点 /admin 页面 | 列出所有需迁移页面 | P1 |
| 清点 PHP 插件 | 分析现有插件类型 | P1 |
| 清点 PHP 主题 | 分析现有主题结构 | P1 |
| 冻结 PHP 新功能 | 禁止新增 PHP 逻辑 | P0 |

### 产出

- `crates/contracts/` - API 合约定义
- `docs/permissions.md` - 权限模型文档
- `docs/plugin-manifest.md` - 插件 manifest 规范
- `docs/theme-manifest.md` - 主题 manifest 规范
- `docs/admin-migration-list.md` - 页面迁移清单

---

## Phase 1: Rust 成为唯一业务核心

**目标**: 权限、内容、设置、媒体、审计全面收口到 Rust

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 实现 settings crate | 系统设置管理 | P0 |
| 实现 media crate | 媒体文件管理 | P0 |
| 实现 audit crate | 审计日志系统 | P1 |
| 实现 RBAC 权限 | 角色权限系统 | P0 |
| 完善 auth crate | 用户管理增强 | P1 |

### 产出

- `crates/settings/` - 设置管理
- `crates/media/` - 媒体管理
- `crates/audit/` - 审计日志

---

## Phase 2: 统一 API

**目标**: 停止双栈 API，将 `/api/*` 与 `/api/v2/*` 统一为 Rust API

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 统一 API 路径 | 移除 v2 前缀 | P0 |
| 更新 Nginx 配置 | 删除 PHP upstream | P0 |
| 验证所有端点 | 确保功能完整 | P0 |

### 产出

- 统一的 `/api/*` 路径
- 更新的 Nginx 配置
- 删除 PHP API 入口

---

## Phase 3: 迁空 `/admin`

**目标**: 所有后台页面迁入 admin-web

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 系统设置页面 | admin-web 设置管理 | P0 |
| 主题管理页面 | admin-web 主题管理 | P0 |
| 插件管理页面 | admin-web 插件管理 | P0 |
| 审计日志页面 | admin-web 日志查看 | P1 |
| 任务管理页面 | admin-web 任务调度 | P1 |
| 媒体管理页面 | admin-web 媒体库 | P1 |

### 产出

- admin-web 完整功能覆盖
- `/admin` 302 重定向到 admin-web
- 删除 PHP 控制器

---

## Phase 4: 重建插件 Runtime

**目标**: Rust 接管插件发现、注册、配置、安装、升级、执行

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 实现 plugin-runtime crate | 插件运行时 | P0 |
| 定义插件 manifest 解析 | TOML 解析 | P0 |
| 实现插件发现 | 扫描 content/plugins | P0 |
| 实现插件注册 | 菜单/页面/事件 | P0 |
| 实现插件安装/升级 | 包管理 | P1 |
| 实现插件配置 | 设置管理 | P1 |

### 产出

- `crates/plugin-runtime/` - 插件运行时
- 插件 manifest 规范
- 插件 API 接口

---

## Phase 5: 重建主题 Runtime

**目标**: Rust/前端接管主题发现、配置、预览、渲染

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 实现 theme-runtime crate | 主题运行时 | P0 |
| 定义主题 manifest 解析 | TOML 解析 | P0 |
| 实现主题发现 | 扫描 content/themes | P0 |
| 实现主题配置 | 设置管理 | P1 |
| 实现主题预览 | 预览功能 | P1 |
| 实现 SSR 渲染 | 服务端渲染 | P2 |

### 产出

- `crates/theme-runtime/` - 主题运行时
- 主题 manifest 规范
- 主题 API 接口

---

## Phase 6: Rust 化运维链路

**目标**: CLI、定时任务、升级脚本、缓存刷新全部 Rust 化

### 任务清单

| 任务 | 说明 | 优先级 |
|------|------|--------|
| 实现 installer-rs | 安装器 | P0 |
| 实现 worker-rs | 队列 Worker | P1 |
| 实现 scheduler-rs | 定时任务 | P1 |
| 实现 CLI 工具 | 命令行工具 | P1 |

### 产出

- `apps/installer-rs/` - 安装器
- `apps/worker-rs/` - Worker
- `apps/scheduler-rs/` - 调度器
- `apps/cli-rs/` - CLI 工具

---

## Phase 7: 删除 PHP

**目标**: 彻底移除 PHP 运行时依赖

### 删除对象

- `compat/`
- `include/`
- `admin/`
- PHP-FPM
- PHP 相关 Nginx 配置

### 验收

- 生产环境无 PHP 运行时仍能完整启动
- 系统功能与数据链路完整可用

---

## 目录重组

最终目录结构：

```
apps/
  admin-web/          # 管理后台 (Vue 3)
  api-gateway-rs/     # API 网关
  worker-rs/          # 队列 Worker
  scheduler-rs/       # 定时任务
  installer-rs/       # 安装器

crates/
  core/               # 核心类型
  contracts/          # API 合约
  auth/               # 认证
  content/            # 内容管理
  settings/           # 设置管理
  media/              # 媒体管理
  plugin-runtime/     # 插件运行时
  theme-runtime/      # 主题运行时
  tasking/            # 任务调度
  audit/              # 审计日志
  search/             # 搜索

content/
  uploads/            # 上传文件
  plugins/            # 插件包
  themes/             # 主题包
  languages/          # 语言包
  cache/              # 缓存

infra/
  nginx/              # Nginx 配置
  docker/             # Docker 配置
  scripts/            # 脚本

legacy/               # 待删除
  php-compat/
  php-admin/
  php-include/
```

---

## 执行顺序

```
Phase 0: 冻结协议 ──────────────────────────────────────┐
                                                         │
Phase 1: Rust 核心 ─────────────────────────────────────┤
                                                         │
Phase 2: 统一 API ──────────────────────────────────────┤
                                                         │
Phase 3: 迁空 /admin ──────────────────────────────────┤
                                                         │
Phase 4: 插件 Runtime ─────────────────────────────────┤
                                                         │
Phase 5: 主题 Runtime ─────────────────────────────────┤
                                                         │
Phase 6: 运维链路 ──────────────────────────────────────┤
                                                         │
Phase 7: 删除 PHP ──────────────────────────────────────┘
```

---

**文档版本**: 1.0  
**制定日期**: 2026-03-29  
**参考文档**: docs/full-rust-replacement-blueprint-v2.md
