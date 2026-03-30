# Manifest 与能力声明 - PRD

## 1. 概述

**任务名称**: Manifest 与能力声明
**所属阶段**: Phase 1 - 边界收敛
**优先级**: P1 (重要)
**预计工时**: 2 周
**前置依赖**: 三态分离设计完成

## 2. 背景

插件和主题是 CMS 系统的重要扩展点。在迁移到 Rust 之前，需要建立标准化的 manifest 格式和能力声明机制，而不是依赖 PHP 动态探测。

## 3. 目标

1. 设计插件 manifest.json 格式
2. 设计主题 manifest.json 格式
3. 定义能力声明规范
4. 编写 manifest 解析器
5. 迁移现有插件/主题到新格式

## 4. 范围

### 包含

- 插件 manifest 规范
- 主题 manifest 规范
- 能力声明规范
- manifest 解析器
- 迁移工具

### 不包含

- Rust 插件系统实现
- Rust 主题系统实现
- 新增插件/主题功能

## 5. 验收标准

- [ ] 插件 manifest 规范文档完整
- [ ] 主题 manifest 规范文档完整
- [ ] 能力声明规范文档完整
- [ ] manifest 解析器可运行
- [ ] 现有插件/主题可迁移

## 6. 任务清单

### 6.1 插件 Manifest 规范

- [ ] 定义基本字段 (name, version, description)
- [ ] 定义依赖声明
- [ ] 定义能力声明
- [ ] 定义 Hook 声明
- [ ] 定义配置声明
- [ ] 编写规范文档

### 6.2 主题 Manifest 规范

- [ ] 定义基本字段 (name, version, description)
- [ ] 定义模板声明
- [ ] 定义资源声明
- [ ] 定义特性声明
- [ ] 编写规范文档

### 6.3 能力声明规范

- [ ] 定义能力类型
- [ ] 定义能力范围
- [ ] 定义能力验证规则
- [ ] 编写规范文档

### 6.4 Manifest 解析器

- [ ] 实现插件 manifest 解析
- [ ] 实现主题 manifest 解析
- [ ] 实现能力验证
- [ ] 编写单元测试

### 6.5 迁移工具

- [ ] 分析现有插件结构
- [ ] 分析现有主题结构
- [ ] 生成 manifest 模板
- [ ] 执行迁移

## 7. 交付物

1. 插件 manifest 规范文档
2. 主题 manifest 规范文档
3. 能力声明规范文档
4. manifest 解析器
5. 迁移工具

## 8. Manifest 示例

### 8.1 插件 Manifest

```json
{
  "name": "example-plugin",
  "version": "1.0.0",
  "description": "示例插件",
  "author": "Author Name",
  "license": "MIT",
  "dependencies": {
    "plog": ">=2.0.0"
  },
  "capabilities": [
    "content:read",
    "content:write",
    "hook:filter:post_content"
  ],
  "hooks": {
    "post_save": "onPostSave",
    "post_delete": "onPostDelete"
  },
  "config": {
    "schema": "config-schema.json",
    "default": "config-default.json"
  }
}
```

### 8.2 主题 Manifest

```json
{
  "name": "example-theme",
  "version": "1.0.0",
  "description": "示例主题",
  "author": "Author Name",
  "license": "MIT",
  "engine": "blade",
  "templates": {
    "index": "templates/index.blade.php",
    "post": "templates/post.blade.php",
    "page": "templates/page.blade.php"
  },
  "assets": {
    "css": ["assets/css/style.css"],
    "js": ["assets/js/main.js"]
  },
  "supports": [
    "responsive",
    "dark-mode",
    "customizer"
  ]
}
```

## 9. 风险

| 风险项 | 影响 | 概率 | 应对措施 |
|--------|------|------|----------|
| 现有插件/主题不兼容 | 高 | 中 | 提供兼容层 |
| 规范设计不合理 | 中 | 中 | 参考成熟框架 |
| 迁移工具不完善 | 中 | 中 | 迭代优化 |

## 10. 参考文档

- [design-direction.md](../../spec/plog-rust-migration/design-direction.md)
- [migration-plan.md](../../spec/plog-rust-migration/migration-plan.md)
- [boundaries-and-protocols.md](../../spec/plog-rust-migration/boundaries-and-protocols.md)
