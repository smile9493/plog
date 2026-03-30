# Plog CMS PHP → Rust 迁移实施计划

## 1. 文档信息

| 项目名称 | Plog CMS PHP → Rust 迁移实施计划 |
|---------|--------------------------------|
| 版本号   | v1.0.0                         |
| 创建日期 | 2026-03-29                     |
| 最后更新 | 2026-03-29                     |
| 状态     | 计划草案                       |

---

## 2. 迁移总览

### 2.1 总体时间线

```
2026 Q1-Q2      2026 Q3-Q4      2027 Q1-Q2      2027 Q3-Q4
    │               │               │               │
    ▼               ▼               ▼               ▼
┌────────┐    ┌────────┐    ┌────────┐    ┌────────┐
│Phase 1 │───►│Phase 2 │───►│Phase 3 │───►│Phase 4 │
│边界收敛│    │Rust 接管│    │掏空 PHP│    │退休旧UI│
│2 个月  │    │4 个月  │    │6 个月  │    │6 个月  │
└────────┘    └────────┘    └────────┘    └────────┘
```

### 2.2 关键里程碑

| 里程碑 | 时间 | 目标 |
|--------|------|------|
| M1 | 2026 Q2 | 边界收敛完成，接口契约固定 |
| M2 | 2026 Q3 | Rust 核心服务上线 |
| M3 | 2026 Q4 | Rust 接管 80% API |
| M4 | 2027 Q1 | PHP 代码量减少 60% |
| M5 | 2027 Q2 | /admin 页面退休 50% |
| M6 | 2027 Q4 | 迁移完成，PHP 最小化 |

---

## 3. Phase 1: 边界收敛 (详细计划)

### 3.1 阶段目标

不急着改语言，先把现有系统整理成稳定边界，为 Rust 迁移做准备。

### 3.2 任务清单

#### T1.1: API 接口梳理与契约固定

**目标**: 固定 admin-api 的接口契约

**任务**:
- [ ] 梳理现有 API 端点清单
- [ ] 编写 OpenAPI/Swagger 规范
- [ ] 定义 API 版本管理策略
- [ ] 建立 API 测试套件

**交付物**:
- OpenAPI 规范文档
- API 端点清单
- 版本管理策略文档

**预计工时**: 2 周

---

#### T1.2: 领域模型接口抽象

