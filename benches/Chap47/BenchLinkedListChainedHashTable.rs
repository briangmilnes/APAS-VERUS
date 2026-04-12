//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Linked-list chained hash table benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vstd::prelude::Ghost;
use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;
use apas_verus::Chap47::LinkedListChainedHashTableStEph::LinkedListChainedHashTableStEph::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&u64, usize) -> usize;
type LLTable = HashTable<u64, u64, LinkedListStEphS<(u64, u64)>, (), HashFn>;

fn hash_fn(k: &u64, size: usize) -> usize {
    (*k as usize) % size
}

fn bench_llchain_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("LLChainHashInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut table: LLTable =
                    <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<u64, u64, LinkedListStEphS<(u64, u64)>, (), HashFn>>::createTable(
                        hash_fn,
                        n,
                        Ghost::assume_new(),
                    );
                for i in 0..n as u64 {
                    LinkedListChainedHashTableStEph::insert(&mut table, i, i);
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_llchain_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("LLChainHashLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut table: LLTable =
            <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<u64, u64, LinkedListStEphS<(u64, u64)>, (), HashFn>>::createTable(
                hash_fn,
                n,
                Ghost::assume_new(),
            );
        for i in 0..n as u64 {
            LinkedListChainedHashTableStEph::insert(&mut table, i, i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| LinkedListChainedHashTableStEph::lookup(&table, &(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_llchain_insert, bench_llchain_lookup);
criterion_main!(benches);
