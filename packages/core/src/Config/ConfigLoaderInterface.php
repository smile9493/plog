<?php

declare(strict_types=1);

namespace Plog\Core\Config;

/**
 * 配置加载器接口
 */
interface ConfigLoaderInterface
{
    /**
     * 加载配置
     *
     * @param string $source 配置源
     * @return array
     */
    public function load(string $source): array;

    /**
     * 检查是否支持该配置源
     *
     * @param string $source 配置源
     * @return bool
     */
    public function supports(string $source): bool;
}
