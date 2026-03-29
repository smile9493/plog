<?php

declare(strict_types=1);

namespace Plog\Theme;

use Plog\Core\Config\ConfigManagerInterface;
use Plog\Core\Event\EventDispatcherInterface;
use InvalidArgumentException;
use RuntimeException;

/**
 * 主题管理器
 */
class ThemeManager
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
     * 主题目录
     *
     * @var string
     */
    private string $themeDir;

    /**
     * 已加载的主题
     *
     * @var array
     */
    private array $themes = [];

    /**
     * 当前激活的主题
     *
     * @var string|null
     */
    private ?string $activeTheme = null;

    /**
     * 模板引擎
     *
     * @var TemplateEngine|null
     */
    private ?TemplateEngine $templateEngine = null;

    /**
     * 资源管理器
     *
     * @var AssetManager|null
     */
    private ?AssetManager $assetManager = null;

    /**
     * 构造函数
     *
     * @param ConfigManagerInterface $config 配置管理器
     * @param EventDispatcherInterface $events 事件调度器
     * @param string $themeDir 主题目录
     */
    public function __construct(
        ConfigManagerInterface $config,
        EventDispatcherInterface $events,
        string $themeDir
    ) {
        $this->config = $config;
        $this->events = $events;
        $this->themeDir = $themeDir;
    }

    /**
     * 发现所有主题
     *
     * @return array
     */
    public function discover(): array
    {
        $themes = [];

        if (!is_dir($this->themeDir)) {
            return $themes;
        }

        $dirs = scandir($this->themeDir);
        foreach ($dirs as $dir) {
            if ($dir === '.' || $dir === '..') {
                continue;
            }

            $manifestFile = $this->themeDir . '/' . $dir . '/theme.json';
            if (file_exists($manifestFile)) {
                $manifest = $this->loadManifest($manifestFile);
                if ($manifest !== null) {
                    $themes[$dir] = $manifest;
                }
            }
        }

        return $themes;
    }

    /**
     * 加载主题Manifest
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
     * 加载主题
     *
     * @param string $name 主题名称
     * @return bool
     */
    public function load(string $name): bool
    {
        if (isset($this->themes[$name])) {
            return true;
        }

        $manifestFile = $this->themeDir . '/' . $name . '/theme.json';
        if (!file_exists($manifestFile)) {
            throw new InvalidArgumentException("Theme not found: {$name}");
        }

        $manifest = $this->loadManifest($manifestFile);
        if ($manifest === null) {
            throw new RuntimeException("Invalid theme manifest: {$name}");
        }

        $this->themes[$name] = [
            'manifest' => $manifest,
            'path' => $this->themeDir . '/' . $name,
        ];

        return true;
    }

    /**
     * 激活主题
     *
     * @param string $name 主题名称
     * @return bool
     */
    public function activate(string $name): bool
    {
        if (!isset($this->themes[$name])) {
            $this->load($name);
        }

        $this->activeTheme = $name;
        $themePath = $this->themes[$name]['path'];

        // 初始化模板引擎
        $this->templateEngine = new TemplateEngine($themePath);

        // 初始化资源管理器
        $manifest = $this->themes[$name]['manifest'];
        $this->assetManager = new AssetManager($manifest['version']);

        // 注册主题资源
        if (isset($manifest['assets'])) {
            $this->registerAssets($manifest['assets']);
        }

        // 触发主题激活事件
        $this->events->dispatch('theme.activated', $name);

        return true;
    }

    /**
     * 注册资源
     *
     * @param array $assets 资源配置
     * @return void
     */
    private function registerAssets(array $assets): void
    {
        if (!$this->assetManager) {
            return;
        }

        if (isset($assets['css'])) {
            foreach ($assets['css'] as $index => $css) {
                $this->assetManager->registerCss('theme-css-' . $index, $css);
            }
        }

        if (isset($assets['js'])) {
            foreach ($assets['js'] as $index => $js) {
                $this->assetManager->registerJs('theme-js-' . $index, $js);
            }
        }
    }

    /**
     * 渲染模板
     *
     * @param string $template 模板名称
     * @param array $data 数据
     * @return string
     */
    public function render(string $template, array $data = []): string
    {
        if (!$this->templateEngine) {
            throw new RuntimeException('No active theme');
        }

        return $this->templateEngine->render($template, $data);
    }

    /**
     * 获取当前激活主题
     *
     * @return string|null
     */
    public function getActiveTheme(): ?string
    {
        return $this->activeTheme;
    }

    /**
     * 获取主题信息
     *
     * @param string $name 主题名称
     * @return array|null
     */
    public function getTheme(string $name): ?array
    {
        return $this->themes[$name] ?? null;
    }

    /**
     * 获取所有主题
     *
     * @return array
     */
    public function getThemes(): array
    {
        return $this->themes;
    }

    /**
     * 获取模板引擎
     *
     * @return TemplateEngine|null
     */
    public function getTemplateEngine(): ?TemplateEngine
    {
        return $this->templateEngine;
    }

    /**
     * 获取资源管理器
     *
     * @return AssetManager|null
     */
    public function getAssetManager(): ?AssetManager
    {
        return $this->assetManager;
    }
}
