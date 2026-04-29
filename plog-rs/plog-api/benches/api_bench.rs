//! API 性能基准测试
//!
//! 运行方式: cargo bench --bench api_bench

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use plog_shared::{ApiResponse, PaginatedData, PaginationMeta};

fn bench_api_response_serialization(c: &mut Criterion) {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    c.bench_function("api_response_ok", |b| {
        b.iter(|| {
            let response = ApiResponse::ok(black_box(data.clone()));
            serde_json::to_string(&response).unwrap()
        })
    });
    
    c.bench_function("api_response_paginated", |b| {
        b.iter(|| {
            let pagination = PaginationMeta::new(1, 20, 100);
            let response = ApiResponse::paginated(
                black_box(data.clone()),
                1,
                20,
                100,
            );
            serde_json::to_string(&response).unwrap()
        })
    });
}

fn bench_pagination_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pagination");
    
    for total in [100, 1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(total), total, |b, total| {
            b.iter(|| {
                let meta = PaginationMeta::new(1, 20, *total);
                black_box(meta.total_pages)
            })
        });
    }
    
    group.finish();
}

fn bench_json_serialization_size(c: &mut Criterion) {
    use serde_json::json;
    
    c.bench_function("small_json", |b| {
        b.iter(|| {
            let value = json!({
                "id": 1,
                "name": "test",
                "active": true
            });
            serde_json::to_string(&value).unwrap()
        })
    });
    
    c.bench_function("medium_json", |b| {
        b.iter(|| {
            let items: Vec<_> = (0..20).map(|i| {
                json!({
                    "id": i,
                    "title": format!("Post {}", i),
                    "content": "Lorem ipsum dolor sit amet",
                    "views": i * 100,
                    "date": "2024-01-01"
                })
            }).collect();
            serde_json::to_string(&items).unwrap()
        })
    });
    
    c.bench_function("large_json", |b| {
        b.iter(|| {
            let items: Vec<_> = (0..100).map(|i| {
                json!({
                    "id": i,
                    "title": format!("Post {}", i),
                    "content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
                    "excerpt": "Short excerpt",
                    "views": i * 100,
                    "likes": i * 10,
                    "comments": i,
                    "date": "2024-01-01",
                    "author": "admin",
                    "tags": ["rust", "performance"]
                })
            }).collect();
            serde_json::to_string(&items).unwrap()
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(5));
    targets = 
        bench_api_response_serialization,
        bench_pagination_calculation,
        bench_json_serialization_size
}

criterion_main!(benches);
