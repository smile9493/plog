# 项目重组方案

## 当前结构
```
f:\mytheme\
├── content/templates/mytheme/    # 我们的主题
├── plog-source/                 # plog官方源码
├── dev/                          # 开发环境配置
├── docker-compose.dev.yml        # Docker配置
└── 各种文档和脚本
```

## 目标结构
```
f:\mytheme\
├── admin/                        # plog后台(从源码)
├── include/                      # plog核心库(从源码)
├── content/
│   ├── templates/
│   │   ├── mytheme/             # 我们的主题
│   │   └── default/             # 默认主题(从源码)
│   ├── plugins/                 # 插件目录
│   ├── uploadfile/              # 上传文件
│   └── languages/               # 语言包(从源码)
├── dev/                          # 开发环境配置
├── index.php                     # 入口文件(从源码)
├── init.php                      # 初始化文件(从源码)
├── config.sample.php             # 配置示例(从源码)
├── docker-compose.dev.yml        # Docker配置
└── 开发工具和文档
```

## 执行步骤

### 1. 备份当前主题
```bash
cp -r content/templates/mytheme /tmp/mytheme_backup
```

### 2. 移动plog源码文件到根目录
```bash
# 移动核心目录
mv plog-source/admin .
mv plog-source/include .
mv plog-source/content/languages content/

# 移动核心文件
mv plog-source/index.php .
mv plog-source/init.php .
mv plog-source/config.sample.php .
mv plog-source/install.php .
mv plog-source/rss.php .
mv plog-source/robots.txt .
mv plog-source/favicon.ico .
mv plog-source/license.txt .

# 移动默认主题
mv plog-source/content/templates/default content/templates/
```

### 3. 清理
```bash
# 删除源码目录
rm -rf plog-source

# 删除不需要的文件
rm -f preview.html preview_style.css
```

### 4. 更新Docker配置
修改 docker-compose.dev.yml 的挂载路径

## 优势

1. **完整项目**: 包含完整的plog系统
2. **便于开发**: 可以修改核心代码
3. **版本控制**: 可以跟踪所有修改
4. **一键部署**: 直接使用整个目录

## 注意事项

- 保留我们的主题在 content/templates/mytheme
- 保留开发工具和文档
- 更新.gitignore排除敏感文件
- 更新Docker挂载配置
