# Python同步工具快速使用指南

## 🚀 快速开始

### 1. 完整同步(推荐)
```bash
python sync_theme.py full
```
自动执行: 连接测试 → 容器检查 → 备份 → 同步 → 验证

### 2. 仅同步文件
```bash
python sync_theme.py sync
```

### 3. 验证主题
```bash
python sync_theme.py verify
```

### 4. 查看日志
```bash
python sync_theme.py logs
```

### 5. 重启容器
```bash
python sync_theme.py restart
```

## 📁 文件说明

- `sync_theme.py` - 主程序脚本
- `sync_config.json` - 配置文件(已配置)
- `sync_config.json.example` - 配置示例
- `SYNC_TOOL_README.md` - 详细文档

## ✅ 测试结果

脚本已测试通过:
- ✅ SSH连接成功
- ✅ 容器状态检查正常
- ✅ 主题文件同步成功
- ✅ 文件权限修正正常
- ✅ 验证功能正常

## 🎯 使用场景

### 场景1: 修改主题后同步
```bash
# 1. 修改本地主题文件
# 2. 执行同步
python sync_theme.py sync
# 3. 刷新浏览器查看效果
```

### 场景2: 完整部署流程
```bash
# 执行完整流程(包含备份)
python sync_theme.py full
```

### 场景3: 排查问题
```bash
# 查看容器日志
python sync_theme.py logs -n 100

# 验证主题文件
python sync_theme.py verify
```

## 📝 配置文件

当前配置:
- 服务器: 192.168.31.14 (root)
- 容器: plog-pro
- 主题: mytheme
- 端口: 8080

如需修改,编辑 `sync_config.json` 文件。

## 🔧 高级用法

### 使用不同配置文件
```bash
python sync_theme.py full -c custom_config.json
```

### 查看更多日志
```bash
python sync_theme.py logs -n 200
```

## 🌐 访问地址

- 前台: http://192.168.31.14:8080
- 后台: http://192.168.31.14:8080/admin

## 📌 注意事项

1. 确保SSH密钥已配置(已完成)
2. 同步后文件立即生效,无需重启容器
3. 每次full同步会自动备份远程主题
4. 文件权限自动修正为www-data:www-data
