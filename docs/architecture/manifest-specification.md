# Plog CMS Manifest 规范文档

## 概述

本文档定义 Plog CMS 系统的插件和主题 manifest 规范，为 PHP → Rust 迁移提供标准化的扩展点声明机制。

**最后更新**: 2026-03-29
**版本**: v1.0.0

---

## 设计原则

1. **声明式**: 通过 manifest 声明能力，而非运行时动态探测
2. **标准化**: 统一的格式，便于跨语言解析
3. **可扩展**: 支持未来扩展新的能力类型
4. **向后兼容**: 支持现有插件/主题迁移

---

## 插件 Manifest 规范

### 文件位置

```
plugins/
└── {plugin-name}/
    ├── manifest.json      # 插件清单
    ├── plugin.php         # 插件入口 (PHP)
    └── src/               # 插件代码
```

### Manifest 格式

```json
{
  "$schema": "https://plog.dev/schemas/plugin-manifest.json",
  "name": "example-plugin",
  "version": "1.0.0",
  "api_version": "1.0",
  "description": "示例插件，演示 manifest 规范",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "homepage": "https://github.com/example/plugin",
  "keywords": ["example", "demo"],
  
  "dependencies": {
    "plog": ">=2.0.0",
    "php": ">=7.4"
  },
  
  "capabilities": [
    "content:read",
    "content:write",
    "hook:filter:post_content",
    "hook:action:post_save"
  ],
  
  "hooks": {
    "filters": {
      "post_content": {
        "handler": "filterPostContent",
        "priority": 10
      },
      "post_excerpt": {
        "handler": "filterPostExcerpt",
        "priority": 10
      }
    },
    "actions": {
      "post_save": {
        "handler": "onPostSave",
        "priority": 10
      },
      "post_delete": {
        "handler": "onPostDelete",
        "priority": 10
      },
      "plugin_activate": {
        "handler": "onActivate"
      },
      "plugin_deactivate": {
        "handler": "onDeactivate"
      }
    }
  },
  
  "config": {
    "schema": "config/schema.json",
    "default": "config/default.json",
    "page": "admin/settings"
  },
  
  "admin": {
    "menu": {
      "title": "示例插件",
      "slug": "example-plugin",
      "icon": "dashicons-admin-plugins",
      "position": 30
    },
    "pages": [
      {
        "title": "设置",
        "slug": "settings",
        "handler": "adminSettingsPage"
      }
    ]
  },
  
  "api": {
    "routes": [
      {
        "method": "GET",
        "path": "/example",
        "handler": "apiGetExample",
        "permission": "content:read"
      }
    ]
  },
  
  "autoload": {
    "psr4": {
      "ExamplePlugin\\": "src/"
    }
  }
}
```

### 字段说明

#### 必填字段

| 字段 | 类型 | 说明 |
|------|------|------|
| name | string | 插件标识，唯一 |
| version | string | 语义化版本号 |
| description | string | 插件描述 |
| author | object/string | 作者信息 |
| capabilities | array | 能力声明列表 |

#### 可选字段

| 字段 | 类型 | 说明 |
|------|------|------|
| api_version | string | API 版本 |
| license | string | 许可证 |
| homepage | string | 主页 URL |
| keywords | array | 关键词 |
| dependencies | object | 依赖声明 |
| hooks | object | Hook 声明 |
| config | object | 配置声明 |
| admin | object | 后台管理声明 |
| api | object | API 路由声明 |
| autoload | object | 自动加载配置 |

---

## 主题 Manifest 规范

### 文件位置

```
themes/
└── {theme-name}/
    ├── manifest.json      # 主题清单
    ├── theme.php          # 主题入口 (可选)
    ├── templates/         # 模板文件
    ├── assets/            # 静态资源
    └── screenshot.png     # 主题截图
```

### Manifest 格式

