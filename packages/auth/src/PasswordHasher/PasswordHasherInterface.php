<?php

declare(strict_types=1);

namespace Plog\Auth\PasswordHasher;

/**
 * 密码哈希器接口
 */
interface PasswordHasherInterface
{
    /**
     * 哈希密码
     *
     * @param string $password 原始密码
     * @return string 哈希后的密码
     */
    public function hash(string $password): string;

    /**
     * 验证密码
     *
     * @param string $password 原始密码
     * @param string $hash 哈希后的密码
     * @return bool
     */
    public function verify(string $password, string $hash): bool;

    /**
     * 检查是否需要重新哈希
     *
     * @param string $hash 哈希后的密码
     * @return bool
     */
    public function needsRehash(string $hash): bool;
}
