# Plog Monorepo 重构 - M1 阶段实施任务清单

## 1. 文档信息

| 项目名称 | Plog CMS Monorepo 重构 - M1 阶段 |
|---------|--------------------------------|
| 版本号   | v1.0.0                         |
| 创建日期 | 2026-03-28                     |
| 最后更新 | 2026-03-28                     |
| 作者     | SDD Agent                      |
| 状态     | 已更新（基于代码审查）         |

## 2. 任务概览

### 2.1 任务统计

- **总任务数**: 15 个主任务
- **子任务数**: 52 个子任务
- **预计工期**: 2-3 周
- **优先级分布**: P0 (核心) 8个, P1 (重要) 5个, P2 (一般) 2个

### 2.2 任务依赖关系

```
任务 1 (目录结构) ──► 任务 2 (Core包) ──► 任务 3 (DB包)
                                              │
                                              ▼
任务 4 (Auth包) ◄─────────────────────────────┘
       │
       ▼
任务 5 (Content包) ──► 任务 6 (Admin API) ──► 任务 7 (工具链)
                                                      │
                                                      ▼
任务 8 (测试) ◄───────────────────────────────────────┘
       │
       ▼
任务 9-15 (文档、迁移、验证)
```

## 3. 详细任务清单

### 任务 1: 创建 Monorepo 目录结构 [P0]

**任务描述:**
创建符合 Monorepo 规范的目录结构,包括应用目录、共享包目录、配置目录等。

**输入:**
- 现有项目根目录
- 目录结构设计文档

**输出:**
- 完整的 Monorepo 目录结构
- 目录结构说明文档

**子任务:**

#### 1.1 创建顶层目录结构
- 创建 `apps/` 目录
- 创建 `packages/` 目录
- 创建 `plugins/` 目录 (空目录,为 M3 阶段预留)
- 创建 `themes/` 目录 (空目录,为 M3 阶段预留)
- 创建 `tooling/` 目录 (空目录,为 M4 阶段预留)
- 创建 `manifests/` 目录
- 创建 `config/` 目录
- 创建 `tests/` 目录

#### 1.2 创建应用目录结构
- 创建 `apps/admin-api/` 目录及其子目录:
  - `src/` (Controller, Middleware, Router, Service)
  - `config/`
  - `public/`
  - `routes/`
  - `tests/`

#### 1.3 创建共享包目录结构
- 创建 `packages/core/` 目录及其子目录:
  - `src/` (Config, Log, Event, Container, Helper)
  - `tests/`
  - `config/`
- 创建 `packages/db/` 目录及其子目录:
  - `src/` (Connection, QueryBuilder, Model)
  - `tests/`
- 创建 `packages/auth/` 目录及其子目录:
  - `src/` (Authenticator, Session, PasswordHasher)
  - `tests/`
- 创建 `packages/content/` 目录及其子目录:
  - `src/` (Models, Services)
  - `tests/`

#### 1.4 创建配置文件
- 创建根目录 `.env.example` 文件
- 创建根目录 `composer.json` 文件
- 创建根目录 `phpunit.xml` 文件
- 创建根目录 `phpcs.xml` 文件
- 创建根目录 `.gitignore` 文件 (更新)

**验收标准:**
- ✅ 所有目录结构创建完成
- ✅ 目录结构符合设计文档要求
- ✅ 配置文件模板创建完成

**代码生成提示:**
```bash
# 创建目录结构的 shell 脚本
mkdir -p apps/admin-api/{src/{Controller,Middleware,Router,Service},config,public,routes,tests}
mkdir -p packages/core/{src/{Config,Log,Event,Container,Helper},tests,config}
mkdir -p packages/db/{src/{Connection,QueryBuilder,Model},tests}
mkdir -p packages/auth/{src/{Authenticator,Session,PasswordHasher},tests}
mkdir -p packages/content/{src/{Models,Services},tests}
mkdir -p plugins themes tooling manifests config tests
```

---

### 任务 2: 实现 Core 包基础功能 [P0]

**任务描述:**
实现 Core 包的核心服务,包括配置管理、日志记录、事件调度和服务容器。

**输入:**
- Core 包目录结构
- 设计文档中的接口定义

**输出:**
- 完整的 Core 包实现
- Core 包单元测试
- Core 包文档

**子任务:**

#### 2.1 实现配置管理器 (Config Manager)
- 创建 `ConfigManagerInterface` 接口
- 创建 `ConfigManager` 类实现
- 创建 `ConfigLoaderInterface` 接口
- 创建 `EnvLoader` 类 (加载 .env 文件)
- 创建 `PhpLoader` 类 (加载 PHP 配置文件)
- 创建 `JsonLoader` 类 (加载 JSON 配置文件)
- 编写配置管理器单元测试

#### 2.2 实现日志记录器 (Logger)
- 集成 Monolog 包
- 创建 `LogManager` 类
- 配置多种日志通道 (file, syslog, stderr)
- 实现日志级别控制
- 编写日志管理器单元测试

#### 2.3 实现事件调度器 (Event Dispatcher)
- 创建 `EventDispatcherInterface` 接口
- 创建 `EventDispatcher` 类实现
- 创建 `Event` 基类
- 实现事件监听器注册和触发
- 支持事件优先级
- 编写事件调度器单元测试

#### 2.4 实现服务容器 (Service Container)
- 创建 `ContainerInterface` 接口 (PSR-11)
- 创建 `Container` 类实现
- 实现依赖注入功能
- 实现服务提供者 (ServiceProvider)
- 支持单例绑定
- 编写服务容器单元测试

