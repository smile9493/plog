<?php

declare(strict_types=1);

namespace Plog\Auth\Session;

/**
 * 会话管理器接口
 */
interface SessionManagerInterface
{
    /**
     * 获取会话值
     *
     * @param string $key 键
     * @param mixed $default 默认值
     * @return mixed
     */
    public function get(string $key, $default = null);

    /**
     * 设置会话值
     *
     * @param string $key 键
     * @param mixed $value 值
     */
    public function set(string $key, $value): void;

    /**
     * 检查会话值是否存在
     *
     * @param string $key 键
     * @return bool
     */
    public function has(string $key): bool;

    /**
     * 移除会话值
     *
     * @param string $key 键
     */
    public function remove(string $key): void;

    /**
     * 获取所有会话数据
     *
     * @return array
     */
    public function all(): array;

    /**
     * 清空会话
     */
    public function clear(): void;

    /**
     * 启动会话
     *
     * @return bool
     */
    public function start(): bool;

    /**
     * 销毁会话
     *
     * @return bool
     */
    public function destroy(): bool;
}
