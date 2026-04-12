//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 05: Kleene closure (persistent) benchmark — mem_star and mem_plus.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::KleeneStPer::KleeneStPer::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;

fn build_kleene(n: usize) -> KleeneStPer<u64> {
    let mut alpha = SetStEph::<u64>::empty();
    for i in 0..n as u64 { alpha.insert(i); }
    KleeneStPer::<u64>::new(alpha)
}

fn bench_mem_star(c: &mut Criterion) {
    let mut group = c.benchmark_group("KleeneMemStar");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 64] {
        let k = build_kleene(n);
        let s: Vec<u64> = (0..n as u64).collect();
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| k.mem_star(&s));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mem_star);
criterion_main!(benches);
