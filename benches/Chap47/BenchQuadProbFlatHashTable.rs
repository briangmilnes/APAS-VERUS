// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 47: Quadratic probing flat hash table benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vstd::prelude::Ghost;
use apas_verus::Chap47::FlatHashTable::FlatHashTable::*;
use apas_verus::Chap47::QuadProbFlatHashTableStEph::QuadProbFlatHashTableStEph::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&u64, usize) -> usize;
type QuadTable = HashTable<u64, u64, FlatEntry<u64, u64>, (), HashFn>;

fn hash_fn(k: &u64, size: usize) -> usize {
    (*k as usize) % size
}

fn bench_quad_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("QuadProbHashInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut table: QuadTable =
                    <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<u64, u64, FlatEntry<u64, u64>, (), HashFn>>::createTable(
                        hash_fn,
                        2 * n,
                        Ghost::assume_new(),
                    );
                for i in 0..n as u64 {
                    QuadProbFlatHashTableStEph::insert(&mut table, i, i);
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_quad_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("QuadProbHashLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let mut table: QuadTable =
            <QuadProbFlatHashTableStEph as ParaHashTableStEphTrait<u64, u64, FlatEntry<u64, u64>, (), HashFn>>::createTable(
                hash_fn,
                2 * n,
                Ghost::assume_new(),
            );
        for i in 0..n as u64 {
            QuadProbFlatHashTableStEph::insert(&mut table, i, i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| QuadProbFlatHashTableStEph::lookup(&table, &(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_quad_insert, bench_quad_lookup);
criterion_main!(benches);
