#!/bin/bash
# 应用后台隐藏样式脚本

echo "=== 应用Emlog后台提示隐藏样式 ==="

# 1. 复制CSS文件到容器
echo "1. 复制CSS文件到容器..."
docker cp f:/mytheme/admin-hide-promo.css emlog-dev:/app/admin/views/css/hide-promo.css

# 2. 在后台header中引入CSS
echo "2. 修改后台header引入CSS..."
docker exec emlog-dev sh -c "
if ! grep -q 'hide-promo.css' /app/admin/views/header.php; then
    sed -i '/<link.*css-main.css/a <link href=\"./css/hide-promo.css\" rel=\"stylesheet\">' /app/admin/views/header.php
    echo 'CSS引入成功'
else
    echo 'CSS已存在,跳过'
fi
"

# 3. 清除浏览器缓存提示
echo "3. 样式已应用!"
echo ""
echo "请清除浏览器缓存后刷新后台页面查看效果"
echo "访问: http://localhost:18080/admin"
