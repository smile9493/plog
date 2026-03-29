<?php

declare(strict_types=1);

namespace Plog\Core\Config;

/**
 * 环境变量配置加载器
 */
class EnvLoader implements ConfigLoaderInterface
{
    /**
     * 加载环境变量配置
     *
     * @param string $source .env 文件路径
     * @return array
     */
    public function load(string $source): array
    {
        if (!file_exists($source)) {
            return [];
        }

        $items = [];
        $lines = file($source, FILE_IGNORE_NEW_LINES | FILE_SKIP_EMPTY_LINES);

        foreach ($lines as $line) {
            // 跳过注释行
            if (strpos(trim($line), '#') === 0) {
                continue;
            }

            // 解析键值对
            if (strpos($line, '=') !== false) {
                list($key, $value) = explode('=', $line, 2);
                $key = trim($key);
                $value = trim($value);

                // 移除引号
                $value = $this->removeQuotes($value);

                // 设置环境变量
                putenv("{$key}={$value}");
                $_ENV[$key] = $value;

                $items[$key] = $value;
            }
        }

        return $items;
    }

    /**
     * 检查是否支持该配置源
     *
     * @param string $source 配置源
     * @return bool
     */
    public function supports(string $source): bool
    {
        return pathinfo($source, PATHINFO_EXTENSION) === 'env' ||
               substr(basename($source), 0, 4) === '.env';
    }

    /**
     * 移除值周围的引号
     *
     * @param string $value 值
     * @return string
     */
    private function removeQuotes(string $value): string
    {
        if (preg_match('/^"(.*)"$/', $value, $matches)) {
            return $matches[1];
        }

        if (preg_match("/^'(.*)'$/", $value, $matches)) {
            return $matches[1];
        }

        return $value;
    }
}
