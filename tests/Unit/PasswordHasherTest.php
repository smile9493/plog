<?php

declare(strict_types=1);

namespace Plog\Tests\Unit;

use PHPUnit\Framework\TestCase;
use Plog\Auth\PasswordHasher\BcryptPasswordHasher;

/**
 * BcryptPasswordHasher 测试
 */
class PasswordHasherTest extends TestCase
{
    /**
     * 测试密码哈希和验证
     */
    public function testPasswordHashAndVerify(): void
    {
        $hasher = new BcryptPasswordHasher();
        $password = 'my-secret-password';

        $hash = $hasher->hash($password);

        $this->assertNotEquals($password, $hash);
        $this->assertTrue($hasher->verify($password, $hash));
        $this->assertFalse($hasher->verify('wrong-password', $hash));
    }

    /**
     * 测试不同密码生成不同哈希
     */
    public function testDifferentPasswordsGenerateDifferentHashes(): void
    {
        $hasher = new BcryptPasswordHasher();

        $hash1 = $hasher->hash('password1');
        $hash2 = $hasher->hash('password2');

        $this->assertNotEquals($hash1, $hash2);
    }

    /**
     * 测试需要重新哈希
     */
    public function testNeedsRehash(): void
    {
        $hasher = new BcryptPasswordHasher(['cost' => 10]);
        $password = 'my-password';

        $hash = $hasher->hash($password);

        // 使用相同配置,不需要重新哈希
        $this->assertFalse($hasher->needsRehash($hash));

        // 使用不同配置,需要重新哈希
        $newHasher = new BcryptPasswordHasher(['cost' => 12]);
        $this->assertTrue($newHasher->needsRehash($hash));
    }
}
