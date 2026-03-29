<?php
/**
 * API 路由处理
 * 处理所有 /api/* 请求
 */

// 设置 CORS 头
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS');
header('Access-Control-Allow-Headers: Content-Type, Authorization');
header('Access-Control-Max-Age: 86400');

// 处理预检请求
if ($_SERVER['REQUEST_METHOD'] === 'OPTIONS') {
    http_response_code(200);
    exit();
}

// 加载 Plog 核心文件
require_once __DIR__ . '/include/lib/common.php';
require_once __DIR__ . '/include/lib/databasepdo.php';
require_once __DIR__ . '/include/lib/mysql.php';
require_once __DIR__ . '/include/model/user_model.php';
require_once __DIR__ . '/include/controller/auth_api_controller.php';

// 获取请求路径
$request_uri = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

// 移除 /api 或 /api.php 前缀
$api_path = str_replace(['/api/', '/api.php'], '', $request_uri);
$api_path = trim($api_path, '/');

// 如果没有路径参数，检查 query string
if (empty($api_path) && isset($_GET['route'])) {
    $api_path = $_GET['route'];
}

// 路由映射
$routes = [
    'auth/login' => ['method' => 'POST', 'controller' => 'Auth_Api_Controller', 'action' => 'login'],
    'auth/logout' => ['method' => 'POST', 'controller' => 'Auth_Api_Controller', 'action' => 'logout'],
    'auth/user' => ['method' => 'GET', 'controller' => 'Auth_Api_Controller', 'action' => 'getCurrentUser'],
];

// 查找匹配的路由
if (isset($routes[$api_path])) {
    $route = $routes[$api_path];
    
    // 检查请求方法
    if ($_SERVER['REQUEST_METHOD'] !== $route['method']) {
        http_response_code(405);
        echo json_encode([
            'success' => false,
            'error' => [
                'code' => 'METHOD_NOT_ALLOWED',
                'message' => '请求方法不允许'
            ]
        ]);
        exit();
    }
    
    // 加载控制器
    $controller_class = $route['controller'];
    if (class_exists($controller_class)) {
        call_user_func([$controller_class, $route['action']]);
    } else {
        http_response_code(500);
        echo json_encode([
            'success' => false,
            'error' => [
                'code' => 'CONTROLLER_NOT_FOUND',
                'message' => '控制器不存在'
            ]
        ]);
    }
} else {
    http_response_code(404);
    echo json_encode([
        'success' => false,
        'error' => [
            'code' => 'NOT_FOUND',
            'message' => 'API 端点不存在'
        ]
    ]);
}
