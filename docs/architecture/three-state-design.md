# Plog CMS 三态分离设计文档

## 概述

本文档定义 Plog CMS 系统的三态分离架构：开发态、构建态、运行态。明确各态的职责边界，为 PHP → Rust 迁移提供清晰的架构指导。

**最后更新**: 2026-03-29
**版本**: v1.0.0

---

## 三态定义

### 1. 开发态 (Development)

**定义**: 开发者日常工作所涉及的内容，包括源代码、测试、文档和配置模板。

**特征**:
- 位于版本控制系统中
- 开发者直接编辑
- 不直接用于生产运行

### 2. 构建态 (Build)

**定义**: 通过构建过程生成的内容，包括编译产物、静态资源和构建配置。

**特征**:
- 由 CI/CD 生成
- 不在版本控制中
- 可以从源代码重建

### 3. 运行态 (Runtime)

**定义**: 生产环境运行所需的内容，包括服务进程、运行配置和数据文件。

**特征**:
- 部署到生产环境
- 包含环境特定配置
- 包含持久化数据

---

## 现有目录结构分析

```
plog-monorepo/
├── 开发态
│   ├── apps/                    # 应用程序源代码
│   ├── packages/                # 共享包源代码
│   ├── plugins/                 # 插件源代码
│   ├── themes/                  # 主题源代码
│   ├── include/                 # 遗留代码
│   ├── admin/                   # 遗留后台
│   ├── tests/                   # 测试代码
│   ├── docs/                    # 文档
│   ├── config/                  # 配置模板
│   ├── manifests/               # 清单文件
│   ├── scripts/                 # 构建脚本
│   ├── tooling/                 # 开发工具
│   ├── composer.json            # 依赖配置
│   ├── phpunit.xml              # 测试配置
│   ├── phpcs.xml                # 代码风格配置
│   └── phpstan.neon             # 静态分析配置
│
├── 构建态
│   ├── admin-web/               # 前端构建产物
│   ├── vendor/                  # Composer 依赖
│   └── *.lock                   # 依赖锁定文件
│
├── 运行态
│   ├── .env                     # 环境变量
│   ├── config.php               # 运行配置
│   ├── content/                 # 运行时内容
│   ├── emlog_data/              # 数据文件
│   ├── logs/                    # 日志文件 (如存在)
│   └── cache/                   # 缓存文件 (如存在)
│
└── 混合态 (待清理)
    ├── test*.html               # 测试页面
    ├── api.php                  # 入口文件
    ├── index.php                # 入口文件
    └── install.php              # 安装脚本
```

---

## 三态边界定义

### 开发态边界

| 目录/文件 | 类型 | 说明 |
|-----------|------|------|
| `apps/` | 应用代码 | 新架构应用 |
| `packages/` | 包代码 | 共享库 |
| `plugins/` | 插件代码 | 插件源码 |
| `themes/` | 主题代码 | 主题源码 |
| `include/` | 遗留代码 | 待迁移 |
| `admin/` | 遗留后台 | 待迁移 |
| `tests/` | 测试代码 | 单元/集成测试 |
| `docs/` | 文档 | API/设计文档 |
| `config/*.sample.php` | 配置模板 | 配置示例 |
| `manifests/` | 清单 | 插件/主题清单 |
| `scripts/` | 脚本 | 构建/部署脚本 |
| `tooling/` | 工具 | 开发工具 |

### 构建态边界

| 目录/文件 | 类型 | 说明 |
|-----------|------|------|
| `admin-web/` | 前端产物 | Vite 构建结果 |
| `vendor/` | 依赖 | Composer 安装 |
| `composer.lock` | 锁定 | 依赖版本锁定 |
| `node_modules/` | 依赖 | npm 安装 (如存在) |

### 运行态边界

| 目录/文件 | 类型 | 说明 |
|-----------|------|------|
| `.env` | 环境配置 | 环境变量 |
| `config.php` | 运行配置 | 主配置文件 |
| `config/*.php` | 运行配置 | 各模块配置 |
| `content/` | 内容数据 | 插件/主题/上传 |
| `emlog_data/` | 数据文件 | 数据库/缓存 |
| `index.php` | 入口 | Web 入口 |
| `api.php` | 入口 | API 入口 |

---

## 三态分离设计

### 目标结构

