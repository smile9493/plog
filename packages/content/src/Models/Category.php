<?php

declare(strict_types=1);

namespace Plog\Content\Models;

use Plog\Db\Connection\ConnectionInterface;

/**
 * 分类模型
 */
class Category
{
    /**
     * 数据库连接
     *
     * @var ConnectionInterface
     */
    private ConnectionInterface $connection;

    /**
     * 分类 ID
     *
     * @var int
     */
    public int $sid;

    /**
     * 分类名称
     *
     * @var string
     */
    public string $sortname;

    /**
     * 分类别名
     *
     * @var string
     */
    public string $alias;

    /**
     * 父分类 ID
     *
     * @var int
     */
    public int $pid = 0;

    /**
     * 排序
     *
     * @var string
     */
    public string $taxis = '0';

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
     * 获取所有分类
     *
     * @return array
     */
    public function all(): array
    {
        return $this->connection->table('sort')
            ->orderBy('taxis', 'ASC')
            ->get();
    }

    /**
     * 根据主键查找
     *
     * @param int $id 分类 ID
     * @return array|null
     */
    public function find(int $id): ?array
    {
        return $this->connection->table('sort')
            ->where('sid', '=', $id)
            ->first();
    }

    /**
     * 根据别名查找
     *
     * @param string $alias 分类别名
     * @return array|null
     */
    public function findByAlias(string $alias): ?array
    {
        return $this->connection->table('sort')
            ->where('alias', '=', $alias)
            ->first();
    }

    /**
     * 创建分类
     *
     * @param array $data 分类数据
     * @return int 分类 ID
     */
    public function create(array $data): int
    {
        return $this->connection->table('sort')->insert($data);
    }

    /**
     * 更新分类
     *
     * @param int $id 分类 ID
     * @param array $data 分类数据
     * @return int 影响的行数
     */
    public function update(int $id, array $data): int
    {
        return $this->connection->table('sort')
            ->where('sid', '=', $id)
            ->update($data);
    }

    /**
     * 删除分类
     *
     * @param int $id 分类 ID
     * @return int 影响的行数
     */
    public function delete(int $id): int
    {
        return $this->connection->table('sort')
            ->where('sid', '=', $id)
            ->delete();
    }

    /**
     * 获取子分类
     *
     * @param int $parentId 父分类 ID
     * @return array
     */
    public function getChildren(int $parentId): array
    {
        return $this->connection->table('sort')
            ->where('pid', '=', $parentId)
            ->orderBy('taxis', 'ASC')
            ->get();
    }
}
