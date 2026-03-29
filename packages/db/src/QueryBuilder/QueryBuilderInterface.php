<?php

declare(strict_types=1);

namespace Plog\Db\QueryBuilder;

/**
 * 查询构建器接口
 */
interface QueryBuilderInterface
{
    /**
     * SELECT 子句
     *
     * @param array|string $columns 列
     * @return self
     */
    public function select($columns = ['*']): self;

    /**
     * WHERE 子句
     *
     * @param string $column 列名
     * @param string $operator 操作符
     * @param mixed $value 值
     * @return self
     */
    public function where(string $column, string $operator, $value): self;

    /**
     * ORDER BY 子句
     *
     * @param string $column 列名
     * @param string $direction 方向
     * @return self
     */
    public function orderBy(string $column, string $direction = 'ASC'): self;

    /**
     * LIMIT 子句
     *
     * @param int $limit 限制数量
     * @return self
     */
    public function limit(int $limit): self;

    /**
     * OFFSET 子句
     *
     * @param int $offset 偏移量
     * @return self
     */
    public function offset(int $offset): self;

    /**
     * 执行查询并获取所有结果
     *
     * @return array
     */
    public function get(): array;

    /**
     * 执行查询并获取第一条结果
     *
     * @return array|null
     */
    public function first(): ?array;

    /**
     * 插入数据
     *
     * @param array $data 数据
     * @return int 插入的 ID
     */
    public function insert(array $data): int;

    /**
     * 更新数据
     *
     * @param array $data 数据
     * @return int 影响的行数
     */
    public function update(array $data): int;

    /**
     * 删除数据
     *
     * @return int 影响的行数
     */
    public function delete(): int;
}
