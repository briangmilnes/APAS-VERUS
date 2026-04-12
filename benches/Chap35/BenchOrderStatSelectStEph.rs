//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 35: Order-statistic selection benchmark (k-th smallest element).

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap35::OrderStatSelectStEph::OrderStatSelectStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn build_array(n: usize) -> ArraySeqStEphS<u64> {
    ArraySeqStEphS { seq: (0..n as u64).rev().collect() }
}

fn bench_select(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrderStatSelect");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[64usize, 256] {
        let k = n / 2;
        group.bench_with_input(BenchmarkId::new("median", n), &n, |b, &n| {
            b.iter_batched(
                || build_array(n),
                |a| ArraySeqStEphS::<u64>::select(&a, k),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_select);
criterion_main!(benches);
