<?php

declare(strict_types=1);

namespace Plog\AdminApi;

use Plog\Core\Config\ConfigManager;
use Plog\Core\Config\EnvLoader;
use Plog\Core\Config\PhpLoader;
use Plog\Core\Event\EventDispatcher;
use Plog\Db\Connection\Connection;
use Plog\AdminApi\Router\ApiRouter;

/**
 * Admin API 应用
 */
class Application
{
    /**
     * 配置管理器
     *
     * @var ConfigManager
     */
    private ConfigManager $config;

    /**
     * 数据库连接
     *
     * @var Connection
     */
    private Connection $db;

    /**
     * 事件调度器
     *
     * @var EventDispatcher
     */
    private EventDispatcher $events;

    /**
     * 路由器
     *
     * @var ApiRouter
     */
    private ApiRouter $router;

    /**
     * 构造函数
     */
    public function __construct()
    {
        $this->bootstrap();
    }

    /**
     * 启动应用
     */
    public function run(): void
    {
        try {
            $this->router->dispatch();
        } catch (\Exception $e) {
            $this->handleError($e);
        }
    }

    /**
     * 引导应用
     */
    private function bootstrap(): void
    {
        // 加载配置
        $this->loadConfig();

        // 初始化数据库连接
        $this->initDatabase();

        // 初始化事件调度器
        $this->events = new EventDispatcher();

        // 初始化路由器
        $this->router = new ApiRouter($this);
    }

    /**
     * 加载配置
     */
    private function loadConfig(): void
    {
        $this->config = new ConfigManager([
            new EnvLoader(),
            new PhpLoader(),
        ]);

        // 加载环境变量
        $envFile = dirname(__DIR__, 3) . '/.env';
        if (file_exists($envFile)) {
            $this->config->load($envFile);
        }

        // 加载应用配置
        $configDir = __DIR__ . '/../config';
        if (is_dir($configDir)) {
            foreach (glob($configDir . '/*.php') as $file) {
                $this->config->load($file);
            }
        }
    }

    /**
     * 初始化数据库连接
     */
    private function initDatabase(): void
    {
        $dbConfig = [
            'driver' => $this->config->get('DB_CONNECTION', 'mysql'),
            'host' => $this->config->get('DB_HOST', 'localhost'),
            'port' => $this->config->get('DB_PORT', 3306),
            'database' => $this->config->get('DB_NAME', 'plog'),
            'username' => $this->config->get('DB_USER', 'root'),
            'password' => $this->config->get('DB_PASSWD', ''),
            'charset' => 'utf8mb4',
        ];

        $this->db = new Connection($dbConfig);
    }

    /**
     * 处理错误
     *
     * @param \Exception $e 异常
     */
    private function handleError(\Exception $e): void
    {
        $debug = $this->config->get('APP_DEBUG', false);

        $response = [
            'success' => false,
            'error' => [
                'code' => $e->getCode() ?: 500,
                'message' => $e->getMessage(),
            ],
        ];

        if ($debug) {
            $response['error']['file'] = $e->getFile();
            $response['error']['line'] = $e->getLine();
            $response['error']['trace'] = $e->getTraceAsString();
        }

        http_response_code($e->getCode() ?: 500);
        header('Content-Type: application/json');
        echo json_encode($response, JSON_UNESCAPED_UNICODE);
    }

    /**
     * 获取配置管理器
     *
     * @return ConfigManager
     */
    public function getConfig(): ConfigManager
    {
        return $this->config;
    }

    /**
     * 获取数据库连接
     *
     * @return Connection
     */
    public function getDb(): Connection
    {
        return $this->db;
    }

    /**
     * 获取事件调度器
     *
     * @return EventDispatcher
     */
    public function getEvents(): EventDispatcher
    {
        return $this->events;
    }
}
