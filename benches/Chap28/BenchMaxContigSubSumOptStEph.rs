//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 28: Maximum contiguous subarray sum (Kadane's algorithm) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap28::MaxContigSubSumOptStEph::MaxContigSubSumOptStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn build_alternating(n: usize) -> ArraySeqStEphS<i32> {
    let seq: Vec<i32> = (0..n).map(|i| if i % 2 == 0 { 1i32 } else { -1i32 }).collect();
    ArraySeqStEphS { seq }
}

fn bench_max_contig_sub_sum_opt(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaxContigSubSumOpt");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[256usize, 1024] {
        let arr = build_alternating(n);
        group.bench_with_input(BenchmarkId::new("alternating", n), &n, |b, _| {
            b.iter(|| ArraySeqStEphS::<i32>::max_contig_sub_sum_opt(&arr));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_max_contig_sub_sum_opt);
criterion_main!(benches);
