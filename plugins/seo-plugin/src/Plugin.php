<?php

declare(strict_types=1);

namespace SeoPlugin;

use Plog\Plugin\PluginInterface;

/**
 * SEO插件
 */
class Plugin implements PluginInterface
{
    /**
     * 插件配置
     *
     * @var array
     */
    private array $config = [];

    /**
     * 获取插件信息
     *
     * @return array
     */
    public function getInfo(): array
    {
        return [
            'name' => 'seo-plugin',
            'version' => '1.0.0',
            'description' => 'SEO优化插件',
            'author' => 'Plog Team',
        ];
    }

    /**
     * 插件安装
     *
     * @return bool
     */
    public function install(): bool
    {
        // 创建数据库表
        // 创建默认配置
        return true;
    }

    /**
     * 插件卸载
     *
     * @return bool
     */
    public function uninstall(): bool
    {
        // 删除数据库表
        // 删除配置
        return true;
    }

    /**
     * 插件启用
     *
     * @return bool
     */
    public function enable(): bool
    {
        // 注册钩子
        // 注册路由
        // 注册菜单
        return true;
    }

    /**
     * 插件禁用
     *
     * @return bool
     */
    public function disable(): bool
    {
        // 移除钩子
        // 移除路由
        // 移除菜单
        return true;
    }

    /**
     * 获取插件配置
     *
     * @return array
     */
    public function getConfig(): array
    {
        return $this->config;
    }

    /**
     * 设置插件配置
     *
     * @param array $config 配置数据
     * @return bool
     */
    public function setConfig(array $config): bool
    {
        $this->config = array_merge($this->config, $config);
        return true;
    }

    /**
     * 生成SEO元数据
     *
     * @param array $post 文章数据
     * @return array
     */
    public function generateMeta(array $post): array
    {
        $meta = [
            'title' => $post['title'] ?? '',
            'description' => $post['excerpt'] ?? '',
            'keywords' => '',
            'og:title' => $post['title'] ?? '',
            'og:description' => $post['excerpt'] ?? '',
            'twitter:title' => $post['title'] ?? '',
            'twitter:description' => $post['excerpt'] ?? '',
        ];

        return $meta;
    }

    /**
     * 生成Sitemap
     *
     * @return string
     */
    public function generateSitemap(): string
    {
        // TODO: 实现sitemap生成
        return '';
    }
}
