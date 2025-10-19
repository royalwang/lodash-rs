use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_lodash::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn generate_test_data(size: usize) -> Vec<i32> {
    let mut data: Vec<i32> = (1..=size as i32).collect();
    data.shuffle(&mut thread_rng());
    data
}

fn bench_map(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("map_1000", |b| {
        b.iter(|| {
            let result = map(black_box(&data), |x| x * 2);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("map_10000", |b| {
        b.iter(|| {
            let result = map(black_box(&data), |x| x * 2);
            black_box(result)
        })
    });
}

fn bench_filter(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("filter_1000", |b| {
        b.iter(|| {
            let result = filter(black_box(&data), |x| x % 2 == 0);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("filter_10000", |b| {
        b.iter(|| {
            let result = filter(black_box(&data), |x| x % 2 == 0);
            black_box(result)
        })
    });
}

fn bench_reduce(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("reduce_1000", |b| {
        b.iter(|| {
            let result = reduce(black_box(&data), |acc, x| acc + x, 0);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("reduce_10000", |b| {
        b.iter(|| {
            let result = reduce(black_box(&data), |acc, x| acc + x, 0);
            black_box(result)
        })
    });
}

fn bench_find(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("find_1000", |b| {
        b.iter(|| {
            let result = find(black_box(&data), |x| *x > 500);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("find_10000", |b| {
        b.iter(|| {
            let result = find(black_box(&data), |x| *x > 5000);
            black_box(result)
        })
    });
}

fn bench_group_by(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("group_by_1000", |b| {
        b.iter(|| {
            let result = group_by(black_box(&data), |x| x % 10);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("group_by_10000", |b| {
        b.iter(|| {
            let result = group_by(black_box(&data), |x| x % 10);
            black_box(result)
        })
    });
}

fn bench_sort_by(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("sort_by_1000", |b| {
        b.iter(|| {
            let result = sort_by(black_box(&data), |x| *x);
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("sort_by_10000", |b| {
        b.iter(|| {
            let result = sort_by(black_box(&data), |x| *x);
            black_box(result)
        })
    });
}

fn bench_chain(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("chain_1000", |b| {
        b.iter(|| {
            let result = chain(black_box(&data))
                .filter(|x| x % 2 == 0)
                .map(|x| x * 3)
                .take(100)
                .collect();
            black_box(result)
        })
    });
    
    let data = generate_test_data(10000);
    c.bench_function("chain_10000", |b| {
        b.iter(|| {
            let result = chain(black_box(&data))
                .filter(|x| x % 2 == 0)
                .map(|x| x * 3)
                .take(1000)
                .collect();
            black_box(result)
        })
    });
}

fn bench_collection_vs_std(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    // lodash-rs map
    c.bench_function("lodash_map_1000", |b| {
        b.iter(|| {
            let result = map(black_box(&data), |x| x * 2);
            black_box(result)
        })
    });
    
    // std iterator map
    c.bench_function("std_map_1000", |b| {
        b.iter(|| {
            let result: Vec<i32> = black_box(&data).iter().map(|x| x * 2).collect();
            black_box(result)
        })
    });
    
    // lodash-rs filter
    c.bench_function("lodash_filter_1000", |b| {
        b.iter(|| {
            let result = filter(black_box(&data), |x| x % 2 == 0);
            black_box(result)
        })
    });
    
    // std iterator filter
    c.bench_function("std_filter_1000", |b| {
        b.iter(|| {
            let result: Vec<i32> = black_box(&data).iter().filter(|x| *x % 2 == 0).cloned().collect();
            black_box(result)
        })
    });
}

fn bench_memory_usage(c: &mut Criterion) {
    let data = generate_test_data(10000);
    
    c.bench_function("memory_efficient_chain", |b| {
        b.iter(|| {
            let result = chain(black_box(&data))
                .filter(|x| x % 2 == 0)
                .map(|x| x * 2)
                .collect();
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    bench_map,
    bench_filter,
    bench_reduce,
    bench_find,
    bench_group_by,
    bench_sort_by,
    bench_chain,
    bench_collection_vs_std,
    bench_memory_usage
);

criterion_main!(benches);
