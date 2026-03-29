<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 用户模型接口
 * 
 * 定义用户实体的基本属性和行为
 */
interface UserInterface
{
    /**
     * 获取用户 ID
     */
    public function getId(): int;

    /**
     * 获取用户名
     */
    public function getUsername(): string;

    /**
     * 获取昵称
     */
    public function getNickname(): string;

    /**
     * 获取邮箱
     */
    public function getEmail(): string;

    /**
     * 获取角色
     */
    public function getRole(): string;

    /**
     * 获取头像 URL
     */
    public function getAvatar(): string;

    /**
     * 获取描述
     */
    public function getDescription(): string;

    /**
     * 获取创建时间
     */
    public function getCreatedAt(): int;

    /**
     * 是否是管理员
     */
    public function isAdmin(): bool;

    /**
     * 是否是编辑
     */
    public function isEditor(): bool;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}
