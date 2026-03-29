<?php
/**
 * Hook 桥接类
 * 
 * 将 PHP Hook 转换为 Rust API 调用
 */

class HookBridge
{
    private $hooks = [];
    private $rustApiUrl;
    
    public function __construct($rustApiUrl = 'http://127.0.0.1:8080')
    {
        $this->rustApiUrl = $rustApiUrl;
    }
    
    /**
     * 注册 Hook
     */
    public function register($type, $name, $callback, $pluginName = 'unknown', $priority = 10)
    {
        if (!isset($this->hooks[$type])) {
            $this->hooks[$type] = [];
        }
        
        if (!isset($this->hooks[$type][$name])) {
            $this->hooks[$type][$name] = [];
        }
        
        $this->hooks[$type][$name][] = [
            'callback' => $callback,
            'plugin' => $pluginName,
            'priority' => $priority,
        ];
        
        // 按优先级排序
        usort($this->hooks[$type][$name], function ($a, $b) {
            return $a['priority'] - $b['priority'];
        });
    }
    
    /**
     * 触发 Hook
     */
    public function trigger($type, $name, $data = null)
    {
        // 先通知 Rust 核心
        $this->notifyRustCore($type, $name, $data);
        
        // 然后执行本地 Hook
        if (!isset($this->hooks[$type][$name])) {
            return $data;
        }
        
        foreach ($this->hooks[$type][$name] as $hook) {
            if (is_callable($hook['callback'])) {
                try {
                    $result = call_user_func($hook['callback'], $data);
                    
                    // filter 类型会修改数据
                    if ($type === 'filter' && $result !== null) {
                        $data = $result;
                    }
                } catch (\Exception $e) {
                    error_log('Hook error: ' . $e->getMessage());
                }
            }
        }
        
        return $data;
    }
    
    /**
     * 移除 Hook
     */
    public function remove($type, $name, $pluginName = null)
    {
        if (!isset($this->hooks[$type][$name])) {
            return false;
        }
        
        if ($pluginName === null) {
            unset($this->hooks[$type][$name]);
        } else {
            foreach ($this->hooks[$type][$name] as $key => $hook) {
                if ($hook['plugin'] === $pluginName) {
                    unset($this->hooks[$type][$name][$key]);
                }
            }
        }
        
        return true;
    }
    
    /**
     * 移除插件的所有 Hook
     */
    public function removeAll($pluginName)
    {
        foreach ($this->hooks as $type => $hooks) {
            foreach ($hooks as $name => $hookList) {
                foreach ($hookList as $key => $hook) {
                    if ($hook['plugin'] === $pluginName) {
                        unset($this->hooks[$type][$name][$key]);
                    }
                }
            }
        }
    }
    
    /**
     * 获取所有 Hook
     */
    public function getAll()
    {
        return $this->hooks;
    }
    
    /**
     * 获取指定类型的 Hook
     */
    public function getByType($type)
    {
        return $this->hooks[$type] ?? [];
    }
    
    /**
     * 通知 Rust 核心
     */
    private function notifyRustCore($type, $name, $data)
    {
        try {
            $url = $this->rustApiUrl . '/api/v2/hooks';
            
            $payload = json_encode([
                'type' => $type,
                'name' => $name,
                'data' => $data,
            ]);
            
            $ch = curl_init();
            curl_setopt_array($ch, [
                CURLOPT_URL => $url,
                CURLOPT_POST => true,
                CURLOPT_POSTFIELDS => $payload,
                CURLOPT_RETURNTRANSFER => true,
                CURLOPT_TIMEOUT => 5,
                CURLOPT_HTTPHEADER => [
                    'Content-Type: application/json',
                ],
            ]);
            
            curl_exec($ch);
            curl_close($ch);
        } catch (\Exception $e) {
            // 静默失败，不影响本地 Hook 执行
        }
    }
    
    /**
     * 检查 Hook 是否存在
     */
    public function has($type, $name)
    {
        return isset($this->hooks[$type][$name]) && !empty($this->hooks[$type][$name]);
    }
}
