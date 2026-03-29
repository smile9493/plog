<?php

declare(strict_types=1);

namespace Plog\Core\Event;

/**
 * 事件调度器
 */
class EventDispatcher implements EventDispatcherInterface
{
    /**
     * 事件监听器
     *
     * @var array
     */
    private array $listeners = [];

    /**
     * 触发事件
     *
     * @param string $eventName 事件名称
     * @param array $payload 事件数据
     * @return array 事件处理结果
     */
    public function dispatch(string $eventName, array $payload = []): array
    {
        $results = [];

        if (!isset($this->listeners[$eventName])) {
            return $results;
        }

        // 按优先级排序(数字越大优先级越高)
        $listeners = $this->listeners[$eventName];
        krsort($listeners);

        foreach ($listeners as $priority => $items) {
            foreach ($items as $listener) {
                $results[] = call_user_func_array($listener, $payload);
            }
        }

        return $results;
    }

    /**
     * 注册监听器
     *
     * @param string $eventName 事件名称
     * @param callable $listener 监听器
     * @param int $priority 优先级
     */
    public function listen(string $eventName, callable $listener, int $priority = 0): void
    {
        if (!isset($this->listeners[$eventName])) {
            $this->listeners[$eventName] = [];
        }

        if (!isset($this->listeners[$eventName][$priority])) {
            $this->listeners[$eventName][$priority] = [];
        }

        $this->listeners[$eventName][$priority][] = $listener;
    }

    /**
     * 移除监听器
     *
     * @param string $eventName 事件名称
     */
    public function forget(string $eventName): void
    {
        unset($this->listeners[$eventName]);
    }

    /**
     * 检查是否有监听器
     *
     * @param string $eventName 事件名称
     * @return bool
     */
    public function hasListeners(string $eventName): bool
    {
        return isset($this->listeners[$eventName]) && !empty($this->listeners[$eventName]);
    }
}
