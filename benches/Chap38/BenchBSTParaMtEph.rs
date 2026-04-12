//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 38: Para BST (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap38::BSTParaMtEph::BSTParaMtEph::*;

fn bench_bstpara_mt_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || ParamBST::<i32>::new(),
                |mut tree| {
                    for i in 0..n as i32 { tree.insert(i); }
                    tree
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstpara_mt_insert);
criterion_main!(benches);
