<?php
/**
 * 插件加载器
 */

class PluginLoader
{
    private $pluginDir;
    private $loadedFiles = [];
    
    public function __construct(array $options = [])
    {
        $this->pluginDir = $options['plugin_dir'] ?? __DIR__ . '/../../content/plugins';
    }
    
    /**
     * 发现所有插件
     */
    public function discover()
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
            
            $pluginPath = $this->pluginDir . '/' . $dir;
            
            if (is_dir($pluginPath)) {
                $plugins[] = $pluginPath;
            }
        }
        
        return $plugins;
    }
    
    /**
     * 获取插件信息
     */
    public function getPluginInfo($pluginDir)
    {
        $pluginFile = $this->findPluginFile($pluginDir);
        
        if (!$pluginFile) {
            return null;
        }
        
        $content = file_get_contents($pluginFile);
        
        return $this->parsePluginHeader($content, $pluginDir);
    }
    
    /**
     * 查找插件主文件
     */
    private function findPluginFile($pluginDir)
    {
        // 尝试目录名.php
        $dirName = basename($pluginDir);
        $pluginFile = $pluginDir . '/' . $dirName . '.php';
        
        if (file_exists($pluginFile)) {
            return $pluginFile;
        }
        
        // 尝试查找任意 .php 文件
        $files = glob($pluginDir . '/*.php');
        
        foreach ($files as $file) {
            $content = file_get_contents($file);
            if (strpos($content, 'Plugin Name:') !== false) {
                return $file;
            }
        }
        
        return null;
    }
    
    /**
     * 解析插件头部信息
     */
    private function parsePluginHeader($content, $pluginDir)
    {
        $info = [
            'name' => '',
            'version' => '1.0.0',
            'description' => '',
            'author' => '',
            'dir' => $pluginDir,
        ];
        
        // 解析 Plugin Name
        if (preg_match('/Plugin Name:\s*(.+)/i', $content, $matches)) {
            $info['name'] = trim($matches[1]);
        }
        
        // 解析 Version
        if (preg_match('/Version:\s*(.+)/i', $content, $matches)) {
            $info['version'] = trim($matches[1]);
        }
        
        // 解析 Description
        if (preg_match('/Description:\s*(.+)/i', $content, $matches)) {
            $info['description'] = trim($matches[1]);
        }
        
        // 解析 Author
        if (preg_match('/Author:\s*(.+)/i', $content, $matches)) {
            $info['author'] = trim($matches[1]);
        }
        
        // 如果没有名称，使用目录名
        if (empty($info['name'])) {
            $info['name'] = basename($pluginDir);
        }
        
        return $info;
    }
    
    /**
     * 加载插件
     */
    public function load($pluginDir, $pluginInfo)
    {
        $pluginFile = $this->findPluginFile($pluginDir);
        
        if (!$pluginFile) {
            return false;
        }
        
        // 防止重复加载
        if (in_array($pluginFile, $this->loadedFiles)) {
            return true;
        }
        
        try {
            require_once $pluginFile;
            $this->loadedFiles[] = $pluginFile;
            return true;
        } catch (\Exception $e) {
            error_log('Plugin load error: ' . $e->getMessage());
            return false;
        }
    }
    
    /**
     * 卸载插件
     */
    public function unload($pluginName)
    {
        // PHP 无法真正卸载已加载的文件
        // 这里只是从跟踪列表中移除
        foreach ($this->loadedFiles as $key => $file) {
            if (strpos($file, $pluginName) !== false) {
                unset($this->loadedFiles[$key]);
            }
        }
        
        return true;
    }
    
    /**
     * 获取已加载的文件
     */
    public function getLoadedFiles()
    {
        return $this->loadedFiles;
    }
}
