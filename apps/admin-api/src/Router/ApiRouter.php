<?php

declare(strict_types=1);

namespace Plog\AdminApi\Router;

use Plog\AdminApi\Application;

/**
 * API 路由器
 */
class ApiRouter
{
    /**
     * 应用实例
     *
     * @var Application
     */
    private Application $app;

    /**
     * 路由规则
     *
     * @var array
     */
    private array $routes = [];

    /**
     * 构造函数
     *
     * @param Application $app 应用实例
     */
    public function __construct(Application $app)
    {
        $this->app = $app;
        $this->loadRoutes();
    }

    /**
     * 加载路由规则
     */
    private function loadRoutes(): void
    {
        $routesFile = __DIR__ . '/../../routes/api.php';
        if (file_exists($routesFile)) {
            $this->routes = require $routesFile;
        }
    }

    /**
     * 分发请求
     */
    public function dispatch(): void
    {
        $method = $_SERVER['REQUEST_METHOD'];
        $uri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

        // 移除前缀 /api
        $uri = preg_replace('/^\/api/', '', $uri);

        $route = $this->findRoute($method, $uri);

        if ($route === null) {
            $this->notFound();
            return;
        }

        $this->executeRoute($route);
    }

    /**
     * 查找路由
     *
     * @param string $method HTTP 方法
     * @param string $uri URI
     * @return array|null
     */
    private function findRoute(string $method, string $uri): ?array
    {
        foreach ($this->routes as $route) {
            if ($route['method'] !== $method) {
                continue;
            }

            $params = [];
            if ($this->matchUri($route['path'], $uri, $params)) {
                $route['params'] = $params;
                return $route;
            }
        }

        return null;
    }

    /**
     * 匹配 URI
     *
     * @param string $pattern 路由模式
     * @param string $uri URI
     * @param array $params 参数
     * @return bool
     */
    private function matchUri(string $pattern, string $uri, array &$params = []): bool
    {
        $params = [];

        // 将路由模式转换为正则表达式
        $regex = preg_replace('/\{([a-zA-Z_][a-zA-Z0-9_]*)\}/', '(?P<$1>[^/]+)', $pattern);
        $regex = '#^' . $regex . '$#';

        if (preg_match($regex, $uri, $matches)) {
            foreach ($matches as $key => $value) {
                if (is_string($key)) {
                    $params[$key] = $value;
                }
            }
            return true;
        }

        return false;
    }

    /**
     * 执行路由
     *
     * @param array $route 路由信息
     */
    private function executeRoute(array $route): void
    {
        $handler = $route['handler'];
        $params = $route['params'] ?? [];

        if (is_callable($handler)) {
            $result = call_user_func_array($handler, $params);
        } elseif (is_array($handler) && count($handler) === 2) {
            $controller = new $handler[0]($this->app);
            $method = $handler[1];
            $result = call_user_func_array([$controller, $method], $params);
        } else {
            $this->error('Invalid route handler', 500);
            return;
        }

        $this->response($result);
    }

    /**
     * 返回响应
     *
     * @param mixed $data 数据
     */
    private function response($data): void
    {
        header('Content-Type: application/json');
        
        // 如果控制器已经返回了标准格式,直接输出
        if (is_array($data) && isset($data['code'])) {
            echo json_encode($data, JSON_UNESCAPED_UNICODE);
        } else {
            // 否则包装成标准格式
            echo json_encode([
                'code' => 200,
                'message' => 'Success',
                'data' => $data,
            ], JSON_UNESCAPED_UNICODE);
        }
    }

    /**
     * 404 错误
     */
    private function notFound(): void
    {
        http_response_code(404);
        header('Content-Type: application/json');
        echo json_encode([
            'success' => false,
            'error' => [
                'code' => 404,
                'message' => 'Not Found',
            ],
        ], JSON_UNESCAPED_UNICODE);
    }

    /**
     * 错误响应
     *
     * @param string $message 错误消息
     * @param int $code 错误码
     */
    private function error(string $message, int $code = 500): void
    {
        http_response_code($code);
        header('Content-Type: application/json');
        echo json_encode([
            'success' => false,
            'error' => [
                'code' => $code,
                'message' => $message,
            ],
        ], JSON_UNESCAPED_UNICODE);
    }
}
