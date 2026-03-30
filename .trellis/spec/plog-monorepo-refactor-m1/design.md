# Plog Monorepo 重构 - M1 阶段技术设计文档

## 1. 文档信息

| 项目名称 | Plog CMS Monorepo 重构 - M1 阶段 |
|---------|--------------------------------|
| 版本号   | v1.0.0                         |
| 创建日期 | 2026-03-28                     |
| 最后更新 | 2026-03-28                     |
| 作者     | SDD Agent                      |
| 状态     | 待审核                         |

## 2. 架构设计概览

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Plog Monorepo 架构                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │  admin-api  │  │  admin-web  │  │site-runtime │         │
│  │  (M1 阶段)  │  │  (M2 阶段)  │  │  (M2 阶段)  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│         │                  │                  │              │
│         └──────────────────┴──────────────────┘              │
│                            │                                  │
│                   ┌────────▼────────┐                        │
│                   │  共享包层 (Packages)  │                    │
│                   ├──────────────────┤                        │
│                   │  core  │  db  │  auth  │  content  │      │
│                   └──────────────────┘                        │
│                            │                                  │
│                   ┌────────▼────────┐                        │
│                   │   基础设施层      │                        │
│                   ├──────────────────┤                        │
│                   │  Config │  Log  │  Event  │  DI          │
│                   └──────────────────┘                        │
│                                                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   plugins   │  │   themes    │  │   tooling   │         │
│  │  (M3 阶段)  │  │  (M3 阶段)  │  │  (M4 阶段)  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 技术栈选型

| 层级 | 技术选型 | 版本要求 | 说明 |
|-----|---------|---------|------|
| **运行时** | PHP | >= 7.4 | 核心语言 |
| **包管理** | Composer | >= 2.0 | 依赖管理 |
| **数据库** | MySQL | >= 5.6 | 数据存储 |
| **API 框架** | 自研轻量框架 | - | 基于 PSR-7/PSR-15 |
| **日志** | Monolog | ^2.0 | PSR-3 实现 |
| **配置** | vlucas/phpdotenv | ^5.0 | 环境变量 |
| **测试** | PHPUnit | ^9.0 | 单元测试 |
| **代码风格** | PHP_CodeSniffer | ^3.0 | PSR-12 检查 |
| **静态分析** | PHPStan | ^1.0 | 类型检查 |

### 2.3 设计原则

1. **依赖倒置原则 (DIP)**: 高层模块不依赖低层模块,都依赖抽象
2. **接口隔离原则 (ISP)**: 使用多个隔离的接口,不使用单一的总接口
3. **单一职责原则 (SRP)**: 每个类只负责一个功能
4. **开闭原则 (OCP)**: 对扩展开放,对修改关闭
5. **里氏替换原则 (LSP)**: 子类可以替换父类出现在父类能够出现的任何地方

## 3. 目录结构设计

### 3.1 Monorepo 根目录结构

```
plog-monorepo/
├── apps/                      # 应用程序目录
│   ├── admin-api/            # 后台 API 服务 (M1)
│   ├── admin-web/            # 后台前端应用 (M2)
│   ├── site-runtime/         # 前台运行时 (M2)
│   └── installer/            # 安装向导 (M2)
│
├── packages/                  # 共享包目录
│   ├── core/                 # 核心包 (M1)
│   ├── db/                   # 数据库包 (M1)
│   ├── auth/                 # 认证包 (M1)
│   ├── content/              # 内容包 (M1)
│   ├── plugin-kit/           # 插件开发包 (M3)
│   └── theme-kit/            # 主题开发包 (M3)
│
├── plugins/                   # 插件目录 (M3)
├── themes/                    # 主题目录 (M3)
├── tooling/                   # 开发工具目录 (M4)
├── manifests/                 # 配置清单目录 (M1)
│
├── config/                    # 全局配置目录
├── tests/                     # 全局测试目录
├── .env.example              # 环境变量示例
├── composer.json             # 根 Composer 配置
├── phpunit.xml               # PHPUnit 配置
├── phpcs.xml                 # 代码风格配置
└── README.md                 # 项目说明
```

### 3.2 应用目录结构 (以 admin-api 为例)

```
apps/admin-api/
├── src/
│   ├── Controller/           # 控制器
│   │   ├── AuthController.php
│   │   ├── PostController.php
│   │   └── ...
│   ├── Middleware/           # 中间件
│   │   ├── AuthMiddleware.php
│   │   ├── CorsMiddleware.php
│   │   └── ...
│   ├── Router/               # 路由
│   │   └── ApiRouter.php
│   ├── Service/              # 服务层
│   │   ├── AuthService.php
│   │   ├── PostService.php
│   │   └── ...
│   └── Application.php       # 应用入口
│
├── config/
│   ├── app.php              # 应用配置
│   ├── database.php         # 数据库配置
│   └── routes.php           # 路由配置
│
├── public/
│   └── index.php            # Web 入口
│
├── routes/
│   └── api.php              # API 路由定义
│
├── tests/                   # 应用测试
├── composer.json            # 应用依赖
└── README.md                # 应用说明
```

### 3.3 共享包目录结构 (以 core 为例)

