//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Linear probing flat hash table benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vstd::prelude::Ghost;
use apas_verus::Chap47::FlatHashTable::FlatHashTable::*;
use apas_verus::Chap47::LinProbFlatHashTableStEph::LinProbFlatHashTableStEph::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&u64, usize) -> usize;
type LinTable = HashTable<u64, u64, FlatEntry<u64, u64>, (), HashFn>;

fn hash_fn(k: &u64, size: usize) -> usize {
    (*k as usize) % size
}

fn bench_linprob_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinProbHashInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    // Use 2*n capacity to keep load factor < 0.5 for linear probing.
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut table: LinTable =
                    <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<u64, u64, FlatEntry<u64, u64>, (), HashFn>>::createTable(
                        hash_fn,
                        2 * n,
                        Ghost::assume_new(),
                    );
                for i in 0..n as u64 {
                    LinProbFlatHashTableStEph::insert(&mut table, i, i);
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_linprob_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinProbHashLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let mut table: LinTable =
            <LinProbFlatHashTableStEph as ParaHashTableStEphTrait<u64, u64, FlatEntry<u64, u64>, (), HashFn>>::createTable(
                hash_fn,
                2 * n,
                Ghost::assume_new(),
            );
        for i in 0..n as u64 {
            LinProbFlatHashTableStEph::insert(&mut table, i, i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| LinProbFlatHashTableStEph::lookup(&table, &(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_linprob_insert, bench_linprob_lookup);
criterion_main!(benches);
