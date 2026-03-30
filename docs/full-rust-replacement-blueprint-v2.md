# 全量 Rust 替换 PHP 的实施蓝图

## 1. 文档目标

本文档面向当前 **Plog CMS** 的混合架构现状，给出一套从 **Rust 主导 + PHP 兼容** 演进到 **PHP 完全退场** 的实施蓝图。

当前已知基线：

- `/api/v2/*` 由 Rust API 提供
- `/api/*` 仍由 PHP 兼容层承接
- `/admin` 仍为 PHP Legacy 后台
- Rust 侧已有 `api / auth / content / core / plugin / theme` 等模块
- `admin-web` 已存在，且用户管理、内容管理已迁移完成
- `compat` 目录已承接 API、插件、主题兼容桥接职责

以上基线来自项目 README。fileciteturn1file0

---

## 2. 总体目标

### 2.1 最终目标

将系统从：

- Rust API + PHP 兼容层 + PHP Legacy 后台

演进为：

- Rust API
- Rust Worker / Scheduler / Installer
- Vue 3 `admin-web`
- Rust Plugin Runtime
- Rust Theme Runtime
- Nginx + MySQL + Redis / Object Storage

最终生产环境不再依赖：

- PHP-FPM
- `compat/`
- `include/`
- `admin/`
- PHP 插件执行链
- PHP 主题模板执行链

### 2.2 非目标

本方案**不是**把现有 PHP 代码逐文件翻译成 Rust，而是：

1. 先冻结契约
2. 再迁移核心实现
3. 最后删除兼容宿主

---

## 3. 当前架构判断

根据 README，项目当前已完成：

- Phase 1：API 合约 + 数据库层
- Phase 2：Rust 微服务（auth、content-api、nginx）
- Phase 3：PHP 兼容层（compat、plugin-compat、theme-compat）
- Phase 4：admin-web 前端迁移（用户管理、内容管理）

并且文档标记为“全部阶段完成”。fileciteturn1file0

这意味着系统已经进入：

> **Rust 主导核心，PHP 承担兼容与遗留运行时。**

因此，后续工作的重点不是“是否引入 Rust”，而是：

> **如何让 PHP 从兼容层继续收缩，直到完全退场。**

---

## 4. 迁移原则

## 4.1 开发态 / 构建态 / 运行态分离

必须明确三种状态：

### 开发态
- `admin-web` 使用 Vite 开发
- Rust API 热更新
- 主题与插件 manifest 校验
- 本地预览与模拟数据调试

### 构建态
- Rust 二进制构建
- 前端静态资源构建
- 插件包构建
- 主题包构建
- manifest 产物生成与签名

### 运行态
- Rust API 服务
- Rust Worker / Scheduler
- Theme Runtime / Plugin Runtime
- 文件存储、缓存、配置加载

禁止把构建逻辑塞进运行时，也禁止把运行时动态解释继续建立在 PHP 上。

## 4.2 先冻结契约，再替换实现

必须优先冻结：

- API 请求 / 响应格式
- 统一错误码
- 分页 / 排序 / 筛选协议
- 用户 / 角色 / 权限模型
- 内容实体模型
- 插件 manifest
- 主题 manifest
- 菜单 / 页面注册机制

## 4.3 唯一事实来源

迁移过程中不能长期存在双写规则：

- 权限规则只能在 Rust 核心定义
- 内容状态流只能在 Rust 核心定义
- 插件能力判定只能在 Rust Runtime 定义
- 主题装载规则只能在 Rust Runtime 定义

## 4.4 兼容层只减不增

自本方案启动后：

- `compat/` 禁止新增业务逻辑
- `include/` 禁止新增服务逻辑
- `admin/` 禁止承接新功能页面
- 新接口不得落入 PHP

---

## 5. 目标终局架构

```text
Browser
  -> Nginx
    -> admin-web (静态资源)
    -> Rust API Gateway
    -> Rust Worker / Scheduler
    -> Rust Theme Runtime
    -> Rust Plugin Runtime
    -> MySQL / Redis / Object Storage
```

### 5.1 应用层

- `apps/admin-web`
- `apps/api-gateway-rs`
- `apps/worker-rs`
- `apps/scheduler-rs`
- `apps/installer-rs`

### 5.2 核心能力层

- `crates/core`
- `crates/contracts`
- `crates/auth`
- `crates/content`
- `crates/settings`
- `crates/media`
- `crates/plugin-runtime`
- `crates/theme-runtime`
- `crates/tasking`
- `crates/audit`
- `crates/search`

### 5.3 内容目录

- `content/uploads`
- `content/plugins`
- `content/themes`
- `content/languages`
- `content/cache`

目录可以保留，但协议要改变：