```
packages/core/
├── src/
│   ├── Config/
│   │   ├── ConfigManager.php
│   │   ├── ConfigLoader.php
│   │   └── EnvLoader.php
│   ├── Log/
│   │   ├── Logger.php
│   │   └── LogManager.php
│   ├── Event/
│   │   ├── EventDispatcher.php
│   │   ├── Event.php
│   │   └── Listener.php
│   ├── Container/
│   │   ├── Container.php
│   │   └── ServiceProvider.php
│   └── Helper/
│       ├── Arr.php
│       ├── Str.php
│       └── ...
│
├── tests/
│   ├── Config/
│   ├── Log/
│   ├── Event/
│   └── Container/
│
├── config/                  # 包默认配置
│   └── core.php
│
├── composer.json
├── phpunit.xml
└── README.md
```

## 4. 核心包设计

### 4.1 Core 包设计

#### 4.1.1 类图

```
┌─────────────────────────────────────────┐
│         ConfigManager                    │
├─────────────────────────────────────────┤
│ - items: array                          │
│ - loaders: ConfigLoaderInterface[]      │
├─────────────────────────────────────────┤
│ + get(key: string, default: mixed): mixed│
│ + set(key: string, value: mixed): void  │
│ + has(key: string): bool                │
│ + load(loader: ConfigLoaderInterface): void│
│ + all(): array                          │
└─────────────────────────────────────────┘
              │
              │ uses
              ▼
┌─────────────────────────────────────────┐
│    ConfigLoaderInterface                 │
├─────────────────────────────────────────┤
│ + load(): array                         │
│ + supports(string $source): bool        │
└─────────────────────────────────────────┘
              △
              │
    ┌─────────┴─────────┬─────────────┐
    │                   │             │
┌───▼────┐        ┌─────▼─────┐  ┌───▼────┐
│EnvLoader│        │PhpLoader  │  │JsonLoader│
└────────┘        └───────────┘  └────────┘
```

#### 4.1.2 接口定义

```php
<?php
namespace Plog\Core\Config;

interface ConfigLoaderInterface
{
    /**
     * 加载配置
     * @return array
     */
    public function load(): array;

    /**
     * 检查是否支持该配置源
     * @param string $source
     * @return bool
     */
    public function supports(string $source): bool;
}

interface ConfigManagerInterface
{
    /**
     * 获取配置值
     * @param string $key 配置键,支持点号分隔 (如: database.mysql.host)
     * @param mixed $default 默认值
     * @return mixed
     */
    public function get(string $key, $default = null);

    /**
     * 设置配置值
     * @param string $key
     * @param mixed $value
     */
    public function set(string $key, $value): void;

    /**
     * 检查配置是否存在
     * @param string $key
     * @return bool
     */
    public function has(string $key): bool;

    /**
     * 获取所有配置
     * @return array
     */
    public function all(): array;
}
```

#### 4.1.3 核心服务实现

**ConfigManager 实现:**

```php
<?php
namespace Plog\Core\Config;

class ConfigManager implements ConfigManagerInterface
{
    private array $items = [];
    private array $loaders = [];

    public function __construct(array $loaders = [])
    {
        foreach ($loaders as $loader) {
            $this->addLoader($loader);
        }
    }

    public function get(string $key, $default = null)
    {
        // 支持点号分隔的键名: database.mysql.host
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

    public function set(string $key, $value): void
    {
        $keys = explode('.', $key);
        $items = &$this->items;

        foreach ($keys as $k) {
            if (!isset($items[$k]) || !is_array($items[$k])) {
                $items[$k] = [];
            }
            $items = &$items[$k];
        }

        $items = $value;
    }

    public function has(string $key): bool
    {
        return $this->get($key) !== null;
    }

    public function all(): array
    {
        return $this->items;
    }

    public function load(string $source): void
    {
        foreach ($this->loaders as $loader) {
            if ($loader->supports($source)) {
                $items = $loader->load($source);
                $this->items = array_merge($this->items, $items);
                return;
            }
        }

        throw new \RuntimeException("No loader supports source: {$source}");
    }

    private function addLoader(ConfigLoaderInterface $loader): void
    {
        $this->loaders[] = $loader;
    }
}
```

**EventDispatcher 实现:**

```php
<?php
namespace Plog\Core\Event;

interface EventDispatcherInterface
{
    /**
     * 触发事件
     * @param string $eventName
     * @param array $payload
     * @return array 事件处理结果
     */
    public function dispatch(string $eventName, array $payload = []): array;

    /**
     * 注册监听器
     * @param string $eventName
     * @param callable $listener
     * @param int $priority 优先级,数字越大优先级越高
     */
    public function listen(string $eventName, callable $listener, int $priority = 0): void;

    /**
     * 移除监听器
     * @param string $eventName
     */
    public function forget(string $eventName): void;
}

class EventDispatcher implements EventDispatcherInterface
{
    private array $listeners = [];

    public function dispatch(string $eventName, array $payload = []): array
    {
        $results = [];

        if (!isset($this->listeners[$eventName])) {
            return $results;
        }

        // 按优先级排序
        $listeners = $this->listeners[$eventName];
        krsort($listeners);

        foreach ($listeners as $priority => $items) {
            foreach ($items as $listener) {
                $results[] = call_user_func_array($listener, $payload);
            }
        }

        return $results;
    }

    public function listen(string $eventName, callable $listener, int $priority = 0): void
    {
        if (!isset($this->listeners[$eventName])) {
            $this->listeners[$eventName] = [];
        }

        if (!isset($this->listeners[$eventName][$priority])) {
            $this->listeners[$eventName][$priority] = [];
        }

        $this->listeners[$eventName][$priority][] = $listener;
    }

    public function forget(string $eventName): void
    {
        unset($this->listeners[$eventName]);
    }
}
```

