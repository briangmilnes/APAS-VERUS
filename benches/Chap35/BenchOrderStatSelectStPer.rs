//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 35: Order-statistic selection (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap35::OrderStatSelectStPer::OrderStatSelectStPer::*;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

fn build_array(n: usize) -> ArraySeqStPerS<u64> {
    ArraySeqStPerS { seq: (0..n as u64).rev().collect() }
}

fn bench_select_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderStatSelectStPer");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let k = n / 2;
        group.bench_with_input(BenchmarkId::new("median", n), &n, |b, &n| {
            b.iter_batched(
                || build_array(n),
                |a| <ArraySeqStPerS<u64> as OrderStatSelectStPerTrait<u64>>::select(&a, k),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_select_st_per);
criterion_main!(benches);