- 插件不再是 PHP 可执行包
- 主题不再是 PHP 模板执行包
- 语言包不再依赖 PHP 数组文件

---

## 6. 各目录的替换策略

## 6.1 `apps/admin-api` / `/api/*`

### 目标

让 Rust 成为**唯一 API 宿主**。

### 路线

1. 保持 `/api/v2/*` 作为 Rust 主实现
2. 将旧 `/api/*` 功能逐项映射到 Rust
3. 通过网关重写将旧客户端流量导向 Rust
4. 稳定后取消 `v2` 前缀
5. 统一为 `/api/*` 全量 Rust

### 完成标志

- PHP 不再处理任何正式 API 请求
- `compat/index.php` 不再作为 API 主入口

---

## 6.2 `compat/`

README 显示当前 `compat/` 下存在：

- `Proxy.php`
- `Router.php`
- `Response.php`
- `Logger.php`
- `plugin/PluginManager.php`
- `plugin/PluginLoader.php`
- `plugin/HookBridge.php`
- `theme/ThemeManager.php`
- `theme/Renderer.php`
- `theme/TemplateEngine.php`

说明它当前既承担 HTTP 桥接，也承担运行时兼容。fileciteturn1file0

### 目标

将 `compat/` 收缩成纯过渡层，并最终删除。

### 路线

#### 阶段 A：HTTP 兼容收缩
- 只保留旧路径重写与响应格式适配
- 业务逻辑一律调用 Rust

#### 阶段 B：插件与主题兼容收缩
- 将 Plugin / Theme 相关桥接迁移至 Rust Runtime
- PHP Loader 不再负责插件发现、主题发现、Hook 绑定

#### 阶段 C：目录删除
- 删除 `compat/`
- 删除 PHP 启动入口

---

## 6.3 `admin/`

### 目标

彻底退役 PHP Legacy 后台。

### 原则

- 新功能只进入 `admin-web`
- 旧功能按模块迁移
- `admin/` 不得继续承载平台新能力

### 模块迁移顺序建议

1. 用户与权限
2. 内容管理
3. 分类 / 标签 / 评论
4. 系统设置
5. 主题管理
6. 插件管理
7. 审计日志
8. 工具页 / 任务页

### 终局

`/admin` 只做：

- 前端路由入口
- 或 302 到新管理端入口

而不再命中 PHP 模板或 PHP 控制器。

---

## 6.4 `include/`

README 显示该目录承接：

- controller
- model
- service
- lib

这类目录通常是 PHP 历史耦合最深的区域。fileciteturn1file0

### 目标

将 `include/` 从“主业务实现层”降级为“待清理遗留层”，最终删除。

### 处理方式

#### controller
- 跟随 `/admin` 和旧 `/api/*` 一起退役

#### service
- 迁移到 Rust `auth/content/settings/plugin/theme` 等 domain service

#### model
- 迁移到 Rust repository / ORM 层
- 统一使用 SeaORM 或 sqlx 方案

#### lib
- 纯工具：直接替换
- 文件与缓存：并入 media/runtime
- 模板工具：并入 theme runtime
- 临时兼容工具：标记废弃，限期删除

---

## 6.5 `content/`

README 显示 `content/` 包含：

- `templates/`
- `plugins/`
- `uploadfile/`
- `cache/`
- `languages/`

这部分目录大概率会继续存在，但其运行协议必须整体升级。fileciteturn1file0

### 目标

保留内容目录，取消 PHP 对其的执行性依赖。

### 演进方向

#### uploads
- 继续保留
- 交由 Rust media 服务管理

#### cache
- 改由 Rust runtime 控制
- 支持本地缓存与 Redis 缓存

#### languages
- 从 PHP 数组迁移为 JSON / TOML / ICU Message Format

#### plugins
- 从 PHP 可执行包迁移为声明式扩展包

#### themes
- 从 PHP 模板包迁移为声明式主题包

---

## 7. 插件系统的 Rust 替换方案

## 7.1 目标

把 PHP 插件系统从：

- 文件扫描
- 动态 include
- HookBridge
- 运行期直接执行 PHP

改造成：

- manifest 驱动
- capability 驱动
- Rust 统一注册
- 受控扩展执行模型

## 7.2 插件包结构建议

```text
content/plugins/<plugin-id>/
  plugin.toml
  assets/
  admin/
  migrations/
  locales/
  web/
```

## 7.3 插件 manifest 建议字段

- `id`
- `name`
- `version`
- `description`
- `author`
- `capabilities`
- `permissions`
- `settings_schema`
- `menus`
- `pages`
- `jobs`
- `events`
- `migrations`
- `admin_assets`

## 7.4 插件执行模型选型

### 推荐路线：声明式插件优先

插件只做：

