// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 55: Cycle detection (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap55::CycleDetectStEph::CycleDetectStEph::*;

fn make_dag(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    let adj: Vec<ArraySeqStEphS<usize>> = (0..n)
        .map(|i| {
            if i + 1 < n {
                ArraySeqStEphS::from_vec(vec![i + 1])
            } else {
                ArraySeqStEphS::from_vec(vec![])
            }
        })
        .collect();
    ArraySeqStEphS::from_vec(adj)
}

fn bench_cycle_detect(c: &mut Criterion) {
    let mut group = c.benchmark_group("CycleDetectStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("dag_n", n), &n, |b, &n| {
            let graph = make_dag(n);
            b.iter(|| CycleDetectStEph::has_cycle(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_cycle_detect);
criterion_main!(benches);
