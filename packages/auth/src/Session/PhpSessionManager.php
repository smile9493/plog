<?php

declare(strict_types=1);

namespace Plog\Auth\Session;

/**
 * PHP 会话管理器
 */
class PhpSessionManager implements SessionManagerInterface
{
    /**
     * 会话是否已启动
     *
     * @var bool
     */
    private bool $started = false;

    /**
     * 构造函数
     *
     * @param array $options 会话选项
     */
    public function __construct(array $options = [])
    {
        $defaults = [
            'name' => 'plog_session',
            'lifetime' => 7200,
            'path' => '/',
            'domain' => null,
            'secure' => false,
            'httponly' => true,
            'samesite' => 'strict',
        ];

        $options = array_merge($defaults, $options);

        if (session_status() === PHP_SESSION_NONE) {
            session_name($options['name']);
            session_set_cookie_params([
                'lifetime' => $options['lifetime'],
                'path' => $options['path'],
                'domain' => $options['domain'],
                'secure' => $options['secure'],
                'httponly' => $options['httponly'],
                'samesite' => $options['samesite'],
            ]);
        }
    }

    /**
     * 启动会话
     *
     * @return bool
     */
    public function start(): bool
    {
        if ($this->started) {
            return true;
        }

        if (session_status() === PHP_SESSION_NONE) {
            $this->started = session_start();
        } else {
            $this->started = true;
        }

        return $this->started;
    }

    /**
     * 获取会话值
     *
     * @param string $key 键
     * @param mixed $default 默认值
     * @return mixed
     */
    public function get(string $key, $default = null)
    {
        $this->start();

        return $_SESSION[$key] ?? $default;
    }

    /**
     * 设置会话值
     *
     * @param string $key 键
     * @param mixed $value 值
     */
    public function set(string $key, $value): void
    {
        $this->start();
        $_SESSION[$key] = $value;
    }

    /**
     * 检查会话值是否存在
     *
     * @param string $key 键
     * @return bool
     */
    public function has(string $key): bool
    {
        $this->start();

        return isset($_SESSION[$key]);
    }

    /**
     * 移除会话值
     *
     * @param string $key 键
     */
    public function remove(string $key): void
    {
        $this->start();
        unset($_SESSION[$key]);
    }

    /**
     * 获取所有会话数据
     *
     * @return array
     */
    public function all(): array
    {
        $this->start();

        return $_SESSION ?? [];
    }

    /**
     * 清空会话
     */
    public function clear(): void
    {
        $this->start();
        $_SESSION = [];
    }

    /**
     * 销毁会话
     *
     * @return bool
     */
    public function destroy(): bool
    {
        if (session_status() === PHP_SESSION_ACTIVE) {
            $this->started = false;
            return session_destroy();
        }

        return true;
    }
}
