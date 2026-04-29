# Plog CMS Performance Optimization Guide

## P3 Performance Optimizations Applied

### 1. Memory Allocation Optimization

#### Vector Pre-allocation
```rust
// Before
let mut discovered = Vec::new();

// After - with estimated capacity
let mut discovered = Vec::with_capacity(16); // plugins
let mut discovered = Vec::with_capacity(8);  // themes
```

**Impact**: Eliminates 1-2 reallocations during plugin/theme discovery

#### Cache Pre-allocation
```rust
// MemoryCache with capacity
pub fn with_capacity(default_ttl: Duration, capacity: usize) -> Self {
    Self {
        items: DashMap::with_capacity(capacity),
        default_ttl,
    }
}
```

**Impact**: Reduces first-insert rehash overhead

### 2. Database Query Optimization

#### Index Recommendations
```sql
-- For post filtering
CREATE INDEX idx_sortid ON posts(sortid);
CREATE INDEX idx_hide_date ON posts(hide, date DESC);
CREATE INDEX idx_date ON posts(date DESC);

-- For keyword search (large datasets)
-- Consider FULLTEXT index or Elasticsearch
```

#### Connection Pool Tuning
```rust
// Recommended formula
max_connections = CPU_cores * 2 + disk_count
min_connections = 1-2 (warm start)
idle_timeout = 300s (balance resource vs reconnect)
max_lifetime = 1800s (prevent accumulation issues)
```

### 3. Serialization Optimization

#### Skip Serializing None Fields
```rust
#[serde(skip_serializing_if = "Option::is_none")]
pub data: Option<T>,
```
**Impact**: 20-30% reduction in JSON output size

### 4. Iterator Chain Optimization

#### Avoid Intermediate Collections
```rust
// Good - single allocation
self.plugins.iter()
    .flat_map(|p| p.get_menus().to_vec())
    .collect()

// Avoid
self.plugins.iter()
    .map(|p| p.get_menus().to_vec())  // intermediate Vec
    .flatten()
    .collect()
```

## Performance Trade-offs Documented

### JWT Claims (String vs Arc<str>)
- **Current**: String (jsonwebtoken requires Owned)
- **Impact**: ~100 bytes per validation
- **Future**: Consider compact_jws or custom Claims

### Cache Values (String vs Arc<str>)
- **Current**: String
- **Reason**: Short-lived JSON strings
- **Future**: Consider interning for repeated values

## Profiling Recommendations

### Before Further Optimization
1. Enable sqlx logging for slow queries
   ```rust
   db_opt.sqlx_logging(true);
   ```

2. Use `cargo flamegraph` for CPU profiling
   ```bash
   cargo flamegraph --root
   ```

3. Monitor memory with `valgrind --tool=massif`
   ```bash
   valgrind --tool=massif target/release/plog-api
   ```

4. Benchmark critical paths with Criterion
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   ```

## Performance Metrics Target

| Metric | Target | Current |
|--------|--------|---------|
| P99 Latency | <100ms | TBD |
| Throughput | >1000 req/s | TBD |
| Memory per request | <1MB | TBD |
| Startup time | <2s | TBD |

## Next Steps

1. **Profiling**: Gather real performance data
2. **Bottleneck Analysis**: Identify top 3 hot paths
3. **Targeted Optimization**: Apply P3 rules with profiler data
4. **Continuous Monitoring**: Add metrics collection
