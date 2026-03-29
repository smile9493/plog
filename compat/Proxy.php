<?php
/**
 * 代理类 - 转发请求到 Rust API
 */

class Proxy
{
    private $config;
    private $baseUrl;
    
    public function __construct(array $config)
    {
        $this->config = $config;
        $this->baseUrl = sprintf(
            'http://%s:%d',
            $config['rust_api']['host'],
            $config['rust_api']['port']
        );
    }
    
    /**
     * 转发请求到 Rust API
     */
    public function forward(array $request, array $route)
    {
        $target = $route['target'];
        $params = $route['params'];
        
        // 替换 URL 参数
        $url = $this->buildUrl($target, $params);
        
        // 构建请求头
        $headers = $this->buildHeaders($request['headers']);
        
        // 执行请求
        return $this->executeRequest(
            $request['method'],
            $url,
            $request['body'],
            $headers
        );
    }
    
    /**
     * 构建 URL
     */
    private function buildUrl($target, $params)
    {
        $url = $target;
        
        // 替换参数
        foreach ($params as $key => $value) {
            $url = str_replace(':' . $key, $value, $url);
        }
        
        return $this->baseUrl . $url;
    }
    
    /**
     * 构建请求头
     */
    private function buildHeaders($headers)
    {
        $result = [];
        
        // 保留授权头
        if (isset($headers['AUTHORIZATION'])) {
            $result[] = 'Authorization: ' . $headers['AUTHORIZATION'];
        }
        
        // 保留内容类型
        if (isset($headers['CONTENT_TYPE'])) {
            $result[] = 'Content-Type: ' . $headers['CONTENT_TYPE'];
        }
        
        // 添加请求 ID
        $requestId = $headers['X_REQUEST_ID'] ?? $this->generateRequestId();
        $result[] = 'X-Request-ID: ' . $requestId;
        
        return $result;
    }
    
    /**
     * 执行 HTTP 请求
     */
    private function executeRequest($method, $url, $body = null, $headers = [])
    {
        $ch = curl_init();
        
        // 基础配置
        curl_setopt_array($ch, [
            CURLOPT_URL => $url,
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_TIMEOUT => $this->config['rust_api']['timeout'],
            CURLOPT_CUSTOMREQUEST => $method,
            CURLOPT_HTTPHEADER => $headers,
        ]);
        
        // 设置请求体
        if ($body !== null) {
            curl_setopt($ch, CURLOPT_POSTFIELDS, $body);
        }
        
        // 执行请求
        $response = curl_exec($ch);
        $status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        $responseHeaders = curl_getinfo($ch);
        
        // 错误处理
        if (curl_errno($ch)) {
            $error = curl_error($ch);
            curl_close($ch);
            throw new \Exception('Proxy request failed: ' . $error);
        }
        
        curl_close($ch);
        
        return [
            'status' => $status,
            'body' => $response,
            'headers' => $responseHeaders,
        ];
    }
    
    /**
     * 生成请求 ID
     */
    private function generateRequestId()
    {
        return sprintf('%04x%04x-%04x-%04x-%04x-%04x%04x%04x',
            mt_rand(0, 0xffff), mt_rand(0, 0xffff),
            mt_rand(0, 0xffff),
            mt_rand(0, 0x0fff) | 0x4000,
            mt_rand(0, 0x3fff) | 0x8000,
            mt_rand(0, 0xffff), mt_rand(0, 0xffff), mt_rand(0, 0xffff)
        );
    }
}
