<?php

declare(strict_types=1);

namespace Plog\Core\Config;

use RuntimeException;

/**
 * 配置管理器
 */
class ConfigManager implements ConfigManagerInterface
{
    /**
     * 配置项
     *
     * @var array
     */
    private array $items = [];

    /**
     * 配置加载器
     *
     * @var ConfigLoaderInterface[]
     */
    private array $loaders = [];

    /**
     * 构造函数
     *
     * @param ConfigLoaderInterface[] $loaders 配置加载器数组
     */
    public function __construct(array $loaders = [])
    {
        foreach ($loaders as $loader) {
            $this->addLoader($loader);
        }
    }

    /**
     * 获取配置值
     *
     * @param string $key 配置键,支持点号分隔
     * @param mixed $default 默认值
     * @return mixed
     */
    public function get(string $key, $default = null)
    {
        $keys = explode('.', $key);
        $value = $this->items;

        foreach ($keys as $k) {
            if (!is_array($value) || !array_key_exists($k, $value)) {
                return $default;
            }
            $value = $value[$k];
        }

        return $value;
    }

    /**
     * 设置配置值
     *
     * @param string $key 配置键
     * @param mixed $value 配置值
     */
    public function set(string $key, $value): void
    {
        $keys = explode('.', $key);
        $items = &$this->items;

        foreach ($keys as $k) {
            if (!isset($items[$k]) || !is_array($items[$k])) {
                $items[$k] = [];
            }
            $items = &$items[$k];
        }

        $items = $value;
    }

    /**
     * 检查配置是否存在
     *
     * @param string $key 配置键
     * @return bool
     */
    public function has(string $key): bool
    {
        return $this->get($key) !== null;
    }

    /**
     * 获取所有配置
     *
     * @return array
     */
    public function all(): array
    {
        return $this->items;
    }

    /**
     * 加载配置源
     *
     * @param string $source 配置源
     * @throws RuntimeException 当没有加载器支持该配置源时
     */
    public function load(string $source): void
    {
        foreach ($this->loaders as $loader) {
            if ($loader->supports($source)) {
                $items = $loader->load($source);
                $this->items = array_merge($this->items, $items);
                return;
            }
        }

        throw new RuntimeException("No loader supports source: {$source}");
    }

    /**
     * 添加配置加载器
     *
     * @param ConfigLoaderInterface $loader 配置加载器
     */
    private function addLoader(ConfigLoaderInterface $loader): void
    {
        $this->loaders[] = $loader;
    }
}
