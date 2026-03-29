<?php

declare(strict_types=1);

namespace Plog\Auth\Authenticator;

/**
 * 用户接口
 */
interface UserInterface
{
    /**
     * 获取用户 ID
     *
     * @return mixed
     */
    public function getId();

    /**
     * 获取用户名
     *
     * @return string
     */
    public function getUsername(): string;

    /**
     * 获取密码哈希
     *
     * @return string
     */
    public function getPasswordHash(): string;

    /**
     * 获取用户角色
     *
     * @return array
     */
    public function getRoles(): array;

    /**
     * 获取用户权限
     *
     * @return array
     */
    public function getPermissions(): array;

    /**
     * 检查是否有指定角色
     *
     * @param string $role 角色名称
     * @return bool
     */
    public function hasRole(string $role): bool;

    /**
     * 检查是否有指定权限
     *
     * @param string $permission 权限名称
     * @return bool
     */
    public function hasPermission(string $permission): bool;
}
