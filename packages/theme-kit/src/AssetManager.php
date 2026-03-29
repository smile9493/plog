<?php

declare(strict_types=1);

namespace Plog\Theme;

/**
 * 资源管理器
 */
class AssetManager
{
    /**
     * CSS资源列表
     *
     * @var array
     */
    private array $css = [];

    /**
     * JS资源列表
     *
     * @var array
     */
    private array $js = [];

    /**
     * 资源版本号
     *
     * @var string
     */
    private string $version;

    /**
     * 是否压缩
     *
     * @var bool
     */
    private bool $minify;

    /**
     * 构造函数
     *
     * @param string $version 版本号
     * @param bool $minify 是否压缩
     */
    public function __construct(string $version = '1.0.0', bool $minify = false)
    {
        $this->version = $version;
        $this->minify = $minify;
    }

    /**
     * 注册CSS资源
     *
     * @param string $name 资源名称
     * @param string $path 资源路径
     * @param array $dependencies 依赖
     * @param array $attributes 属性
     * @return void
     */
    public function registerCss(string $name, string $path, array $dependencies = [], array $attributes = []): void
    {
        $this->css[$name] = [
            'path' => $path,
            'dependencies' => $dependencies,
            'attributes' => $attributes,
        ];
    }

    /**
     * 注册JS资源
     *
     * @param string $name 资源名称
     * @param string $path 资源路径
     * @param array $dependencies 依赖
     * @param array $attributes 属性
     * @return void
     */
    public function registerJs(string $name, string $path, array $dependencies = [], array $attributes = []): void
    {
        $this->js[$name] = [
            'path' => $path,
            'dependencies' => $dependencies,
            'attributes' => $attributes,
        ];
    }

    /**
     * 输出CSS标签
     *
     * @param array $names 资源名称列表
     * @return string
     */
    public function renderCss(array $names = []): string
    {
        if (empty($names)) {
            $names = array_keys($this->css);
        }

        $html = '';
        $rendered = [];

        foreach ($names as $name) {
            $html .= $this->renderCssItem($name, $rendered);
        }

        return $html;
    }

    /**
     * 输出单个CSS标签
     *
     * @param string $name 资源名称
     * @param array $rendered 已渲染列表
     * @return string
     */
    private function renderCssItem(string $name, array &$rendered): string
    {
        if (in_array($name, $rendered) || !isset($this->css[$name])) {
            return '';
        }

        $html = '';
        $css = $this->css[$name];

        // 先渲染依赖
        foreach ($css['dependencies'] as $dep) {
            $html .= $this->renderCssItem($dep, $rendered);
        }

        // 添加版本号
        $path = $this->addVersion($css['path']);

        // 构建属性
        $attrs = $this->buildAttributes($css['attributes']);

        $html .= "<link rel=\"stylesheet\" href=\"{$path}\"{$attrs}>\n";
        $rendered[] = $name;

        return $html;
    }

    /**
     * 输出JS标签
     *
     * @param array $names 资源名称列表
     * @return string
     */
    public function renderJs(array $names = []): string
    {
        if (empty($names)) {
            $names = array_keys($this->js);
        }

        $html = '';
        $rendered = [];

        foreach ($names as $name) {
            $html .= $this->renderJsItem($name, $rendered);
        }

        return $html;
    }

    /**
     * 输出单个JS标签
     *
     * @param string $name 资源名称
     * @param array $rendered 已渲染列表
     * @return string
     */
    private function renderJsItem(string $name, array &$rendered): string
    {
        if (in_array($name, $rendered) || !isset($this->js[$name])) {
            return '';
        }

        $html = '';
        $js = $this->js[$name];

        // 先渲染依赖
        foreach ($js['dependencies'] as $dep) {
            $html .= $this->renderJsItem($dep, $rendered);
        }

        // 添加版本号
        $path = $this->addVersion($js['path']);

        // 构建属性
        $attrs = $this->buildAttributes($js['attributes']);

        $html .= "<script src=\"{$path}\"{$attrs}></script>\n";
        $rendered[] = $name;

        return $html;
    }

    /**
     * 添加版本号
     *
     * @param string $path 路径
     * @return string
     */
    private function addVersion(string $path): string
    {
        $separator = strpos($path, '?') === false ? '?' : '&';
        return $path . $separator . 'v=' . $this->version;
    }

    /**
     * 构建属性字符串
     *
     * @param array $attributes 属性数组
     * @return string
     */
    private function buildAttributes(array $attributes): string
    {
        if (empty($attributes)) {
            return '';
        }

        $attrs = [];
        foreach ($attributes as $key => $value) {
            if (is_bool($value) && $value) {
                $attrs[] = $key;
            } else {
                $attrs[] = "{$key}=\"{$value}\"";
            }
        }

        return ' ' . implode(' ', $attrs);
    }

    /**
     * 获取所有CSS资源
     *
     * @return array
     */
    public function getCss(): array
    {
        return $this->css;
    }

    /**
     * 获取所有JS资源
     *
     * @return array
     */
    public function getJs(): array
    {
        return $this->js;
    }
}
