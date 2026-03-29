<?php

declare(strict_types=1);

namespace Plog\Content\Models;

use Plog\Db\Connection\ConnectionInterface;

/**
 * 标签模型
 */
class Tag
{
    /**
     * 数据库连接
     *
     * @var ConnectionInterface
     */
    private ConnectionInterface $connection;

    /**
     * 标签 ID
     *
     * @var int
     */
    public int $tid;

    /**
     * 标签名称
     *
     * @var string
     */
    public string $tagname;

    /**
     * 标签别名
     *
     * @var string
     */
    public string $tagalias;

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
     * 获取所有标签
     *
     * @return array
     */
    public function all(): array
    {
        return $this->connection->table('tag')->get();
    }

    /**
     * 根据主键查找
     *
     * @param int $id 标签 ID
     * @return array|null
     */
    public function find(int $id): ?array
    {
        return $this->connection->table('tag')
            ->where('tid', '=', $id)
            ->first();
    }

    /**
     * 根据别名查找
     *
     * @param string $alias 标签别名
     * @return array|null
     */
    public function findByAlias(string $alias): ?array
    {
        return $this->connection->table('tag')
            ->where('tagalias', '=', $alias)
            ->first();
    }

    /**
     * 创建标签
     *
     * @param array $data 标签数据
     * @return int 标签 ID
     */
    public function create(array $data): int
    {
        return $this->connection->table('tag')->insert($data);
    }

    /**
     * 更新标签
     *
     * @param int $id 标签 ID
     * @param array $data 标签数据
     * @return int 影响的行数
     */
    public function update(int $id, array $data): int
    {
        return $this->connection->table('tag')
            ->where('tid', '=', $id)
            ->update($data);
    }

    /**
     * 删除标签
     *
     * @param int $id 标签 ID
     * @return int 影响的行数
     */
    public function delete(int $id): int
    {
        return $this->connection->table('tag')
            ->where('tid', '=', $id)
            ->delete();
    }

    /**
     * 获取文章的标签
     *
     * @param int $postId 文章 ID
     * @return array
     */
    public function getPostTags(int $postId): array
    {
        return $this->connection->query(
            'SELECT t.* FROM tag t 
             INNER JOIN tag_relation tr ON t.tid = tr.tid 
             WHERE tr.gid = ?',
            [$postId]
        )->fetchAll();
    }
}
