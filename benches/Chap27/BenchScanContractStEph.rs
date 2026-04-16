// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 27: Scan by contraction (sequential) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap27::ScanContractStEph::ScanContractStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use vstd::prelude::Ghost;

fn build_seq(n: usize) -> ArraySeqStEphS<usize> {
    ArraySeqStEphS::<usize>::from_vec((1..=n).collect())
}

fn bench_scan_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("ScanContractSum");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| {
                <ArraySeqStEphS<usize> as ScanContractStEphTrait<usize>>::scan_contract(
                    &a,
                    &|x: &usize, y: &usize| x.wrapping_add(*y),
                    Ghost::assume_new(),
                    0usize,
                )
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_scan_sum);
criterion_main!(benches);
