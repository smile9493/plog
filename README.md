# Plog - 现代化博客系统

<div align="center">

![Plog Logo](https://img.shields.io/badge/Plog-Pro%201.0.0-blue.svg)
![PHP Version](https://img.shields.io/badge/PHP-7.4%2B-purple.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

**一个简洁、现代、可定制的博客系统**

[快速开始](#快速开始) · [功能特性](#功能特性) · [文档](#文档) · [贡献](#贡献)

</div>

---

## 📖 项目简介

Plog是一个基于PHP开发的现代化博客系统,采用深色科技风格设计,支持主题和插件扩展。

### ✨ 核心特性

- 🎨 **现代UI设计** - 深色科技风格,霓虹色配色
- 🚀 **高性能** - 优化的代码结构,快速响应
- 🔧 **高度可定制** - 支持主题和插件扩展
- 📱 **响应式布局** - 完美适配各种设备
- 🔐 **安全可靠** - 完善的安全机制
- 🐳 **Docker支持** - 一键部署开发环境

## 🚀 快速开始

### 环境要求

- PHP >= 7.4
- MySQL >= 5.6
- Apache/Nginx Web服务器
- Docker (可选,用于开发环境)

### 安装步骤

#### 方式一: Docker开发环境 (推荐)

```bash
# 1. 克隆项目
git clone https://github.com/smile9493/plog.git
cd plog

# 2. 启动开发环境
./scripts/dev-tools.sh start

# 3. 访问安装页面
# http://localhost:18080
```

#### 方式二: 传统安装

```bash
# 1. 下载源码
git clone https://github.com/smile9493/plog.git

# 2. 配置Web服务器
# 将项目目录设置为网站根目录

# 3. 创建数据库
mysql -u root -p
CREATE DATABASE plog DEFAULT CHARACTER SET utf8mb4;

# 4. 访问安装页面
# http://your-domain.com/install.php
```

### 数据库配置

安装时填写以下信息:
- **主机**: `localhost` 或 `mysql-dev` (Docker环境)
- **数据库**: `plog`
- **用户**: `plog`
- **密码**: `your_password`

## 🎨 功能特性

### 主题系统

- 🎭 **多主题支持** - 可切换不同主题
- 🎨 **自定义主题** - 支持自定义开发
- 📱 **响应式设计** - 自适应各种设备

### 插件系统

- 🔌 **插件扩展** - 丰富的插件生态
- 🛠️ **自定义插件** - 支持自定义开发
- ⚙️ **插件管理** - 后台可视化管理

### 内容管理

- 📝 **文章管理** - 强大的编辑器
- 📂 **分类标签** - 灵活的内容组织
- 💬 **评论系统** - 支持评论互动
- 🖼️ **媒体管理** - 图片视频管理

### SEO优化

- 🔍 **SEO友好** - 优化的URL结构
- 📊 **站点地图** - 自动生成sitemap
- 🏷️ **Meta标签** - 自定义SEO信息

## 📁 项目结构

```
plog/
├── admin/              # 后台管理
├── include/            # 核心库
├── content/
│   ├── templates/      # 主题目录
│   │   ├── mytheme/   # 默认主题
│   │   └── default/   # 备用主题
│   ├── plugins/       # 插件目录
│   └── uploadfile/    # 上传文件
├── config/            # 配置文件
├── scripts/           # 开发脚本
├── docs/              # 项目文档
└── index.php          # 入口文件
```

## 🛠️ 开发指南

### 开发环境

```bash
# 启动开发环境
./scripts/dev-tools.sh start

# 查看状态
./scripts/dev-tools.sh status

# 查看日志
./scripts/dev-tools.sh logs

# 停止环境
./scripts/dev-tools.sh stop
```

### 主题开发

1. 创建主题目录: `content/templates/your-theme/`
2. 创建必要文件:
   - `header.php` - 页头
   - `footer.php` - 页脚
   - `log_list.php` - 文章列表
   - `echo_log.php` - 文章详情
   - `style.css` - 样式文件
3. 在后台启用主题

### 插件开发

1. 创建插件目录: `content/plugins/your-plugin/`
2. 创建插件文件: `your-plugin.php`
3. 在后台启用插件

## 📚 文档

- [安装指南](docs/INSTALL.md)
- [开发文档](docs/DEV_ENVIRONMENT.md)
- [主题开发](docs/THEME_DEV.md)
- [插件开发](docs/PLUGIN_DEV.md)
- [API文档](docs/API.md)

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议!

### 贡献步骤

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 📝 更新日志

### v1.0.0 (2026-03-28)

- ✨ 初始版本发布
- 🎨 深色科技风格主题
- 🔧 完整的博客功能
- 🐳 Docker开发环境支持
- 📱 响应式设计

## 📄 许可证

本项目基于 [MIT License](LICENSE) 开源协议。

## 🙏 致谢

- 感谢所有贡献者
- 感谢开源社区的支持

## 📮 联系方式

- **项目主页**: https://github.com/smile9493/plog
- **问题反馈**: https://github.com/smile9493/plog/issues
- **作者**: smile9493

---

<div align="center">

**⭐ 如果这个项目对你有帮助,请给一个Star支持一下! ⭐**

Made with ❤️ by smile9493

</div>