- 注册菜单
- 注册页面
- 注册资源定义
- 注册任务
- 注册事件订阅
- 提供前端静态资源
- 暴露 schema 与 manifest

真正业务执行仍由平台 Rust API 完成。

### 可选增强：WASM / JS 沙箱

当确实需要可编程扩展时，可引入：

- WASM 扩展
- JS 沙箱扩展

但禁止回退到 PHP 动态可执行模型。

## 7.5 插件迁移策略

1. 清点现有 PHP 插件
2. 区分：
   - 仅配置型
   - 仅 UI 型
   - Hook 型
   - 数据处理型
3. 优先迁移配置型与 UI 型
4. 为 Hook 型设计 Rust event bus
5. 为数据处理型设计任务与 handler 注册协议
6. 为老插件设置截止迁移窗口

---

## 8. 主题系统的 Rust 替换方案

## 8.1 目标

让主题系统从 PHP Renderer / TemplateEngine 迁移为：

- theme manifest
- layout / slot 协议
- Rust 或前端驱动的渲染链
- 可预览、可构建、可校验的主题包

## 8.2 主题包结构建议

```text
content/themes/<theme-id>/
  theme.toml
  assets/
  templates/
  preview/
  locales/
```

## 8.3 主题 manifest 建议字段

- `id`
- `name`
- `version`
- `author`
- `description`
- `layouts`
- `slots`
- `page_templates`
- `settings_schema`
- `assets`
- `preview`
- `supported_features`

## 8.4 渲染路线建议

### 路线 A：前端组件化渲染

适合后台、控制台、现代站点：

- 页面数据由 Rust API 提供
- 主题提供配置、布局、资源
- 组件负责渲染

### 路线 B：Rust 服务端渲染

适合传统 CMS 输出页：

- Rust 控制模板渲染
- 主题仅提供模板与资源
- 不允许运行任意脚本代码

### 推荐

对当前项目，更推荐：

- 管理端使用前端组件化
- 前台站点如需 SSR，则由 Rust 控制模板引擎

不要继续沿用 PHP 可执行模板模式。

---

## 9. 后台管理端迁移蓝图

## 9.1 目标

让 `admin-web` 成为唯一后台 UI 入口。

## 9.2 必备能力

- RBAC 菜单驱动
- 路由注册机制
- 资源 CRUD 页框架
- 表格 / 表单 schema 渲染
- 主题管理界面
- 插件管理界面
- 系统设置中心
- 日志与审计页
- 任务与计划页

## 9.3 页面模型建议

将后台页面分为：

- ResourceListPage
- ResourceFormPage
- ResourceDetailPage
- SettingsPage
- PluginPage
- ThemePage
- TaskPage
- DashboardPage

并用 manifest / schema 驱动，而不是每个业务项目都手写一套。

## 9.4 `/admin` 收口策略

### 过渡期
- `/admin/legacy/*` 指向老页面
- `/admin/*` 默认命中新前端

### 中期
- 老页面只保留少数高复杂模块

### 终局
- `/admin` 完全由前端接管
- PHP legacy 页面整体删除

---

## 10. 运维与基础设施替换

## 10.1 需要 Rust 化的运维链路

- 安装器
- 升级器
- 数据迁移器
- 定时任务
- 队列 Worker
- 缓存重建
- 配置导入导出
- 日志采集与审计导出

## 10.2 建议应用

- `apps/installer-rs`
- `apps/migrator-rs`
- `apps/worker-rs`
- `apps/scheduler-rs`

## 10.3 Nginx 调整方向

当前 README 已显示 Nginx 负责前端、Rust API、PHP Compat、PHP Legacy 的反代分流。fileciteturn1file0

最终应调整为：

- `/api/*` -> Rust API
- `/admin` -> admin-web 入口
- `/assets/*` -> 静态资源
- `/uploads/*` -> 媒体资源
- 不再存在 PHP upstream

---

## 11. 分阶段实施计划

## Phase 0：冻结协议与清点资产

### 目标
- 完整输出 contract 规范
- 清点 PHP 遗留点
- 冻结 PHP 新逻辑增长

### 产出
- API contract vNext
- 权限模型文档
- 插件 manifest spec
- 主题 manifest spec
- 页面注册 spec
- `admin` 页面迁移清单
- 插件 / 主题兼容清单

### 验收
- 新功能不再落到 PHP
- 所有迁移对象都有台账

---

## Phase 1：Rust 成为唯一业务核心

### 目标
- 权限、内容、设置、媒体、审计全面收口到 Rust

### 要求
- PHP compat 只能转发和适配
- 任何业务规则不得只存在于 PHP

### 验收
- 任意核心接口都可以不经过 PHP 完成调用

---

## Phase 2：统一 API

