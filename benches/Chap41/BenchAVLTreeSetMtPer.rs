//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 41: AVL tree set (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;

fn bench_avlset_mt_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSetMtPerInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut s = <AVLTreeSetMtPer<u64> as AVLTreeSetMtPerTrait<u64>>::empty();
                for i in 0..n as u64 { s = s.insert(i); }
                s
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_avlset_mt_per_insert);
criterion_main!(benches);
