<?php

declare(strict_types=1);

namespace Plog\Db\Connection;

use PDO;
use Plog\Db\QueryBuilder\QueryBuilderInterface;

/**
 * 数据库连接接口
 */
interface ConnectionInterface
{
    /**
     * 获取 PDO 实例
     *
     * @return PDO
     */
    public function getPdo(): PDO;

    /**
     * 创建查询构建器
     *
     * @param string $table 表名
     * @return QueryBuilderInterface
     */
    public function table(string $table): QueryBuilderInterface;

    /**
     * 执行原生 SQL
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return \PDOStatement
     */
    public function query(string $sql, array $bindings = []): \PDOStatement;

    /**
     * 执行查询并获取所有结果
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return array
     */
    public function select(string $sql, array $bindings = []): array;

    /**
     * 执行插入语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 插入的 ID
     */
    public function insert(string $sql, array $bindings = []): int;

    /**
     * 执行更新语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 影响的行数
     */
    public function update(string $sql, array $bindings = []): int;

    /**
     * 执行删除语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 影响的行数
     */
    public function delete(string $sql, array $bindings = []): int;

    /**
     * 开启事务
     *
     * @return bool
     */
    public function beginTransaction(): bool;

    /**
     * 提交事务
     *
     * @return bool
     */
    public function commit(): bool;

    /**
     * 回滚事务
     *
     * @return bool
     */
    public function rollBack(): bool;
}
