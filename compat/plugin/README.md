# 插件兼容层

## 概述

Phase 3 的插件兼容层，桥接 PHP 插件和 Rust 核心。

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                  Plugin Compat Layer                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │ PHP Plugin  │────►│  Hook Bridge│────►│ Rust Core   │  │
│  │ (老插件)    │     │  (Hook桥接) │     │ (新核心)    │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                               │
│  ┌─────────────┐                                           │
│  │ Rust Plugin │  (新插件直接运行在 Rust 环境)              │
│  │ (新插件)    │                                           │
│  └─────────────┘                                           │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## 目录结构

```
compat/plugin/
├── PluginManager.php  # 插件管理器
├── PluginLoader.php   # 插件加载器
├── HookBridge.php     # Hook 桥接
├── PluginConfig.php   # 配置管理
└── README.md          # 本文档
```

## 使用方式

### 初始化

```php
require_once 'compat/plugin/PluginManager.php';

$manager = new PluginManager();
$manager->loadAll();
```

### 触发 Hook

```php
// Action Hook
do_action('post_save', $post);

// Filter Hook
$content = apply_filters('post_content', $content);
```

### 注册 Hook

```php
add_hook('action', 'post_save', function($post) {
    // 处理文章保存
});
```

### 获取插件配置

```php
$config = $manager->getPluginConfig('my_plugin');
$manager->setPluginConfig('my_plugin', ['key' => 'value']);
```

## Hook 类型

| 类型 | 说明 | 示例 |
|------|------|------|
| action | 执行动作 | post_save, post_delete |
| filter | 过滤内容 | post_content, post_title |

## 插件头部信息

```php
<?php
/**
 * Plugin Name: My Plugin
 * Version: 1.0.0
 * Description: My plugin description
 * Author: Author Name
 */
```

## 兼容性

- 支持现有 Emlog 插件格式
- 支持 Hook 系统
- 支持插件配置
- 自动桥接到 Rust 核心
