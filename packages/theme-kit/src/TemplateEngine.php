<?php

declare(strict_types=1);

namespace Plog\Theme;

use InvalidArgumentException;
use RuntimeException;

/**
 * 模板引擎
 */
class TemplateEngine
{
    /**
     * 主题目录
     *
     * @var string
     */
    private string $themeDir;

    /**
     * 模板缓存
     *
     * @var array
     */
    private array $cache = [];

    /**
     * 全局变量
     *
     * @var array
     */
    private array $globals = [];

    /**
     * 构造函数
     *
     * @param string $themeDir 主题目录
     */
    public function __construct(string $themeDir)
    {
        $this->themeDir = $themeDir;
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
        $templateFile = $this->findTemplate($template);
        
        if ($templateFile === null) {
            throw new InvalidArgumentException("Template not found: {$template}");
        }

        // 合并全局变量
        $data = array_merge($this->globals, $data);

        // 提取变量到当前作用域
        extract($data);

        // 开始输出缓冲
        ob_start();

        try {
            include $templateFile;
            return ob_get_contents();
        } finally {
            ob_end_clean();
        }
    }

    /**
     * 查找模板文件
     *
     * @param string $template 模板名称
     * @return string|null
     */
    private function findTemplate(string $template): ?string
    {
        // 检查缓存
        if (isset($this->cache[$template])) {
            return $this->cache[$template];
        }

        // 可能的模板文件路径
        $paths = [
            $this->themeDir . '/templates/' . $template . '.php',
            $this->themeDir . '/' . $template . '.php',
            $this->themeDir . '/templates/' . $template . '.html',
        ];

        foreach ($paths as $path) {
            if (file_exists($path)) {
                $this->cache[$template] = $path;
                return $path;
            }
        }

        return null;
    }

    /**
     * 设置全局变量
     *
     * @param string $name 变量名
     * @param mixed $value 变量值
     * @return void
     */
    public function setGlobal(string $name, $value): void
    {
        $this->globals[$name] = $value;
    }

    /**
     * 批量设置全局变量
     *
     * @param array $globals 变量数组
     * @return void
     */
    public function setGlobals(array $globals): void
    {
        $this->globals = array_merge($this->globals, $globals);
    }

    /**
     * 清除缓存
     *
     * @return void
     */
    public function clearCache(): void
    {
        $this->cache = [];
    }

    /**
     * 包含子模板
     *
     * @param string $template 模板名称
     * @param array $data 数据
     * @return string
     */
    public function include(string $template, array $data = []): string
    {
        return $this->render($template, $data);
    }

    /**
     * 检查模板是否存在
     *
     * @param string $template 模板名称
     * @return bool
     */
    public function exists(string $template): bool
    {
        return $this->findTemplate($template) !== null;
    }
}
