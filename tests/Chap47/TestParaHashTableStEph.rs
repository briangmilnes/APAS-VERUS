//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::marker::PhantomData;
use std::rc::Rc;

use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::VecChainedHashTableStEph::VecChainedHashTableStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_createtable() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|k| *k as N));
    let table: HashTable<i32, String, Vec<(i32, String)>, ()> =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(
            hash_fn_gen,
            10,
        );
    assert_eq!(table.initial_size, 10);
    assert_eq!(table.current_size, 10);
    assert_eq!(table.num_elements, 0);
}

#[test]
fn test_loadandsize_empty() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|k| *k as N));
    let hash_fn: HashFun<i32> = hash_fn_gen(10);
    let table: HashTable<i32, String, Vec<(i32, String)>, ()> = HashTable {
        table: Vec::new(),
        hash_fn_gen,
        hash_fn,
        initial_size: 10,
        current_size: 10,
        num_elements: 0,
        metrics: (),
        _phantom: PhantomData,
    };
    let load_size =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::loadAndSize(&table);
    assert_eq!(load_size.load, 0.0);
    assert_eq!(load_size.size, 10);
}

#[test]
fn test_metrics() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|k| *k as N));
    let hash_fn: HashFun<i32> = hash_fn_gen(10);
    let table: HashTable<i32, String, Vec<(i32, String)>, ()> = HashTable {
        table: Vec::new(),
        hash_fn_gen,
        hash_fn,
        initial_size: 10,
        current_size: 10,
        num_elements: 0,
        metrics: (),
        _phantom: PhantomData,
    };
    let _metrics =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::metrics(&table);
}

#[test]
fn test_loadandsize_with_elements() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|_size| Box::new(|k| *k as N));
    let hash_fn: HashFun<i32> = hash_fn_gen(10);
    let table: HashTable<i32, String, Vec<(i32, String)>, ()> = HashTable {
        table: Vec::new(),
        hash_fn_gen,
        hash_fn,
        initial_size: 10,
        current_size: 10,
        num_elements: 5,
        metrics: (),
        _phantom: PhantomData,
    };
    let load_size =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::loadAndSize(&table);
    assert_eq!(load_size.load, 0.5);
    assert_eq!(load_size.size, 10);
}
