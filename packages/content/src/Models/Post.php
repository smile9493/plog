<?php

declare(strict_types=1);

namespace Plog\Content\Models;

use Plog\Db\Connection\ConnectionInterface;

/**
 * 文章模型
 */
class Post
{
    /**
     * 数据库连接
     *
     * @var ConnectionInterface
     */
    private ConnectionInterface $connection;

    /**
     * 文章 ID
     *
     * @var int
     */
    public int $gid;

    /**
     * 文章标题
     *
     * @var string
     */
    public string $title;

    /**
     * 文章别名
     *
     * @var string
     */
    public string $slug;

    /**
     * 文章内容
     *
     * @var string
     */
    public string $content;

    /**
     * 文章摘要
     *
     * @var string
     */
    public string $excerpt = '';

    /**
     * 作者 ID
     *
     * @var int
     */
    public int $author;

    /**
     * 分类 ID
     *
     * @var int
     */
    public int $sortid = 0;

    /**
     * 发布时间
     *
     * @var string
     */
    public string $date;

    /**
     * 隐藏状态
     *
     * @var string
     */
    public string $hide = 'n';

    /**
     * 允许评论
     *
     * @var string
     */
    public string $allow_remark = 'y';

    /**
     * 置顶
     *
     * @var string
     */
    public string $top = 'n';

    /**
     * 类型
     *
     * @var string
     */
    public string $type = 'blog';

    /**
     * 密码
     *
     * @var string
     */
    public string $password = '';

    /**
     * 文章状态常量
     */
    const STATUS_DRAFT = 'draft';
    const STATUS_PUBLISHED = 'n';
    const STATUS_HIDDEN = 'y';

    /**
     * 文章类型常量
     */
    const TYPE_POST = 'blog';
    const TYPE_PAGE = 'page';

    /**
     * 构造函数
     *
     * @param ConnectionInterface $connection 数据库连接
     */
    public function __construct(ConnectionInterface $connection)
    {
        $this->connection = $connection;
    }

    /**
     * 获取所有文章
     *
     * @return array
     */
    public function all(): array
    {
        return $this->connection->table('blog')
            ->where('type', '=', self::TYPE_POST)
            ->orderBy('date', 'DESC')
            ->get();
    }

    /**
     * 获取已发布的文章
     *
     * @return array
     */
    public function published(): array
    {
        return $this->connection->table('blog')
            ->where('hide', '=', 'n')
            ->where('type', '=', self::TYPE_POST)
            ->orderBy('date', 'DESC')
            ->get();
    }

    /**
     * 根据主键查找
     *
     * @param int $id 文章 ID
     * @return array|null
     */
    public function find(int $id): ?array
    {
        return $this->connection->table('blog')
            ->where('gid', '=', $id)
            ->first();
    }

    /**
     * 根据别名查找
     *
     * @param string $slug 文章别名
     * @return array|null
     */
    public function findBySlug(string $slug): ?array
    {
        return $this->connection->table('blog')
            ->where('slug', '=', $slug)
            ->first();
    }

    /**
     * 创建文章
     *
     * @param array $data 文章数据
     * @return int 文章 ID
     */
    public function create(array $data): int
    {
        $data['date'] = $data['date'] ?? date('Y-m-d H:i:s');
        $data['type'] = $data['type'] ?? self::TYPE_POST;

        return $this->connection->table('blog')->insert($data);
    }

    /**
     * 更新文章
     *
     * @param int $id 文章 ID
     * @param array $data 文章数据
     * @return int 影响的行数
     */
    public function update(int $id, array $data): int
    {
        return $this->connection->table('blog')
            ->where('gid', '=', $id)
            ->update($data);
    }

    /**
     * 删除文章
     *
     * @param int $id 文章 ID
     * @return int 影响的行数
     */
    public function delete(int $id): int
    {
        return $this->connection->table('blog')
            ->where('gid', '=', $id)
            ->delete();
    }

    /**
     * 分页获取文章
     *
     * @param int $page 页码
     * @param int $perPage 每页数量
     * @return array
     */
    public function paginate(int $page = 1, int $perPage = 20): array
    {
        $offset = ($page - 1) * $perPage;

        return $this->connection->table('blog')
            ->where('type', '=', self::TYPE_POST)
            ->orderBy('date', 'DESC')
            ->limit($perPage)
            ->offset($offset)
            ->get();
    }
}
