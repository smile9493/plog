<?php

declare(strict_types=1);

namespace Plog\Auth\PasswordHasher;

/**
 * Bcrypt 密码哈希器
 */
class BcryptPasswordHasher implements PasswordHasherInterface
{
    /**
     * 哈希算法
     *
     * @var int
     */
    private int $algorithm = PASSWORD_BCRYPT;

    /**
     * 哈希选项
     *
     * @var array
     */
    private array $options;

    /**
     * 构造函数
     *
     * @param array $options 哈希选项
     */
    public function __construct(array $options = [])
    {
        $this->options = array_merge([
            'cost' => 12,
        ], $options);
    }

    /**
     * 哈希密码
     *
     * @param string $password 原始密码
     * @return string 哈希后的密码
     */
    public function hash(string $password): string
    {
        return password_hash($password, $this->algorithm, $this->options);
    }

    /**
     * 验证密码
     *
     * @param string $password 原始密码
     * @param string $hash 哈希后的密码
     * @return bool
     */
    public function verify(string $password, string $hash): bool
    {
        return password_verify($password, $hash);
    }

    /**
     * 检查是否需要重新哈希
     *
     * @param string $hash 哈希后的密码
     * @return bool
     */
    public function needsRehash(string $hash): bool
    {
        return password_needs_rehash($hash, $this->algorithm, $this->options);
    }
}