### 4.2 DB 包设计

#### 4.2.1 类图

```
┌─────────────────────────────────────────┐
│      ConnectionManager                   │
├─────────────────────────────────────────┤
│ - connections: array                     │
│ - default: string                        │
├─────────────────────────────────────────┤
│ + connection(name?: string): Connection  │
│ + addConnection(config: array): void     │
│ + disconnect(name?: string): void        │
└─────────────────────────────────────────┘
              │
              │ creates
              ▼
┌─────────────────────────────────────────┐
│         Connection                       │
├─────────────────────────────────────────┤
│ - pdo: PDO                               │
│ - config: array                          │
├─────────────────────────────────────────┤
│ + query(sql: string): QueryBuilder       │
│ + table(table: string): QueryBuilder     │
│ + transaction(): Transaction             │
│ + getPdo(): PDO                          │
└─────────────────────────────────────────┘
              │
              │ provides
              ▼
┌─────────────────────────────────────────┐
│         QueryBuilder                     │
├─────────────────────────────────────────┤
│ - connection: Connection                 │
│ - query: array                           │
├─────────────────────────────────────────┤
│ + select(columns: array): self           │
│ + from(table: string): self              │
│ + where(column, operator, value): self   │
│ + orderBy(column, direction): self       │
│ + limit(limit): self                     │
│ + get(): array                           │
│ + first(): ?array                        │
│ + insert(data: array): int               │
│ + update(data: array): int               │
│ + delete(): int                          │
└─────────────────────────────────────────┘
```

#### 4.2.2 接口定义

```php
<?php
namespace Plog\Db;

interface ConnectionInterface
{
    /**
     * 获取 PDO 实例
     * @return \PDO
     */
    public function getPdo(): \PDO;

    /**
     * 创建查询构建器
     * @param string $table
     * @return QueryBuilderInterface
     */
    public function table(string $table): QueryBuilderInterface;

    /**
     * 执行原生 SQL
     * @param string $sql
     * @param array $bindings
     * @return \PDOStatement
     */
    public function query(string $sql, array $bindings = []): \PDOStatement;

    /**
     * 开启事务
     * @return TransactionInterface
     */
    public function beginTransaction(): TransactionInterface;
}

interface QueryBuilderInterface
{
    /**
     * SELECT 子句
     * @param array|string $columns
     * @return self
     */
    public function select($columns = ['*']): self;

    /**
     * FROM 子句
     * @param string $table
     * @return self
     */
    public function from(string $table): self;

    /**
     * WHERE 子句
     * @param string $column
     * @param string $operator
     * @param mixed $value
     * @return self
     */
    public function where(string $column, string $operator, $value): self;

    /**
     * ORDER BY 子句
     * @param string $column
     * @param string $direction
     * @return self
     */
    public function orderBy(string $column, string $direction = 'ASC'): self;

    /**
     * LIMIT 子句
     * @param int $limit
     * @return self
     */
    public function limit(int $limit): self;

    /**
     * OFFSET 子句
     * @param int $offset
     * @return self
     */
    public function offset(int $offset): self;

    /**
     * 执行查询并获取所有结果
     * @return array
     */
    public function get(): array;

    /**
     * 执行查询并获取第一条结果
     * @return array|null
     */
    public function first(): ?array;

    /**
     * 插入数据
     * @param array $data
     * @return int 插入的 ID
     */
    public function insert(array $data): int;

    /**
     * 更新数据
     * @param array $data
     * @return int 影响的行数
     */
    public function update(array $data): int;

    /**
     * 删除数据
     * @return int 影响的行数
     */
    public function delete(): int;
}
```

#### 4.2.3 Model 基类设计

