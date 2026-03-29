<?php

declare(strict_types=1);

namespace Plog\Core\Event;

/**
 * 事件调度器接口
 */
interface EventDispatcherInterface
{
    /**
     * 触发事件
     *
     * @param string $eventName 事件名称
     * @param array $payload 事件数据
     * @return array 事件处理结果
     */
    public function dispatch(string $eventName, array $payload = []): array;

    /**
     * 注册监听器
     *
     * @param string $eventName 事件名称
     * @param callable $listener 监听器
     * @param int $priority 优先级,数字越大优先级越高
     */
    public function listen(string $eventName, callable $listener, int $priority = 0): void;

    /**
     * 移除监听器
     *
     * @param string $eventName 事件名称
     */
    public function forget(string $eventName): void;

    /**
     * 检查是否有监听器
     *
     * @param string $eventName 事件名称
     * @return bool
     */
    public function hasListeners(string $eventName): bool;
}