```json
{
  "$schema": "https://plog.dev/schemas/theme-manifest.json",
  "name": "example-theme",
  "version": "1.0.0",
  "api_version": "1.0",
  "description": "示例主题，演示 manifest 规范",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://example.com"
  },
  "license": "MIT",
  "homepage": "https://github.com/example/theme",
  "keywords": ["responsive", "modern", "clean"],
  
  "engine": "blade",
  "engine_version": "1.0",
  
  "templates": {
    "index": {
      "file": "templates/index.blade.php",
      "description": "首页模板"
    },
    "post": {
      "file": "templates/post.blade.php",
      "description": "文章模板"
    },
    "page": {
      "file": "templates/page.blade.php",
      "description": "页面模板"
    },
    "category": {
      "file": "templates/category.blade.php",
      "description": "分类模板"
    },
    "tag": {
      "file": "templates/tag.blade.php",
      "description": "标签模板"
    },
    "archive": {
      "file": "templates/archive.blade.php",
      "description": "归档模板"
    },
    "search": {
      "file": "templates/search.blade.php",
      "description": "搜索模板"
    },
    "404": {
      "file": "templates/404.blade.php",
      "description": "404 模板"
    },
    "header": {
      "file": "templates/partials/header.blade.php",
      "description": "头部局部模板"
    },
    "footer": {
      "file": "templates/partials/footer.blade.php",
      "description": "底部局部模板"
    },
    "sidebar": {
      "file": "templates/partials/sidebar.blade.php",
      "description": "侧边栏局部模板"
    }
  },
  
  "assets": {
    "css": [
      "assets/css/style.css",
      "assets/css/responsive.css"
    ],
    "js": [
      "assets/js/main.js"
    ],
    "images": [
      "assets/images/*"
    ]
  },
  
  "supports": [
    "responsive",
    "dark-mode",
    "customizer",
    "widgets",
    "menus",
    "post-thumbnails",
    "custom-header",
    "custom-background"
  ],
  
  "menus": {
    "primary": "主导航",
    "footer": "底部导航",
    "social": "社交链接"
  },
  
  "widgets": {
    "sidebar": {
      "name": "侧边栏",
      "description": "显示在侧边栏的小组件"
    },
    "footer-1": {
      "name": "底部栏 1",
      "description": "显示在底部的第一列"
    },
    "footer-2": {
      "name": "底部栏 2",
      "description": "显示在底部的第二列"
    }
  },
  
  "customizer": {
    "sections": [
      {
        "id": "theme_options",
        "title": "主题选项",
        "settings": [
          {
            "id": "primary_color",
            "type": "color",
            "label": "主题色",
            "default": "#0073aa"
          },
          {
            "id": "show_sidebar",
            "type": "checkbox",
            "label": "显示侧边栏",
            "default": true
          }
        ]
      }
    ]
  },
  
  "dependencies": {
    "plog": ">=2.0.0",
    "php": ">=7.4"
  }
}
```

### 字段说明

#### 必填字段

| 字段 | 类型 | 说明 |
|------|------|------|
| name | string | 主题标识，唯一 |
| version | string | 语义化版本号 |
| description | string | 主题描述 |
| author | object/string | 作者信息 |
| engine | string | 模板引擎 |
| templates | object | 模板声明 |

#### 可选字段

| 字段 | 类型 | 说明 |
|------|------|------|
| api_version | string | API 版本 |
| license | string | 许可证 |
| homepage | string | 主页 URL |
| keywords | array | 关键词 |
| engine_version | string | 引擎版本 |
| assets | object | 静态资源 |
| supports | array | 支持特性 |
| menus | object | 菜单位置 |
| widgets | object | 小组件区域 |
| customizer | object | 自定义选项 |
| dependencies | object | 依赖声明 |

---

## 能力声明规范

### 能力格式

```
{resource}:{action}
```

### 能力分类

#### 内容能力

| 能力 | 说明 |
|------|------|
| content:read | 读取内容 |
| content:write | 写入内容 |
| content:delete | 删除内容 |
| content:publish | 发布内容 |

#### 用户能力

| 能力 | 说明 |
|------|------|
| user:read | 读取用户信息 |
| user:write | 写入用户信息 |
| user:delete | 删除用户 |

#### Hook 能力

