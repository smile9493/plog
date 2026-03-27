# 项目更名说明

## 更名内容

项目已从 **Plog** 更名为 **Plog**

## 已完成的修改

### 1. 核心常量
- ✅ `PLOG_ROOT` → `PLOG_ROOT`
- ✅ `PLOG_VERSION` → `PLOG_VERSION`
- ✅ `PLOG_VERSION_TIMESTAMP` → `PLOG_VERSION_TIMESTAMP`

### 2. 版本信息
- ✅ 版本号更新为: `pro 1.0.0`

### 3. 文档更新
- ✅ README.md 已更新为 Plog
- ✅ 项目说明已更新

### 4. .gitignore完善
- ✅ 添加了完整的忽略规则
- ✅ 包含IDE、依赖、临时文件等

## 批量替换工具

由于项目中仍有大量Plog引用(约1000+处),已创建批量替换脚本:

### 使用方法
```bash
# 执行批量替换
bash scripts/rename-to-plog.sh
```

### 替换规则
- `PLOG_ROOT` → `PLOG_ROOT`
- `PLOG_VERSION` → `PLOG_VERSION`
- `PLOG_VERSION_TIMESTAMP` → `PLOG_VERSION_TIMESTAMP`
- `plog` → `plog`
- `Plog` → `Plog`
- `PLOG` → `PLOG`

## 注意事项

### 需要手动检查的地方
1. **数据库配置**: 数据库名、用户名可能需要更新
2. **Docker配置**: 容器名、镜像名
3. **外部链接**: 官网链接、文档链接
4. **版权声明**: license.txt中的版权信息

### 建议操作顺序
1. 先执行批量替换脚本
2. 检查核心配置文件
3. 更新数据库相关配置
4. 测试系统功能
5. 提交代码

## 兼容性说明

- 保持与Plog的兼容性
- 可以使用Plog的插件和主题
- 数据库结构保持不变

## 后续工作

如需完成全部更名,请执行:
```bash
bash scripts/rename-to-plog.sh
```

这将替换所有PHP、文档、配置文件中的Plog引用。
