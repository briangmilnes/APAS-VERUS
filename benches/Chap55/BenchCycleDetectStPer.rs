// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 55: Cycle detection (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap55::CycleDetectStPer::CycleDetectStPer::*;

fn make_dag(n: usize) -> ArraySeqStPerS<ArraySeqStPerS<usize>> {
    let adj: Vec<ArraySeqStPerS<usize>> = (0..n)
        .map(|i| {
            if i + 1 < n {
                ArraySeqStPerS { seq: vec![i + 1] }
            } else {
                ArraySeqStPerS { seq: vec![] }
            }
        })
        .collect();
    ArraySeqStPerS { seq: adj }
}

fn bench_cycle_detect_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("CycleDetectStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("dag_n", n), &n, |b, &n| {
            let graph = make_dag(n);
            b.iter(|| CycleDetectStPer::has_cycle(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_cycle_detect_per);
criterion_main!(benches);
