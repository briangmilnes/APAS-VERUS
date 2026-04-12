//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 49: Minimum edit distance (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap49::MinEditDistStPer::MinEditDistStPer::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

fn bench_min_edit_dist_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("MinEditDistStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            let source = ArraySeqStPerS { seq: (0..n as u8).collect() };
            let target = ArraySeqStPerS { seq: (10..10 + n as u8).collect() };
            let solver = MinEditDistStPerS::<u8>::from_sequences(source, target);
            b.iter(|| solver.min_edit_distance());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_min_edit_dist_per);
criterion_main!(benches);
