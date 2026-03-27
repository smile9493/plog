#!/bin/bash
# 项目更名脚本: Emlog -> Plog

echo "=== 项目更名: Emlog -> Plog ==="
echo ""

# 定义替换规则
declare -A replacements=(
    ["EMLOG_ROOT"]="PLOG_ROOT"
    ["EMLOG_VERSION"]="PLOG_VERSION"
    ["EMLOG_VERSION_TIMESTAMP"]="PLOG_VERSION_TIMESTAMP"
    ["emlog"]="plog"
    ["Emlog"]="Plog"
    ["EMLOG"]="PLOG"
)

# 统计函数
count_occurrences() {
    local pattern=$1
    grep -r "$pattern" --include="*.php" --include="*.md" --include="*.json" --include="*.yml" . 2>/dev/null | grep -v "Binary file" | wc -l
}

# 显示统计
echo "当前Emlog引用统计:"
for pattern in "${!replacements[@]}"; do
    count=$(count_occurrences "$pattern")
    if [ "$count" -gt 0 ]; then
        echo "  $pattern: $count 处"
    fi
done

echo ""
read -p "确认执行批量替换? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "已取消"
    exit 0
fi

# 执行替换
echo ""
echo "开始批量替换..."

# PHP文件
find . -type f -name "*.php" -not -path "./.git/*" -not -path "./vendor/*" | while read file; do
    for pattern in "${!replacements[@]}"; do
        replacement="${replacements[$pattern]}"
        sed -i "s/$pattern/$replacement/g" "$file"
    done
done

# 文档文件
find . -type f -name "*.md" -not -path "./.git/*" | while read file; do
    for pattern in "${!replacements[@]}"; do
        replacement="${replacements[$pattern]}"
        sed -i "s/$pattern/$replacement/g" "$file"
    done
done

# 配置文件
find . -type f \( -name "*.json" -o -name "*.yml" \) -not -path "./.git/*" | while read file; do
    for pattern in "${!replacements[@]}"; do
        replacement="${replacements[$pattern]}"
        sed -i "s/$pattern/$replacement/g" "$file"
    done
done

echo "✅ 批量替换完成"

# 显示结果统计
echo ""
echo "替换后统计:"
for pattern in "${!replacements[@]}"; do
    count=$(count_occurrences "$pattern")
    if [ "$count" -gt 0 ]; then
        echo "  $pattern: $count 处(未替换)"
    fi
done

echo ""
echo "项目已更名为 Plog"
