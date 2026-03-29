<?php

declare(strict_types=1);

namespace Plog\Tests\Unit;

use PHPUnit\Framework\TestCase;
use Plog\Core\Config\ConfigManager;
use Plog\Core\Config\EnvLoader;
use Plog\Core\Config\PhpLoader;

/**
 * ConfigManager 测试
 */
class ConfigManagerTest extends TestCase
{
    /**
     * 测试基本配置操作
     */
    public function testBasicConfigOperations(): void
    {
        $config = new ConfigManager();

        // 测试设置和获取
        $config->set('app.name', 'Plog');
        $this->assertEquals('Plog', $config->get('app.name'));

        // 测试嵌套配置
        $config->set('database.mysql.host', 'localhost');
        $this->assertEquals('localhost', $config->get('database.mysql.host'));

        // 测试默认值
        $this->assertEquals('default', $config->get('nonexistent', 'default'));

        // 测试配置是否存在
        $this->assertTrue($config->has('app.name'));
        $this->assertFalse($config->has('nonexistent'));
    }

    /**
     * 测试获取所有配置
     */
    public function testGetAllConfig(): void
    {
        $config = new ConfigManager();

        $config->set('app.name', 'Plog');
        $config->set('app.version', '1.0.0');

        $all = $config->all();

        $this->assertIsArray($all);
        $this->assertArrayHasKey('app', $all);
        $this->assertEquals('Plog', $all['app']['name']);
        $this->assertEquals('1.0.0', $all['app']['version']);
    }
}
