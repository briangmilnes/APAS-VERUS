// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 51: Bottom-up DP (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap51::BottomUpDPStPer::BottomUpDPStPer::*;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

fn bench_bottom_up_dp_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            let s: Vec<char> = (0..n).map(|i| (b'a' + (i % 26) as u8) as char).collect();
            let t: Vec<char> = (0..n).map(|i| (b'b' + (i % 26) as u8) as char).collect();
            let dp = BottomUpDPStPerS::new(
                ArraySeqStPerS { seq: s },
                ArraySeqStPerS { seq: t },
            );
            b.iter(|| dp.med_bottom_up());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bottom_up_dp_per);
criterion_main!(benches);
