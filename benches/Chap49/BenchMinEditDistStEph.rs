//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum edit distance (DP) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap49::MinEditDistStEph::MinEditDistStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn build_solver(src_len: usize, tgt_len: usize) -> MinEditDistStEphS<u8> {
    let source = ArraySeqStEphS { seq: (0..src_len as u8).collect() };
    let target = ArraySeqStEphS { seq: (10..10 + tgt_len as u8).collect() };
    MinEditDistStEphS::<u8>::from_sequences(source, target)
}

fn bench_min_edit_dist(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinEditDist");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            b.iter_batched(
                || build_solver(n, n),
                |mut solver| solver.min_edit_distance(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_min_edit_dist);
criterion_main!(benches);
