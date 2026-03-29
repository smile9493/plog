<?php

declare(strict_types=1);

namespace Plog\Plugin;

use Plog\Core\Config\ConfigManagerInterface;
use Plog\Core\Event\EventDispatcherInterface;
use InvalidArgumentException;
use RuntimeException;

/**
 * 插件管理器
 */
class PluginManager
{
    /**
     * 配置管理器
     *
     * @var ConfigManagerInterface
     */
    private ConfigManagerInterface $config;

    /**
     * 事件调度器
     *
     * @var EventDispatcherInterface
     */
    private EventDispatcherInterface $events;

    /**
     * 钩子系统
     *
     * @var HookSystem
     */
    private HookSystem $hooks;

    /**
     * 已加载的插件
     *
     * @var array
     */
    private array $plugins = [];

    /**
     * 插件目录
     *
     * @var string
     */
    private string $pluginDir;

    /**
     * 构造函数
     *
     * @param ConfigManagerInterface $config 配置管理器
     * @param EventDispatcherInterface $events 事件调度器
     * @param string $pluginDir 插件目录
     */
    public function __construct(
        ConfigManagerInterface $config,
        EventDispatcherInterface $events,
        string $pluginDir
    ) {
        $this->config = $config;
        $this->events = $events;
        $this->hooks = new HookSystem();
        $this->pluginDir = $pluginDir;
    }

    /**
     * 发现所有插件
     *
     * @return array
     */
    public function discover(): array
    {
        $plugins = [];

        if (!is_dir($this->pluginDir)) {
            return $plugins;
        }

        $dirs = scandir($this->pluginDir);
        foreach ($dirs as $dir) {
            if ($dir === '.' || $dir === '..') {
                continue;
            }

            $manifestFile = $this->pluginDir . '/' . $dir . '/plugin.json';
            if (file_exists($manifestFile)) {
                $manifest = $this->loadManifest($manifestFile);
                if ($manifest !== null) {
                    $plugins[$dir] = $manifest;
                }
            }
        }

        return $plugins;
    }

    /**
     * 加载插件Manifest
     *
     * @param string $file 文件路径
     * @return array|null
     */
    private function loadManifest(string $file): ?array
    {
        $content = file_get_contents($file);
        if ($content === false) {
            return null;
        }

        $manifest = json_decode($content, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            return null;
        }

        // 验证必需字段
        $required = ['name', 'version', 'type', 'entry'];
        foreach ($required as $field) {
            if (!isset($manifest[$field])) {
                return null;
            }
        }

        return $manifest;
    }

    /**
     * 加载插件
     *
     * @param string $name 插件名称
     * @return bool
     */
    public function load(string $name): bool
    {
        if (isset($this->plugins[$name])) {
            return true;
        }

        $manifestFile = $this->pluginDir . '/' . $name . '/plugin.json';
        if (!file_exists($manifestFile)) {
            throw new InvalidArgumentException("Plugin not found: {$name}");
        }

        $manifest = $this->loadManifest($manifestFile);
        if ($manifest === null) {
            throw new RuntimeException("Invalid plugin manifest: {$name}");
        }

        // 检查依赖
        if (!$this->checkDependencies($manifest)) {
            throw new RuntimeException("Plugin dependencies not satisfied: {$name}");
        }

        // 加载插件类
        $entryFile = $this->pluginDir . '/' . $name . '/' . $manifest['entry'];
        if (!file_exists($entryFile)) {
            throw new RuntimeException("Plugin entry file not found: {$name}");
        }

        require_once $entryFile;

        $this->plugins[$name] = [
            'manifest' => $manifest,
            'enabled' => false,
        ];

        return true;
    }

    /**
     * 启用插件
     *
     * @param string $name 插件名称
     * @return bool
     */
    public function enable(string $name): bool
    {
        if (!isset($this->plugins[$name])) {
            $this->load($name);
        }

        $this->plugins[$name]['enabled'] = true;

        // 触发插件启用事件
        $this->events->dispatch('plugin.enabled', $name);

        return true;
    }

    /**
     * 禁用插件
     *
     * @param string $name 插件名称
     * @return bool
     */
    public function disable(string $name): bool
    {
        if (!isset($this->plugins[$name])) {
            return false;
        }

        $this->plugins[$name]['enabled'] = false;

        // 触发插件禁用事件
        $this->events->dispatch('plugin.disabled', $name);

        return true;
    }

    /**
     * 检查插件是否已启用
     *
     * @param string $name 插件名称
     * @return bool
     */
    public function isEnabled(string $name): bool
    {
        return isset($this->plugins[$name]) && $this->plugins[$name]['enabled'];
    }

    /**
     * 获取插件信息
     *
     * @param string $name 插件名称
     * @return array|null
     */
    public function getPlugin(string $name): ?array
    {
        return $this->plugins[$name] ?? null;
    }

    /**
     * 获取所有插件
     *
     * @return array
     */
    public function getPlugins(): array
    {
        return $this->plugins;
    }

    /**
     * 检查依赖
     *
     * @param array $manifest 插件manifest
     * @return bool
     */
    private function checkDependencies(array $manifest): bool
    {
        if (!isset($manifest['dependencies'])) {
            return true;
        }

        foreach ($manifest['dependencies'] as $package => $version) {
            // 检查核心依赖
            if ($package === 'core') {
                // TODO: 版本比较
                continue;
            }

            // 检查插件依赖
            if (!isset($this->plugins[$package])) {
                return false;
            }
        }

        return true;
    }

    /**
     * 获取钩子系统
     *
     * @return HookSystem
     */
    public function getHooks(): HookSystem
    {
        return $this->hooks;
    }
}
