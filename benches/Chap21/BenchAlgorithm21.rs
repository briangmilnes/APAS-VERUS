//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 21: Algorithm 21.1 (2D tabulate), 21.2 (3D tabulate),
//!             21.5 (brute-force primes), 21.6 (prime sieve) benchmarks.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap21::Algorithm21_1::Algorithm21_1::*;
use apas_verus::Chap21::Algorithm21_2::Algorithm21_2::*;
use apas_verus::Chap21::Algorithm21_5::Algorithm21_5::*;
use apas_verus::Chap21::Algorithm21_6::Algorithm21_6::*;

fn bench_points2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Points2DTabFlat");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| points2d_tab_flat(n));
        });
    }
    group.finish();
}

fn bench_points3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("Points3DTabFlat");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[8usize, 16] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| points3d_tab_flat(n));
        });
    }
    group.finish();
}

fn bench_primes_bf(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimesBF");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[100usize, 500] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| primes_bf(n));
        });
    }
    group.finish();
}

fn bench_prime_sieve(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimeSieve");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[1000usize, 5000] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| prime_sieve(n));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_points2d, bench_points3d, bench_primes_bf, bench_prime_sieve);
criterion_main!(benches);
