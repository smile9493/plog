<?php
/**
 * API 认证控制器
 * 处理用户登录、登出等认证相关操作
 */
class Auth_Api_Controller {
    
    /**
     * 处理登录请求
     * POST /api/auth/login
     */
    public static function login() {
        global $CACHE;
        
        header('Content-Type: application/json; charset=utf-8');
        
        // 只接受 POST 请求
        if ($_SERVER['REQUEST_METHOD'] !== 'POST') {
            http_response_code(405);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'METHOD_NOT_ALLOWED',
                    'message' => '请求方法不允许'
                ]
            ]);
            return;
        }
        
        // 获取 POST 数据
        $input = file_get_contents('php://input');
        $data = json_decode($input, true);
        
        // 验证输入
        if (empty($data['username']) || empty($data['password'])) {
            http_response_code(400);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'VALIDATION_ERROR',
                    'message' => '用户名和密码不能为空'
                ]
            ]);
            return;
        }
        
        $username = trim($data['username']);
        $password = $data['password'];
        
        // 连接数据库
        require_once __DIR__ . '/../../config.php';
        $conn = new mysqli(DB_HOST, DB_USER, DB_PASSWD, DB_NAME);
        if ($conn->connect_error) {
            http_response_code(500);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'DATABASE_ERROR',
                    'message' => '数据库连接失败'
                ]
            ]);
            return;
        }
        $conn->set_charset('utf8mb4');
        
        // 查询用户
        $stmt = $conn->prepare("SELECT * FROM " . DB_PREFIX . "user WHERE username = ?");
        $stmt->bind_param("s", $username);
        $stmt->execute();
        $result = $stmt->get_result();
        $user = $result->fetch_assoc();
        $stmt->close();
        
        if ($user) {
            // 验证密码
            $password_hash = md5($password);
            if ($user['password'] === $password_hash) {
                // 登录成功
                // 生成 session
                if (session_status() === PHP_SESSION_NONE) {
                    session_start();
                }
                $_SESSION['uid'] = $user['uid'];
                $_SESSION['role'] = $user['role'];
                $_SESSION['username'] = $user['username'];
                
                echo json_encode([
                    'success' => true,
                    'data' => [
                        'token' => session_id(),
                        'user' => [
                            'uid' => $user['uid'],
                            'username' => $user['username'],
                            'nickname' => $user['nickname'] ?: $user['username'],
                            'role' => $user['role'],
                            'email' => $user['email'] ?? ''
                        ]
                    ]
                ]);
            } else {
                // 密码错误
                http_response_code(401);
                echo json_encode([
                    'success' => false,
                    'error' => [
                        'code' => 'INVALID_CREDENTIALS',
                        'message' => '用户名或密码错误'
                    ]
                ]);
            }
        } else {
            // 用户不存在
            http_response_code(401);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'INVALID_CREDENTIALS',
                    'message' => '用户名或密码错误'
                ]
            ]);
        }
        
        $conn->close();
    }
    
    /**
     * 处理登出请求
     * POST /api/auth/logout
     */
    public static function logout() {
        header('Content-Type: application/json; charset=utf-8');
        
        // 销毁 session
        session_destroy();
        
        echo json_encode([
            'success' => true,
            'message' => '登出成功'
        ]);
    }
    
    /**
     * 获取当前用户信息
     * GET /api/auth/user
     */
    public static function getCurrentUser() {
        header('Content-Type: application/json; charset=utf-8');
        
        if (!isset($_SESSION['uid'])) {
            http_response_code(401);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'UNAUTHORIZED',
                    'message' => '未登录'
                ]
            ]);
            return;
        }
        
        $User_Model = new User_Model();
        $user = $User_Model->getOneUser($_SESSION['uid']);
        
        if ($user) {
            echo json_encode([
                'success' => true,
                'data' => [
                    'uid' => $user['uid'],
                    'username' => $user['username'],
                    'nickname' => $user['nickname'] ?: $user['username'],
                    'role' => $user['role'],
                    'email' => $user['email']
                ]
            ]);
        } else {
            http_response_code(404);
            echo json_encode([
                'success' => false,
                'error' => [
                    'code' => 'USER_NOT_FOUND',
                    'message' => '用户不存在'
                ]
            ]);
        }
    }
}
