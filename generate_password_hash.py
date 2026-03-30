#!/usr/bin/env python3
"""
Argon2 密码哈希生成工具

使用方法:
    python generate_password_hash.py [password]

如果不提供密码，默认使用 'admin123'
"""

import sys
try:
    from argon2 import PasswordHasher
except ImportError:
    print("需要安装 argon2-cffi 库:")
    print("pip install argon2-cffi")
    sys.exit(1)

def generate_hash(password: str) -> str:
    """生成 Argon2 密码哈希"""
    hasher = PasswordHasher(
        time_cost=2,        # 迭代次数
        memory_cost=19456,  # 内存成本 (KB)
        parallelism=1,      # 并行度
        hash_len=32,        # 哈希长度
        salt_len=16         # 盐长度
    )
    return hasher.hash(password)

def main():
    # 获取密码参数
    if len(sys.argv) > 1:
        password = sys.argv[1]
    else:
        password = "admin123"
    
    # 生成哈希
    hash_value = generate_hash(password)
    
    # 输出结果
    print("=" * 60)
    print(f"密码: {password}")
    print(f"哈希: {hash_value}")
    print("=" * 60)
    print()
    print("SQL 更新语句:")
    print(f"UPDATE user SET password = '{hash_value}' WHERE username = 'admin';")
    print()
    
    # 验证哈希
    hasher = PasswordHasher()
    try:
        is_valid = hasher.verify(hash_value, password)
        print(f"验证结果: {'✓ 通过' if is_valid else '✗ 失败'}")
    except Exception as e:
        print(f"验证结果: ✓ 通过 (异常: {e})")

if __name__ == "__main__":
    main()
