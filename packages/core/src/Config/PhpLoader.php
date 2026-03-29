<?php

declare(strict_types=1);

namespace Plog\Core\Config;

/**
 * PHP 配置文件加载器
 */
class PhpLoader implements ConfigLoaderInterface
{
    /**
     * 加载 PHP 配置文件
     *
     * @param string $source PHP 配置文件路径
     * @return array
     */
    public function load(string $source): array
    {
        if (!file_exists($source)) {
            return [];
        }

        $items = require $source;

        return is_array($items) ? $items : [];
    }

    /**
     * 检查是否支持该配置源
     *
     * @param string $source 配置源
     * @return bool
     */
    public function supports(string $source): bool
    {
        return pathinfo($source, PATHINFO_EXTENSION) === 'php';
    }
}
