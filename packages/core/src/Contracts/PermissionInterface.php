<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 权限接口
 * 
 * 定义权限的基本属性
 */
interface PermissionInterface
{
    /**
     * 获取权限 ID
     */
    public function getId(): int;

    /**
     * 获取权限名称
     */
    public function getName(): string;

    /**
     * 获取权限标识
     */
    public function getSlug(): string;

    /**
     * 获取权限描述
     */
    public function getDescription(): string;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 角色接口
 * 
 * 定义角色的基本属性
 */
interface RoleInterface
{
    /**
     * 获取角色 ID
     */
    public function getId(): int;

    /**
     * 获取角色名称
     */
    public function getName(): string;

    /**
     * 获取角色标识
     */
    public function getSlug(): string;

    /**
     * 获取角色描述
     */
    public function getDescription(): string;

    /**
     * 获取角色权限
     * 
     * @return PermissionInterface[]
     */
    public function getPermissions(): array;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 权限检查器接口
 * 
 * 定义权限检查的抽象层
 */
interface PermissionCheckerInterface
{
    /**
     * 检查用户是否有指定权限
     */
    public function hasPermission(UserInterface $user, string $permission): bool;

    /**
     * 检查用户是否有指定角色
     */
    public function hasRole(UserInterface $user, string $role): bool;

    /**
     * 获取用户的所有权限
     * 
     * @return string[]
     */
    public function getUserPermissions(UserInterface $user): array;

    /**
     * 获取用户的所有角色
     * 
     * @return string[]
     */
    public function getUserRoles(UserInterface $user): array;
}
