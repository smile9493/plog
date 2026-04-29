# Performance Tuning Configuration

## 环境变量配置

### 日志级别 (慢查询监控)
```bash
# 基础配置
export RUST_LOG="plog_api=info,tower_http=info"

# 启用慢查询监控
export RUST_LOG="plog_api=info,sqlx=debug,tower_http=debug"

# 详细调试
export RUST_LOG="plog_api=debug,sqlx=trace,tower_http=debug"
```

### 数据库连接池
```bash
# 配置文件 config/settings.toml
[database]
url = "mysql://user:pass@localhost/plog"
max_connections = 20  # CPU核数 * 2 + 磁盘数
min_connections = 2   # 预热连接
```

## 性能分析工具

### 1. CPU Profiling (flamegraph)
```bash
# 安装
cargo install flamegraph

# 运行分析
cargo flamegraph --root --bench api_bench

# 附着到运行进程
sudo flamegraph -o flamegraph.svg -p <PID>
```

### 2. 内存分析 (massif)
```bash
# 安装 valgrind
apt-get install valgrind

# 运行分析
valgrind --tool=massif target/release/plog-api

# 查看结果
ms_print massif.out.<pid>
```

### 3. 基准测试 (criterion)
```bash
# 运行所有基准
cargo bench

# 运行特定基准
cargo bench --bench api_bench -- "api_response"

# 生成 HTML 报告
cargo bench -- --save-baseline new
```

### 4. 内存泄漏检测
```bash
# 使用 valgrind
valgrind --leak-check=full --show-leak-kinds=all target/release/plog-api

# 使用 AddressSanitizer
RUSTFLAGS="-Zsanitizer=address" cargo run --target x86_64-unknown-linux-gnu
```

## 性能指标监控

### 关键指标
| 指标 | 健康值 | 告警阈值 |
|------|--------|----------|
| P99 延迟 | <100ms | >500ms |
| 错误率 | <1% | >5% |
| 数据库慢查询 | <10/min | >100/min |
| 连接池使用率 | <80% | >90% |

### 监控命令
```bash
# 实时请求统计
watch -n 1 'curl -s http://localhost:3000/health | jq'

# 数据库连接数
mysql -e "SHOW STATUS LIKE 'Threads_connected'"

# 进程内存
ps aux | grep plog-api | awk '{print $6}'
```

## 性能调优建议

### 1. 数据库
```sql
-- 推荐索引
CREATE INDEX idx_posts_date ON posts(date DESC);
CREATE INDEX idx_posts_sortid ON posts(sortid);
CREATE INDEX idx_posts_hide_date ON posts(hide, date DESC);

-- 查询分析
EXPLAIN ANALYZE SELECT * FROM posts WHERE hide = 'n' ORDER BY date DESC LIMIT 20;
```

### 2. 连接池公式
```
max_connections = CPU_cores * 2 + disk_count

示例:
- 8 核 CPU, 1 磁盘: max_connections = 8 * 2 + 1 = 17
- 16 核 CPU, 2 磁盘: max_connections = 16 * 2 + 2 = 34
```

### 3. Tokio 运行时
```bash
# 多运行时配置 (高级)
# 默认: 单运行时，核心数自动检测

# 自定义工作线程数
TOKIO_WORKER_THREADS=8 cargo run --release
```

### 4. 内存优化
```bash
# 减少 TLS 开销
export MALLOC_CONF="tcache:false"

# 系统级优化
echo 1 > /proc/sys/net/core/somaxconn
```

## 常见性能问题

### 问题 1: 慢查询
```
症状: P99 延迟 >500ms
诊断: 查看 sqlx=debug 日志
解决: 添加索引 / 优化查询
```

### 问题 2: 连接池耗尽
```
症状: 连接超时错误
诊断: 检查 max_connections 使用率
解决: 增加 max_connections 或减少连接持有时间
```

### 问题 3: 内存泄漏
```
症状: 内存持续增长
诊断: valgrind massif 分析
解决: 检查 DashMap 清理 / Arc 循环引用
```

### 问题 4: CPU 热路径
```
症状: CPU 使用率高
诊断: flamegraph 分析
解决: 优化序列化 / 减少 clone
```

## 性能回归测试

### 自动化脚本
```bash
#!/bin/bash
# performance_check.sh

# 基准测试
cargo bench --bench api_bench > bench_results.txt

# 与上次对比
if [ -f bench_baseline.txt ]; then
    cargo bench --bench api_bench -- --baseline baseline
fi

# 保存本次基准
cp bench_results.txt bench_baseline.txt

# 火焰图
cargo flamegraph --root -o flamegraph.svg --bench api_bench

echo "Performance check complete"
```

### CI 集成
```yaml
# .github/workflows/performance.yml
- name: Performance Benchmark
  run: |
    cargo bench --bench api_bench
    # 检查性能回归
    # P99 > 100ms 则失败
```
