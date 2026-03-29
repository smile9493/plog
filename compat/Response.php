<?php
/**
 * 响应类
 */

class Response
{
    /**
     * 发送成功响应
     */
    public static function send($data, $status = 200, $headers = [])
    {
        http_response_code($status);
        
        // 设置内容类型
        header('Content-Type: application/json; charset=utf-8');
        
        // 设置响应头
        foreach ($headers as $key => $value) {
            if (is_string($key)) {
                header("$key: $value");
            }
        }
        
        // 发送响应
        echo $data;
        exit();
    }
    
    /**
     * 发送错误响应
     */
    public static function error($code, $message, $status = 400)
    {
        http_response_code($status);
        header('Content-Type: application/json; charset=utf-8');
        
        echo json_encode([
            'success' => false,
            'error' => [
                'code' => $code,
                'message' => $message,
            ],
        ]);
        
        exit();
    }
    
    /**
     * 发送 JSON 响应
     */
    public static function json($data, $status = 200)
    {
        http_response_code($status);
        header('Content-Type: application/json; charset=utf-8');
        
        echo json_encode($data, JSON_UNESCAPED_UNICODE);
        exit();
    }
    
    /**
     * 转换 v2 响应为 v1 格式
     */
    public static function convertV2toV1($v2Response)
    {
        $data = json_decode($v2Response, true);
        
        if (!$data) {
            return $v2Response;
        }
        
        // 如果是成功响应
        if (isset($data['success']) && $data['success']) {
            return json_encode([
                'success' => true,
                'data' => $data['data'] ?? null,
            ]);
        }
        
        // 如果是错误响应
        if (isset($data['error'])) {
            return json_encode([
                'success' => false,
                'error' => $data['error']['message'] ?? 'Unknown error',
                'code' => $data['error']['code'] ?? 'ERROR',
            ]);
        }
        
        return $v2Response;
    }
}
