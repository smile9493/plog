<?php

declare(strict_types=1);

namespace Plog\Auth\Authenticator;

use Plog\Auth\PasswordHasher\PasswordHasherInterface;
use Plog\Auth\Session\SessionManagerInterface;

/**
 * 会话认证器
 */
class SessionAuthenticator implements AuthenticatorInterface
{
    /**
     * 会话管理器
     *
     * @var SessionManagerInterface
     */
    private SessionManagerInterface $session;

    /**
     * 密码哈希器
     *
     * @var PasswordHasherInterface
     */
    private PasswordHasherInterface $hasher;

    /**
     * 用户提供者回调
     *
     * @var callable
     */
    private $userProvider;

    /**
     * 当前用户
     *
     * @var UserInterface|null
     */
    private ?UserInterface $user = null;

    /**
     * 构造函数
     *
     * @param SessionManagerInterface $session 会话管理器
     * @param PasswordHasherInterface $hasher 密码哈希器
     * @param callable $userProvider 用户提供者
     */
    public function __construct(
        SessionManagerInterface $session,
        PasswordHasherInterface $hasher,
        callable $userProvider
    ) {
        $this->session = $session;
        $this->hasher = $hasher;
        $this->userProvider = $userProvider;
    }

    /**
     * 尝试登录
     *
     * @param array $credentials 凭证
     * @return bool
     */
    public function attempt(array $credentials): bool
    {
        $user = call_user_func($this->userProvider, $credentials['username'] ?? null);

        if (!$user instanceof UserInterface) {
            return false;
        }

        if (!$this->hasher->verify($credentials['password'] ?? '', $user->getPasswordHash())) {
            return false;
        }

        $this->login($user);

        return true;
    }

    /**
     * 登录用户
     *
     * @param UserInterface $user 用户
     */
    public function login(UserInterface $user): void
    {
        $this->user = $user;
        $this->session->set('auth_user_id', $user->getId());
    }

    /**
     * 登出
     */
    public function logout(): void
    {
        $this->user = null;
        $this->session->remove('auth_user_id');
    }

    /**
     * 检查是否已登录
     *
     * @return bool
     */
    public function check(): bool
    {
        return $this->user() !== null;
    }

    /**
     * 获取当前用户
     *
     * @return UserInterface|null
     */
    public function user(): ?UserInterface
    {
        if ($this->user !== null) {
            return $this->user;
        }

        $userId = $this->session->get('auth_user_id');

        if ($userId === null) {
            return null;
        }

        $this->user = call_user_func($this->userProvider, $userId);

        return $this->user;
    }

    /**
     * 获取当前用户 ID
     *
     * @return mixed
     */
    public function id()
    {
        $user = $this->user();

        return $user !== null ? $user->getId() : null;
    }
}
