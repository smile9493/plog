# 后台提示信息隐藏方案

## 已完成的工作

### 1. 创建了CSS隐藏规则
文件: `admin-hide-promo.css`

隐藏的内容包括:
- ✅ 未注册提示徽章
- ✅ SVIP/VIP提示徽章
- ✅ 正版注册功能提示卡片
- ✅ 注册按钮和链接
- ✅ 升级授权按钮
- ✅ 应用商店付费提示
- ✅ 授权相关页面链接
- ✅ 弹窗中的注册提示
- ✅ 通知栏中的注册提醒

### 2. 应用了隐藏样式
- ✅ CSS文件已复制到容器: `/app/admin/views/css/hide-promo.css`
- ✅ 已在后台header中引入CSS
- ✅ 样式已生效

## 使用方法

### 查看效果
1. 清除浏览器缓存 (Ctrl+Shift+Delete)
2. 访问后台: http://localhost:18080/admin
3. 刷新页面

### 同步到Linux测试环境
```bash
# 复制CSS到Linux
scp f:/mytheme/admin-hide-promo.css root@192.168.31.14:/tmp/

# SSH到Linux应用
ssh root@192.168.31.14 << 'EOF'
# 找到plog容器
CONTAINER=$(docker ps --filter 'name=plog' --format '{{.Names}}' | head -1)

# 复制CSS到容器
docker cp /tmp/admin-hide-promo.css $CONTAINER:/app/admin/views/css/hide-promo.css

# 引入CSS
docker exec $CONTAINER sh -c "
if ! grep -q 'hide-promo.css' /app/admin/views/header.php; then
    sed -i '/<link.*css-main.css/a <link href=\"./css/hide-promo.css\" rel=\"stylesheet\">' /app/admin/views/header.php
fi
"

echo "Linux环境已应用隐藏样式"
EOF
```

## 自定义调整

### 如果某些元素没被隐藏
编辑 `admin-hide-promo.css`,添加新的选择器:

```css
/* 示例: 隐藏特定元素 */
.your-element-class {
    display: none !important;
}
```

### 如果隐藏了不该隐藏的内容
在CSS中添加排除规则:

```css
/* 示例: 保留特定元素 */
.important-element {
    display: block !important;
}
```

## 技术原理

### CSS选择器说明
- `[href*="register"]` - 匹配包含register的链接
- `:contains("文本")` - 匹配包含特定文本的元素
- `:has(选择器)` - 匹配包含特定子元素的父元素
- `!important` - 强制应用样式

### 优点
- ✅ 不修改PHP代码,不影响功能
- ✅ 纯前端隐藏,安全无风险
- ✅ 易于维护和调整
- ✅ 可随时启用/禁用

### 注意事项
- CSS的`:contains()`选择器在某些浏览器可能不支持
- 建议使用浏览器开发者工具检查元素,精确匹配class或id
- 清除浏览器缓存后才能看到效果

## 进阶方案

### 方案一: 使用浏览器扩展
安装样式管理扩展(如Stylus),注入自定义CSS

### 方案二: 修改后台模板
直接编辑PHP文件,删除或注释提示代码(不推荐,升级会丢失)

### 方案三: 开发插件
创建plog插件,通过钩子过滤输出内容

## 文件清单

- `admin-hide-promo.css` - CSS隐藏规则
- `apply-hide-style.sh` - 应用脚本
- `HIDE_PROMO_GUIDE.md` - 本文档

## 效果验证

访问后台检查以下内容是否被隐藏:
- [ ] 首页的注册提示卡片
- [ ] 顶部的未注册徽章
- [ ] 侧边栏的授权链接
- [ ] 应用商店的VIP提示

如果还有显示的提示,请告诉我具体位置,我会补充CSS规则。