#### 2.5 实现辅助函数 (Helpers)
- 创建 `Arr` 类 (数组操作)
- 创建 `Str` 类 (字符串操作)
- 创建 `Path` 类 (路径操作)
- 编写辅助函数单元测试

#### 2.6 创建 Core 包配置
- 创建 `packages/core/composer.json`
- 创建 `packages/core/README.md`
- 创建 `packages/core/phpunit.xml`

**验收标准:**
- ✅ 所有核心服务实现完成
- ✅ 单元测试覆盖率 >= 80%
- ✅ 代码符合 PSR-12 规范
- ✅ 所有公共接口有文档注释

**代码生成提示:**
```php
// ConfigManager 实现示例
namespace Plog\Core\Config;

class ConfigManager implements ConfigManagerInterface
{
    private array $items = [];

    public function get(string $key, $default = null)
    {
        $keys = explode('.', $key);
        $value = $this->items;

        foreach ($keys as $k) {
            if (!is_array($value) || !array_key_exists($k, $value)) {
                return $default;
            }
            $value = $value[$k];
        }

        return $value;
    }
    // ... 其他方法实现
}
```

---

### 任务 3: 实现 DB 包数据库访问层 [P0]

**任务描述:**
实现 DB 包的数据库访问层,包括连接管理、查询构建器和模型基类。

**输入:**
- DB 包目录结构
- 现有数据库连接代码 (include/lib/databasepdo.php)
- 设计文档中的接口定义

**输出:**
- 完整的 DB 包实现
- DB 包单元测试
- DB 包文档

**子任务:**

#### 3.1 实现数据库连接管理器
- 创建 `ConnectionInterface` 接口
- 创建 `ConnectionManager` 类
- 创建 `Connection` 类
- 支持多数据库连接配置
- 支持读写分离
- 实现连接池 (可选)
- 编写连接管理器单元测试

#### 3.2 实现查询构建器 (Query Builder)
- 创建 `QueryBuilderInterface` 接口
- 创建 `QueryBuilder` 类实现
- 实现 SELECT 查询
- 实现 WHERE 条件 (支持链式调用)
- 实现 JOIN 查询
- 实现 ORDER BY, LIMIT, OFFSET
- 实现 INSERT, UPDATE, DELETE
- 实现聚合函数 (count, sum, avg, max, min)
- 编写查询构建器单元测试

#### 3.3 实现模型基类 (Model)
- 创建 `Model` 抽象基类
- 实现 CRUD 操作
- 支持批量赋值 (fillable)
- 支持时间戳自动维护
- 实现软删除 (可选)
- 编写模型基类单元测试

#### 3.4 实现数据库迁移工具
- 创建 `Migration` 类
- 创建 `Schema` 构建器
- 支持表创建、修改、删除
- 编写迁移工具单元测试

#### 3.5 创建 DB 包配置
- 创建 `packages/db/composer.json`
- 创建 `packages/db/README.md`
- 创建 `packages/db/phpunit.xml`
- 创建数据库配置文件模板

**验收标准:**
- ✅ 数据库连接功能正常
- ✅ 查询构建器支持链式调用
- ✅ 模型基类支持 CRUD 操作
- ✅ 单元测试覆盖率 >= 75%
- ✅ 兼容现有数据库表结构

**代码生成提示:**
```php
// QueryBuilder 实现示例
namespace Plog\Db;

class QueryBuilder implements QueryBuilderInterface
{
    public function where(string $column, string $operator, $value): self
    {
        $this->wheres[] = [
            'column' => $column,
            'operator' => $operator,
            'value' => $value,
            'boolean' => 'AND'
        ];
        return $this;
    }

    public function get(): array
    {
        $sql = $this->buildSelectSql();
        $statement = $this->connection->query($sql, $this->bindings);
        return $statement->fetchAll(PDO::FETCH_ASSOC);
    }
}
```

---

### 任务 4: 实现 Auth 包用户认证模块 [P0]

**任务描述:**
实现 Auth 包的用户认证和授权功能,包括认证器、会话管理、密码加密等。

**输入:**
- Auth 包目录结构
- 现有认证代码 (include/lib/loginauth.php, passwordhash.php)
- 设计文档中的接口定义

**输出:**
- 完整的 Auth 包实现
- Auth 包单元测试
- Auth 包文档

**子任务:**

#### 4.1 实现用户认证器 (Authenticator)
- 创建 `UserInterface` 接口
- 创建 `AuthenticatorInterface` 接口
- 创建 `Authenticator` 类实现
- 实现用户登录验证
- 实现用户登出
- 实现登录状态检查
- 编写认证器单元测试

#### 4.2 实现会话管理器 (Session Manager)
- 创建 `SessionManagerInterface` 接口
- 创建 `SessionManager` 类
- 实现 PHP Session 管理
- 支持会话配置 (lifetime, path, domain, secure, httponly)
- 实现会话数据存取
- 编写会话管理器单元测试

#### 4.3 实现密码加密器 (Password Hasher)
- 创建 `PasswordHasherInterface` 接口
- 创建 `PasswordHasher` 类
- 使用 bcrypt 或 argon2 加密
- 实现密码验证
- 编写密码加密器单元测试

#### 4.4 实现权限检查器 (Permission Checker)
- 创建 `PermissionChecker` 类
- 实现角色检查
- 实现权限检查
- 支持角色和权限双重验证
- 编写权限检查器单元测试

#### 4.5 实现用户模型
- 创建 `User` 模型类
- 映射现有用户表结构
- 实现用户数据访问
- 编写用户模型单元测试

