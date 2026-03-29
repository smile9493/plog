<?php

declare(strict_types=1);

namespace Plog\Theme;

/**
 * 主题接口
 */
interface ThemeInterface
{
    /**
     * 获取主题信息
     *
     * @return array
     */
    public function getInfo(): array;

    /**
     * 主题安装
     *
     * @return bool
     */
    public function install(): bool;

    /**
     * 主题卸载
     *
     * @return bool
     */
    public function uninstall(): bool;

    /**
     * 主题激活
     *
     * @return bool
     */
    public function activate(): bool;

    /**
     * 主题停用
     *
     * @return bool
     */
    public function deactivate(): bool;

    /**
     * 获取主题配置
     *
     * @return array
     */
    public function getConfig(): array;

    /**
     * 设置主题配置
     *
     * @param array $config 配置数据
     * @return bool
     */
    public function setConfig(array $config): bool;

    /**
     * 渲染模板
     *
     * @param string $template 模板名称
     * @param array $data 数据
     * @return string
     */
    public function render(string $template, array $data = []): string;
}
