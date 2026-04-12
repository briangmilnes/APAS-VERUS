//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 51: Bottom-up DP (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap51::BottomUpDPMtEph::BottomUpDPMtEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_bottom_up_dp_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BottomUpDPMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let s: Vec<char> = (0..n).map(|i| (b'a' + (i % 26) as u8) as char).collect();
                    let t: Vec<char> = (0..n).map(|i| (b'b' + (i % 26) as u8) as char).collect();
                    BottomUpDPMtEphS::new(
                        ArraySeqMtEphS { seq: s },
                        ArraySeqMtEphS { seq: t },
                    )
                },
                |mut dp| dp.med_bottom_up_parallel(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bottom_up_dp_mt);
criterion_main!(benches);