#### 4.6 创建 Auth 包配置
- 创建 `packages/auth/composer.json`
- 创建 `packages/auth/README.md`
- 创建 `packages/auth/phpunit.xml`
- 创建认证配置文件模板

**验收标准:**
- ✅ 用户认证功能正常
- ✅ 密码加密使用安全算法
- ✅ 会话管理功能正常
- ✅ 权限检查功能正常
- ✅ 单元测试覆盖率 >= 75%
- ✅ 兼容现有用户表结构

**代码生成提示:**
```php
// Authenticator 实现示例
namespace Plog\Auth;

class Authenticator implements AuthenticatorInterface
{
    public function attempt(array $credentials): bool
    {
        $user = $this->userProvider->retrieveByCredentials($credentials);

        if ($user && $this->userProvider->validateCredentials($user, $credentials)) {
            $this->login($user);
            return true;
        }

        return false;
    }

    public function login(UserInterface $user): void
    {
        $this->sessionManager->set('auth.user_id', $user->getId());
    }
}
```

---

### 任务 5: 实现 Content 包内容管理核心 [P0]

**任务描述:**
实现 Content 包的内容管理核心模型和业务逻辑,包括文章、分类、标签、评论等。

**输入:**
- Content 包目录结构
- 现有模型代码 (include/model/*.php)
- 设计文档中的接口定义

**输出:**
- 完整的 Content 包实现
- Content 包单元测试
- Content 包文档

**子任务:**

#### 5.1 实现文章模型 (Post Model)
- 创建 `Post` 模型类
- 映射现有 blog 表结构
- 实现文章 CRUD 操作
- 实现文章状态管理 (draft, published, private, hidden)
- 实现文章类型区分 (post, page)
- 实现文章别名 (slug) 自动生成
- 编写文章模型单元测试

#### 5.2 实现分类模型 (Category Model)
- 创建 `Category` 模型类
- 映射现有 sort 表结构
- 实现分类 CRUD 操作
- 实现层级分类支持
- 实现分类排序
- 编写分类模型单元测试

#### 5.3 实现标签模型 (Tag Model)
- 创建 `Tag` 模型类
- 映射现有 tag 表结构
- 实现标签 CRUD 操作
- 实现文章-标签关联
- 编写标签模型单元测试

#### 5.4 实现评论模型 (Comment Model)
- 创建 `Comment` 模型类
- 映射现有 comment 表结构
- 实现评论 CRUD 操作
- 实现评论审核机制
- 实现评论层级支持
- 编写评论模型单元测试

#### 5.5 实现媒体模型 (Media Model)
- 创建 `Media` 模型类
- 映射现有 media 表结构
- 实现媒体文件管理
- 编写媒体模型单元测试

#### 5.6 实现内容服务层
- 创建 `PostService` 类
- 创建 `CategoryService` 类
- 创建 `TagService` 类
- 创建 `CommentService` 类
- 实现业务逻辑封装
- 编写服务层单元测试

#### 5.7 创建 Content 包配置
- 创建 `packages/content/composer.json`
- 创建 `packages/content/README.md`
- 创建 `packages/content/phpunit.xml`

**验收标准:**
- ✅ 所有模型支持 CRUD 操作
- ✅ 模型关联关系正确
- ✅ 业务逻辑封装完整
- ✅ 单元测试覆盖率 >= 70%
- ✅ 兼容现有数据库表结构

**代码生成提示:**
```php
// Post 模型实现示例
namespace Plog\Content\Models;

class Post extends Model
{
    protected string $table = 'blog';
    protected string $primaryKey = 'gid';

    const STATUS_DRAFT = 'draft';
    const STATUS_PUBLISHED = 'public';

    public function published(): array
    {
        return $this->newQuery()
            ->where('hide', '=', 'n')
            ->where('type', '=', self::TYPE_POST)
            ->orderBy('date', 'DESC')
            ->get();
    }
}
```

---

### 任务 6: 搭建 Admin API 基础服务 [P0]

**任务描述:**
搭建 Admin API 的基础服务,包括路由、中间件、控制器和响应格式化。

**输入:**
- Admin API 目录结构
- Core、DB、Auth、Content 包
- 设计文档中的 API 设计

**输出:**
- 完整的 Admin API 基础服务
- API 单元测试
- API 文档

**子任务:**

#### 6.1 实现 API 路由
- 创建 `Router` 类
- 实现 RESTful 路由定义
- 支持路由分组
- 支持路由中间件
- 创建路由配置文件
- 编写路由单元测试

#### 6.2 实现中间件
- 创建 `MiddlewareInterface` 接口
- 创建 `AuthMiddleware` (认证中间件)
- 创建 `CorsMiddleware` (跨域中间件)
- 创建 `LogMiddleware` (日志中间件)
- 创建 `PermissionMiddleware` (权限中间件)
- 实现中间件链式调用
- 编写中间件单元测试

#### 6.3 实现控制器
- 创建 `Controller` 基类
- 创建 `AuthController` (认证控制器)
- 创建 `PostController` (文章控制器)
- 创建 `CategoryController` (分类控制器)
- 创建 `TagController` (标签控制器)
- 创建 `CommentController` (评论控制器)
- 编写控制器单元测试

#### 6.4 实现响应格式化
- 创建 `JsonResponse` 类
- 实现统一响应格式
- 实现错误响应格式
- 实现分页响应格式
- 编写响应格式化单元测试

#### 6.5 实现请求验证
- 创建 `Validator` 类
- 实现常用验证规则
- 实现自定义验证规则
- 编写验证器单元测试

#### 6.6 创建 API 入口
- 创建 `public/index.php` 入口文件
- 创建 `Application` 类
- 实现应用启动流程
- 创建 `.htaccess` 文件 (URL 重写)

#### 6.7 创建 Admin API 配置
- 创建 `apps/admin-api/composer.json`
- 创建 `apps/admin-api/README.md`
- 创建 `apps/admin-api/phpunit.xml`
- 创建应用配置文件

**验收标准:**
- ✅ API 路由功能正常
- ✅ 中间件链式调用正常
- ✅ 控制器响应格式统一
- ✅ 请求验证功能正常
- ✅ 单元测试覆盖率 >= 70%
- ✅ API 文档完整

**代码生成提示:**
```php
// PostController 实现示例
namespace Plog\AdminApi\Controller;

class PostController
{
    public function index(RequestInterface $request): JsonResponse
    {
        $page = $request->get('page', 1);
        $perPage = $request->get('per_page', 20);

        $result = $this->postService->paginate($page, $perPage);

        return new JsonResponse([
            'success' => true,
            'data' => $result['data'],
            'meta' => [
                'total' => $result['total'],
                'page' => $page,
                'per_page' => $perPage
            ]
        ]);
    }
}
```

---

### 任务 7: 配置开发工具链 [P1]

**任务描述:**
配置开发工具链,包括 Composer、PHPUnit、PHP_CodeSniffer、PHPStan 等。

**输入:**
- 根目录结构
- 设计文档中的工具链配置

**输出:**
- 完整的开发工具链配置
- 工具链使用文档

**子任务:**

#### 7.1 配置 Composer
- 创建根 `composer.json` (完整配置)
- 配置自动加载 (PSR-4)
- 配置开发依赖
- 配置自定义脚本
- 配置 Monorepo 仓库
- 执行 `composer install`

#### 7.2 配置 PHPUnit
- 创建 `phpunit.xml` 配置文件
- 配置测试套件
- 配置测试覆盖率报告
- 创建测试引导文件
- 创建测试基类

#### 7.3 配置 PHP_CodeSniffer
- 创建 `phpcs.xml` 配置文件
- 配置 PSR-12 标准
- 配置检查规则
- 配置排除规则

#### 7.4 配置 PHPStan
- 创建 `phpstan.neon` 配置文件
- 配置检查级别
- 配置忽略错误
- 配置自动加载

#### 7.5 创建开发脚本
- 创建 `scripts/test.sh` (运行测试)
- 创建 `scripts/cs-check.sh` (代码风格检查)
- 创建 `scripts/cs-fix.sh` (代码风格修复)
- 创建 `scripts/stan.sh` (静态分析)
- 创建 `scripts/check.sh` (综合检查)

#### 7.6 创建环境配置
- 创建 `.env.example` 文件 (完整配置项)
- 创建 `config/app.php` (应用配置)
- 创建 `config/database.php` (数据库配置)
- 创建 `config/logging.php` (日志配置)

**验收标准:**
- ✅ Composer 依赖安装成功
- ✅ PHPUnit 可正常运行测试
- ✅ PHP_CodeSniffer 可检查代码风格
- ✅ PHPStan 可进行静态分析
- ✅ 所有脚本可正常执行

**代码生成提示:**
```json
// composer.json 配置示例
{
  "name": "plog/plog-monorepo",
  "require": {
    "php": "^7.4|^8.0",
    "vlucas/phpdotenv": "^5.0",
    "monolog/monolog": "^2.0"
  },
  "require-dev": {
    "phpunit/phpunit": "^9.0",
    "squizlabs/php_codesniffer": "^3.0",
    "phpstan/phpstan": "^1.0"
  },
  "autoload": {
    "psr-4": {
      "Plog\\Core\\": "packages/core/src/",
      "Plog\\Db\\": "packages/db/src/"
    }
  },
  "scripts": {
    "test": "phpunit",
    "cs:check": "phpcs --standard=PSR12",
    "stan": "phpstan analyse"
  }
}
```

---

### 任务 8: 编写单元测试 [P1]

**任务描述:**
为核心包和 API 编写完整的单元测试,确保代码质量。

**输入:**
- 所有已实现的包和 API
- 测试配置

**输出:**
- 完整的单元测试套件
- 测试覆盖率报告

**子任务:**

#### 8.1 编写 Core 包测试
- ConfigManager 测试
- Logger 测试
- EventDispatcher 测试
- Container 测试
- Helper 测试

#### 8.2 编写 DB 包测试
- ConnectionManager 测试
- QueryBuilder 测试
- Model 测试
- Migration 测试

#### 8.3 编写 Auth 包测试
- Authenticator 测试
- SessionManager 测试
- PasswordHasher 测试
- PermissionChecker 测试

#### 8.4 编写 Content 包测试
- Post 模型测试
- Category 模型测试
- Tag 模型测试
- Comment 模型测试
- Service 层测试

#### 8.5 编写 API 测试
- 路由测试
- 中间件测试
- 控制器测试
- 响应格式化测试

#### 8.6 生成测试覆盖率报告
- 运行所有测试
- 生成覆盖率报告
- 分析覆盖率数据
- 补量是否达标 (>= 60%)

**验收标准:**
- ✅ 所有测试通过
- ✅ 测试覆盖率 >= 60%
- ✅ 核心包覆盖率 >= 75%
- ✅ 无严重代码质量问题

---

### 任务 9: 编写项目文档 [P1]

**任务描述:**
编写完整的项目文档,包括架构说明、开发指南、API 文档等。

**输入:**
- 已实现的所有代码
- 设计文档

**输出:**
- 完整的项目文档

**子任务:**

#### 9.1 编写根目录 README
- 项目介绍
- 目录结构说明
- 快速开始指南
- 开发指南
- 测试指南
- 部署指南

#### 9.2 编写包文档
- Core 包 README
- DB 包 README
- Auth 包 README
- Content 包 README

#### 9.3 编写 API 文档
- API 接口列表
- 请求/响应格式说明
- 认证说明
- 错误码说明

#### 9.4 编写开发指南
- 环境搭建指南
- 代码规范指南
- 测试编写指南
- Git 工作流指南

#### 9.5 编写迁移指南
- 代码迁移说明
- 数据库迁移说明
- 配置迁移说明

**验收标准:**
- ✅ 所有文档编写完成
- ✅ 文档内容准确完整
- ✅ 文档格式规范

---

### 任务 10: 代码迁移实施 [P1]

**任务描述:**
将现有核心代码迁移到新的包结构中,保持功能不变。

**输入:**
- 现有代码 (include/)
- 新的包结构
- 迁移映射表

**输出:**
- 迁移后的代码
- 迁移验证报告

**子任务:**

#### 10.1 迁移数据库代码
- 迁移 `include/lib/databasepdo.php` 到 `packages/db/src/Connection.php`
- 调整命名空间
- 添加类型声明
- 保持功能不变

#### 10.2 迁移认证代码
- 迁移 `include/lib/loginauth.php` 到 `packages/auth/src/Authenticator.php`
- 迁移 `include/lib/passwordhash.php` 到 `packages/auth/src/PasswordHasher.php`
- 调整命名空间
- 添加类型声明

#### 10.3 迁移模型代码
- 迁移 `include/model/log_model.php` 到 `packages/content/src/Models/Post.php`
- 迁移 `include/model/user_model.php` 到 `packages/auth/src/Models/User.php`
- 迁移 `include/model/sort_model.php` 到 `packages/content/src/Models/Category.php`
- 迁移 `include/model/tag_model.php` 到 `packages/content/src/Models/Tag.php`
- 迁移 `include/model/comment_model.php` 到 `packages/content/src/Models/Comment.php`
- 调整命名空间和代码结构

#### 10.4 迁移工具函数
- 迁移 `include/lib/common.php` 到 `packages/core/src/Helper/*.php`
- 按功能拆分工具函数
- 调整命名空间

#### 10.5 验证迁移结果
- 运行单元测试
- 对比新旧代码功能
- 修复迁移问题
- 生成迁移报告

**验收标准:**
- ✅ 所有核心代码迁移完成
- ✅ 迁移后功能保持不变
- ✅ 单元测试全部通过
- ✅ 代码符合 PSR-12 规范

---

### 任务 11: 集成测试 [P2]

**任务描述:**
进行集成测试,验证各组件协同工作是否正常。

**输入:**
- 所有已实现的包和 API
- 测试数据库

**输出:**
- 集成测试报告

**子任务:**

#### 11.1 数据库集成测试
- 测试数据库连接
- 测试查询构建器
- 测试模型关联
- 测试事务处理

#### 11.2 认证集成测试
- 测试用户登录流程
- 测试会话管理
- 测试权限验证

#### 11.3 API 集成测试
- 测试 API 路由
- 测试中间件链
- 测试控制器响应
- 测试错误处理

#### 11.4 端到端测试
- 测试完整的请求流程
- 测试数据一致性
- 测试性能指标

**验收标准:**
- ✅ 所有集成测试通过
- ✅ 组件协同工作正常
- ✅ 无严重性能问题

---

### 任务 12: 性能优化 [P2]

**任务描述:**
对关键路径进行性能优化,确保系统性能达标。

**输入:**
- 已实现的代码
- 性能测试结果

**输出:**
- 优化后的代码
- 性能测试报告

**子任务:**

#### 12.1 数据库查询优化
- 分析慢查询
- 优化 N+1 查询
- 添加必要索引
- 优化查询语句

#### 12.2 缓存优化
- 实现配置缓存
- 实现查询结果缓存
- 实现路由缓存

#### 12.3 代码优化
- 优化热点代码
- 减少不必要的对象创建
- 优化自动加载

#### 12.4 性能测试
- 进行压力测试
- 分析性能瓶颈
- 生成性能报告

**验收标准:**
- ✅ API 响应时间 < 200ms (P95)
- ✅ 数据库查询优化完成
- ✅ 无明显性能瓶颈

---

### 任务 13: 安全检查 [P1]

**任务描述:**
进行安全检查,确保系统安全性。

**输入:**
- 已实现的代码
- 安全检查工具

**输出:**
- 安全检查报告
- 安全修复

**子任务:**

#### 13.1 代码安全检查
- 检查 SQL 注入风险
- 检查 XSS 风险
- 检查 CSRF 风险
- 检查文件包含风险

#### 13.2 依赖安全检查
- 检查依赖包安全漏洞
- 更新有漏洞的依赖
- 生成依赖安全报告

#### 13.3 配置安全检查
- 检查敏感配置泄露
- 检查文件权限
- 检查调试模式关闭

#### 13.4 安全加固
- 修复安全问题
- 添加安全头
- 配置安全策略

**验收标准:**
- ✅ 无高危安全漏洞
- ✅ 依赖包安全
- ✅ 配置安全
- ✅ 安全策略配置完成

---

### 任务 14: 代码质量检查 [P1]

**任务描述:**
进行代码质量检查,确保代码符合规范。

**输入:**
- 已实现的代码
- 代码质量工具

**输出:**
- 代码质量报告
- 代码改进

**子任务:**

#### 14.1 代码风格检查
- 运行 PHP_CodeSniffer
- 修复代码风格问题
- 确保符合 PSR-12

#### 14.2 静态分析
- 运行 PHPStan
- 修复类型问题
- 修复潜在错误

#### 14.3 代码复杂度分析
- 分析代码复杂度
- 重构复杂代码
- 降低圈复杂度

#### 14.4 代码重复检查
- 检查重复代码
- 提取公共代码
- 减少代码重复

**验收标准:**
- ✅ 代码风格检查通过
- ✅ 静态分析无错误
- ✅ 代码复杂度合理
- ✅ 无明显代码重复

---

### 任务 15: 最终验收 [P0]

**任务描述:**
进行最终验收,确保 M1 阶段目标全部达成。

**输入:**
- 所有已实现的功能
- 需求规格文档
- 验收标准

**输出:**
- 验收报告
- M1 阶段总结

**子任务:**

#### 15.1 功能验收
- 检查 Monorepo 目录结构
- 检查核心包功能
- 检查 Admin API 功能
- 检查开发工具链配置
- 生成功能验收报告

#### 15.2 质量验收
- 检查代码风格
- 检查测试覆盖率
- 检查静态分析结果
- 生成质量验收报告

#### 15.3 兼容性验收
- 检查数据库兼容性
- 检查功能兼容性
- 检查迁移脚本
- 生成兼容性验收报告

#### 15.4 文档验收
- 检查文档完整性
- 检查文档准确性
- 生成文档验收报告

#### 15.5 生成 M1 阶段总结
- 总结完成情况
- 总结遇到的问题
- 总结经验教训
- 规划 M2 阶段工作

**验收标准:**
- ✅ 所有功能验收通过
- ✅ 所有质量验收通过
- ✅ 所有兼容性验收通过
- ✅ 文档验收通过
- ✅ M1 阶段总结完成

---

## 4. 任务执行建议

### 4.1 执行顺序

建议按照以下顺序执行任务:

1. **第一周**: 任务 1-3 (目录结构、Core 包、DB 包)
2. **第二周**: 任务 4-6 (Auth 包、Content 包、Admin API)
3. **第三周**: 任务 7-15 (工具链、测试、文档、验收)

### 4.2 关键里程碑

- **里程碑 1**: 目录结构和 Core 包完成 (第 1 周末)
- **里程碑 2**: 所有核心包和 API 完成 (第 2 周末)
- **里程碑 3**: M1 阶段验收通过 (第 3 周末)

### 4.3 风险提示

- **依赖风险**: 包之间有依赖关系,需按顺序实现
- **兼容性风险**: 迁移代码时需保持功能不变
- **测试风险**: 需要充分的测试覆盖

### 4.4 质量保证

- 每个任务完成后进行代码审查
- 每个包完成后运行单元测试
- 每周进行集成测试
- 最终进行全面验收

## 5. 附录

### 5.1 任务状态跟踪

| 任务 ID | 任务名称 | 优先级 | 状态 | 完成日期 |
|--------|---------|--------|------|---------|
| 1 | 创建 Monorepo 目录结构 | P0 | 待开始 | - |
| 2 | 实现 Core 包基础功能 | P0 | 待开始 | - |
| 3 | 实现 DB 包数据库访问层 | P0 | 待开始 | - |
| 4 | 实现 Auth 包用户认证模块 | P0 | 待开始 | - |
| 5 | 实现 Content 包内容管理核心 | P0 | 待开始 | - |
| 6 | 搭建 Admin API 基础服务 | P0 | 待开始 | - |
| 7 | 配置开发工具链 | P1 | 待开始 | - |
| 8 | 编写单元测试 | P1 | 待开始 | - |
| 9 | 编写项目文档 | P1 | 待开始 | - |
| 10 | 代码迁移实施 | P1 | 待开始 | - |
| 11 | 集成测试 | P2 | 待开始 | - |
| 12 | 性能优化 | P2 | 待开始 | - |
| 13 | 安全检查 | P1 | 待开始 | - |
| 14 | 代码质量检查 | P1 | 待开始 | - |
| 15 | 最终验收 | P0 | 待开始 | - |

### 5.2 变更历史

| 版本 | 日期 | 变更内容 | 作者 |
|-----|------|---------|-----|
| v1.0.0 | 2026-03-28 | 初始版本创建 | SDD Agent |
| v1.1.0 | 2026-03-28 | 基于代码审查更新任务 | SDD Agent |

---

## 6. 基于代码审查的重构任务

### 6.1 审查结果摘要

根据代码审查报告（refactor-review.md），当前实现完成度约 **65%**，主要缺失：

- **Core 包**: 缺少 Logger、Container、Helper 类
- **DB 包**: 缺少 Model 基类、ConnectionManager、Migration
- **Auth 包**: 缺少 PermissionChecker、User Model
- **Content 包**: 缺少 Comment/Media Model、Service 层、模型关联
- **Admin API**: 缺少 Middleware、Response、Validator、完整 Controller
- **测试**: 完全缺失（0%覆盖率）

### 6.2 重构任务清单

#### 重构任务 R1: 完善 Core 包 [P0]

**状态**: ⚠️ 部分完成 (70%)
**优先级**: 最高
**预计工时**: 2-3 天

**子任务**:

##### R1.1 实现 Logger (4小时)
- [ ] 创建 `LogManager` 类
- [ ] 集成 Monolog 包
- [ ] 配置多通道日志 (file, syslog, stderr)
- [ ] 实现日志级别控制
- [ ] 编写单元测试

##### R1.2 实现 Container (6小时)
- [ ] 创建 `ContainerInterface` (PSR-11)
- [ ] 创建 `Container` 类实现
- [ ] 实现依赖注入功能
- [ ] 实现服务提供者 (ServiceProvider)
- [ ] 支持单例绑定
- [ ] 编写单元测试

##### R1.3 实现 Helper 类 (3小时)
- [ ] 创建 `Arr` 类 (数组操作)
- [ ] 创建 `Str` 类 (字符串操作)
- [ ] 创建 `Path` 类 (路径操作)
- [ ] 编写单元测试

##### R1.4 实现 JsonLoader (1小时)
- [ ] 创建 `JsonLoader` 类
- [ ] 支持 JSON 配置文件加载
- [ ] 编写单元测试

**验收标准**:
- ✅ Logger 支持 PSR-3 规范
- ✅ Container 支持 PSR-11 规范
- ✅ 所有 Helper 类有完整测试
- ✅ Core 包测试覆盖率 >= 80%

---

#### 重构任务 R2: 完善 DB 包 [P0]

**状态**: ⚠️ 部分完成 (80%)
**优先级**: 最高
**预计工时**: 2-3 天
**依赖**: R1 (Container)

**子任务**:

##### R2.1 实现 Model 基类 (6小时)
- [ ] 创建 `Model` 抽象基类
- [ ] 实现 CRUD 操作封装
- [ ] 支持批量赋值 (fillable)
- [ ] 支持时间戳自动维护
- [ ] 实现软删除
- [ ] 编写单元测试

##### R2.2 实现 ConnectionManager (4小时)
- [ ] 创建 `ConnectionManager` 类
- [ ] 支持多数据库连接管理
- [ ] 支持读写分离
- [ ] 实现连接池 (可选)
- [ ] 编写单元测试

##### R2.3 实现 Migration (4小时)
- [ ] 创建 `Migration` 类
- [ ] 创建 `Schema` 构建器
- [ ] 支持表创建、修改、删除
- [ ] 编写单元测试

**验收标准**:
- ✅ Model 基类支持完整 CRUD
- ✅ ConnectionManager 支持多连接
- ✅ Migration 工具可用
- ✅ DB 包测试覆盖率 >= 75%

---

#### 重构任务 R3: 完善 Auth 包 [P0]

**状态**: ⚠️ 部分完成 (75%)
**优先级**: 高
**预计工时**: 1-2 天
**依赖**: R2 (Model 基类)

**子任务**:

##### R3.1 实现 PermissionChecker (4小时)
- [ ] 创建 `PermissionChecker` 类
- [ ] 实现角色检查
- [ ] 实现权限检查
- [ ] 支持角色和权限双重验证
- [ ] 编写单元测试

##### R3.2 实现 User Model (3小时)
- [ ] 创建 `User` 模型类 (继承 Model 基类)
- [ ] 映射现有用户表结构
- [ ] 实现用户数据访问
- [ ] 编写单元测试

**验收标准**:
- ✅ PermissionChecker 功能完整
- ✅ User Model 支持完整 CRUD
- ✅ Auth 包测试覆盖率 >= 75%

---

#### 重构任务 R4: 完善 Content 包 [P0]

**状态**: ⚠️ 部分完成 (60%)
**优先级**: 高
**预计工时**: 2-3 天
**依赖**: R2 (Model 基类)

**子任务**:

##### R4.1 实现 Comment Model (3小时)
- [ ] 创建 `Comment` 模型类 (继承 Model 基类)
- [ ] 映射 comment 表结构
- [ ] 实现评论审核机制
- [ ] 实现评论层级支持
- [ ] 编写单元测试

##### R4.2 实现 Media Model (2小时)
- [ ] 创建 `Media` 模型类 (继承 Model 基类)
- [ ] 映射 media 表结构
- [ ] 实现媒体文件管理
- [ ] 编写单元测试

##### R4.3 实现 Service 层 (6小时)
- [ ] 创建 `PostService` 类
- [ ] 创建 `CategoryService` 类
- [ ] 创建 `TagService` 类
- [ ] 创建 `CommentService` 类
- [ ] 实现业务逻辑封装
- [ ] 编写单元测试

##### R4.4 实现模型关联 (4小时)
- [ ] 实现文章-分类关联
- [ ] 实现文章-标签关联 (多对多)
- [ ] 实现文章-评论关联 (一对多)
- [ ] 编写关联测试

##### R4.5 完善 Category 和 Tag Model (2小时)
- [ ] 完善 Category Model 实现
- [ ] 完善 Tag Model 实现
- [ ] 添加关联方法
- [ ] 编写单元测试

**验收标准**:
- ✅ 所有模型支持完整 CRUD
- ✅ 模型关联关系正确
- ✅ Service 层业务逻辑完整
- ✅ Content 包测试覆盖率 >= 70%

---

#### 重构任务 R5: 完善 Admin API [P1]

**状态**: ⚠️ 部分完成 (50%)
**优先级**: 高
**预计工时**: 3-4 天
**依赖**: R1-R4

**子任务**:

##### R5.1 实现 Middleware (6小时)
- [ ] 创建 `MiddlewareInterface` 接口
- [ ] 创建 `AuthMiddleware` (认证中间件)
- [ ] 创建 `CorsMiddleware` (跨域中间件)
- [ ] 创建 `LogMiddleware` (日志中间件)
- [ ] 创建 `PermissionMiddleware` (权限中间件)
- [ ] 实现中间件链式调用
- [ ] 编写单元测试

##### R5.2 实现 Response (3小时)
- [ ] 创建 `JsonResponse` 类
- [ ] 实现统一响应格式
- [ ] 实现错误响应格式
- [ ] 实现分页响应格式
- [ ] 编写单元测试

##### R5.3 实现 Validator (4小时)
- [ ] 创建 `Validator` 类
- [ ] 实现常用验证规则
- [ ] 实现自定义验证规则
- [ ] 编写单元测试

##### R5.4 完善 Controller (4小时)
- [ ] 创建 `TagController` 类
- [ ] 创建 `CommentController` 类
- [ ] 创建 `MediaController` 类
- [ ] 为所有 Controller 添加请求验证
- [ ] 为所有 Controller 实现响应格式化
- [ ] 编写单元测试

##### R5.5 实现 API 入口 (2小时)
- [ ] 创建 `public/index.php` 入口文件
- [ ] 创建 `.htaccess` 文件 (URL 重写)
- [ ] 实现应用启动流程

**验收标准**:
- ✅ 中间件链式调用正常
- ✅ 响应格式统一
- ✅ 请求验证功能完整
- ✅ API 测试覆盖率 >= 70%

---

#### 重构任务 R6: 完善配置文件 [P1]

**状态**: ⚠️ 部分完成 (80%)
**优先级**: 中
**预计工时**: 1 天

**子任务**:

##### R6.1 创建应用配置 (2小时)
- [ ] 创建 `config/app.php`
- [ ] 配置应用名称、环境、URL 等

##### R6.2 创建数据库配置 (1小时)
- [ ] 创建 `config/database.php`
- [ ] 配置数据库连接参数

##### R6.3 创建日志配置 (1小时)
- [ ] 创建 `config/logging.php`
- [ ] 配置日志通道和级别

##### R6.4 创建安全配置 (2小时)
- [ ] 创建 `config/security.php`
- [ ] 配置密码加密、会话、CORS 等

##### R6.5 完善包配置 (2小时)
- [ ] 完善各包的 `composer.json`
- [ ] 添加包说明文档

**验收标准**:
- ✅ 所有配置文件创建完成
- ✅ 配置项有清晰注释
- ✅ 配置加载优先级正确

---

#### 重构任务 R7: 编写单元测试 [P1]

**状态**: ❌ 未开始 (0%)
**优先级**: 高
**预计工时**: 3-4 天
**依赖**: R1-R6

**子任务**:

##### R7.1 Core 包测试 (1天)
- [ ] ConfigManager 测试
- [ ] Logger 测试
- [ ] EventDispatcher 测试
- [ ] Container 测试
- [ ] Helper 测试

##### R7.2 DB 包测试 (1天)
- [ ] ConnectionManager 测试
- [ ] QueryBuilder 测试
- [ ] Model 测试
- [ ] Migration 测试

##### R7.3 Auth 包测试 (0.5天)
- [ ] Authenticator 测试
- [ ] SessionManager 测试
- [ ] PasswordHasher 测试
- [ ] PermissionChecker 测试

##### R7.4 Content 包测试 (1天)
- [ ] Post 模型测试
- [ ] Category 模型测试
- [ ] Tag 模型测试
- [ ] Comment 模型测试
- [ ] Service 层测试

##### R7.5 API 测试 (0.5天)
- [ ] 路由测试
- [ ] 中间件测试
- [ ] 控制器测试
- [ ] 响应格式化测试

##### R7.6 生成覆盖率报告 (1小时)
- [ ] 运行所有测试
- [ ] 生成覆盖率报告
- [ ] 分析覆盖率数据

**验收标准**:
- ✅ 所有测试通过
- ✅ 整体测试覆盖率 >= 60%
- ✅ 核心包测试覆盖率 >= 75%

---

### 6.3 重构执行计划

#### 第一阶段 (第 1-2 周)

**目标**: 完成核心包和基础功能

**任务**:
- R1: 完善 Core 包 (2-3 天)
- R2: 完善 DB 包 (2-3 天)
- R3: 完善 Auth 包 (1-2 天)
- R4: 完善 Content 包 (2-3 天)

**里程碑**: 所有核心包功能完整，测试覆盖率达标

---

#### 第二阶段 (第 3 周)

**目标**: 完成 API 服务和测试

**任务**:
- R5: 完善 Admin API (3-4 天)
- R6: 完善配置文件 (1 天)
- R7: 编写单元测试 (3-4 天)

**里程碑**: API 服务完整，所有测试通过

---

#### 第三阶段 (第 4 周，可选)

**目标**: 完善文档和集成测试

**任务**:
- 任务 8: 编写文档 (2 天)
- 任务 9: 集成测试 (1-2 天)

**里程碑**: 文档完整，集成测试通过

---

### 6.4 重构任务状态跟踪

| 任务 ID | 任务名称 | 优先级 | 当前状态 | 完成度 | 预计工时 |
|--------|---------|--------|---------|--------|---------|
| R1 | 完善 Core 包 | P0 | ⚠️ 部分完成 | 70% | 2-3 天 |
| R2 | 完善 DB 包 | P0 | ⚠️ 部分完成 | 80% | 2-3 天 |
| R3 | 完善 Auth 包 | P0 | ⚠️ 部分完成 | 75% | 1-2 天 |
| R4 | 完善 Content 包 | P0 | ⚠️ 部分完成 | 60% | 2-3 天 |
| R5 | 完善 Admin API | P1 | ⚠️ 部分完成 | 50% | 3-4 天 |
| R6 | 完善配置文件 | P1 | ⚠️ 部分完成 | 80% | 1 天 |
| R7 | 编写单元测试 | P1 | ❌ 未开始 | 0% | 3-4 天 |