**目标**: 将 packages/* 中真正可复用的领域接口抽象出来

**任务**:
- [ ] 定义用户模型接口
- [ ] 定义权限模型接口
- [ ] 定义内容模型接口
- [ ] 定义插件模型接口
- [ ] 定义主题模型接口
- [ ] 编写接口文档

**交付物**:
- 领域接口定义 (PHP Interface)
- 接口文档

**预计工时**: 2 周

---

#### T1.3: 三态分离设计

**目标**: 明确哪些是开发态、构建态、运行态

**任务**:
- [ ] 分析现有代码，识别三态边界
- [ ] 编写三态分离设计文档
- [ ] 调整代码组织结构

**三态定义**:

| 状态 | 包含内容 | 说明 |
|------|----------|------|
| 开发态 | 源代码、测试、文档 | 开发者日常工作 |
| 构建态 | 编译产物、构建配置 | CI/CD 处理 |
| 运行态 | 服务进程、配置文件 | 生产环境运行 |

**交付物**:
- 三态分离设计文档
- 代码结构调整方案

**预计工时**: 1 周

---

#### T1.4: Manifest 与能力声明

**目标**: 给插件和主题补上 manifest 与能力声明

**任务**:
- [ ] 设计插件 manifest.json 格式
- [ ] 设计主题 manifest.json 格式
- [ ] 定义能力声明规范
- [ ] 编写 manifest 解析器
- [ ] 迁移现有插件/主题到新格式

**插件 Manifest 示例**:
```json
{
  "name": "example-plugin",
  "version": "1.0.0",
  "description": "示例插件",
  "author": "Author Name",
  "capabilities": [
    "content:read",
    "content:write",
    "hook:filter:post_content"
  ],
  "hooks": {
    "post_save": "onPostSave",
    "post_delete": "onPostDelete"
  },
  "config": {
    "schema": "config-schema.json",
    "default": "config-default.json"
  }
}
```

**主题 Manifest 示例**:
```json
{
  "name": "example-theme",
  "version": "1.0.0",
  "description": "示例主题",
  "author": "Author Name",
  "engine": "blade",
  "templates": {
    "index": "templates/index.blade.php",
    "post": "templates/post.blade.php",
    "page": "templates/page.blade.php"
  },
  "assets": {
    "css": ["assets/css/style.css"],
    "js": ["assets/js/main.js"]
  },
  "supports": [
    "responsive",
    "dark-mode",
    "customizer"
  ]
}
```

**交付物**:
- Manifest 规范文档
- 能力声明规范
- 解析器实现
- 迁移工具

**预计工时**: 2 周

---

### 3.3 Phase 1 验收标准

- [ ] 所有 API 端点有明确的接口契约
- [ ] 核心领域模型有清晰的接口定义
- [ ] 插件和主题有标准化的 manifest 格式
- [ ] 三态分离边界清晰
- [ ] 现有功能不受影响

---

## 4. Phase 2: Rust 接管 admin-api (详细计划)

### 4.1 阶段目标

创建新的 Rust 服务，逐步接管 admin-api 的功能。

### 4.2 技术栈确定

| 组件 | 选择 | 版本 |
|------|------|------|
| **语言** | Rust | 1.75+ |
| **Web 框架** | Axum | 0.7+ |
| **ORM** | SeaORM | 1.0+ |
| **数据库驱动** | sqlx-mysql | - |
| **序列化** | serde | 1.0+ |
| **日志** | tracing | 0.1+ |
| **配置** | config | 0.14+ |
| **认证** | jsonwebtoken | 9.0+ |
| **HTTP 客户端** | reqwest | 0.11+ |

### 4.3 任务清单

#### T2.1: Rust 项目初始化

**目标**: 搭建 Rust 项目基础结构

**任务**:
- [ ] 创建 Cargo workspace
- [ ] 配置项目结构
- [ ] 设置 CI/CD
- [ ] 配置开发环境

**项目结构**:
```
plog-rs/
├── Cargo.toml
├── crates/
│   ├── core/           # 核心库
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── auth/           # 认证模块
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── content/        # 内容模块
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── plugin/         # 插件模块
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── theme/          # 主题模块
│   │   ├── src/
│   │   └── Cargo.toml
│   └── api/            # API 服务
│       ├── src/
│       └── Cargo.toml
├── migrations/         # 数据库迁移
├── config/             # 配置文件
└── tests/              # 集成测试
```

**交付物**:
- Cargo workspace 配置
- 项目结构
- CI/CD 配置

**预计工时**: 1 周

---

#### T2.2: 数据库访问层

**目标**: 实现 Rust 数据库访问层

**任务**:
- [ ] 配置 SeaORM 连接
- [ ] 定义实体 (Entity)
- [ ] 生成数据库迁移
- [ ] 编写 Repository 层
- [ ] 编写单元测试

**实体定义示例**:
```rust
// crates/content/src/entities/post.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub gid: i32,
    pub title: String,
    pub slug: Option<String>,
    pub content: String,
    pub excerpt: Option<String>,
    pub author_id: i32,
    pub sortid: i32,
    pub date: DateTime,
    pub hide: String,
    pub r#type: String,
    // ... 其他字段
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

**交付物**:
- SeaORM 实体定义
- 数据库迁移
- Repository 实现
- 单元测试

**预计工时**: 2 周

---

#### T2.3: 认证服务

**目标**: 实现 Rust 认证服务

**任务**:
- [ ] 实现用户认证
- [ ] 实现 JWT 签发
- [ ] 实现会话管理
- [ ] 实现权限检查
- [ ] 编写单元测试

**认证流程**:
```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────►│  Auth API   │────►│   Database  │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │
       │  POST /login      │                   │
       │──────────────────►│  验证用户         │
       │                   │──────────────────►│
       │                   │◄──────────────────│
       │  返回 JWT         │                   │
       │◄──────────────────│                   │
       │                   │                   │
       │  请求 + JWT       │                   │
       │──────────────────►│  验证 JWT         │
       │                   │  检查权限         │
       │  响应             │                   │
       │◄──────────────────│                   │
```

**交付物**:
- 认证服务实现
- JWT 处理
- 权限检查
- 单元测试

**预计工时**: 2 周

---

#### T2.4: 内容管理 API

**目标**: 实现 Rust 内容管理 API

**任务**:
- [ ] 实现文章 CRUD API
- [ ] 实现分类管理 API
- [ ] 实现标签管理 API
- [ ] 实现评论管理 API
- [ ] 编写集成测试

**API 端点**:
```
# 文章管理
GET    /api/v2/posts          # 获取文章列表
GET    /api/v2/posts/:id      # 获取文章详情
POST   /api/v2/posts          # 创建文章
PUT    /api/v2/posts/:id      # 更新文章
DELETE /api/v2/posts/:id      # 删除文章

# 分类管理
GET    /api/v2/categories     # 获取分类列表
POST   /api/v2/categories     # 创建分类
PUT    /api/v2/categories/:id # 更新分类
DELETE /api/v2/categories/:id # 删除分类

# 标签管理
GET    /api/v2/tags           # 获取标签列表
POST   /api/v2/tags           # 创建标签
PUT    /api/v2/tags/:id       # 更新标签
DELETE /api/v2/tags/:id       # 删除标签

# 评论管理
GET    /api/v2/comments       # 获取评论列表
POST   /api/v2/comments       # 创建评论
PUT    /api/v2/comments/:id   # 更新评论
DELETE /api/v2/comments/:id   # 删除评论
```

**交付物**:
- 内容管理 API 实现
- 请求验证
- 响应格式化
- 集成测试

**预计工时**: 3 周

---

#### T2.5: Nginx 路由配置

**目标**: 配置 Nginx 路由分流

**任务**:
- [ ] 编写 Nginx 配置
- [ ] 配置 SSL/TLS
- [ ] 配置负载均衡
- [ ] 编写部署脚本

**Nginx 配置示例**:
```nginx
upstream rust_api {
    server 127.0.0.1:8080;
}

upstream php_api {
    server 127.0.0.1:9000;
}

server {
    listen 443 ssl;
    server_name admin.example.com;

    # Rust API (v2)
    location /api/v2/ {
        proxy_pass http://rust_api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # PHP API (v1)
    location /api/ {
        fastcgi_pass php_api;
        fastcgi_param SCRIPT_FILENAME $document_root/index.php;
        include fastcgi_params;
    }

    # 传统后台
    location /admin {
        fastcgi_pass php_api;
        fastcgi_param SCRIPT_FILENAME $document_root/admin/index.php;
        include fastcgi_params;
    }

    # 独立前端
    location /admin-web {
        alias /var/www/admin-web/dist;
        try_files $uri $uri/ /admin-web/index.html;
    }
}
```

**交付物**:
- Nginx 配置文件
- SSL 证书配置
- 部署脚本

**预计工时**: 1 周

---

### 4.4 Phase 2 验收标准

- [ ] Rust 服务可独立运行
- [ ] Nginx 路由分流正常
- [ ] admin-web 可消费 Rust API
- [ ] 旧 PHP 后台不受影响
- [ ] API 响应时间 < 100ms (P95)
- [ ] 单元测试覆盖率 >= 70%

---

## 5. Phase 3: 掏空 PHP (详细计划)

### 5.1 阶段目标

让 PHP 变薄，只做兼容入口，真实业务规则在 Rust。

### 5.2 任务清单

#### T3.1: PHP 兼容层框架

**目标**: 建立 PHP 兼容层框架

**任务**:
- [ ] 设计兼容层架构
- [ ] 实现请求转发
- [ ] 实现响应转换
- [ ] 编写文档

**兼容层架构**:
```
┌─────────────────────────────────────────────────────────────┐
│                  PHP Compat Layer                            │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │  PHP Entry  │────►│   Router    │────►│   Proxy     │  │
│  │  (入口)     │     │  (路由)     │     │  (转发)     │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                     │        │
│                                                     ▼        │
│                                            ┌─────────────┐  │
│                                            │ Rust API    │  │
│                                            └─────────────┘  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

**预计工时**: 2 周

---

#### T3.2: PHP-Rust 桥接服务

**目标**: 实现 PHP 和 Rust 之间的桥接

**任务**:
- [ ] 实现 HTTP 客户端封装
- [ ] 实现错误处理
- [ ] 实现重试机制
- [ ] 实现缓存层
- [ ] 编写单元测试

**桥接策略**:
1. PHP 接收请求
2. PHP 转发到 Rust API
3. Rust 处理业务逻辑
4. Rust 返回响应
5. PHP 转换响应格式
6. PHP 返回给客户端

**预计工时**: 2 周

---

#### T3.3: 插件兼容层

**目标**: 保持老插件可运行

**任务**:
- [ ] 分析现有插件系统
- [ ] 设计兼容层接口
- [ ] 实现插件加载器
- [ ] 实现 Hook 桥接
- [ ] 编写迁移工具

**插件兼容策略**:
```
┌─────────────────────────────────────────────────────────────┐
│                  Plugin Compat Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │ PHP Plugin  │────►│  Hook Bridge│────►│ Rust Core   │  │
│  │ (老插件)    │     │  (Hook桥接) │     │ (新核心)    │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                               │
│  ┌─────────────┐                                           │
│  │ Rust Plugin │  (新插件直接运行在 Rust 环境)              │
│  │ (新插件)    │                                           │
│  └─────────────┘                                           │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

**预计工时**: 3 周

---

#### T3.4: 主题兼容层

**目标**: 保持老主题可渲染

**任务**:
- [ ] 分析现有主题系统
- [ ] 设计渲染管线抽象
- [ ] 实现 PHP 模板引擎适配
- [ ] 实现 Rust 模板引擎
- [ ] 编写迁移工具

**渲染管线**:
```
┌─────────────────────────────────────────────────────────────┐
│                  Theme Render Pipeline                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │   Request   │────►│   Router    │────►│  Renderer   │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                     │        │
│                           ┌─────────────────────────┘        │
│                           │                                  │
│              ┌────────────┴────────────┐                    │
│              ▼                         ▼                    │
│     ┌─────────────┐           ┌─────────────┐              │
│     │ PHP Render  │           │ Rust Render │              │
│     │ (老主题)    │           │ (新主题)    │              │
│     └─────────────┘           └─────────────┘              │
│              │                         │                    │
│              ▼                         ▼                    │
│     ┌─────────────┐           ┌─────────────┐              │
│     │   Response  │           │   Response  │              │
│     └─────────────┘           └─────────────┘              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

**预计工时**: 3 周

---

### 5.3 Phase 3 验收标准

- [ ] PHP 代码量减少 60%+
- [ ] 核心业务逻辑在 Rust
- [ ] 老插件仍可运行
- [ ] 老主题仍可渲染
- [ ] API 响应时间 < 80ms (P95)
- [ ] 数据一致性 100%

---

## 6. Phase 4: 逐步退休 /admin (详细计划)

### 6.1 阶段目标

等 admin-web 覆盖率足够高，逐模块下线旧 PHP 页面。

### 6.2 模块退休顺序

| 顺序 | 模块 | 开始时间 | 完成时间 | 依赖条件 |
|------|------|----------|----------|----------|
| 1 | 用户与角色 | 2027 Q3 | 2027 Q3 | admin-web 用户管理完成 |
| 2 | 内容管理 | 2027 Q3 | 2027 Q4 | admin-web 文章管理完成 |
| 3 | 菜单/导航 | 2027 Q3 | 2027 Q4 | admin-web 菜单管理完成 |
| 4 | 插件管理 | 2027 Q4 | 2027 Q4 | admin-web 插件管理完成 |
| 5 | 主题配置 | 2027 Q4 | 2027 Q4 | admin-web 主题配置完成 |
| 6 | 系统设置 | 2027 Q4 | 2027 Q4 | admin-web 设置页面完成 |

### 6.3 退休验收标准

每个模块退休前需满足:
- [ ] admin-web 对应功能完成
- [ ] 功能测试通过
- [ ] 用户验收通过
- [ ] 数据迁移完成
- [ ] 回滚方案就绪

### 6.4 最终验收

- [ ] /admin 页面全部退休
- [ ] admin-web 功能覆盖率 100%
- [ ] 用户无感知切换
- [ ] 性能指标达标
- [ ] PHP 代码量占比 < 10%

---

## 7. 资源需求

### 7.1 人力资源

| 角色 | Phase 1 | Phase 2 | Phase 3 | Phase 4 |
|------|---------|---------|---------|---------|
| Rust 开发 | 0 | 2 | 2 | 1 |
| PHP 开发 | 1 | 1 | 1 | 0 |
| 前端开发 | 0 | 1 | 1 | 1 |
| 测试 | 0 | 1 | 1 | 1 |
| 运维 | 0 | 0.5 | 0.5 | 0.5 |

### 7.2 时间投入

| 阶段 | 时间 | 人月 |
|------|------|------|
| Phase 1 | 2 个月 | 2 人月 |
| Phase 2 | 4 个月 | 18 人月 |
| Phase 3 | 6 个月 | 27 人月 |
| Phase 4 | 6 个月 | 21 人月 |
| **总计** | 18 个月 | 68 人月 |

---

## 8. 风险登记

| ID | 风险描述 | 影响 | 概率 | 应对措施 | 责任人 |
|----|---------|------|------|----------|--------|
| R1 | Rust 团队技能不足 | 高 | 中 | 培训 + 小规模试点 | - |
| R2 | 数据库迁移失败 | 高 | 低 | 完整备份 + 回滚方案 | - |
| R3 | 性能回退 | 中 | 中 | 性能基准测试 + 监控 | - |
| R4 | 老插件/主题失效 | 高 | 中 | 兼容层保证 + 迁移工具 | - |
| R5 | 开发周期延长 | 中 | 中 | 分阶段交付 + MVP 思维 | - |
| R6 | 团队抵触 | 中 | 中 | 渐进式迁移 + 培训支持 | - |

---

## 9. 变更历史

| 版本 | 日期 | 变更内容 | 作者 |
|-----|------|---------|------|
| v1.0.0 | 2026-03-29 | 初始版本创建 | AI Agent |
