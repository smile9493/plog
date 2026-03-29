<?php

declare(strict_types=1);

namespace Plog\Db\QueryBuilder;

use Plog\Db\Connection\Connection;

/**
 * 查询构建器
 */
class QueryBuilder implements QueryBuilderInterface
{
    /**
     * 数据库连接
     *
     * @var Connection
     */
    private Connection $connection;

    /**
     * 表名
     *
     * @var string
     */
    private string $table;

    /**
     * 查询类型
     *
     * @var string
     */
    private string $type = 'select';

    /**
     * 选择的列
     *
     * @var array
     */
    private array $columns = ['*'];

    /**
     * WHERE 条件
     *
     * @var array
     */
    private array $wheres = [];

    /**
     * ORDER BY 条件
     *
     * @var array
     */
    private array $orders = [];

    /**
     * LIMIT 值
     *
     * @var int|null
     */
    private ?int $limit = null;

    /**
     * OFFSET 值
     *
     * @var int|null
     */
    private ?int $offset = null;

    /**
     * 绑定参数
     *
     * @var array
     */
    private array $bindings = [];

    /**
     * 构造函数
     *
     * @param Connection $connection 数据库连接
     * @param string $table 表名
     */
    public function __construct(Connection $connection, string $table)
    {
        $this->connection = $connection;
        $this->table = $table;
    }

    /**
     * SELECT 子句
     *
     * @param array|string $columns 列
     * @return self
     */
    public function select($columns = ['*']): self
    {
        $this->columns = is_array($columns) ? $columns : func_get_args();
        return $this;
    }

    /**
     * WHERE 子句
     *
     * @param string $column 列名
     * @param string $operator 操作符
     * @param mixed $value 值
     * @return self
     */
    public function where(string $column, string $operator, $value): self
    {
        $this->wheres[] = [
            'column' => $column,
            'operator' => $operator,
            'value' => $value,
            'boolean' => 'AND',
        ];

        $this->bindings[] = $value;

        return $this;
    }

    /**
     * ORDER BY 子句
     *
     * @param string $column 列名
     * @param string $direction 方向
     * @return self
     */
    public function orderBy(string $column, string $direction = 'ASC'): self
    {
        $this->orders[] = [
            'column' => $column,
            'direction' => $direction,
        ];

        return $this;
    }

    /**
     * LIMIT 子句
     *
     * @param int $limit 限制数量
     * @return self
     */
    public function limit(int $limit): self
    {
        $this->limit = $limit;
        return $this;
    }

    /**
     * OFFSET 子句
     *
     * @param int $offset 偏移量
     * @return self
     */
    public function offset(int $offset): self
    {
        $this->offset = $offset;
        return $this;
    }

    /**
     * 执行查询并获取所有结果
     *
     * @return array
     */
    public function get(): array
    {
        $sql = $this->buildSelectQuery();

        return $this->connection->select($sql, $this->bindings);
    }

    /**
     * 执行查询并获取第一条结果
     *
     * @return array|null
     */
    public function first(): ?array
    {
        $result = $this->limit(1)->get();

        return $result[0] ?? null;
    }

    /**
     * 插入数据
     *
     * @param array $data 数据
     * @return int 插入的 ID
     */
    public function insert(array $data): int
    {
        $sql = $this->buildInsertQuery($data);
        $bindings = array_values($data);

        return $this->connection->insert($sql, $bindings);
    }

    /**
     * 更新数据
     *
     * @param array $data 数据
     * @return int 影响的行数
     */
    public function update(array $data): int
    {
        $sql = $this->buildUpdateQuery($data);
        $bindings = array_merge(array_values($data), $this->bindings);

        return $this->connection->update($sql, $bindings);
    }

    /**
     * 删除数据
     *
     * @return int 影响的行数
     */
    public function delete(): int
    {
        $sql = $this->buildDeleteQuery();

        return $this->connection->delete($sql, $this->bindings);
    }

    /**
     * 构建 SELECT 查询
     *
     * @return string
     */
    private function buildSelectQuery(): string
    {
        $sql = 'SELECT ' . implode(', ', $this->columns) . ' FROM ' . $this->table;

        if (!empty($this->wheres)) {
            $sql .= ' WHERE ' . $this->buildWhereClause();
        }

        if (!empty($this->orders)) {
            $sql .= ' ORDER BY ' . $this->buildOrderClause();
        }

        if ($this->limit !== null) {
            $sql .= ' LIMIT ' . $this->limit;
        }

        if ($this->offset !== null) {
            $sql .= ' OFFSET ' . $this->offset;
        }

        return $sql;
    }

    /**
     * 构建 INSERT 查询
     *
     * @param array $data 数据
     * @return string
     */
    private function buildInsertQuery(array $data): string
    {
        $columns = implode(', ', array_keys($data));
        $placeholders = implode(', ', array_fill(0, count($data), '?'));

        return "INSERT INTO {$this->table} ({$columns}) VALUES ({$placeholders})";
    }

    /**
     * 构建 UPDATE 查询
     *
     * @param array $data 数据
     * @return string
     */
    private function buildUpdateQuery(array $data): string
    {
        $sets = [];
        foreach (array_keys($data) as $column) {
            $sets[] = "{$column} = ?";
        }

        $sql = 'UPDATE ' . $this->table . ' SET ' . implode(', ', $sets);

        if (!empty($this->wheres)) {
            $sql .= ' WHERE ' . $this->buildWhereClause();
        }

        return $sql;
    }

    /**
     * 构建 DELETE 查询
     *
     * @return string
     */
    private function buildDeleteQuery(): string
    {
        $sql = 'DELETE FROM ' . $this->table;

        if (!empty($this->wheres)) {
            $sql .= ' WHERE ' . $this->buildWhereClause();
        }

        return $sql;
    }

    /**
     * 构建 WHERE 子句
     *
     * @return string
     */
    private function buildWhereClause(): string
    {
        $clauses = [];

        foreach ($this->wheres as $where) {
            $clauses[] = $where['column'] . ' ' . $where['operator'] . ' ?';
        }

        return implode(' AND ', $clauses);
    }

    /**
     * 构建 ORDER BY 子句
     *
     * @return string
     */
    private function buildOrderClause(): string
    {
        $clauses = [];

        foreach ($this->orders as $order) {
            $clauses[] = $order['column'] . ' ' . $order['direction'];
        }

        return implode(', ', $clauses);
    }
}
