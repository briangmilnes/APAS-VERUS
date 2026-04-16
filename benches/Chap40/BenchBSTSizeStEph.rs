// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 40: Size-augmented BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap40::BSTSizeStEph::BSTSizeStEph::*;

fn bench_bstsize_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTSizeInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut tree = <BSTSizeStEph<u64> as BSTSizeStEphTrait<u64>>::new();
                for i in 0..n as u64 { tree.insert(i, i * 6364136223846793005); }
                tree
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstsize_insert);
criterion_main!(benches);
