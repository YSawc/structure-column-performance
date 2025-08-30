use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reqwest;
use serde_json::json;
use std::time::Duration;
use tokio::runtime::Runtime;

fn benchmark_column_storage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("column_storage_1000", |b| {
        b.to_async(&rt).iter(|| async {
            let client = reqwest::Client::new();
            let response = client
                .get("http://localhost:3000/benchmark/column/1000")
                .send()
                .await
                .unwrap();
            black_box(response.json::<serde_json::Value>().await.unwrap());
        });
    });
}

fn benchmark_json_storage(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("json_storage_1000", |b| {
        b.to_async(&rt).iter(|| async {
            let client = reqwest::Client::new();
            let response = client
                .get("http://localhost:3000/benchmark/json/1000")
                .send()
                .await
                .unwrap();
            black_box(response.json::<serde_json::Value>().await.unwrap());
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = benchmark_column_storage, benchmark_json_storage
);
criterion_main!(benches);