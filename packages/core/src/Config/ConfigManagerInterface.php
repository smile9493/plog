<?php

declare(strict_types=1);

namespace Plog\Core\Config;

/**
 * 配置管理器接口
 */
interface ConfigManagerInterface
{
    /**
     * 获取配置值
     *
     * @param string $key 配置键,支持点号分隔 (如: database.mysql.host)
     * @param mixed $default 默认值
     * @return mixed
     */
    public function get(string $key, $default = null);

    /**
     * 设置配置值
     *
     * @param string $key 配置键
     * @param mixed $value 配置值
     */
    public function set(string $key, $value): void;

    /**
     * 检查配置是否存在
     *
     * @param string $key 配置键
     * @return bool
     */
    public function has(string $key): bool;

    /**
     * 获取所有配置
     *
     * @return array
     */
    public function all(): array;

    /**
     * 加载配置源
     *
     * @param string $source 配置源
     */
    public function load(string $source): void;
}
