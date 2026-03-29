<?php

declare(strict_types=1);

namespace Plog\Tests\Api;

/**
 * 认证 API 测试
 */
class AuthApiTest extends ApiTestCase
{
    private string $testUsername = 'admin';
    private string $testPassword = 'password123';

    /**
     * 测试登录成功
     */
    public function testLoginSuccess(): void
    {
        $response = $this->post('/auth/login', [
            'username' => $this->testUsername,
            'password' => $this->testPassword,
        ]);

        $this->assertSuccess($response);
        $this->assertArrayHasKey('token', $response['body']['data'] ?? []);
        $this->assertArrayHasKey('user', $response['body']['data'] ?? []);
    }

    /**
     * 测试登录失败 - 错误密码
     */
    public function testLoginWrongPassword(): void
    {
        $response = $this->post('/auth/login', [
            'username' => $this->testUsername,
            'password' => 'wrong_password',
        ]);

        $this->assertError($response, 401, 'INVALID_CREDENTIALS');
    }

    /**
     * 测试登录失败 - 缺少参数
     */
    public function testLoginMissingParams(): void
    {
        $response = $this->post('/auth/login', [
            'username' => $this->testUsername,
        ]);

        $this->assertError($response, 400, 'VALIDATION_ERROR');
    }

    /**
     * 测试登出
     */
    public function testLogout(): void
    {
        // 先登录获取 token
        $loginResponse = $this->post('/auth/login', [
            'username' => $this->testUsername,
            'password' => $this->testPassword,
        ]);

        $token = $loginResponse['body']['data']['token'] ?? '';
        $this->setToken($token);

        $response = $this->post('/auth/logout');
        $this->assertSuccess($response);
    }

    /**
     * 测试获取当前用户
     */
    public function testGetCurrentUser(): void
    {
        // 先登录获取 token
        $loginResponse = $this->post('/auth/login', [
            'username' => $this->testUsername,
            'password' => $this->testPassword,
        ]);

        $token = $loginResponse['body']['data']['token'] ?? '';
        $this->setToken($token);

        $response = $this->get('/auth/user');
        $this->assertSuccess($response);
        $this->assertArrayHasKey('uid', $response['body']['data'] ?? []);
        $this->assertArrayHasKey('username', $response['body']['data'] ?? []);
    }

    /**
     * 测试获取当前用户 - 未授权
     */
    public function testGetCurrentUserUnauthorized(): void
    {
        $response = $this->get('/auth/user');
        $this->assertError($response, 401, 'AUTH_REQUIRED');
    }
}
