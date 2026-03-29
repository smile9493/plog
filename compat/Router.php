<?php
/**
 * 路由类
 */

class Router
{
    private $config;
    private $routes;
    
    public function __construct(array $config)
    {
        $this->config = $config;
        $this->routes = $config['routes']['v1'] ?? [];
    }
    
    /**
     * 匹配路由
     */
    public function match(array $request)
    {
        $uri = $request['uri'];
        $method = $request['method'];
        
        // 移除 /api 前缀
        $path = preg_replace('#^/api/?#', '', $uri);
        $path = trim($path, '/');
        
        // 直接匹配
        if (isset($this->routes[$path])) {
            return [
                'target' => $this->routes[$path],
                'params' => [],
            ];
        }
        
        // 参数匹配
        foreach ($this->routes as $pattern => $target) {
            $params = $this->matchPattern($pattern, $path);
            if ($params !== false) {
                return [
                    'target' => $target,
                    'params' => $params,
                ];
            }
        }
        
        return null;
    }
    
    /**
     * 模式匹配
     */
    private function matchPattern($pattern, $path)
    {
        // 将 :param 替换为正则
        $regex = preg_replace('/:([a-zA-Z_]+)/', '(?P<$1>[^/]+)', $pattern);
        $regex = '#^' . $regex . '$#';
        
        if (preg_match($regex, $path, $matches)) {
            $params = [];
            foreach ($matches as $key => $value) {
                if (is_string($key)) {
                    $params[$key] = $value;
                }
            }
            return $params;
        }
        
        return false;
    }
    
    /**
     * 获取所有路由
     */
    public function getRoutes()
    {
        return $this->routes;
    }
}