| 能力 | 说明 |
|------|------|
| hook:filter:* | 所有过滤器 |
| hook:action:* | 所有动作 |
| hook:filter:{name} | 特定过滤器 |
| hook:action:{name} | 特定动作 |

#### API 能力

| 能力 | 说明 |
|------|------|
| api:access | 访问 API |
| api:admin | 管理 API |

#### 管理能力

| 能力 | 说明 |
|------|------|
| admin:access | 访问后台 |
| admin:settings | 管理设置 |
| admin:plugins | 管理插件 |
| admin:themes | 管理主题 |

---

## Manifest 解析器

### PHP 实现

```php
<?php

namespace Plog\Core\Manifest;

class ManifestParser
{
    /**
     * 解析插件 manifest
     */
    public function parsePlugin(string $path): PluginManifest
    {
        $file = $path . '/manifest.json';
        
        if (!file_exists($file)) {
            throw new ManifestException("Manifest file not found: {$file}");
        }
        
        $content = file_get_contents($file);
        $data = json_decode($content, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new ManifestException("Invalid JSON: " . json_last_error_msg());
        }
        
        $this->validatePluginManifest($data);
        
        return new PluginManifest($data);
    }
    
    /**
     * 解析主题 manifest
     */
    public function parseTheme(string $path): ThemeManifest
    {
        $file = $path . '/manifest.json';
        
        if (!file_exists($file)) {
            throw new ManifestException("Manifest file not found: {$file}");
        }
        
        $content = file_get_contents($file);
        $data = json_decode($content, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new ManifestException("Invalid JSON: " . json_last_error_msg());
        }
        
        $this->validateThemeManifest($data);
        
        return new ThemeManifest($data);
    }
    
    /**
     * 验证插件 manifest
     */
    private function validatePluginManifest(array $data): void
    {
        $required = ['name', 'version', 'description', 'capabilities'];
        
        foreach ($required as $field) {
            if (!isset($data[$field])) {
                throw new ManifestException("Required field missing: {$field}");
            }
        }
        
        // 验证版本格式
        if (!preg_match('/^\d+\.\d+\.\d+$/', $data['version'])) {
            throw new ManifestException("Invalid version format: {$data['version']}");
        }
        
        // 验证能力声明
        if (!is_array($data['capabilities'])) {
            throw new ManifestException("Capabilities must be an array");
        }
    }
    
    /**
     * 验证主题 manifest
     */
    private function validateThemeManifest(array $data): void
    {
        $required = ['name', 'version', 'description', 'engine', 'templates'];
        
        foreach ($required as $field) {
            if (!isset($data[$field])) {
                throw new ManifestException("Required field missing: {$field}");
            }
        }
        
        // 验证版本格式
        if (!preg_match('/^\d+\.\d+\.\d+$/', $data['version'])) {
            throw new ManifestException("Invalid version format: {$data['version']}");
        }
        
        // 验证模板声明
        if (!is_array($data['templates'])) {
            throw new ManifestException("Templates must be an object");
        }
    }
}
```

### Rust 实现 (预览)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Author,
    pub capabilities: Vec<String>,
    pub hooks: Option<Hooks>,
    pub config: Option<Config>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Author,
    pub engine: String,
    pub templates: std::collections::HashMap<String, Template>,
    pub supports: Option<Vec<String>>,
}

impl PluginManifest {
    pub fn parse(path: &str) -> Result<Self, ManifestError> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&content)?;
        Ok(manifest)
    }
    
    pub fn validate(&self) -> Result<(), ManifestError> {
        // 验证逻辑
        Ok(())
    }
}
```

---

## 迁移指南

### 从现有插件迁移

1. 分析现有插件结构
2. 创建 manifest.json 文件
3. 声明能力
4. 声明 Hook
5. 测试兼容性

### 从现有主题迁移

1. 分析现有主题结构
2. 创建 manifest.json 文件
3. 声明模板
4. 声明资源
5. 测试渲染

---

## 变更历史

| 版本 | 日期 | 变更内容 |
|------|------|----------|
| v1.0.0 | 2026-03-29 | 初始版本 |