```php
<?php
namespace Plog\Db\Model;

abstract class Model
{
    /**
     * 关联的表名
     * @var string
     */
    protected string $table;

    /**
     * 主键字段
     * @var string
     */
    protected string $primaryKey = 'id';

    /**
     * 可填充字段
     * @var array
     */
    protected array $fillable = [];

    /**
     * 是否使用时间戳
     * @var bool
     */
    protected bool $timestamps = true;

    /**
     * 数据库连接
     * @var ConnectionInterface
     */
    protected ConnectionInterface $connection;

    public function __construct(ConnectionInterface $connection)
    {
        $this->connection = $connection;
    }

    /**
     * 查找所有记录
     * @return array
     */
    public function all(): array
    {
        return $this->newQuery()->get();
    }

    /**
     * 根据主键查找
     * @param int $id
     * @return static|null
     */
    public function find(int $id): ?self
    {
        $result = $this->newQuery()
            ->where($this->primaryKey, '=', $id)
            ->first();

        return $result ? $this->newInstance($result) : null;
    }

    /**
     * 创建新记录
     * @param array $data
     * @return static
     */
    public function create(array $data): self
    {
        $data = $this->filterFillable($data);

        if ($this->timestamps) {
            $data['created_at'] = date('Y-m-d H:i:s');
            $data['updated_at'] = date('Y-m-d H:i:s');
        }

        $id = $this->newQuery()->insert($data);
        return $this->find($id);
    }

    /**
     * 更新记录
     * @param int $id
     * @param array $data
     * @return bool
     */
    public function update(int $id, array $data): bool
    {
        $data = $this->filterFillable($data);

        if ($this->timestamps) {
            $data['updated_at'] = date('Y-m-d H:i:s');
        }

        $affected = $this->newQuery()
            ->where($this->primaryKey, '=', $id)
            ->update($data);

        return $affected > 0;
    }

    /**
     * 删除记录
     * @param int $id
     * @return bool
     */
    public function delete(int $id): bool
    {
        $affected = $this->newQuery()
            ->where($this->primaryKey, '=', $id)
            ->delete();

        return $affected > 0;
    }

    /**
     * 创建新的查询构建器
     * @return QueryBuilderInterface
     */
    protected function newQuery(): QueryBuilderInterface
    {
        return $this->connection->table($this->table);
    }

    /**
     * 过滤可填充字段
     * @param array $data
     * @return array
     */
    protected function filterFillable(array $data): array
    {
        if (empty($this->fillable)) {
            return $data;
        }

        return array_intersect_key($data, array_flip($this->fillable));
    }

    /**
     * 创建新实例
     * @param array $attributes
     * @return static
     */
    abstract protected function newInstance(array $attributes): self;
}
```

### 4.3 Auth 包设计

#### 4.3.1 类图

```
┌─────────────────────────────────────────┐
│         Authenticator                    │
├─────────────────────────────────────────┤
│ - userProvider: UserProviderInterface    │
│ - sessionManager: SessionManagerInterface│
├─────────────────────────────────────────┤
│ + attempt(credentials: array): bool      │
│ + login(user: UserInterface): void       │
│ + logout(): void                         │
│ + check(): bool                          │
│ + user(): ?UserInterface                 │
└─────────────────────────────────────────┘
              │
              │ uses
              ▼
┌─────────────────────────────────────────┐
│    UserProviderInterface                 │
├─────────────────────────────────────────┤
│ + retrieveById(id: mixed): ?UserInterface│
│ + retrieveByCredentials(array): ?User    │
│ + validateCredentials(user, creds): bool │
└─────────────────────────────────────────┘
```

#### 4.3.2 接口定义

```php
<?php
namespace Plog\Auth;

interface UserInterface
{
    /**
     * 获取用户 ID
     * @return mixed
     */
    public function getId();

    /**
     * 获取用户名
     * @return string
     */
    public function getUsername(): string;

    /**
     * 获取密码哈希
     * @return string
     */
    public function getPasswordHash(): string;

    /**
     * 获取用户角色
     * @return array
     */
    public function getRoles(): array;

    /**
     * 获取用户权限
     * @return array
     */
    public function getPermissions(): array;
}

interface AuthenticatorInterface
{
    /**
     * 尝试登录
     * @param array $credentials
     * @return bool
     */
    public function attempt(array $credentials): bool;

    /**
     * 登录用户
     * @param UserInterface $user
     */
    public function login(UserInterface $user): void;

    /**
     * 登出
     */
    public function logout(): void;

    /**
     * 检查是否已登录
     * @return bool
     */
    public function check(): bool;

    /**
     * 获取当前用户
     * @return UserInterface|null
     */
    public function user(): ?UserInterface;
}

interface PasswordHasherInterface
{
    /**
     * 哈希密码
     * @param string $password
     * @return string
     */
    public function hash(string $password): string;

    /**
     * 验证密码
     * @param string $password
     * @param string $hash
     * @return bool
     */
    public function verify(string $password, string $hash): bool;
}
```

### 4.4 Content 包设计

#### 4.4.1 模型关系图

```
┌──────────────┐       ┌──────────────┐
│     Post     │       │   Category   │
├──────────────┤       ├──────────────┤
│ id           │       │ id           │
│ title        │       │ name         │
│ slug         │       │ slug         │
│ content      │◄──────│ parent_id    │
│ excerpt      │       │ sort_order   │
│ author_id    │       └──────────────┘
│ category_id  │
│ status       │       ┌──────────────┐
│ created_at   │       │     Tag      │
│ updated_at   │       ├──────────────┤
└──────────────┘       │ id           │
       │               │ name         │
       │               │ slug         │
       │               └──────────────┘
       │                      │
       │                      │
       │    ┌─────────────────▼─────┐
       │    │   post_tag (pivot)    │
       │    ├───────────────────────┤
       │    │ post_id               │
       │    │ tag_id                │
       │    └───────────────────────┘
       │
       │
       ▼
┌──────────────┐
│   Comment    │
├──────────────┤
│ id           │
│ post_id      │
│ user_id      │
│ content      │
│ status       │
│ created_at   │
└──────────────┘
```

#### 4.4.2 Post 模型设计

