# Plog主题同步工具使用说明

## 功能特性

- ✅ SSH密钥认证连接
- ✅ SCP文件同步
- ✅ Docker容器管理
- ✅ 自动备份远程主题
- ✅ 文件权限自动修正
- ✅ 同步结果验证

## 安装要求

- Python 3.6+
- OpenSSH客户端(Windows 10/11自带)
- 已配置SSH密钥认证

## 配置文件

编辑 `sync_config.json` 配置服务器和主题信息:

```json
{
  "server": {
    "host": "192.168.31.14",
    "user": "root",
    "port": 22
  },
  "theme": {
    "name": "mytheme",
    "local_path": "f:/mytheme/content/templates",
    "remote_path": "/opt/1panel/docker/compose/plog/data/content/templates/mytheme"
  },
  "docker": {
    "container_name": "plog-pro",
    "web_port": 8080,
    "file_owner": "www-data:www-data"
  }
}
```

## 使用方法

### 1. 完整同步流程(推荐)
```bash
python sync_theme.py full
```
执行: 连接测试 → 容器检查 → 备份主题 → 同步文件 → 验证结果

### 2. 仅同步主题文件
```bash
python sync_theme.py sync
```

### 3. 验证主题文件
```bash
python sync_theme.py verify
```

### 4. 重启Docker容器
```bash
python sync_theme.py restart
```

### 5. 查看容器日志
```bash
python sync_theme.py logs
python sync_theme.py logs -n 100  # 查看最近100行
```

### 6. 备份远程主题
```bash
python sync_theme.py backup
```

## 命令参数

```
python sync_theme.py <action> [options]

操作类型:
  sync      同步主题文件
  verify    验证主题文件
  restart   重启Docker容器
  logs      查看容器日志
  backup    备份远程主题
  full      完整同步流程

可选参数:
  -c, --config   配置文件路径 (默认: sync_config.json)
  -n, --lines    查看日志的行数 (默认: 50)
```

## 使用示例

### 示例1: 首次同步
```bash
# 1. 配置SSH密钥(如果未配置)
ssh root@192.168.31.14
# 在Linux上执行:
mkdir -p ~/.ssh && chmod 700 ~/.ssh
echo "你的公钥内容" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
exit

# 2. 修改配置文件
# 编辑 sync_config.json

# 3. 执行完整同步
python sync_theme.py full
```

### 示例2: 日常更新主题
```bash
# 修改本地主题文件后,执行同步
python sync_theme.py sync

# 或执行完整流程(包含备份)
python sync_theme.py full
```

### 示例3: 排查问题
```bash
# 查看容器日志
python sync_theme.py logs -n 100

# 验证主题文件
python sync_theme.py verify

# 重启容器
python sync_theme.py restart
```

## 工作流程

```
┌─────────────┐
│  本地主题    │
│  (Windows)  │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  SSH连接测试 │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  检查容器    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  备份主题    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  SCP同步     │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  修正权限    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  验证结果    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  Docker容器  │
│  (Linux)    │
└─────────────┘
```

## 故障排查

### 1. SSH连接失败
```
❌ SSH连接失败,请检查密钥配置
```
**解决方法**:
- 检查SSH密钥是否已添加到服务器
- 测试手动连接: `ssh root@192.168.31.14`

### 2. 容器未找到
```
❌ 未找到plog容器: plog-pro
```
**解决方法**:
- 检查容器名称是否正确
- 使用 `docker ps` 查看运行中的容器

### 3. 同步失败
```
❌ SCP同步失败
```
**解决方法**:
- 检查本地主题路径是否正确
- 检查远程目录权限
- 查看详细错误信息

### 4. 权限问题
```
❌ 文件权限错误
```
**解决方法**:
- 检查配置文件中的 `file_owner` 设置
- 确保SSH用户有sudo权限

## 高级用法

### 使用不同配置文件
```bash
python sync_theme.py full -c custom_config.json
```

### 集成到CI/CD
```bash
# 在CI/CD脚本中
python sync_theme.py sync
python sync_theme.py verify
```

### 自动化部署脚本
```bash
#!/bin/bash
# deploy.sh
echo "开始部署..."
python sync_theme.py full
if [ $? -eq 0 ]; then
    echo "部署成功!"
else
    echo "部署失败!"
    exit 1
fi
```

## 注意事项

1. **SSH密钥认证**: 必须先配置SSH密钥,不支持密码认证
2. **文件权限**: 同步后会自动修正文件所有者为www-data
3. **备份**: 每次完整同步会自动备份远程主题
4. **容器重启**: 同步后不需要重启容器,文件立即生效

## 技术栈

- Python 3.6+
- OpenSSH (ssh, scp)
- subprocess (命令执行)
- pathlib (路径处理)
- json (配置管理)
