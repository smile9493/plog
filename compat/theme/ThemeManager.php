<?php
/**
 * 主题兼容层
 * 
 * Phase 3: 桥接 PHP 主题和 Rust 核心
 */

// 防止直接访问
defined('PLOG_COMPAT') or define('PLOG_COMPAT', true);

require_once __DIR__ . '/ThemeLoader.php';
require_once __DIR__ . '/Renderer.php';
require_once __DIR__ . '/TemplateEngine.php';

/**
 * 主题管理器
 */
class ThemeManager
{
    private $loader;
    private $renderer;
    private $engine;
    private $currentTheme;
    private $themes = [];
    
    public function __construct(array $options = [])
    {
        $this->loader = new ThemeLoader($options);
        $this->engine = new TemplateEngine($options);
        $this->renderer = new Renderer($this->engine, $options);
    }
    
    /**
     * 加载所有主题
     */
    public function loadAll()
    {
        $this->themes = $this->loader->discover();
        return count($this->themes);
    }
    
    /**
     * 设置当前主题
     */
    public function setTheme($themeName)
    {
        if (!isset($this->themes[$themeName])) {
            return false;
        }
        
        $this->currentTheme = $themeName;
        $this->renderer->setTheme($this->themes[$themeName]);
        
        return true;
    }
    
    /**
     * 获取当前主题
     */
    public function getCurrentTheme()
    {
        return $this->currentTheme;
    }
    
    /**
     * 获取主题信息
     */
    public function getThemeInfo($themeName = null)
    {
        $themeName = $themeName ?? $this->currentTheme;
        
        if (!isset($this->themes[$themeName])) {
            return null;
        }
        
        return $this->themes[$themeName];
    }
    
    /**
     * 获取所有主题
     */
    public function getAllThemes()
    {
        return $this->themes;
    }
    
    /**
     * 渲染模板
     */
    public function render($template, $data = [])
    {
        return $this->renderer->render($template, $data);
    }
    
    /**
     * 渲染页面
     */
    public function renderPage($pageType, $data = [])
    {
        $template = $this->resolveTemplate($pageType);
        
        if (!$template) {
            return $this->render404();
        }
        
        return $this->render($template, $data);
    }
    
    /**
     * 解析模板
     */
    private function resolveTemplate($pageType)
    {
        $theme = $this->themes[$this->currentTheme] ?? null;
        
        if (!$theme) {
            return null;
        }
        
        $templateMap = [
            'index' => 'log_list.php',
            'post' => 'echo_log.php',
            'page' => 'page.php',
            'category' => 'log_list.php',
            'tag' => 'log_list.php',
            'search' => 'log_list.php',
            '404' => '404.php',
        ];
        
        $templateFile = $templateMap[$pageType] ?? null;
        
        if (!$templateFile) {
            return null;
        }
        
        $templatePath = $theme['dir'] . '/' . $templateFile;
        
        return file_exists($templatePath) ? $templatePath : null;
    }
    
    /**
     * 渲染 404 页面
     */
    private function render404()
    {
        $theme = $this->themes[$this->currentTheme] ?? null;
        
        if ($theme) {
            $templatePath = $theme['dir'] . '/404.php';
            if (file_exists($templatePath)) {
                return $this->render($templatePath, ['error' => 'Page not found']);
            }
        }
        
        return '<h1>404 - Page Not Found</h1>';
    }
    
    /**
     * 获取渲染器
     */
    public function getRenderer()
    {
        return $this->renderer;
    }
    
    /**
     * 获取模板引擎
     */
    public function getEngine()
    {
        return $this->engine;
    }
}

// 全局实例
if (!isset($GLOBALS['themeManager'])) {
    $GLOBALS['themeManager'] = new ThemeManager();
}

/**
 * 全局渲染函数
 */
function render_template($template, $data = [])
{
    return $GLOBALS['themeManager']->render($template, $data);
}

/**
 * 全局页面渲染函数
 */
function render_page($pageType, $data = [])
{
    return $GLOBALS['themeManager']->renderPage($pageType, $data);
}
