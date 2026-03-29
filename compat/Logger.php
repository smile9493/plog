<?php
/**
 * 日志类
 */

class Logger
{
    private static $instance;
    private $config;
    
    private function __construct(array $config)
    {
        $this->config = $config;
    }
    
    /**
     * 获取单例实例
     */
    public static function getInstance(array $config = [])
    {
        if (self::$instance === null) {
            self::$instance = new self($config);
        }
        return self::$instance;
    }
    
    /**
     * 记录信息
     */
    public function info($message, $context = [])
    {
        $this->log('INFO', $message, $context);
    }
    
    /**
     * 记录警告
     */
    public function warning($message, $context = [])
    {
        $this->log('WARNING', $message, $context);
    }
    
    /**
     * 记录错误
     */
    public function error($message, $context = [])
    {
        $this->log('ERROR', $message, $context);
    }
    
    /**
     * 记录调试信息
     */
    public function debug($message, $context = [])
    {
        $this->log('DEBUG', $message, $context);
    }
    
    /**
     * 写入日志
     */
    private function log($level, $message, $context = [])
    {
        if (!$this->config['logging']['enabled'] ?? true) {
            return;
        }
        
        $logFile = $this->config['logging']['file'] ?? __DIR__ . '/logs/compat.log';
        
        $timestamp = date('Y-m-d H:i:s');
        $contextStr = !empty($context) ? json_encode($context, JSON_UNESCAPED_UNICODE) : '';
        
        $logEntry = sprintf(
            "[%s] [%s] %s %s\n",
            $timestamp,
            $level,
            $message,
            $contextStr
        );
        
        // 确保目录存在
        $logDir = dirname($logFile);
        if (!is_dir($logDir)) {
            mkdir($logDir, 0755, true);
        }
        
        file_put_contents($logFile, $logEntry, FILE_APPEND | LOCK_EX);
    }
}
