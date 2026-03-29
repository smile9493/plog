<?php

declare(strict_types=1);

namespace Plog\AdminApi\Controller;

use Plog\AdminApi\Application;
use Plog\DB\Connection\ConnectionInterface;

/**
 * 认证控制器
 */
class AuthController
{
    /**
     * 应用实例
     *
     * @var Application
     */
    private Application $app;

    /**
     * 数据库连接
     *
     * @var ConnectionInterface
     */
    private ConnectionInterface $db;

    /**
     * 构造函数
     *
     * @param Application $app 应用实例
     */
    public function __construct(Application $app)
    {
        $this->app = $app;
        $this->db = $app->getDb();
    }

    /**
     * 用户登录
     *
     * @return array
     */
    public function login(): array
    {
        $data = json_decode(file_get_contents('php://input'), true);
        
        $username = $data['username'] ?? '';
        $password = $data['password'] ?? '';

        if (empty($username) || empty($password)) {
            http_response_code(400);
            return [
                'code' => 400,
                'message' => '用户名和密码不能为空',
            ];
        }

        // 从数据库查询用户
        $sql = "SELECT * FROM user WHERE username = ?";
        $stmt = $this->db->query($sql, [$username]);
        $user = $stmt->fetch();

        if (!$user) {
            http_response_code(401);
            return [
                'code' => 401,
                'message' => '用户名或密码错误',
            ];
        }

        // 验证密码
        if (!password_verify($password, $user['password'])) {
            http_response_code(401);
            return [
                'code' => 401,
                'message' => '用户名或密码错误',
            ];
        }

        // 生成简单的token (实际应用中应该使用JWT)
        $token = base64_encode(json_encode([
            'user_id' => $user['uid'],
            'username' => $user['username'],
            'exp' => time() + 3600 * 24 // 24小时过期
        ]));

        return [
            'code' => 200,
            'message' => '登录成功',
            'data' => [
                'token' => $token,
                'user' => [
                    'id' => (int)$user['uid'],
                    'username' => $user['username'],
                    'nickname' => $user['nickname'] ?? $user['username'],
                    'role' => $user['role'],
                    'avatar' => $user['avatar'] ?? '',
                ]
            ]
        ];
    }

    /**
     * 用户登出
     *
     * @return array
     */
    public function logout(): array
    {
        return [
            'code' => 200,
            'message' => '登出成功',
        ];
    }

    /**
     * 获取当前用户信息
     *
     * @return array
     */
    public function user(): array
    {
        // 从Authorization头获取token
        $headers = getallheaders();
        $authHeader = $headers['Authorization'] ?? '';
        
        if (empty($authHeader) || !preg_match('/Bearer\s+(.*)$/i', $authHeader, $matches)) {
            http_response_code(401);
            return [
                'code' => 401,
                'message' => '未授权',
            ];
        }

        $token = $matches[1];
        $tokenData = json_decode(base64_decode($token), true);

        if (!$tokenData || $tokenData['exp'] < time()) {
            http_response_code(401);
            return [
                'code' => 401,
                'message' => 'Token已过期',
            ];
        }

        // 从数据库获取用户信息
        $sql = "SELECT * FROM user WHERE uid = ?";
        $stmt = $this->db->query($sql, [$tokenData['user_id']]);
        $user = $stmt->fetch();

        if (!$user) {
            http_response_code(404);
            return [
                'code' => 404,
                'message' => '用户不存在',
            ];
        }

        return [
            'code' => 200,
            'message' => '获取成功',
            'data' => [
                'id' => (int)$user['uid'],
                'username' => $user['username'],
                'nickname' => $user['nickname'] ?? $user['username'],
                'role' => $user['role'],
                'avatar' => $user['avatar'] ?? '',
            ]
        ];
    }
}