```php
<?php
namespace Plog\Content\Models;

use Plog\Db\Model\Model;
use Plog\Db\ConnectionInterface;

class Post extends Model
{
    protected string $table = 'blog';
    protected string $primaryKey = 'gid';
    protected array $fillable = [
        'title', 'slug', 'content', 'excerpt',
        'author_id', 'sortid', 'date', 'hide',
        'allow_remark', 'top', 'type', 'password'
    ];
    protected bool $timestamps = false; // 使用现有字段

    // 文章状态常量
    const STATUS_DRAFT = 'draft';
    const STATUS_PUBLISHED = 'public';
    const STATUS_PRIVATE = 'private';
    const STATUS_HIDDEN = 'hide';

    // 文章类型常量
    const TYPE_POST = 'blog';
    const TYPE_PAGE = 'page';

    public function __construct(ConnectionInterface $connection)
    {
        parent::__construct($connection);
    }

    /**
     * 获取已发布的文章
     * @return array
     */
    public function published(): array
    {
        return $this->newQuery()
            ->where('hide', '=', 'n')
            ->where('type', '=', self::TYPE_POST)
            ->orderBy('date', 'DESC')
            ->get();
    }

    /**
     * 根据别名查找
     * @param string $slug
     * @return self|null
     */
    public function findBySlug(string $slug): ?self
    {
        $result = $this->newQuery()
            ->where('slug', '=', $slug)
            ->first();

        return $result ? $this->newInstance($result) : null;
    }

    /**
     * 获取文章分类
     * @return Category|null
     */
    public function category(): ?Category
    {
        // 实现关联查询
    }

    /**
     * 获取文章标签
     * @return array
     */
    public function tags(): array
    {
        // 实现多对多关联
    }

    /**
     * 获取文章评论
     * @return array
     */
    public function comments(): array
    {
        // 实现一对多关联
    }

    protected function newInstance(array $attributes): self
    {
        $post = new self($this->connection);
        // 设置属性
        foreach ($attributes as $key => $value) {
            $post->{$key} = $value;
        }
        return $post;
    }
}
```

## 5. Admin API 设计

### 5.1 API 架构

```
┌─────────────────────────────────────────────────────┐
│                   Request Flow                       │
├─────────────────────────────────────────────────────┤
│                                                      │
│  Request ──► Router ──► Middleware ──► Controller   │
│                                                      │
│                      │                               │
│                      ▼                               │
│                                                      │
│              Service Layer ──► Repository Layer      │
│                                                      │
│                      │                               │
│                      ▼                               │
│                                                      │
│                   Database                           │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### 5.2 路由设计

```php
<?php
// routes/api.php

use Plog\AdminApi\Controller\AuthController;
use Plog\AdminApi\Controller\PostController;
use Plog\AdminApi\Controller\CategoryController;

// 认证路由 (无需认证)
$router->group(['prefix' => 'auth'], function ($router) {
    $router->post('login', [AuthController::class, 'login']);
    $router->post('logout', [AuthController::class, 'logout']);
    $router->get('user', [AuthController::class, 'user'])->middleware('auth');
    $router->post('refresh', [AuthController::class, 'refresh'])->middleware('auth');
});

// 内容管理路由 (需要认证)
$router->group(['prefix' => 'posts', 'middleware' => ['auth', 'permission:manage_posts']], function ($router) {
    $router->get('/', [PostController::class, 'index']);
    $router->get('/{id}', [PostController::class, 'show']);
    $router->post('/', [PostController::class, 'store']);
    $router->put('/{id}', [PostController::class, 'update']);
    $router->delete('/{id}', [PostController::class, 'destroy']);
});

// 分类管理路由
$router->group(['prefix' => 'categories', 'middleware' => ['auth', 'permission:manage_categories']], function ($router) {
    $router->get('/', [CategoryController::class, 'index']);
    $router->get('/{id}', [CategoryController::class, 'show']);
    $router->post('/', [CategoryController::class, 'store']);
    $router->put('/{id}', [CategoryController::class, 'update']);
    $router->delete('/{id}', [CategoryController::class, 'destroy']);
});
```

### 5.3 中间件设计

```php
<?php
namespace Plog\AdminApi\Middleware;

use Plog\Core\Http\RequestInterface;
use Plog\Core\Http\ResponseInterface;
use Plog\Auth\AuthenticatorInterface;

class AuthMiddleware
{
    private AuthenticatorInterface $auth;

    public function __construct(AuthenticatorInterface $auth)
    {
        $this->auth = $auth;
    }

    public function handle(RequestInterface $request, callable $next): ResponseInterface
    {
        // 检查是否已登录
        if (!$this->auth->check()) {
            return $this->unauthorizedResponse();
        }

        // 将用户信息注入请求
        $request->setUser($this->auth->user());

        return $next($request);
    }

    private function unauthorizedResponse(): ResponseInterface
    {
        return new JsonResponse([
            'success' => false,
            'error' => [
                'code' => 401,
                'message' => 'Unauthorized'
            ]
        ], 401);
    }
}
```

### 5.4 控制器设计

```php
<?php
namespace Plog\AdminApi\Controller;

use Plog\Core\Http\RequestInterface;
use Plog\Core\Http\JsonResponse;
use Plog\Content\Services\PostService;

class PostController
{
    private PostService $postService;

    public function __construct(PostService $postService)
    {
        $this->postService = $postService;
    }