### 目标
- 停止双栈 API
- 将 `/api/*` 与 `/api/v2/*` 统一为 Rust API

### 验收
- PHP 不再承接 API 请求
- Nginx 中 PHP API upstream 删除

---

## Phase 3：迁空 `/admin`

### 目标
- 所有后台页面迁入 `admin-web`

### 验收
- `/admin` 不再渲染 PHP 页面
- PHP controller 全部退休

---

## Phase 4：重建插件 Runtime

### 目标
- Rust 接管插件发现、注册、配置、安装、升级、执行

### 验收
- 新插件协议不依赖 PHP
- PHP PluginManager / HookBridge 退役

---

## Phase 5：重建主题 Runtime

### 目标
- Rust / 前端接管主题发现、配置、预览、渲染

### 验收
- 新主题协议不依赖 PHP Renderer / TemplateEngine

---

## Phase 6：Rust 化运维链路

### 目标
- CLI、定时任务、升级脚本、缓存刷新全部 Rust 化

### 验收
- 无需 PHP CLI 即可完成日常运维

---

## Phase 7：删除 PHP

### 删除对象
- `compat/`
- `include/`
- `admin/`
- PHP-FPM
- 与 PHP 相关的 Nginx 配置
- PHP 测试与构建链

### 验收
- 生产环境无 PHP 运行时仍能完整启动
- 系统功能与数据链路完整可用

---

## 12. 目录重组建议

建议逐步收敛为：

```text
apps/
  admin-web/
  api-gateway-rs/
  worker-rs/
  scheduler-rs/
  installer-rs/

crates/
  core/
  contracts/
  auth/
  content/
  settings/
  media/
  plugin-runtime/
  theme-runtime/
  tasking/
  audit/
  search/
  sdk-rs/
  sdk-ts-gen/

content/
  uploads/
  plugins/
  themes/
  languages/
  cache/

infra/
  nginx/
  docker/
  scripts/

legacy/
  php-compat/
  php-admin/
  php-include/
```

将旧 PHP 区域统一收纳到 `legacy/` 下，有利于工程治理与删除计划管理。

---

## 13. 回滚策略

全量替换过程中必须保留可回滚能力。

## 13.1 API 回滚
- 网关保留版本路由
- 可临时将个别模块回切到兼容模式
- 通过 feature flag 控制模块启用

## 13.2 页面回滚
- `admin-web` 页面按模块开关发布
- 极端情况下保留短期 legacy fallback

## 13.3 插件回滚
- 插件安装/升级具备版本锁定能力
- 新旧 manifest 兼容期明确时间窗

## 13.4 数据回滚
- 迁移脚本必须幂等
- 数据库变更需伴随回滚脚本或可逆策略

---

## 14. 风险清单

## 14.1 最大风险：API 完成即误判“已经替换完 PHP”

实际最难的是：

- `/admin`
- 插件运行时
- 主题运行时
- 运维链路

## 14.2 最大风险：永久兼容 PHP 插件 / 主题

如果“永久兼容”成为默认策略，PHP 很可能永远无法退场。

## 14.3 最大风险：边迁移边继续给 PHP 加新功能

这会导致兼容层重新膨胀。

## 14.4 最大风险：双写规则

一旦出现：
- PHP 校验一套
- Rust 校验一套
- 前端自己再补一套

系统将长期不稳定。

---

## 15. 成功标准

当以下条件全部成立时，PHP 才算真正可以退场：

1. `/api/*` 全量由 Rust 处理
2. `/admin` 不再渲染 PHP 页面
3. `compat/` 已删除
4. `include/` 已删除或完全不被调用
5. 插件不依赖 PHP Loader
6. 主题不依赖 PHP Renderer
7. 运维链路不依赖 PHP CLI
8. 生产环境移除 PHP-FPM 后系统仍完整运行

---

## 16. 近期优先级建议

如果现在开始执行，建议按以下顺序推进：

### 第一优先级
- 建立 contract 仓
- 冻结 PHP 新逻辑
- 清点 `/admin` 缺口
- 清点插件 / 主题 PHP 执行点
- Rust 接管全部 API

### 第二优先级
- `admin-web` 补齐主题、插件、设置、日志、任务页
- 设计并落地 plugin manifest + runtime
- 设计并落地 theme manifest + render pipeline

### 第三优先级
- Rust 化 installer / worker / scheduler
- 升级语言包格式
- 删除 PHP-FPM 依赖

---

## 17. 一句话路线总结

> 先让 Rust 成为唯一核心，再让 `admin-web` 成为唯一后台入口，然后把插件与主题从 PHP 执行模型改造成 manifest + capability 模型，最后删除 `compat / include / admin` 与 PHP-FPM，完成 PHP 的彻底退场。

