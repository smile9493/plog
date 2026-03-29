# Plog CMS 领域模型接口文档

## 概述

本文档描述 Plog CMS 系统的领域模型接口定义。这些接口是 PHP 和 Rust 实现的共同契约。

**最后更新**: 2026-03-29
**版本**: v1.0.0

---

## 接口目录

```
packages/core/src/Contracts/
├── UserInterface.php           # 用户接口
├── UserRepositoryInterface.php # 用户仓储接口
├── PermissionInterface.php     # 权限接口
├── PostInterface.php           # 内容接口 (Post, Category, Tag, Comment)
├── PluginInterface.php         # 插件接口
└── ThemeInterface.php          # 主题接口
```

---

## 用户模块

### UserInterface

用户实体的基本属性和行为。

```php
interface UserInterface
{
    public function getId(): int;
    public function getUsername(): string;
    public function getNickname(): string;
    public function getEmail(): string;
    public function getRole(): string;
    public function getAvatar(): string;
    public function getDescription(): string;
    public function getCreatedAt(): int;
    public function isAdmin(): bool;
    public function isEditor(): bool;
    public function toArray(): array;
}
```

### UserRepositoryInterface

用户数据访问的抽象层。

```php
interface UserRepositoryInterface
{
    public function findById(int $id): ?UserInterface;
    public function findByUsername(string $username): ?UserInterface;
    public function findByEmail(string $email): ?UserInterface;
    public function findAll(): array;
    public function paginate(int $page, int $perPage): array;
    public function create(array $data): UserInterface;
    public function update(int $id, array $data): bool;
    public function delete(int $id): bool;
    public function count(): int;
}
```

---

## 权限模块

### PermissionInterface

权限的基本属性。

```php
interface PermissionInterface
{
    public function getId(): int;
    public function getName(): string;
    public function getSlug(): string;
    public function getDescription(): string;
    public function toArray(): array;
}
```

### RoleInterface

角色的基本属性。

```php
interface RoleInterface
{
    public function getId(): int;
    public function getName(): string;
    public function getSlug(): string;
    public function getDescription(): string;
    public function getPermissions(): array;
    public function toArray(): array;
}
```

### PermissionCheckerInterface

权限检查的抽象层。

```php
interface PermissionCheckerInterface
{
    public function hasPermission(UserInterface $user, string $permission): bool;
    public function hasRole(UserInterface $user, string $role): bool;
    public function getUserPermissions(UserInterface $user): array;
    public function getUserRoles(UserInterface $user): array;
}
```

---

## 内容模块

### PostInterface

文章实体的基本属性和行为。

**状态常量**:
- `STATUS_DRAFT` = 'draft'
- `STATUS_PUBLISHED` = 'public'
- `STATUS_PRIVATE` = 'private'
- `STATUS_HIDDEN` = 'hide'

**类型常量**:
- `TYPE_POST` = 'blog'
- `TYPE_PAGE` = 'page'

```php
interface PostInterface
{
    public function getId(): int;
    public function getTitle(): string;
    public function getContent(): string;
    public function getContentRaw(): string;
    public function getExcerpt(): string;
    public function getAuthorId(): int;
    public function getCategoryId(): int;
    public function getCover(): string;
    public function getDate(): int;
    public function getViews(): int;
    public function getCommentCount(): int;
    public function getLikeCount(): int;
    public function isTop(): bool;
    public function isAllowComment(): bool;
    public function hasPassword(): bool;
    public function isDraft(): bool;
    public function isPublished(): bool;
    public function getTags(): array;
    public function getCategory(): ?CategoryInterface;
    public function toArray(): array;
}
```

### CategoryInterface

分类实体的基本属性和行为。

```php
interface CategoryInterface
{
    public function getId(): int;
    public function getName(): string;
    public function getSlug(): string;
    public function getParentId(): int;
    public function getSortOrder(): int;
    public function getDescription(): string;
    public function hasChildren(): bool;
    public function getChildren(): array;
    public function getPostCount(): int;
    public function toArray(): array;
}
```

### TagInterface

标签实体的基本属性和行为。

```php
interface TagInterface
{
    public function getId(): int;
    public function getName(): string;
    public function getSlug(): string;
    public function getPostCount(): int;
    public function toArray(): array;
}
```

### CommentInterface

评论实体的基本属性和行为。

**状态常量**:
- `STATUS_PENDING` = 'n'
- `STATUS_APPROVED` = 'y'
- `STATUS_SPAM` = 'spam'

```php
interface CommentInterface
{
    public function getId(): int;
    public function getPostId(): int;
    public function getParentId(): int;
    public function getContent(): string;
    public function getAuthorName(): string;
    public function getAuthorEmail(): string;
    public function getAuthorUrl(): string;
    public function getAuthorIp(): string;
    public function getDate(): int;
    public function getStatus(): string;
    public function isApproved(): bool;
    public function isReply(): bool;
    public function toArray(): array;
}
```

