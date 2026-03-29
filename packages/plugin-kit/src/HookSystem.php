<?php

declare(strict_types=1);

namespace Plog\Plugin;

/**
 * 钩子系统
 */
class HookSystem
{
    /**
     * Action钩子列表
     *
     * @var array
     */
    private array $actions = [];

    /**
     * Filter钩子列表
     *
     * @var array
     */
    private array $filters = [];

    /**
     * 注册Action钩子
     *
     * @param string $hook 钩子名称
     * @param callable $callback 回调函数
     * @param int $priority 优先级
     * @return void
     */
    public function addAction(string $hook, callable $callback, int $priority = 10): void
    {
        if (!isset($this->actions[$hook])) {
            $this->actions[$hook] = [];
        }

        $this->actions[$hook][] = [
            'callback' => $callback,
            'priority' => $priority,
        ];

        // 按优先级排序
        usort($this->actions[$hook], function ($a, $b) {
            return $a['priority'] <=> $b['priority'];
        });
    }

    /**
     * 执行Action钩子
     *
     * @param string $hook 钩子名称
     * @param mixed ...$args 参数
     * @return void
     */
    public function doAction(string $hook, ...$args): void
    {
        if (!isset($this->actions[$hook])) {
            return;
        }

        foreach ($this->actions[$hook] as $action) {
            call_user_func_array($action['callback'], $args);
        }
    }

    /**
     * 移除Action钩子
     *
     * @param string $hook 钩子名称
     * @param callable $callback 回调函数
     * @return bool
     */
    public function removeAction(string $hook, callable $callback): bool
    {
        if (!isset($this->actions[$hook])) {
            return false;
        }

        foreach ($this->actions[$hook] as $key => $action) {
            if ($action['callback'] === $callback) {
                unset($this->actions[$hook][$key]);
                return true;
            }
        }

        return false;
    }

    /**
     * 注册Filter钩子
     *
     * @param string $hook 钩子名称
     * @param callable $callback 回调函数
     * @param int $priority 优先级
     * @return void
     */
    public function addFilter(string $hook, callable $callback, int $priority = 10): void
    {
        if (!isset($this->filters[$hook])) {
            $this->filters[$hook] = [];
        }

        $this->filters[$hook][] = [
            'callback' => $callback,
            'priority' => $priority,
        ];

        // 按优先级排序
        usort($this->filters[$hook], function ($a, $b) {
            return $a['priority'] <=> $b['priority'];
        });
    }

    /**
     * 应用Filter钩子
     *
     * @param string $hook 钩子名称
     * @param mixed $value 值
     * @param mixed ...$args 额外参数
     * @return mixed
     */
    public function applyFilters(string $hook, $value, ...$args)
    {
        if (!isset($this->filters[$hook])) {
            return $value;
        }

        foreach ($this->filters[$hook] as $filter) {
            $value = call_user_func_array($filter['callback'], array_merge([$value], $args));
        }

        return $value;
    }

    /**
     * 移除Filter钩子
     *
     * @param string $hook 钩子名称
     * @param callable $callback 回调函数
     * @return bool
     */
    public function removeFilter(string $hook, callable $callback): bool
    {
        if (!isset($this->filters[$hook])) {
            return false;
        }

        foreach ($this->filters[$hook] as $key => $filter) {
            if ($filter['callback'] === $callback) {
                unset($this->filters[$hook][$key]);
                return true;
            }
        }

        return false;
    }

    /**
     * 检查Action钩子是否存在
     *
     * @param string $hook 钩子名称
     * @return bool
     */
    public function hasAction(string $hook): bool
    {
        return isset($this->actions[$hook]) && count($this->actions[$hook]) > 0;
    }

    /**
     * 检查Filter钩子是否存在
     *
     * @param string $hook 钩子名称
     * @return bool
     */
    public function hasFilter(string $hook): bool
    {
        return isset($this->filters[$hook]) && count($this->filters[$hook]) > 0;
    }
}
