# SSH密钥配置指南

## 当前状态
- Windows公钥已存在: `C:\Users\你的用户名\.ssh\id_ed25519.pub`
- Linux服务器: 192.168.31.14
- 用户: root
- 密码: qqwwee

## 配置步骤

### 方法1: 手动配置(推荐)

1. **在Windows上查看公钥**
   ```powershell
   cat ~/.ssh/id_ed25519.pub
   ```
   公钥内容:
   ```
   ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1 your_email@example.com
   ```

2. **SSH连接到Linux服务器**
   ```powershell
   ssh root@192.168.31.14
   ```
   密码: qqwwee

3. **在Linux上配置公钥**
   ```bash
   # 创建.ssh目录
   mkdir -p ~/.ssh
   chmod 700 ~/.ssh

   # 添加公钥到authorized_keys
   echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILgwnqdAksrFJ57OLMPPIU4ty7dpYgyyzpD7EXeLUYQ1 your_email@example.com" >> ~/.ssh/authorized_keys

   # 设置权限
   chmod 600 ~/.ssh/authorized_keys

   # 验证配置
   cat ~/.ssh/authorized_keys
   ls -la ~/.ssh/
   ```

4. **退出Linux,在Windows上测试密钥连接**
   ```powershell
   ssh root@192.168.31.14 "echo '密钥认证成功'"
   ```

### 方法2: 使用ssh-copy-id(如果可用)

Windows上可能没有ssh-copy-id命令,可以使用以下PowerShell脚本替代:

```powershell
# 获取公钥
$pubkey = cat ~/.ssh/id_ed25519.pub

# 使用密码认证连接并添加公钥
# 注意: 这需要手动输入密码
ssh root@192.168.31.14 "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '$pubkey' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys"
```

## 配置完成后

配置完成后,可以使用以下命令同步主题文件:

```powershell
# 同步主题目录到Linux
scp -r f:\mytheme\content\templates\mytheme root@192.168.31.14:/tmp/

# 或者直接同步到plog容器挂载目录
ssh root@192.168.31.14 "docker ps | grep plog"
```