```
plog-monorepo/
│
├── src/                         # 开发态：源代码
│   ├── apps/                    # 应用程序
│   ├── packages/                # 共享包
│   ├── plugins/                 # 插件
│   └── themes/                  # 主题
│
├── tests/                       # 开发态：测试
│   ├── unit/                    # 单元测试
│   ├── integration/             # 集成测试
│   └── api/                     # API 测试
│
├── docs/                        # 开发态：文档
│   ├── api/                     # API 文档
│   ├── design/                  # 设计文档
│   └── guides/                  # 指南
│
├── config/                      # 开发态：配置模板
│   ├── app.sample.php           # 应用配置模板
│   ├── database.sample.php      # 数据库配置模板
│   └── logging.sample.php       # 日志配置模板
│
├── manifests/                   # 开发态：清单
│   ├── plugins/                 # 插件清单
│   └── themes/                  # 主题清单
│
├── scripts/                     # 开发态：脚本
│   ├── build.sh                 # 构建脚本
│   ├── deploy.sh                # 部署脚本
│   └── test.sh                  # 测试脚本
│
├── tooling/                     # 开发态：工具
│   ├── codegen/                 # 代码生成
│   └── migration/               # 迁移工具
│
├── build/                       # 构建态：构建产物 (gitignore)
│   ├── web/                     # 前端构建
│   └── rust/                    # Rust 构建
│
├── storage/                     # 运行态：存储 (gitignore)
│   ├── app/                     # 应用存储
│   │   ├── content/             # 内容文件
│   │   └── uploads/             # 上传文件
│   ├── logs/                    # 日志
│   └── cache/                   # 缓存
│
├── runtime/                     # 运行态：运行时 (gitignore)
│   ├── .env                     # 环境变量
│   └── config/                  # 运行配置
│
├── public/                      # 运行态：Web 入口
│   ├── index.php                # PHP 入口
│   ├── api.php                  # API 入口
│   └── assets/                  # 静态资源 (symlink)
│
├── composer.json                # 开发态：依赖
├── Cargo.toml                   # 开发态：Rust 依赖
├── Dockerfile                   # 开发态：容器配置
└── docker-compose.yml           # 开发态：容器编排
```

---

## 配置文件分类

### 开发态配置

| 文件 | 说明 | 版本控制 |
|------|------|----------|
| `composer.json` | PHP 依赖 | ✓ |
| `Cargo.toml` | Rust 依赖 | ✓ |
| `phpunit.xml` | 测试配置 | ✓ |
| `phpcs.xml` | 代码风格 | ✓ |
| `phpstan.neon` | 静态分析 | ✓ |
| `Dockerfile` | 容器配置 | ✓ |
| `docker-compose.yml` | 容器编排 | ✓ |
| `.gitignore` | Git 忽略 | ✓ |

### 运行态配置

| 文件 | 说明 | 版本控制 |
|------|------|----------|
| `.env` | 环境变量 | ✗ |
| `config.php` | 主配置 | ✗ |
| `config/database.php` | 数据库配置 | ✗ |
| `config/logging.php` | 日志配置 | ✗ |

### 配置模板

| 文件 | 说明 | 版本控制 |
|------|------|----------|
| `.env.example` | 环境变量模板 | ✓ |
| `config.sample.php` | 主配置模板 | ✓ |

---

## 迁移步骤

### Phase 1: 目录重组

1. 创建新目录结构
2. 移动源代码到 `src/`
3. 移动测试到 `tests/`
4. 移动文档到 `docs/`
5. 更新自动加载配置

### Phase 2: 配置分离

1. 创建配置模板
2. 移动运行配置到 `runtime/`
3. 更新配置加载逻辑
4. 更新 `.gitignore`

### Phase 3: 存储分离

1. 创建 `storage/` 目录
2. 移动内容文件
3. 移动日志文件
4. 移动缓存文件
5. 更新文件路径配置

### Phase 4: 入口整理

1. 创建 `public/` 目录
2. 移动入口文件
3. 配置 Web 服务器
4. 测试访问

---

## .gitignore 更新

```gitignore
# 构建态
/build/
/vendor/
/node_modules/
composer.lock
package-lock.json

# 运行态
/runtime/.env
/runtime/config/*.php
/storage/
*.log

# IDE
/.idea/
/.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
```

---

## 验证清单

- [ ] 所有源代码在 `src/` 目录
- [ ] 所有测试在 `tests/` 目录
- [ ] 所有文档在 `docs/` 目录
- [ ] 运行配置与源代码分离
- [ ] 数据文件与源代码分离
- [ ] `.gitignore` 正确配置
- [ ] 自动加载正常工作
- [ ] 测试可以运行
- [ ] 应用可以启动

---

## 变更历史

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| v1.0.0 | 2026-03-29 | 初始版本 |
