# 主题兼容层

## 概述

Phase 3 的主题兼容层，桥接 PHP 主题和 Rust 核心。

## 架构

```
┌─────────────────────────────────────────────────────────────┐
│                  Theme Render Pipeline                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐  │
│  │   Request   │────►│   Router    │────►│  Renderer   │  │
│  └─────────────┘     └─────────────┘     └─────────────┘  │
│                                                     │        │
│                           ┌─────────────────────────┘        │
│                           │                                  │
│              ┌────────────┴────────────┐                    │
│              ▼                         ▼                    │
│     ┌─────────────┐           ┌─────────────┐              │
│     │ PHP Render  │           │ Rust Render │              │
│     │ (老主题)    │           │ (新主题)    │              │
│     └─────────────┘           └─────────────┘              │
│              │                         │                    │
│              ▼                         ▼                    │
│     ┌─────────────┐           ┌─────────────┐              │
│     │   Response  │           │   Response  │              │
│     └─────────────┘           └─────────────┘              │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## 目录结构

```
compat/theme/
├── ThemeManager.php   # 主题管理器
├── ThemeLoader.php    # 主题加载器
├── Renderer.php       # 渲染器
├── TemplateEngine.php # 模板引擎
└── README.md          # 本文档
```

## 使用方式

### 初始化

```php
require_once 'compat/theme/ThemeManager.php';

$manager = new ThemeManager();
$manager->loadAll();
$manager->setTheme('default');
```

### 渲染模板

```php
// 渲染单个模板
$html = $manager->render('header.php', ['title' => 'My Blog']);

// 渲染页面
$html = $manager->renderPage('index', ['posts' => $posts]);
```

### 全局函数

```php
// 渲染模板
$html = render_template('header.php', ['title' => 'My Blog']);

// 渲染页面
$html = render_page('index', ['posts' => $posts]);
```

## 模板语法转换

### PHP → Tera

| PHP 语法 | Tera 语法 |
|----------|-----------|
| `<?php echo $var; ?>` | `{{ var }}` |
| `<?= $var ?>` | `{{ var }}` |
| `<?php if ($cond): ?>` | `{% if cond %}` |
| `<?php else: ?>` | `{% else %}` |
| `<?php endif; ?>` | `{% endif %}` |
| `<?php foreach ($items as $item): ?>` | `{% for item in items %}` |
| `<?php endforeach; ?>` | `{% endfor %}` |

## 页面类型

| 类型 | 模板文件 | 说明 |
|------|----------|------|
| index | log_list.php | 首页 |
| post | echo_log.php | 文章页 |
| page | page.php | 页面 |
| category | log_list.php | 分类页 |
| tag | log_list.php | 标签页 |
| search | log_list.php | 搜索页 |
| 404 | 404.php | 404 页面 |

## 兼容性

- 支持现有 Emlog 主题格式
- 支持 PHP 原生模板
- 支持 Tera 模板转换
- 自动回退到 PHP 渲染
