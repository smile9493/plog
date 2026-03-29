<?php

declare(strict_types=1);

namespace Plog\Db\Connection;

use PDO;
use PDOException;
use Plog\Db\QueryBuilder\QueryBuilder;

/**
 * 数据库连接
 */
class Connection implements ConnectionInterface
{
    /**
     * PDO 实例
     *
     * @var PDO
     */
    private PDO $pdo;

    /**
     * 连接配置
     *
     * @var array
     */
    private array $config;

    /**
     * 构造函数
     *
     * @param array $config 连接配置
     */
    public function __construct(array $config)
    {
        $this->config = $config;
        $this->pdo = $this->createPdo();
    }

    /**
     * 创建 PDO 实例
     *
     * @return PDO
     * @throws PDOException
     */
    private function createPdo(): PDO
    {
        $dsn = $this->buildDsn();

        $options = [
            PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
            PDO::ATTR_EMULATE_PREPARES => false,
            PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
        ];

        if (isset($this->config['options'])) {
            $options = array_merge($options, $this->config['options']);
        }

        return new PDO(
            $dsn,
            $this->config['username'] ?? null,
            $this->config['password'] ?? null,
            $options
        );
    }

    /**
     * 构建 DSN
     *
     * @return string
     */
    private function buildDsn(): string
    {
        $driver = $this->config['driver'] ?? 'mysql';

        if ($driver === 'sqlite') {
            return "sqlite:{$this->config['database']}";
        }

        $host = $this->config['host'] ?? 'localhost';
        $port = $this->config['port'] ?? 3306;
        $database = $this->config['database'] ?? '';
        $charset = $this->config['charset'] ?? 'utf8mb4';

        return "{$driver}:host={$host};port={$port};dbname={$database};charset={$charset}";
    }

    /**
     * 获取 PDO 实例
     *
     * @return PDO
     */
    public function getPdo(): PDO
    {
        return $this->pdo;
    }

    /**
     * 创建查询构建器
     *
     * @param string $table 表名
     * @return QueryBuilder
     */
    public function table(string $table): QueryBuilder
    {
        return new QueryBuilder($this, $table);
    }

    /**
     * 执行原生 SQL
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return \PDOStatement
     */
    public function query(string $sql, array $bindings = []): \PDOStatement
    {
        $statement = $this->pdo->prepare($sql);
        $statement->execute($bindings);

        return $statement;
    }

    /**
     * 执行查询并获取所有结果
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return array
     */
    public function select(string $sql, array $bindings = []): array
    {
        return $this->query($sql, $bindings)->fetchAll();
    }

    /**
     * 执行插入语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 插入的 ID
     */
    public function insert(string $sql, array $bindings = []): int
    {
        $this->query($sql, $bindings);

        return (int) $this->pdo->lastInsertId();
    }

    /**
     * 执行更新语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 影响的行数
     */
    public function update(string $sql, array $bindings = []): int
    {
        return $this->query($sql, $bindings)->rowCount();
    }

    /**
     * 执行删除语句
     *
     * @param string $sql SQL 语句
     * @param array $bindings 绑定参数
     * @return int 影响的行数
     */
    public function delete(string $sql, array $bindings = []): int
    {
        return $this->query($sql, $bindings)->rowCount();
    }

    /**
     * 开启事务
     *
     * @return bool
     */
    public function beginTransaction(): bool
    {
        return $this->pdo->beginTransaction();
    }

    /**
     * 提交事务
     *
     * @return bool
     */
    public function commit(): bool
    {
        return $this->pdo->commit();
    }

    /**
     * 回滚事务
     *
     * @return bool
     */
    public function rollBack(): bool
    {
        return $this->pdo->rollBack();
    }
}
