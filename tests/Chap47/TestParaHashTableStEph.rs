//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::marker::PhantomData;

use vstd::prelude::Ghost;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::VecChainedHashTableStEph::VecChainedHashTableStEph::*;
use apas_verus::Types::Types::*;

type HashFn = fn(&i32, usize) -> usize;
type VecChainTable = HashTable<i32, String, Vec<(i32, String)>, (), HashFn>;

fn mod_hash(k: &i32, size: usize) -> usize { (*k as usize) % size }

#[test]
fn test_createtable() {
    let table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );
    assert_eq!(table.initial_size, 10);
    assert_eq!(table.current_size, 10);
    assert_eq!(table.num_elements, 0);
}

#[test]
fn test_loadandsize_empty() {
    let table: VecChainTable = HashTable {
        table: Vec::new(),
        hash_fn: mod_hash as HashFn,
        initial_size: 10,
        current_size: 10,
        num_elements: 0,
        metrics: (),
        spec_hash: Ghost::assume_new(),
        _phantom: PhantomData,
    };
    let load_size =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::loadAndSize(&table);
    assert_eq!(load_size.load, 0); // 0 elements
    assert_eq!(load_size.size, 10);
}

#[test]
fn test_metrics() {
    let table: VecChainTable = HashTable {
        table: Vec::new(),
        hash_fn: mod_hash as HashFn,
        initial_size: 10,
        current_size: 10,
        num_elements: 0,
        metrics: (),
        spec_hash: Ghost::assume_new(),
        _phantom: PhantomData,
    };
    let _metrics =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::metrics(&table);
}

#[test]
fn test_loadandsize_with_elements() {
    let table: VecChainTable = HashTable {
        table: Vec::new(),
        hash_fn: mod_hash as HashFn,
        initial_size: 10,
        current_size: 10,
        num_elements: 5,
        metrics: (),
        spec_hash: Ghost::assume_new(),
        _phantom: PhantomData,
    };
    let load_size =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::loadAndSize(&table);
    assert_eq!(load_size.load, 5); // 5 elements, α = 5/10 = 0.5
    assert_eq!(load_size.size, 10);
}
