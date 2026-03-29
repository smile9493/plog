<?php

declare(strict_types=1);

namespace Plog\Core\Contracts;

/**
 * 插件接口
 * 
 * 定义插件的基本属性和行为
 */
interface PluginInterface
{
    /**
     * 获取插件名称
     */
    public function getName(): string;

    /**
     * 获取插件版本
     */
    public function getVersion(): string;

    /**
     * 获取插件描述
     */
    public function getDescription(): string;

    /**
     * 获取插件作者
     */
    public function getAuthor(): string;

    /**
     * 获取插件标识
     */
    public function getSlug(): string;

    /**
     * 是否已激活
     */
    public function isActive(): bool;

    /**
     * 获取插件能力
     * 
     * @return string[]
     */
    public function getCapabilities(): array;

    /**
     * 获取插件配置
     */
    public function getConfig(): array;

    /**
     * 转换为数组
     */
    public function toArray(): array;
}

/**
 * 插件注册表接口
 * 
 * 定义插件注册和管理的抽象层
 */
interface PluginRegistryInterface
{
    /**
     * 注册插件
     */
    public function register(PluginInterface $plugin): void;

    /**
     * 获取所有插件
     * 
     * @return PluginInterface[]
     */
    public function getAll(): array;

    /**
     * 根据名称获取插件
     */
    public function get(string $name): ?PluginInterface;

    /**
     * 插件是否存在
     */
    public function has(string $name): bool;

    /**
     * 激活插件
     */
    public function activate(string $name): bool;

    /**
     * 停用插件
     */
    public function deactivate(string $name): bool;

    /**
     * 获取已激活的插件
     * 
     * @return PluginInterface[]
     */
    public function getActive(): array;
}

/**
 * Hook 接口
 * 
 * 定义 Hook 系统的抽象层
 */
interface HookInterface
{
    /**
     * 注册 Hook
     */
    public function register(string $hook, callable $callback, int $priority = 10): void;

    /**
     * 触发 Hook
     */
    public function trigger(string $hook, ...$args): array;

    /**
     * 移除 Hook
     */
    public function remove(string $hook, ?callable $callback = null): void;

    /**
     * Hook 是否存在
     */
    public function has(string $hook): bool;
}
