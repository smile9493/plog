<?php

declare(strict_types=1);

namespace Plog\Tests\Api;

use PHPUnit\Framework\TestCase;

/**
 * API 测试基类
 */
abstract class ApiTestCase extends TestCase
{
    protected string $baseUrl;
    protected string $apiVersion = 'v1';
    protected ?string $token = null;

    protected function setUp(): void
    {
        parent::setUp();
        $this->baseUrl = $_ENV['API_BASE_URL'] ?? 'http://localhost:8080/api';
    }

    /**
     * 发送 GET 请求
     */
    protected function get(string $endpoint, array $params = []): array
    {
        return $this->request('GET', $endpoint, $params);
    }

    /**
     * 发送 POST 请求
     */
    protected function post(string $endpoint, array $data = []): array
    {
        return $this->request('POST', $endpoint, [], $data);
    }

    /**
     * 发送 PUT 请求
     */
    protected function put(string $endpoint, array $data = []): array
    {
        return $this->request('PUT', $endpoint, [], $data);
    }

    /**
     * 发送 DELETE 请求
     */
    protected function delete(string $endpoint): array
    {
        return $this->request('DELETE', $endpoint);
    }

    /**
     * 发送 HTTP 请求
     */
    protected function request(
        string $method,
        string $endpoint,
        array $params = [],
        ?array $data = null
    ): array {
        $url = $this->baseUrl . '/' . $this->apiVersion . $endpoint;

        if (!empty($params)) {
            $url .= '?' . http_build_query($params);
        }

        $ch = curl_init();

        $headers = [
            'Content-Type: application/json',
            'Accept: application/json',
        ];

        if ($this->token) {
            $headers[] = 'Authorization: Bearer ' . $this->token;
        }

        curl_setopt($ch, CURLOPT_URL, $url);
        curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
        curl_setopt($ch, CURLOPT_HTTPHEADER, $headers);

        switch ($method) {
            case 'POST':
                curl_setopt($ch, CURLOPT_POST, true);
                if ($data) {
                    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode($data));
                }
                break;
            case 'PUT':
                curl_setopt($ch, CURLOPT_CUSTOMREQUEST, 'PUT');
                if ($data) {
                    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode($data));
                }
                break;
            case 'DELETE':
                curl_setopt($ch, CURLOPT_CUSTOMREQUEST, 'DELETE');
                break;
        }

        $response = curl_exec($ch);
        $httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        return [
            'status' => $httpCode,
            'body' => json_decode($response, true) ?? [],
        ];
    }

    /**
     * 设置认证 Token
     */
    protected function setToken(string $token): void
    {
        $this->token = $token;
    }

    /**
     * 断言响应成功
     */
    protected function assertSuccess(array $response): void
    {
        $this->assertEquals(200, $response['status']);
        $this->assertTrue($response['body']['success'] ?? false);
    }

    /**
     * 断言响应失败
     */
    protected function assertError(array $response, int $statusCode, string $errorCode = null): void
    {
        $this->assertEquals($statusCode, $response['status']);
        $this->assertFalse($response['body']['success'] ?? true);

        if ($errorCode) {
            $this->assertEquals($errorCode, $response['body']['error']['code'] ?? '');
        }
    }

    /**
     * 断言分页响应
     */
    protected function assertPaginated(array $response): void
    {
        $this->assertSuccess($response);
        $this->assertArrayHasKey('pagination', $response['body']['data'] ?? []);
    }
}
