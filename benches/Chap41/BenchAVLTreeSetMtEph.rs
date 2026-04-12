//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 41: AVL tree set (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_avlset_mt_eph_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("AVLTreeSetMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <AVLTreeSetMtEph<u64> as AVLTreeSetMtEphTrait<u64>>::empty(),
                |mut s| {
                    for i in 0..n as u64 { s.insert(i); }
                    s
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_avlset_mt_eph_insert);
criterion_main!(benches);
