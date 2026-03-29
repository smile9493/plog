<?php
/**
 * 主题加载器
 */

class ThemeLoader
{
    private $themeDir;
    
    public function __construct(array $options = [])
    {
        $this->themeDir = $options['theme_dir'] ?? __DIR__ . '/../../content/templates';
    }
    
    /**
     * 发现所有主题
     */
    public function discover()
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
            
            $themePath = $this->themeDir . '/' . $dir;
            
            if (is_dir($themePath)) {
                $themeInfo = $this->getThemeInfo($themePath);
                
                if ($themeInfo) {
                    $themes[$themeInfo['name']] = $themeInfo;
                }
            }
        }
        
        return $themes;
    }
    
    /**
     * 获取主题信息
     */
    public function getThemeInfo($themePath)
    {
        // 检查是否有 header.php
        $headerFile = $themePath . '/header.php';
        
        if (!file_exists($headerFile)) {
            return null;
        }
        
        // 尝试从 preview.jpg 或 header.php 读取信息
        $info = [
            'name' => basename($themePath),
            'version' => '1.0.0',
            'description' => '',
            'author' => '',
            'dir' => $themePath,
            'templates' => $this->scanTemplates($themePath),
        ];
        
        // 尝试从 header.php 读取更多信息
        $headerContent = file_get_contents($headerFile);
        
        if (preg_match('/Theme Name:\s*(.+)/i', $headerContent, $matches)) {
            $info['name'] = trim($matches[1]);
        }
        
        if (preg_match('/Version:\s*(.+)/i', $headerContent, $matches)) {
            $info['version'] = trim($matches[1]);
        }
        
        if (preg_match('/Description:\s*(.+)/i', $headerContent, $matches)) {
            $info['description'] = trim($matches[1]);
        }
        
        if (preg_match('/Author:\s*(.+)/i', $headerContent, $matches)) {
            $info['author'] = trim($matches[1]);
        }
        
        return $info;
    }
    
    /**
     * 扫描模板文件
     */
    private function scanTemplates($themePath)
    {
        $templates = [];
        
        $files = glob($themePath . '/*.php');
        
        foreach ($files as $file) {
            $name = basename($file, '.php');
            $templates[$name] = $file;
        }
        
        return $templates;
    }
    
    /**
     * 加载主题
     */
    public function load($themeName)
    {
        $themePath = $this->themeDir . '/' . $themeName;
        
        if (!is_dir($themePath)) {
            return false;
        }
        
        return $this->getThemeInfo($themePath);
    }
    
    /**
     * 检查主题是否存在
     */
    public function exists($themeName)
    {
        $themePath = $this->themeDir . '/' . $themeName;
        return is_dir($themePath);
    }
}
