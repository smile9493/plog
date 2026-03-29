<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 用户仓储接口
 * 
 * 定义用户数据访问的抽象层
 */
interface UserRepositoryInterface
{
    /**
     * 根据 ID 获取用户
     */
    public function findById(int $id): ?UserInterface;

    /**
     * 根据用户名获取用户
     */
    public function findByUsername(string $username): ?UserInterface;

    /**
     * 根据邮箱获取用户
     */
    public function findByEmail(string $email): ?UserInterface;

    /**
     * 获取所有用户
     * 
     * @return UserInterface[]
     */
    public function findAll(): array;

    /**
     * 分页获取用户
     * 
     * @return UserInterface[]
     */
    public function paginate(int $page, int $perPage): array;

    /**
     * 创建用户
     */
    public function create(array $data): UserInterface;

    /**
     * 更新用户
     */
    public function update(int $id, array $data): bool;

    /**
     * 删除用户
     */
    public function delete(int $id): bool;

    /**
     * 获取用户总数
     */
    public function count(): int;
}
