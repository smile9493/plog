<?php

declare(strict_types=1);

namespace Plog\Plugin;

/**
 * 插件接口
 */
interface PluginInterface
{
    /**
     * 获取插件信息
     *
     * @return array
     */
    public function getInfo(): array;

    /**
     * 插件安装
     *
     * @return bool
     */
    public function install(): bool;

    /**
     * 插件卸载
     *
     * @return bool
     */
    public function uninstall(): bool;

    /**
     * 插件启用
     *
     * @return bool
     */
    public function enable(): bool;

    /**
     * 插件禁用
     *
     * @return bool
     */
    public function disable(): bool;

    /**
     * 获取插件配置
     *
     * @return array
     */
    public function getConfig(): array;

    /**
     * 设置插件配置
     *
     * @param array $config 配置数据
     * @return bool
     */
    public function setConfig(array $config): bool;
}
