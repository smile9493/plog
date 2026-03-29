<?php

declare(strict_types=1);

namespace Plog\Core\Helper;

/**
 * 数组辅助类
 */
class Arr
{
    /**
     * 获取数组项
     *
     * @param array $array 数组
     * @param string $key 键,支持点号分隔
     * @param mixed $default 默认值
     * @return mixed
     */
    public static function get(array $array, string $key, $default = null)
    {
        $keys = explode('.', $key);
        $value = $array;

        foreach ($keys as $k) {
            if (!is_array($value) || !array_key_exists($k, $value)) {
                return $default;
            }
            $value = $value[$k];
        }

        return $value;
    }

    /**
     * 设置数组项
     *
     * @param array $array 数组
     * @param string $key 键
     * @param mixed $value 值
     * @return array
     */
    public static function set(array $array, string $key, $value): array
    {
        $keys = explode('.', $key);
        $current = &$array;

        foreach ($keys as $k) {
            if (!isset($current[$k]) || !is_array($current[$k])) {
                $current[$k] = [];
            }
            $current = &$current[$k];
        }

        $current = $value;

        return $array;
    }

    /**
     * 检查数组项是否存在
     *
     * @param array $array 数组
     * @param string $key 键
     * @return bool
     */
    public static function has(array $array, string $key): bool
    {
        return self::get($array, $key) !== null;
    }
}

/**
 * 字符串辅助类
 */
class Str
{
    /**
     * 生成随机字符串
     *
     * @param int $length 长度
     * @return string
     */
    public static function random(int $length = 16): string
    {
        $characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
        $charactersLength = strlen($characters);
        $randomString = '';

        for ($i = 0; $i < $length; $i++) {
            $randomString .= $characters[random_int(0, $charactersLength - 1)];
        }

        return $randomString;
    }

    /**
     * 转换为蛇形命名
     *
     * @param string $value 字符串
     * @return string
     */
    public static function snake(string $value): string
    {
        return strtolower(preg_replace('/([a-z])([A-Z])/', '$1_$2', $value));
    }

    /**
     * 转换为驼峰命名
     *
     * @param string $value 字符串
     * @return string
     */
    public static function camel(string $value): string
    {
        return lcfirst(str_replace(' ', '', ucwords(str_replace(['-', '_'], ' ', $value))));
    }
}
