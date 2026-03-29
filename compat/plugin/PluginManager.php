<?php
/**
 * 插件兼容层
 * 
 * Phase 3: 桥接 PHP 插件和 Rust 核心
 */

// 防止直接访问
defined('PLOG_COMPAT') or define('PLOG_COMPAT', true);

require_once __DIR__ . '/PluginLoader.php';
require_once __DIR__ . '/HookBridge.php';
require_once __DIR__ . '/PluginConfig.php';

/**
 * 插件管理器
 */
class PluginManager
{
    private $loader;
    private $hookBridge;
    private $config;
    private $plugins = [];
    
    public function __construct(array $options = [])
    {
        $this->loader = new PluginLoader($options);
        $this->hookBridge = new HookBridge();
        $this->config = new PluginConfig($options);
    }
    
    /**
     * 加载所有插件
     */
    public function loadAll()
    {
        $pluginDirs = $this->loader->discover();
        
        foreach ($pluginDirs as $pluginDir) {
            $this->loadPlugin($pluginDir);
        }
        
        return count($this->plugins);
    }
    
    /**
     * 加载单个插件
     */
    public function loadPlugin($pluginDir)
    {
        $pluginInfo = $this->loader->getPluginInfo($pluginDir);
        
        if (!$pluginInfo) {
            return false;
        }
        
        // 检查依赖
        if (!$this->checkDependencies($pluginInfo)) {
            return false;
        }
        
        // 加载插件文件
        $result = $this->loader->load($pluginDir, $pluginInfo);
        
        if ($result) {
            $this->plugins[$pluginInfo['name']] = $pluginInfo;
            
            // 注册 Hook
            $this->registerHooks($pluginInfo);
        }
        
        return $result;
    }
    
    /**
     * 卸载插件
     */
    public function unloadPlugin($pluginName)
    {
        if (!isset($this->plugins[$pluginName])) {
            return false;
        }
        
        // 移除 Hook
        $this->hookBridge->removeAll($pluginName);
        
        // 卸载插件
        $this->loader->unload($pluginName);
        
        unset($this->plugins[$pluginName]);
        
        return true;
    }
    
    /**
     * 检查插件依赖
     */
    private function checkDependencies($pluginInfo)
    {
        if (!isset($pluginInfo['dependencies'])) {
            return true;
        }
        
        foreach ($pluginInfo['dependencies'] as $dependency) {
            if (!isset($this->plugins[$dependency])) {
                return false;
            }
        }
        
        return true;
    }
    
    /**
     * 注册 Hook
     */
    private function registerHooks($pluginInfo)
    {
        if (!isset($pluginInfo['hooks'])) {
            return;
        }
        
        foreach ($pluginInfo['hooks'] as $hook) {
            $this->hookBridge->register(
                $hook['type'],
                $hook['name'],
                $hook['callback'],
                $pluginInfo['name'],
                $hook['priority'] ?? 10
            );
        }
    }
    
    /**
     * 触发 Hook
     */
    public function triggerHook($type, $name, $data = null)
    {
        return $this->hookBridge->trigger($type, $name, $data);
    }
    
    /**
     * 获取已加载的插件
     */
    public function getLoadedPlugins()
    {
        return $this->plugins;
    }
    
    /**
     * 获取 Hook 桥接器
     */
    public function getHookBridge()
    {
        return $this->hookBridge;
    }
    
    /**
     * 获取插件配置
     */
    public function getPluginConfig($pluginName)
    {
        return $this->config->get($pluginName);
    }
    
    /**
     * 设置插件配置
     */
    public function setPluginConfig($pluginName, $config)
    {
        return $this->config->set($pluginName, $config);
    }
}

// 全局实例
if (!isset($GLOBALS['pluginManager'])) {
    $GLOBALS['pluginManager'] = new PluginManager();
}

/**
 * 全局 Hook 触发函数
 */
function do_action($hookName, $data = null)
{
    return $GLOBALS['pluginManager']->triggerHook('action', $hookName, $data);
}

/**
 * 全局 Hook 过滤函数
 */
function apply_filters($hookName, $data)
{
    return $GLOBALS['pluginManager']->triggerHook('filter', $hookName, $data);
}

/**
 * 注册 Hook 函数
 */
function add_hook($type, $hookName, $callback, $priority = 10)
{
    return $GLOBALS['pluginManager']->getHookBridge()->register($type, $hookName, $callback, 'global', $priority);
}
