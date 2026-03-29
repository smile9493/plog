<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 文章接口
 * 
 * 定义文章实体的基本属性和行为
 */
interface PostInterface
{
    /**
     * 文章状态常量
     */
    public const STATUS_DRAFT = 'draft';
    public const STATUS_PUBLISHED = 'public';
    public const STATUS_PRIVATE = 'private';
    public const STATUS_HIDDEN = 'hide';

    /**
     * 文章类型常量
     */
    public const TYPE_POST = 'blog';
    public const TYPE_PAGE = 'page';

    /**
     * 获取文章 ID
     */
    public function getId(): int;

    /**
     * 获取标题
     */
    public function getTitle(): string;

    /**
     * 获取内容
     */
    public function getContent(): string;

    /**
     * 获取原始内容
     */
    public function getContentRaw(): string;

    /**
     * 获取摘要
     */
    public function getExcerpt(): string;

    /**
     * 获取作者 ID
     */
    public function getAuthorId(): int;

    /**
     * 获取分类 ID
     */
    public function getCategoryId(): int;

    /**
     * 获取封面图
     */
    public function getCover(): string;

    /**
     * 获取发布时间
     */
    public function getDate(): int;

    /**
     * 获取浏览量
     */
    public function getViews(): int;

    /**
     * 获取评论数
     */
    public function getCommentCount(): int;

    /**
     * 获取点赞数
     */
    public function getLikeCount(): int;

    /**
     * 是否置顶
     */
    public function isTop(): bool;

    /**
     * 是否允许评论
     */
    public function isAllowComment(): bool;

    /**
     * 是否有密码
     */
    public function hasPassword(): bool;

    /**
     * 是否是草稿
     */
    public function isDraft(): bool;

    /**
     * 是否已发布
     */
    public function isPublished(): bool;

    /**
     * 获取标签
     * 
     * @return TagInterface[]
     */
    public function getTags(): array;

    /**
     * 获取分类
     */
    public function getCategory(): ?CategoryInterface;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 分类接口
 * 
 * 定义分类实体的基本属性和行为
 */
interface CategoryInterface
{
    /**
     * 获取分类 ID
     */
    public function getId(): int;

    /**
     * 获取分类名称
     */
    public function getName(): string;

    /**
     * 获取分类别名
     */
    public function getSlug(): string;

    /**
     * 获取父分类 ID
     */
    public function getParentId(): int;

    /**
     * 获取排序顺序
     */
    public function getSortOrder(): int;

    /**
     * 获取描述
     */
    public function getDescription(): string;

    /**
     * 是否有子分类
     */
    public function hasChildren(): bool;

    /**
     * 获取子分类
     * 
     * @return CategoryInterface[]
     */
    public function getChildren(): array;

    /**
     * 获取文章数量
     */
    public function getPostCount(): int;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 标签接口
 * 
 * 定义标签实体的基本属性和行为
 */
interface TagInterface
{
    /**
     * 获取标签 ID
     */
    public function getId(): int;

    /**
     * 获取标签名称
     */
    public function getName(): string;

    /**
     * 获取标签别名
     */
    public function getSlug(): string;

    /**
     * 获取文章数量
     */
    public function getPostCount(): int;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 评论接口
 * 
 * 定义评论实体的基本属性和行为
 */
interface CommentInterface
{
    /**
     * 评论状态常量
     */
    public const STATUS_PENDING = 'n';
    public const STATUS_APPROVED = 'y';
    public const STATUS_SPAM = 'spam';

    /**
     * 获取评论 ID
     */
    public function getId(): int;

    /**
     * 获取文章 ID
     */
    public function getPostId(): int;

    /**
     * 获取父评论 ID
     */
    public function getParentId(): int;

    /**
     * 获取评论内容
     */
    public function getContent(): string;

    /**
     * 获取评论者名称
     */
    public function getAuthorName(): string;

    /**
     * 获取评论者邮箱
     */
    public function getAuthorEmail(): string;

    /**
     * 获取评论者 URL
     */
    public function getAuthorUrl(): string;

    /**
     * 获取评论者 IP
     */
    public function getAuthorIp(): string;

    /**
     * 获取评论时间
     */
    public function getDate(): int;

    /**
     * 获取状态
     */
    public function getStatus(): string;

    /**
     * 是否已审核
     */
    public function isApproved(): bool;

    /**
     * 是否是回复
     */
    public function isReply(): bool;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 内容仓储接口
 * 
 * 定义内容数据访问的抽象层
 */
interface ContentRepositoryInterface
{
    /**
     * 根据 ID 获取文章
     */
    public function findPostById(int $id): ?PostInterface;

    /**
     * 根据别名获取文章
     */
    public function findPostBySlug(string $slug): ?PostInterface;

    /**
     * 获取文章列表
     * 
     * @return PostInterface[]
     */
    public function getPosts(array $filters = [], int $page = 1, int $perPage = 20): array;

    /**
     * 创建文章
     */
    public function createPost(array $data): PostInterface;

    /**
     * 更新文章
     */
    public function updatePost(int $id, array $data): bool;

    /**
     * 删除文章
     */
    public function deletePost(int $id): bool;

    /**
     * 根据 ID 获取分类
     */
    public function findCategoryById(int $id): ?CategoryInterface;

    /**
     * 获取所有分类
     * 
     * @return CategoryInterface[]
     */
    public function getAllCategories(): array;

    /**
     * 根据 ID 获取标签
     */
    public function findTagById(int $id): ?TagInterface;

    /**
     * 根据名称获取标签
     */
    public function findTagByName(string $name): ?TagInterface;

    /**
     * 获取文章的评论
     * 
     * @return CommentInterface[]
     */
    public function getPostComments(int $postId, int $page = 1): array;

    /**
     * 创建评论
     */
    public function createComment(array $data): CommentInterface;
}
