//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap47::ChainedHashTable::ChainedHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::VecChainedHashTableStEph::VecChainedHashTableStEph::*;
use apas_verus::Types::Types::*;
use std::rc::Rc;

type VecChainTable = HashTable<i32, String, Vec<(i32, String)>, ()>;

#[test]
fn test_vec_entry_new() {
    let entry: Vec<(i32, String)> = <Vec<(i32, String)> as EntryTrait<i32, String>>::new();
    assert!(entry.is_empty());
}

#[test]
fn test_vec_entry_insert() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert_eq!(entry.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("one".to_string()));
}

#[test]
fn test_vec_entry_update() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 1, "ONE".to_string());
    assert_eq!(entry.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("ONE".to_string()));
}

#[test]
fn test_vec_entry_delete() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 2, "two".to_string());
    assert!(EntryTrait::delete(&mut entry, &1));
    assert_eq!(EntryTrait::lookup(&entry, &1), None);
    assert_eq!(EntryTrait::lookup(&entry, &2), Some("two".to_string()));
}

#[test]
fn test_vec_chained_insert_lookup() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(
            hash_fn_gen,
            10,
        );

    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    VecChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    VecChainedHashTableStEph::insert(&mut table, 11, "eleven".to_string());

    assert_eq!(VecChainedHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(
        VecChainedHashTableStEph::lookup(&table, &11),
        Some("eleven".to_string())
    );
}

#[test]
fn test_vec_chained_delete() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(
            hash_fn_gen,
            10,
        );

    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    VecChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(VecChainedHashTableStEph::delete(&mut table, &1));
    assert_eq!(VecChainedHashTableStEph::lookup(&table, &1), None);
}

#[test]
fn test_vec_entry_delete_nonexistent() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert!(!EntryTrait::delete(&mut entry, &999)); // Delete non-existent key
    assert_eq!(entry.len(), 1); // Original entry still there
}

#[test]
fn test_vec_chained_resize() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(
            hash_fn_gen.clone(),
            4,
        );

    for _ in 0..4 {
        table.table.push(Vec::new());
    }

    // Insert some values
    VecChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    VecChainedHashTableStEph::insert(&mut table, 5, "five".to_string());
    VecChainedHashTableStEph::insert(&mut table, 9, "nine".to_string());

    // Resize to larger table
    let resized_table = VecChainedHashTableStEph::resize(&table, 8);

    // Verify values still accessible in resized table
    assert_eq!(VecChainedHashTableStEph::lookup(&resized_table, &1), Some("one".to_string()));
    assert_eq!(VecChainedHashTableStEph::lookup(&resized_table, &5), Some("five".to_string()));
    assert_eq!(VecChainedHashTableStEph::lookup(&resized_table, &9), Some("nine".to_string()));
    assert_eq!(resized_table.current_size, 8);
}

#[test]
fn test_vec_chained_hash_index() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, ()>>::createTable(
            hash_fn_gen,
            10,
        );

    // hash_index is used internally but we can verify it compiles and doesn't panic
    let index = VecChainedHashTableStEph::hash_index(&table, &5);
    assert!(index < 10); // Should be within table size
}
