// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 41: AVL tree set (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;

fn bench_avlset_st_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetStPerInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut s = <AVLTreeSetStPer<u64> as AVLTreeSetStPerTrait<u64>>::empty();
                for i in 0..n as u64 { s = s.insert(i); }
                s
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_avlset_st_per_insert);
criterion_main!(benches);
