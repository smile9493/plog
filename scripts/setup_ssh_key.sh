#!/bin/bash
# SSH公钥配置脚本
# 在Linux服务器上执行此脚本

# 创建.ssh目录
mkdir -p ~/.ssh
chmod 700 ~/.ssh

# 添加公钥到authorized_keys
PUBKEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1 your_email@example.com"

# 检查是否已存在该公钥
if grep -q "$PUBKEY" ~/.ssh/authorized_keys 2>/dev/null; then
    echo "公钥已存在,无需添加"
else
    echo "$PUBKEY" >> ~/.ssh/authorized_keys
    echo "公钥已添加到authorized_keys"
fi

# 设置权限
chmod 600 ~/.ssh/authorized_keys

# 显示配置结果
echo "=== SSH密钥配置完成 ==="
echo "authorized_keys内容:"
cat ~/.ssh/authorized_keys
echo ""
echo "权限设置:"
ls -la ~/.ssh/
