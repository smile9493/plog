# Plog Monorepo - M1 阶段实施完成

## 项目概述

Plog CMS 已成功重构为 Monorepo 架构,完成了 M1 阶段的所有核心任务。

## 已完成的功能

### 1. Monorepo 目录结构 ✓

创建了完整的 Monorepo 目录结构:

```
plog/
├── apps/                    # 应用程序
│   └── admin-api/          # 后台 API 服务
├── packages/               # 共享包
│   ├── core/              # 核心包
│   ├── db/                # 数据库包
│   ├── auth/              # 认证包
│   └── content/           # 内容包
├── plugins/               # 插件目录 (M3)
├── themes/                # 主题目录 (M3)
├── tooling/               # 开发工具 (M4)
├── manifests/             # 配置清单
├── config/                # 全局配置
└── tests/                 # 测试目录
```

### 2. Core 包 ✓

实现了核心基础功能:

- **配置管理器** (ConfigManager)
  - 支持点号分隔的配置键
  - 多配置源加载
  - 环境变量支持

- **配置加载器**
  - EnvLoader: 加载 .env 文件
  - PhpLoader: 加载 PHP 配置文件

- **事件调度器** (EventDispatcher)
  - 事件监听和触发
  - 优先级支持
  - 多监听器管理

- **辅助函数**
  - Arr: 数组操作辅助
  - Str: 字符串操作辅助

### 3. DB 包 ✓

实现了数据库访问层:

- **数据库连接** (Connection)
  - PDO 封装
  - 支持 MySQL 和 SQLite
  - 事务支持

- **查询构建器** (QueryBuilder)
  - 链式查询操作
  - WHERE、ORDER BY、LIMIT 支持
  - INSERT、UPDATE、DELETE 操作

### 4. Auth 包 ✓

实现了认证模块:

- **密码哈希器** (BcryptPasswordHasher)
  - Bcrypt 加密
  - 密码验证
  - 重新哈希检查

- **会话管理器** (PhpSessionManager)
  - PHP Session 封装
  - 会话配置
  - 安全选项

- **认证器** (SessionAuthenticator)
  - 用户登录/登出
  - 会话认证
  - 用户状态检查

### 5. Content 包 ✓

实现了内容管理核心:

- **文章模型** (Post)
  - CRUD 操作
  - 分页查询
  - 状态管理

- **分类模型** (Category)
  - 层级分类支持
  - CRUD 操作

- **标签模型** (Tag)
  - 标签管理
  - 文章标签关联

### 6. Admin API ✓

搭建了后台 API 服务:

- **应用框架** (Application)
  - 配置加载
  - 数据库初始化
  - 错误处理

- **路由器** (ApiRouter)
  - RESTful 路由
  - 参数解析
  - 路由分发

- **控制器**
  - PostController: 文章管理 API
  - CategoryController: 分类管理 API
  - AuthController: 认证 API

### 7. 开发工具链 ✓

配置了完整的开发工具:

- **Composer**: 依赖管理和自动加载
- **PHPUnit**: 单元测试框架
- **PHP_CodeSniffer**: 代码风格检查 (PSR-12)
- **PHPStan**: 静态分析工具

## API 接口

### 认证接口

- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出
- `GET /api/auth/user` - 获取当前用户信息

### 文章接口

- `GET /api/posts` - 获取文章列表
- `GET /api/posts/{id}` - 获取文章详情
- `POST /api/posts` - 创建文章
- `PUT /api/posts/{id}` - 更新文章
- `DELETE /api/posts/{id}` - 删除文章

### 分类接口

- `GET /api/categories` - 获取分类列表
- `GET /api/categories/{id}` - 获取分类详情
- `POST /api/categories` - 创建分类
- `PUT /api/categories/{id}` - 更新分类
- `DELETE /api/categories/{id}` - 删除分类

## 安装和使用

### 1. 安装依赖

```bash
composer install
```

### 2. 配置环境

复制环境配置文件:

```bash
cp .env.example .env
```

编辑 `.env` 文件,配置数据库连接信息。

### 3. 运行测试

```bash
# 运行所有测试
composer test

# 生成测试覆盖率报告
composer test:coverage

# 代码风格检查
composer cs:check

# 代码风格修复
composer cs:fix

# 静态分析
composer stan

# 完整检查
composer check
```

### 4. 启动 API 服务

```bash
# 使用 PHP 内置服务器
php -S localhost:8000 -t apps/admin-api/public

# 或配置 Web 服务器指向 apps/admin-api/public
```

## 技术栈

- **PHP**: ^7.4|^8.0
- **数据库**: MySQL 5.6+ / SQLite
- **包管理**: Composer 2.0+
- **测试**: PHPUnit 9.0
- **代码风格**: PHP_CodeSniffer 3.0 (PSR-12)
- **静态分析**: PHPStan 1.0

## 架构特点

### 1. Monorepo 架构

- 统一管理多个应用和包
- 代码复用和共享
- 独立版本管理

### 2. 分层设计

- **应用层**: admin-api
- **业务层**: content, auth
- **数据层**: db
- **基础层**: core

### 3. 依赖注入

- 通过构造函数注入依赖
- 接口驱动设计
- 易于测试和扩展

### 4. PSR 标准

- PSR-4: 自动加载
- PSR-12: 代码风格
- PSR-3: 日志接口
- PSR-7: HTTP 消息
- PSR-15: HTTP 中间件

## 下一步计划

### M2 阶段: 后台管理界面

- 实现前端管理界面
- 完善后台功能
- 用户管理
- 媒体管理

### M3 阶段: 插件和主题系统

- 插件系统实现
- 主题系统重构
- Manifest 驱动
- 扩展机制

### M4 阶段: 开发工具链

- 完善开发工具
- 构建优化
- 部署脚本
- 文档完善

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License

## 联系方式

- 项目主页: https://github.com/smile9493/plog
- 问题反馈: https://github.com/smile9493/plog/issues
