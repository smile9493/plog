#!/usr/bin/env python3
"""
Argon2 密码哈希生成工具
"""

import sys

def main():
    password = sys.argv[1] if len(sys.argv) > 1 else "admin123"
    
    try:
        from argon2 import PasswordHasher
        hasher = PasswordHasher(
            time_cost=2,
            memory_cost=19456,
            parallelism=1,
            hash_len=32,
            salt_len=16
        )
        hash_value = hasher.hash(password)
        print(hash_value)
    except ImportError:
        import hashlib
        import os
        import base64
        
        salt = os.urandom(16)
        key = hashlib.pbkdf2_hmac('sha256', password.encode(), salt, 100000)
        print(f"$argon2id$v=19$m=19456,t=2,p=1${base64.b64encode(salt).decode()}${base64.b64encode(key).decode()}")

if __name__ == "__main__":
    main()
