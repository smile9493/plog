<?php

declare(strict_types=1);

use Plog\AdminApi\Controller\PostController;
use Plog\AdminApi\Controller\CategoryController;
use Plog\AdminApi\Controller\AuthController;

return [
    // 认证路由
    [
        'method' => 'GET',
        'path' => '/auth',
        'handler' => function() {
            return [
                'endpoints' => [
                    'login' => '/auth/login',
                    'logout' => '/auth/logout',
                    'user' => '/auth/user',
                ],
                'methods' => [
                    'login' => 'POST',
                    'logout' => 'POST',
                    'user' => 'GET',
                ],
            ];
        },
    ],
    [
        'method' => 'POST',
        'path' => '/auth/login',
        'handler' => [AuthController::class, 'login'],
    ],
    [
        'method' => 'POST',
        'path' => '/auth/logout',
        'handler' => [AuthController::class, 'logout'],
    ],
    [
        'method' => 'GET',
        'path' => '/auth/user',
        'handler' => [AuthController::class, 'user'],
    ],

    // 文章路由
    [
        'method' => 'GET',
        'path' => '/posts',
        'handler' => [PostController::class, 'index'],
    ],
    [
        'method' => 'GET',
        'path' => '/posts/{id}',
        'handler' => [PostController::class, 'show'],
    ],
    [
        'method' => 'POST',
        'path' => '/posts',
        'handler' => [PostController::class, 'store'],
    ],
    [
        'method' => 'PUT',
        'path' => '/posts/{id}',
        'handler' => [PostController::class, 'update'],
    ],
    [
        'method' => 'DELETE',
        'path' => '/posts/{id}',
        'handler' => [PostController::class, 'destroy'],
    ],

    // 分类路由
    [
        'method' => 'GET',
        'path' => '/categories',
        'handler' => [CategoryController::class, 'index'],
    ],
    [
        'method' => 'GET',
        'path' => '/categories/{id}',
        'handler' => [CategoryController::class, 'show'],
    ],
    [
        'method' => 'POST',
        'path' => '/categories',
        'handler' => [CategoryController::class, 'store'],
    ],
    [
        'method' => 'PUT',
        'path' => '/categories/{id}',
        'handler' => [CategoryController::class, 'update'],
    ],
    [
        'method' => 'DELETE',
        'path' => '/categories/{id}',
        'handler' => [CategoryController::class, 'destroy'],
    ],
];
