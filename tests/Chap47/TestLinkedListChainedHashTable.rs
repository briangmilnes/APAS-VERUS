// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
use vstd::prelude::Ghost;
use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;
use apas_verus::Chap47::ChainedHashTable::ChainedHashTable::*;
use apas_verus::Chap47::LinkedListChainedHashTableStEph::LinkedListChainedHashTableStEph::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&i32, usize) -> usize;
type LLChainTable = HashTable<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>;

fn mod_hash(k: &i32, size: usize) -> usize { (*k as usize) % size }

#[test]
fn test_linkedlist_entry_new() {
    let entry: LinkedListStEphS<(i32, String)> = <LinkedListStEphS<(i32, String)> as EntryTrait<i32, String>>::new();
    assert!(entry.seq.is_empty());
}

#[test]
fn test_linkedlist_entry_insert() {
    let mut entry: LinkedListStEphS<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert_eq!(entry.seq.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("one".to_string()));
}

#[test]
fn test_linkedlist_entry_update() {
    let mut entry: LinkedListStEphS<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 1, "ONE".to_string());
    assert_eq!(entry.seq.len(), 1);
    assert_eq!(EntryTrait::lookup(&entry, &1), Some("ONE".to_string()));
}

#[test]
fn test_linkedlist_entry_delete() {
    let mut entry: LinkedListStEphS<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    EntryTrait::insert(&mut entry, 2, "two".to_string());
    assert!(EntryTrait::delete(&mut entry, &1));
    assert_eq!(EntryTrait::lookup(&entry, &1), None);
    assert_eq!(EntryTrait::lookup(&entry, &2), Some("two".to_string()));
}

#[test]
fn test_linkedlist_chained_insert_lookup() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());

    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &0),
        Some("zero".to_string())
    );
    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &1),
        Some("one".to_string())
    );
}

#[test]
fn test_linkedlist_chained_delete() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(LinkedListChainedHashTableStEph::delete(&mut table, &1));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &1), None);
    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &0),
        Some("zero".to_string())
    );
}

#[test]
fn test_collision_handling() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 2, "two".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 4, "four".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 3, "three".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 5, "five".to_string());

    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &0), Some("zero".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &2), Some("two".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &4), Some("four".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &3), Some("three".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &5), Some("five".to_string()));

    assert!(LinkedListChainedHashTableStEph::delete(&mut table, &2));
    assert!(LinkedListChainedHashTableStEph::delete(&mut table, &3));

    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &0), Some("zero".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &2), None);
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &4), Some("four".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &3), None);
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&table, &5), Some("five".to_string()));
}

#[test]
fn test_entry_delete_not_found() {
    let mut entry: LinkedListStEphS<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert!(!EntryTrait::delete(&mut entry, &999));
}

#[test]
fn test_entry_lookup_not_found() {
    let mut entry: LinkedListStEphS<(i32, String)> = EntryTrait::new();
    EntryTrait::insert(&mut entry, 1, "one".to_string());
    assert_eq!(EntryTrait::lookup(&entry, &999), None);
}

#[test]
fn test_resize_empty_table() {
    let table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    let new_table = LinkedListChainedHashTableStEph::resize(&table, 4);
    assert_eq!(new_table.current_size, 4);
    assert_eq!(new_table.num_elements, 0);
}

#[test]
fn test_resize_with_elements() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 2, "two".to_string());

    let new_table = LinkedListChainedHashTableStEph::resize(&table, 4);
    assert_eq!(new_table.current_size, 4);

    assert_eq!(LinkedListChainedHashTableStEph::lookup(&new_table, &0), Some("zero".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&new_table, &1), Some("one".to_string()));
    assert_eq!(LinkedListChainedHashTableStEph::lookup(&new_table, &2), Some("two".to_string()));
}

#[test]
fn test_load_and_size() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    let result = LinkedListChainedHashTableStEph::loadAndSize(&table);
    assert_eq!(result.size, 2);

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    let result = LinkedListChainedHashTableStEph::loadAndSize(&table);
    assert_eq!(result.size, 2);

    LinkedListChainedHashTableStEph::delete(&mut table, &0);
    let result = LinkedListChainedHashTableStEph::loadAndSize(&table);
    assert_eq!(result.size, 2);
}

#[test]
fn test_update_existing_key() {
    let mut table: LLChainTable =
        <LinkedListChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, LinkedListStEphS<(i32, String)>, (), HashFn>>::createTable(
            mod_hash,
            2,
            Ghost::assume_new(),
        );

    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zero".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 0, "ZERO".to_string());
    LinkedListChainedHashTableStEph::insert(&mut table, 0, "zer0".to_string());

    assert_eq!(
        LinkedListChainedHashTableStEph::lookup(&table, &0),
        Some("zer0".to_string())
    );
}