### ContentRepositoryInterface

内容数据访问的抽象层。

```php
interface ContentRepositoryInterface
{
    // 文章
    public function findPostById(int $id): ?PostInterface;
    public function findPostBySlug(string $slug): ?PostInterface;
    public function getPosts(array $filters = [], int $page = 1, int $perPage = 20): array;
    public function createPost(array $data): PostInterface;
    public function updatePost(int $id, array $data): bool;
    public function deletePost(int $id): bool;

    // 分类
    public function findCategoryById(int $id): ?CategoryInterface;
    public function getAllCategories(): array;

    // 标签
    public function findTagById(int $id): ?TagInterface;
    public function findTagByName(string $name): ?TagInterface;

    // 评论
    public function getPostComments(int $postId, int $page = 1): array;
    public function createComment(array $data): CommentInterface;
}
```

---

## 插件模块

### PluginInterface

插件的基本属性和行为。

```php
interface PluginInterface
{
    public function getName(): string;
    public function getVersion(): string;
    public function getDescription(): string;
    public function getAuthor(): string;
    public function getSlug(): string;
    public function isActive(): bool;
    public function getCapabilities(): array;
    public function getConfig(): array;
    public function toArray(): array;
}
```

### PluginRegistryInterface

插件注册和管理的抽象层。

```php
interface PluginRegistryInterface
{
    public function register(PluginInterface $plugin): void;
    public function getAll(): array;
    public function get(string $name): ?PluginInterface;
    public function has(string $name): bool;
    public function activate(string $name): bool;
    public function deactivate(string $name): bool;
    public function getActive(): array;
}
```

### HookInterface

Hook 系统的抽象层。

```php
interface HookInterface
{
    public function register(string $hook, callable $callback, int $priority = 10): void;
    public function trigger(string $hook, ...$args): array;
    public function remove(string $hook, ?callable $callback = null): void;
    public function has(string $hook): bool;
}
```

---

## 主题模块

### ThemeInterface

主题的基本属性和行为。

```php
interface ThemeInterface
{
    public function getName(): string;
    public function getVersion(): string;
    public function getDescription(): string;
    public function getAuthor(): string;
    public function getSlug(): string;
    public function getPath(): string;
    public function getUrl(): string;
    public function getEngine(): string;
    public function getScreenshot(): string;
    public function getConfig(): array;
    public function getSupports(): array;
    public function toArray(): array;
}
```

### TemplateInterface

模板的基本属性和行为。

```php
interface TemplateInterface
{
    public function getName(): string;
    public function getPath(): string;
    public function getType(): string;
    public function render(array $data = []): string;
    public function exists(): bool;
}
```

### RenderPipelineInterface

渲染流程的抽象层。

```php
interface RenderPipelineInterface
{
    public function setTheme(ThemeInterface $theme): void;
    public function getTheme(): ThemeInterface;
    public function render(string $template, array $data = []): string;
    public function renderPost(PostInterface $post): string;
    public function renderCategoryList(CategoryInterface $category, array $posts): string;
    public function renderTagList(TagInterface $tag, array $posts): string;
    public function renderSearchResults(string $keyword, array $posts): string;
    public function addBeforeRenderHook(callable $hook): void;
    public function addAfterRenderHook(callable $hook): void;
}
```

---

## 设计原则

### SOLID 原则

1. **单一职责原则 (SRP)**: 每个接口只负责一个职责
2. **开闭原则 (OCP)**: 接口对扩展开放，对修改关闭
3. **里氏替换原则 (LSP)**: 实现可以替换接口
4. **接口隔离原则 (ISP)**: 使用多个小接口
5. **依赖倒置原则 (DIP)**: 依赖抽象而非实现

### 命名规范

- 接口以 `Interface` 后缀结尾
- Repository 以 `RepositoryInterface` 结尾
- Service 以 `ServiceInterface` 结尾

---

## 使用示例

### 注入依赖

```php
use Plog\Core\Contracts\UserRepositoryInterface;
use Plog\Core\Contracts\PostInterface;

class PostController
{
    private UserRepositoryInterface $userRepository;
    
    public function __construct(UserRepositoryInterface $userRepository)
    {
        $this->userRepository = $userRepository;
    }
}
```

### 创建实现

```php
use Plog\Core\Contracts\UserInterface;

class User implements UserInterface
{
    public function getId(): int
    {
        return $this->id;
    }
    
    // ... 其他方法实现
}
```

---

## 迁移指南

### 从现有代码迁移

1. 识别现有代码中的模型类
2. 创建对应的接口实现
3. 更新依赖注入配置
4. 测试接口兼容性

### Rust 实现

```rust
// Rust 中的对应实现
pub trait UserInterface {
    fn get_id(&self) -> i32;
    fn get_username(&self) -> &str;
    fn get_nickname(&self) -> &str;
    // ... 其他方法
}
```
