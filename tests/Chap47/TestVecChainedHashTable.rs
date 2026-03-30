//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use vstd::prelude::Ghost;
use apas_verus::Chap47::ChainedHashTable::ChainedHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::VecChainedHashTableStEph::VecChainedHashTableStEph::*;
use apas_verus::Types::Types::*;

type HashFn = fn(&i32, usize) -> usize;
type VecChainTable = HashTable<i32, String, Vec<(i32, String)>, (), HashFn>;

fn mod_hash(k: &i32, size: usize) -> usize { (*k as usize) % size }

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
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
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
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
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
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            4,
            Ghost::assume_new(),
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
    let table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );

    // hash_index is used internally but we can verify it compiles and doesn't panic
    let index = VecChainedHashTableStEph::hash_index(&table, &5);
    assert!(index < 10); // Should be within table size
}

#[test]
fn test_vec_entry_multiple_inserts() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    for i in 0..10 {
        EntryTrait::insert(&mut entry, i, format!("val_{i}"));
    }
    assert_eq!(entry.len(), 10);
    for i in 0..10 {
        assert_eq!(EntryTrait::lookup(&entry, &i), Some(format!("val_{i}")));
    }
}

#[test]
fn test_vec_entry_delete_all() {
    let mut entry: Vec<(i32, String)> = EntryTrait::new();
    for i in 0..5 {
        EntryTrait::insert(&mut entry, i, format!("val_{i}"));
    }
    for i in 0..5 {
        assert!(EntryTrait::delete(&mut entry, &i));
    }
    assert!(entry.is_empty());
}

#[test]
fn test_vec_chained_many_collisions() {
    // Use a hash function that forces collisions (all keys hash to same bucket).
    fn always_zero(_k: &i32, _size: usize) -> usize { 0 }

    let mut table: HashTable<i32, String, Vec<(i32, String)>, (), fn(&i32, usize) -> usize> =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), fn(&i32, usize) -> usize>>::createTable(
            always_zero,
            4,
            Ghost::assume_new(),
        );
    for _ in 0..4 {
        table.table.push(Vec::new());
    }

    for i in 0..20 {
        VecChainedHashTableStEph::insert(&mut table, i, format!("v{i}"));
    }

    // All 20 elements should be findable despite collisions.
    for i in 0..20 {
        assert_eq!(VecChainedHashTableStEph::lookup(&table, &i), Some(format!("v{i}")));
    }
}

#[test]
fn test_vec_chained_insert_then_delete_then_lookup() {
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );
    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    VecChainedHashTableStEph::insert(&mut table, 5, "five".to_string());
    VecChainedHashTableStEph::insert(&mut table, 15, "fifteen".to_string());
    assert!(VecChainedHashTableStEph::delete(&mut table, &5));
    assert_eq!(VecChainedHashTableStEph::lookup(&table, &5), None);
    assert_eq!(VecChainedHashTableStEph::lookup(&table, &15), Some("fifteen".to_string()));
}

#[test]
fn test_vec_chained_overwrite_value() {
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );
    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    VecChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    VecChainedHashTableStEph::insert(&mut table, 1, "ONE".to_string());
    assert_eq!(VecChainedHashTableStEph::lookup(&table, &1), Some("ONE".to_string()));
}

#[test]
fn test_vec_chained_lookup_nonexistent() {
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );
    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    assert_eq!(VecChainedHashTableStEph::lookup(&table, &999), None);
}

#[test]
fn test_vec_chained_delete_nonexistent() {
    let mut table: VecChainTable =
        <VecChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, Vec<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            10,
            Ghost::assume_new(),
        );
    for _ in 0..10 {
        table.table.push(Vec::new());
    }

    assert!(!VecChainedHashTableStEph::delete(&mut table, &999));
}
