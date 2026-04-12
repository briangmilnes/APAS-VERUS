//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 51: Bottom-up DP (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap51::BottomUpDPMtPer::BottomUpDPMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;

fn bench_bottom_up_dp_mt_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("BottomUpDPMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            let s: Vec<char> = (0..n).map(|i| (b'a' + (i % 26) as u8) as char).collect();
            let t: Vec<char> = (0..n).map(|i| (b'b' + (i % 26) as u8) as char).collect();
            let dp = BottomUpDPMtPerS::new(
                ArraySeqMtPerS { seq: s },
                ArraySeqMtPerS { seq: t },
            );
            b.iter(|| dp.med_bottom_up_parallel());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bottom_up_dp_mt_per);
criterion_main!(benches);
