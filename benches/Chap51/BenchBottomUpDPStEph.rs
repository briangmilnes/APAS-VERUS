// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 51: Bottom-up DP (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap51::BottomUpDPStEph::BottomUpDPStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn bench_bottom_up_dp(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let s: Vec<char> = (0..n).map(|i| (b'a' + (i % 26) as u8) as char).collect();
                    let t: Vec<char> = (0..n).map(|i| (b'b' + (i % 26) as u8) as char).collect();
                    BottomUpDPStEphS::new(
                        ArraySeqStEphS { seq: s },
                        ArraySeqStEphS { seq: t },
                    )
                },
                |mut dp| dp.med_bottom_up(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bottom_up_dp);
criterion_main!(benches);
