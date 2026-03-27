# Plog完整开发项目

## 项目简介

Plog是一个基于Plog开发的现代化博客系统,采用深色科技风格主题。

## 项目结构

```
f:\mytheme\
├── admin/                        # 后台管理
├── include/                      # 核心库
├── content/
│   ├── templates/
│   │   ├── mytheme/             # 自定义主题 ⭐
│   │   └── default/             # 默认主题
│   ├── plugins/                 # 插件目录
│   ├── uploadfile/              # 上传文件
│   └── languages/               # 语言包
├── config/                       # 配置文件
├── scripts/                      # 开发脚本
├── docs/                         # 项目文档
├── dev/                          # 开发数据
├── index.php                     # 入口文件
└── init.php                      # 初始化文件
```

## 快速开始

### 1. 启动开发环境
```powershell
.\scripts\dev-tools.ps1 start
```

### 2. 访问服务
- **前台**: http://localhost:18080
- **后台**: http://localhost:18080/admin
- **phpMyAdmin**: http://localhost:18081

### 3. 首次安装
访问 http://localhost:18080,填写数据库信息:
- 主机: `mysql-dev`
- 数据库: `plog_dev`
- 用户: `plog`
- 密码: `plog123456`

### 4. 启用主题
后台 → 外观 → 模板管理 → 启用 mytheme

## 开发工作流

### 主题开发
```
1. 修改 content/templates/mytheme/ 下的文件
2. 保存后立即刷新浏览器查看效果
3. 测试通过后同步到Linux
```

### 同步到Linux测试环境
```bash
python scripts/sync_theme.py full
```

## 开发命令

### 环境管理
```bash
.\scripts\dev-tools.ps1 start      # 启动
.\scripts\dev-tools.ps1 stop       # 停止
.\scripts\dev-tools.ps1 restart    # 重启
.\scripts\dev-tools.ps1 status     # 状态
```

### 同步工具
```bash
python scripts/sync_theme.py full      # 完整同步
python scripts/sync_theme.py sync      # 同步文件
python scripts/sync_theme.py verify    # 验证
```

## 项目特点

### ✅ 完整的博客系统
- 包含所有核心文件
- 可以修改后端代码
- 支持深度定制

### ✅ 清晰的项目结构
- 文档统一管理 (docs/)
- 脚本统一管理 (scripts/)
- 配置统一管理 (config/)

### ✅ 开发环境
- Docker容器化
- 数据库可视化管理
- 实时文件同步

### ✅ 自定义主题
- 现代科技风格
- 深色背景设计
- 霓虹色配色
- 响应式布局

## 技术栈

- **Plog Pro 1.0.0** - 博客系统
- **PHP 7.4 + Apache** - 运行环境
- **MySQL 5.7** - 数据库
- **Docker** - 容器化
- **Python 3** - 同步脚本

## 版本信息

- **Plog版本**: pro 1.0.0
- **主题版本**: 1.0
- **PHP版本**: 7.4
- **MySQL版本**: 5.7

## 文档索引

- **docs/README.md** - 本文档
- **docs/DEV_ENVIRONMENT.md** - 开发环境详细指南
- **docs/QUICK_START.md** - 快速开始指南

## 许可证

本项目基于Plog开发,遵循相关开源协议。