    /**
     * 获取文章列表
     * GET /api/posts
     */
    public function index(RequestInterface $request): JsonResponse
    {
        $page = $request->get('page', 1);
        $perPage = $request->get('per_page', 20);
        $status = $request->get('status');

        $result = $this->postService->paginate($page, $perPage, $status);

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

    /**
     * 获取文章详情
     * GET /api/posts/{id}
     */
    public function show(RequestInterface $request, int $id): JsonResponse
    {
        $post = $this->postService->find($id);

        if (!$post) {
            return new JsonResponse([
                'success' => false,
                'error' => [
                    'code' => 404,
                    'message' => 'Post not found'
                ]
            ], 404);
        }

        return new JsonResponse([
            'success' => true,
            'data' => $post
        ]);
    }

    /**
     * 创建文章
     * POST /api/posts
     */
    public function store(RequestInterface $request): JsonResponse
    {
        $data = $request->all();

        // 验证数据
        $validator = $this->validate($data, [
            'title' => 'required|string|max:255',
            'content' => 'required|string',
            'category_id' => 'integer',
            'status' => 'in:draft,published,private'
        ]);

        if ($validator->fails()) {
            return new JsonResponse([
                'success' => false,
                'error' => [
                    'code' => 422,
                    'message' => 'Validation failed',
                    'errors' => $validator->errors()
                ]
            ], 422);
        }

        $post = $this->postService->create($data);

        return new JsonResponse([
            'success' => true,
            'data' => $post
        ], 201);
    }

    /**
     * 更新文章
     * PUT /api/posts/{id}
     */
    public function update(RequestInterface $request, int $id): JsonResponse
    {
        $data = $request->all();

        $post = $this->postService->update($id, $data);

        if (!$post) {
            return new JsonResponse([
                'success' => false,
                'error' => [
                    'code' => 404,
                    'message' => 'Post not found'
                ]
            ], 404);
        }

        return new JsonResponse([
            'success' => true,
            'data' => $post
        ]);
    }

    /**
     * 删除文章
     * DELETE /api/posts/{id}
     */
    public function destroy(int $id): JsonResponse
    {
        $success = $this->postService->delete($id);

        if (!$success) {
            return new JsonResponse([
                'success' => false,
                'error' => [
                    'code' => 404,
                    'message' => 'Post not found'
                ]
            ], 404);
        }

        return new JsonResponse([
            'success' => true,
            'message' => 'Post deleted successfully'
        ]);
    }
}
```

### 5.5 响应格式规范

**成功响应:**

```json
{
  "success": true,
  "data": {
    // 实际数据
  },
  "meta": {
    // 元数据 (分页信息等)
  }
}
```

**错误响应:**

```json
{
  "success": false,
  "error": {
    "code": 400,
    "message": "Error message",
    "errors": {
      // 详细错误信息 (验证错误等)
    }
  }
}
```

## 6. 数据库设计

### 6.1 现有数据库表结构 (保持不变)

M1 阶段保持现有数据库表结构不变,主要包括:

- `blog` - 文章表
- `sort` - 分类表
- `tag` - 标签表
- `tag_relation` - 标签关联表
- `comment` - 评论表
- `user` - 用户表
- `options` - 配置表
- `media` - 媒体表
- `navi` - 导航表
- `link` - 友链表

### 6.2 数据库连接配置

```php
<?php
// config/database.php

return [
    'default' => env('DB_CONNECTION', 'mysql'),

    'connections' => [
        'mysql' => [
            'driver' => 'mysql',
            'host' => env('DB_HOST', 'localhost'),
            'port' => env('DB_PORT', 3306),
            'database' => env('DB_NAME', 'plog'),
            'username' => env('DB_USER', 'root'),
            'password' => env('DB_PASSWD', ''),
            'charset' => 'utf8mb4',
            'collation' => 'utf8mb4_unicode_ci',
            'prefix' => '',
            'strict' => true,
            'engine' => null,
            'options' => [
                PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
                PDO::ATTR_EMULATE_PREPARES => false,
            ],
        ],

        'mysql_read' => [
            // 读库配置 (读写分离)
            'driver' => 'mysql',
            'host' => env('DB_HOST_READ', env('DB_HOST', 'localhost')),
            // ... 其他配置同上
        ],
    ],
];
```

## 7. 配置管理设计

### 7.1 环境变量配置

```bash
# .env.example

# 应用配置
APP_NAME=Plog CMS
APP_ENV=local
APP_DEBUG=true
APP_URL=http://localhost

# 数据库配置
DB_CONNECTION=mysql
DB_HOST=localhost
DB_PORT=3306
DB_NAME=plog
DB_USER=plog
DB_PASSWD=your_password

# 认证配置
AUTH_DRIVER=session
AUTH_SECRET=your-secret-key-here
AUTH_TOKEN_TTL=3600

# 日志配置
LOG_CHANNEL=file
LOG_LEVEL=debug
LOG_PATH=logs/app.log

# 缓存配置
CACHE_DRIVER=file
CACHE_PATH=cache/
CACHE_TTL=3600
```

### 7.2 配置加载优先级

```
环境变量 (.env) > 配置文件 (config/*.php) > 默认值
```

## 8. 开发工具链设计

### 8.1 Composer 配置

**根 composer.json:**

```json
{
  "name": "plog/plog-monorepo",
  "type": "project",
  "description": "Plog CMS Monorepo",
  "keywords": ["cms", "blog", "monorepo"],
  "license": "MIT",
  "require": {
    "php": "^7.4|^8.0",
    "ext-pdo": "*",
    "ext-json": "*",
    "vlucas/phpdotenv": "^5.0",
    "monolog/monolog": "^2.0",
    "psr/container": "^1.0",
    "psr/log": "^1.1",
    "psr/http-message": "^1.0"
  },
  "require-dev": {
    "phpunit/phpunit": "^9.0",
    "squizlabs/php_codesniffer": "^3.0",
    "phpstan/phpstan": "^1.0"
  },
  "autoload": {
    "psr-4": {
      "Plog\\Core\\": "packages/core/src/",
      "Plog\\Db\\": "packages/db/src/",
      "Plog\\Auth\\": "packages/auth/src/",
      "Plog\\Content\\": "packages/content/src/",
      "Plog\\AdminApi\\": "apps/admin-api/src/"
    }
  },
  "autoload-dev": {
    "psr-4": {
      "Plog\\Tests\\": "tests/"
    }
  },
  "scripts": {
    "test": "phpunit",
    "test:coverage": "phpunit --coverage-html coverage",
    "cs:check": "phpcs --standard=PSR12",
    "cs:fix": "phpcbf --standard=PSR12",
    "stan": "phpstan analyse",
    "check": [
      "@cs:check",
      "@stan",
      "@test"
    ]
  },
  "repositories": [
    {
      "type": "path",
      "url": "packages/*",
      "options": {
        "symlink": true
      }
    }
  ],
  "minimum-stability": "stable",
  "prefer-stable": true
}
```

### 8.2 PHPUnit 配置

```xml
<?xml version="1.0" encoding="UTF-8"?>
<phpunit xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation="https://schema.phpunit.de/9.0/phpunit.xsd"
         bootstrap="vendor/autoload.php"
         colors="true"
         verbose="true">
    <testsuites>
        <testsuite name="Unit">
            <directory suffix="Test.php">tests/Unit</directory>
        </testsuite>
        <testsuite name="Integration">
            <directory suffix="Test.php">tests/Integration</directory>
        </testsuite>
    </testsuites>

    <coverage processUncoveredFiles="true">
        <include>
            <directory suffix=".php">packages/*/src</directory>
            <directory suffix=".php">apps/*/src</directory>
        </include>
        <report>
            <html outputDirectory="coverage"/>
        </report>
    </coverage>

    <php>
        <env name="APP_ENV" value="testing"/>
        <env name="DB_CONNECTION" value="sqlite"/>
        <env name="DB_DATABASE" value=":memory:"/>
    </php>
</phpunit>
```

### 8.3 PHP_CodeSniffer 配置

```xml
<?xml version="1.0"?>
<ruleset name="Plog">
    <description>Plog CMS Coding Standard</description>

    <!-- 使用 PSR-12 标准 -->
    <rule ref="PSR12"/>

    <!-- 检查的文件 -->
    <file>packages</file>
    <file>apps</file>

    <!-- 排除的目录 -->
    <exclude-pattern>*/vendor/*</exclude-pattern>
    <exclude-pattern>*/tests/*</exclude-pattern>

    <!-- 显示进度 -->
    <arg value="p"/>
    <arg name="colors"/>

    <!-- 报告格式 -->
    <arg name="report" value="full"/>
</ruleset>
```

## 9. 迁移策略设计

### 9.1 代码迁移流程

```
┌─────────────────────────────────────────────────────┐
│              代码迁移流程                             │
├─────────────────────────────────────────────────────┤
│                                                      │
│  1. 分析现有代码结构                                 │
│     └─► 识别核心功能模块                             │
│                                                      │
│  2. 创建新的包结构                                   │
│     └─► 建立目录和接口                               │
│                                                      │
│  3. 迁移核心代码                                     │
│     └─► 保持功能不变                                 │
│     └─► 调整命名空间                                 │
│     └─► 添加类型声明                                 │
│                                                      │
│  4. 编写单元测试                                     │
│     └─► 确保功能正确                                 │
│                                                      │
│  5. 集成测试                                         │
│     └─► 验证整体功能                                 │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### 9.2 迁移映射表

| 原文件路径 | 新包路径 | 说明 |
|-----------|---------|------|
| `include/lib/databasepdo.php` | `packages/db/src/Connection.php` | 数据库连接 |
| `include/lib/common.php` | `packages/core/src/Helper/*.php` | 工具函数 |
| `include/lib/loginauth.php` | `packages/auth/src/Authenticator.php` | 认证逻辑 |
| `include/lib/passwordhash.php` | `packages/auth/src/PasswordHasher.php` | 密码哈希 |
| `include/model/log_model.php` | `packages/content/src/Models/Post.php` | 文章模型 |
| `include/model/user_model.php` | `packages/auth/src/Models/User.php` | 用户模型 |
| `include/model/sort_model.php` | `packages/content/src/Models/Category.php` | 分类模型 |
| `include/model/tag_model.php` | `packages/content/src/Models/Tag.php` | 标签模型 |
| `include/model/comment_model.php` | `packages/content/src/Models/Comment.php` | 评论模型 |

## 10. 性能优化设计

### 10.1 数据库优化

- **连接池**: 使用持久连接减少连接开销
- **查询优化**: 使用查询构建器避免 N+1 查询
- **索引优化**: 保持现有索引,必要时添加新索引
- **慢查询日志**: 记录执行时间超过阈值的查询

### 10.2 缓存策略

```php
<?php
namespace Plog\Core\Cache;

interface CacheInterface
{
    public function get(string $key, $default = null);
    public function set(string $key, $value, int $ttl = 3600): bool;
    public function has(string $key): bool;
    public function delete(string $key): bool;
    public function flush(): bool;
}

// 使用示例
$cache->set('posts:featured', $posts, 3600);
$posts = $cache->get('posts:featured', function () {
    return $this->postService->getFeatured();
});
```

## 11. 安全设计

### 11.1 安全措施

- **SQL 注入防护**: 使用 PDO 预处理语句
- **XSS 防护**: 输出时转义 HTML
- **CSRF 防护**: API 使用 Token 认证
- **密码安全**: 使用 bcrypt 或 argon2 加密
- **会话安全**: 使用安全的会话配置

### 11.2 安全配置

```php
<?php
// config/security.php

return [
    'password' => [
        'algorithm' => PASSWORD_BCRYPT,
        'options' => [
            'cost' => 12,
        ],
    ],

    'session' => [
        'name' => 'plog_session',
        'lifetime' => 7200,
        'path' => '/',
        'domain' => null,
        'secure' => env('APP_ENV') === 'production',
        'httponly' => true,
        'samesite' => 'strict',
    ],

    'cors' => [
        'allowed_origins' => ['*'],
        'allowed_methods' => ['GET', 'POST', 'PUT', 'DELETE'],
        'allowed_headers' => ['*'],
        'exposed_headers' => [],
        'max_age' => 0,
        'supports_credentials' => false,
    ],
];
```

## 12. 测试策略

### 12.1 测试层次

```
┌─────────────────────────────────────────┐
│           测试金字塔                      │
├─────────────────────────────────────────┤
│                                          │
│              / \                          │
│             /   \                         │
│            / E2E \                        │
│           /───────\                       │
│          /         \                      │
│         / Integration \                   │
│        /─────────────\                    │
│       /               \                   │
│      /    Unit Tests   \                  │
│     /───────────────────\                 │
│                                          │
└─────────────────────────────────────────┘

Unit Tests: 70% (快速,隔离)
Integration Tests: 20% (组件交互)
E2E Tests: 10% (完整流程)
```

### 12.2 测试覆盖率目标

- **核心包**: >= 80%
- **API 控制器**: >= 70%
- **整体项目**: >= 60%

## 13. 部署设计

### 13.1 目录结构 (生产环境)

```
/var/www/plog/
├── current -> releases/20260328120000/  # 当前版本软链接
├── releases/
│   └── 20260328120000/                  # 发布版本
│       ├── apps/
│       ├── packages/
│       ├── vendor/
│       └── public/
├── shared/
│   ├── .env                             # 环境配置
│   ├── storage/
│   │   ├── logs/
│   │   ├── cache/
│   │   └── uploads/
│   └── config/
└── .gitignore
```

### 13.2 部署流程

```bash
#!/bin/bash
# 部署脚本示例

# 1. 拉取代码
git pull origin main

# 2. 安装依赖
composer install --no-dev --optimize-autoloader

# 3. 运行迁移
php artisan migrate

# 4. 清除缓存
php artisan cache:clear

# 5. 重启服务
# systemctl restart php-fpm
```

## 14. 监控和日志

### 14.1 日志配置

```php
<?php
// config/logging.php

return [
    'default' => env('LOG_CHANNEL', 'stack'),

    'channels' => [
        'stack' => [
            'driver' => 'stack',
            'channels' => ['file', 'syslog'],
        ],

        'file' => [
            'driver' => 'monolog',
            'handler' => StreamHandler::class,
            'path' => storage_path('logs/app.log'),
            'level' => 'debug',
        ],

        'syslog' => [
            'driver' => 'monolog',
            'handler' => SyslogHandler::class,
            'level' => 'info',
        ],
    ],
];
```

### 14.2 监控指标

- **应用指标**: 请求响应时间、错误率、吞吐量
- **数据库指标**: 查询时间、连接数、慢查询
- **系统指标**: CPU、内存、磁盘使用率

## 15. 附录

### 15.1 技术债务

- [ ] 完善类型声明 (PHP 8.0+ 特性)
- [ ] 添加更多单元测试
- [ ] 优化查询性能
- [ ] 完善文档

### 15.2 未来规划

- **M2 阶段**: 完整后台管理界面
- **M3 阶段**: 插件和主题系统
- **M4 阶段**: 完整开发工具链

### 15.3 参考资源

- [PHP-FIG PSR 标准](https://www.php-fig.org/psr/)
- [Composer 文档](https://getcomposer.org/doc/)
- [PHPUnit 文档](https://phpunit.de/documentation.html)
- [Monolog 文档](https://github.com/Seldaek/monolog)

### 15.4 变更历史

| 版本 | 日期 | 变更内容 | 作者 |
|-----|------|---------|-----|
| v1.0.0 | 2026-03-28 | 初始版本创建 | SDD Agent |
