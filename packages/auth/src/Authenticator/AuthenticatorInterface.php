<?php

declare(strict_types=1);

namespace Plog\Auth\Authenticator;

/**
 * 认证器接口
 */
interface AuthenticatorInterface
{
    /**
     * 尝试登录
     *
     * @param array $credentials 凭证 (username, password)
     * @return bool
     */
    public function attempt(array $credentials): bool;

    /**
     * 登录用户
     *
     * @param UserInterface $user 用户
     */
    public function login(UserInterface $user): void;

    /**
     * 登出
     */
    public function logout(): void;

    /**
     * 检查是否已登录
     *
     * @return bool
     */
    public function check(): bool;

    /**
     * 获取当前用户
     *
     * @return UserInterface|null
     */
    public function user(): ?UserInterface;

    /**
     * 获取当前用户 ID
     *
     * @return mixed
     */
    public function id();
}
