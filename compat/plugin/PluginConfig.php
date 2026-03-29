<?php
/**
 * 插件配置管理
 */

class PluginConfig
{
    private $configDir;
    private $cache = [];
    
    public function __construct(array $options = [])
    {
        $this->configDir = $options['config_dir'] ?? __DIR__ . '/../../content/plugin_configs';
        
        if (!is_dir($this->configDir)) {
            mkdir($this->configDir, 0755, true);
        }
    }
    
    /**
     * 获取插件配置
     */
    public function get($pluginName, $key = null, $default = null)
    {
        // 检查缓存
        if (isset($this->cache[$pluginName])) {
            $config = $this->cache[$pluginName];
        } else {
            $config = $this->loadConfig($pluginName);
            $this->cache[$pluginName] = $config;
        }
        
        if ($key === null) {
            return $config;
        }
        
        return $config[$key] ?? $default;
    }
    
    /**
     * 设置插件配置
     */
    public function set($pluginName, $key, $value = null)
    {
        if (is_array($key)) {
            // 批量设置
            $config = $this->get($pluginName);
            $config = array_merge($config, $key);
        } else {
            // 单个设置
            $config = $this->get($pluginName);
            $config[$key] = $value;
        }
        
        $this->cache[$pluginName] = $config;
        
        return $this->saveConfig($pluginName, $config);
    }
    
    /**
     * 删除配置项
     */
    public function delete($pluginName, $key = null)
    {
        if ($key === null) {
            // 删除整个配置
            $this->cache[$pluginName] = [];
            return $this->saveConfig($pluginName, []);
        }
        
        $config = $this->get($pluginName);
        unset($config[$key]);
        
        $this->cache[$pluginName] = $config;
        
        return $this->saveConfig($pluginName, $config);
    }
    
    /**
     * 加载配置文件
     */
    private function loadConfig($pluginName)
    {
        $configFile = $this->getConfigPath($pluginName);
        
        if (!file_exists($configFile)) {
            return [];
        }
        
        $content = file_get_contents($configFile);
        $config = json_decode($content, true);
        
        return is_array($config) ? $config : [];
    }
    
    /**
     * 保存配置文件
     */
    private function saveConfig($pluginName, $config)
    {
        $configFile = $this->getConfigPath($pluginName);
        
        $content = json_encode($config, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);
        
        return file_put_contents($configFile, $content) !== false;
    }
    
    /**
     * 获取配置文件路径
     */
    private function getConfigPath($pluginName)
    {
        $safeName = preg_replace('/[^a-zA-Z0-9_-]/', '_', $pluginName);
        return $this->configDir . '/' . $safeName . '.json';
    }
    
    /**
     * 检查配置是否存在
     */
    public function has($pluginName, $key = null)
    {
        if ($key === null) {
            $configFile = $this->getConfigPath($pluginName);
            return file_exists($configFile);
        }
        
        $config = $this->get($pluginName);
        return isset($config[$key]);
    }
    
    /**
     * 清除缓存
     */
    public function clearCache($pluginName = null)
    {
        if ($pluginName === null) {
            $this->cache = [];
        } else {
            unset($this->cache[$pluginName]);
        }
    }
}
