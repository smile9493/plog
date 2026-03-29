<?php
/**
 * Plog CMS PHP 兼容层
 * 
 * Phase 3: 将请求转发到 Rust API
 */

// 防止直接访问
defined('PLOG_COMPAT') or define('PLOG_COMPAT', true);

// 加载配置
require_once __DIR__ . '/config.php';
require_once __DIR__ . '/Logger.php';
require_once __DIR__ . '/Proxy.php';
require_once __DIR__ . '/Router.php';
require_once __DIR__ . '/Response.php';

/**
 * 兼容层主类
 */
class CompatLayer
{
    private $config;
    private $router;
    private $proxy;
    private $logger;
    
    public function __construct()
    {
        $this->config = require __DIR__ . '/config.php';
        $this->logger = Logger::getInstance($this->config);
        $this->router = new Router($this->config);
        $this->proxy = new Proxy($this->config);
    }
    
    /**
     * 处理请求
     */
    public function handle()
    {
        try {
            // 设置 CORS
            $this->setCorsHeaders();
            
            // 处理预检请求
            if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
                Response::send('', 200);
                return;
            }
            
            // 解析请求
            $request = $this->parseRequest();
            
            $this->logger->info('Incoming request', [
                'method' => $request['method'],
                'uri' => $request['uri'],
            ]);
            
            // 路由匹配
            $route = $this->router->match($request);
            
            if (!$route) {
                $this->logger->warning('Route not found', ['uri' => $request['uri']]);
                Response::error('NOT_FOUND', 'API 端点不存在', 404);
                return;
            }
            
            $this->logger->debug('Route matched', ['target' => $route['target']]);
            
            // 转发到 Rust API
            $result = $this->proxy->forward($request, $route);
            
            $this->logger->info('Request completed', [
                'status' => $result['status'],
                'target' => $route['target'],
            ]);
            
            // 返回响应
            Response::send($result['body'], $result['status'], $result['headers']);
            
        } catch (\Exception $e) {
            $this->logger->error('Request failed', [
                'error' => $e->getMessage(),
                'trace' => $e->getTraceAsString(),
            ]);
            Response::error('INTERNAL_ERROR', $e->getMessage(), 500);
        }
    }
    
    /**
     * 解析请求
     */
    private function parseRequest()
    {
        $uri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
        $method = $_SERVER['REQUEST_METHOD'];
        
        // 获取请求体
        $body = null;
        if (in_array($method, ['POST', 'PUT', 'PATCH'])) {
            $body = file_get_contents('php://input');
        }
        
        // 获取请求头
        $headers = [];
        foreach ($_SERVER as $key => $value) {
            if (strpos($key, 'HTTP_') === 0) {
                $header = str_replace('_', '-', substr($key, 5));
                $headers[$header] = $value;
            }
        }
        
        return [
            'uri' => $uri,
            'method' => $method,
            'body' => $body,
            'headers' => $headers,
            'query' => $_GET,
        ];
    }
    
    /**
     * 设置 CORS 头
     */
    private function setCorsHeaders()
    {
        header('Access-Control-Allow-Origin: *');
        header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
        header('Access-Control-Allow-Headers: Content-Type, Authorization, X-Request-ID');
        header('Access-Control-Max-Age: 86400');
    }
}

// 启动兼容层
$compat = new CompatLayer();
$compat->handle();
