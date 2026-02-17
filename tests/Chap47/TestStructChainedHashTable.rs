//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap47::ChainedHashTable::ChainedHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;
use apas_verus::Chap47::StructChainedHashTable::StructChainedHashTable::*;
use apas_verus::Types::Types::*;
use std::rc::Rc;

type StructChainTable = HashTable<i32, String, ChainList<i32, String>, ()>;

#[test]
fn test_chainlist_new() {
    let list = ChainList::<i32, String>::new();
    assert!(list.head.is_none());
}

#[test]
fn test_chainlist_insert() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("one".to_string()));
}

#[test]
fn test_chainlist_insert_multiple() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 2, "two".to_string());
    EntryTrait::insert(&mut list, 3, "three".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("one".to_string()));
    assert_eq!(EntryTrait::lookup(&list, &2), Some("two".to_string()));
    assert_eq!(EntryTrait::lookup(&list, &3), Some("three".to_string()));
}

#[test]
fn test_chainlist_update() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 1, "ONE".to_string());
    assert_eq!(EntryTrait::lookup(&list, &1), Some("ONE".to_string()));
}

#[test]
fn test_chainlist_delete() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    EntryTrait::insert(&mut list, 2, "two".to_string());
    assert!(EntryTrait::delete(&mut list, &1));
    assert_eq!(EntryTrait::lookup(&list, &1), None);
    assert_eq!(EntryTrait::lookup(&list, &2), Some("two".to_string()));
}

#[test]
fn test_chainlist_delete_not_found() {
    let mut list: ChainList<i32, String> = EntryTrait::new();
    EntryTrait::insert(&mut list, 1, "one".to_string());
    assert!(!EntryTrait::delete(&mut list, &999));
}

#[test]
fn test_struct_chained_insert_lookup() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: StructChainTable =
        <StructChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, ChainList<i32, String>, ()>>::createTable(
            hash_fn_gen,
            10,
        );

    for _ in 0..10 {
        table.table.push(ChainList::new());
    }

    StructChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    StructChainedHashTableStEph::insert(&mut table, 11, "eleven".to_string());

    assert_eq!(StructChainedHashTableStEph::lookup(&table, &1), Some("one".to_string()));
    assert_eq!(
        StructChainedHashTableStEph::lookup(&table, &11),
        Some("eleven".to_string())
    );
}

#[test]
fn test_struct_chained_delete() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: StructChainTable =
        <StructChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, ChainList<i32, String>, ()>>::createTable(
            hash_fn_gen,
            10,
        );

    for _ in 0..10 {
        table.table.push(ChainList::new());
    }

    StructChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    assert!(StructChainedHashTableStEph::delete(&mut table, &1));
    assert_eq!(StructChainedHashTableStEph::lookup(&table, &1), None);
}

#[test]
fn test_chainlist_default() {
    let list: ChainList<i32, String> = Default::default();
    assert!(list.head.is_none());
    assert_eq!(EntryTrait::lookup(&list, &1), None);
}

#[test]
fn test_struct_chained_resize() {
    let hash_fn_gen: HashFunGen<i32> = Rc::new(|size| Box::new(move |k| (*k as N) % size));
    let mut table: StructChainTable =
        <StructChainedHashTableStEph as ParaHashTableStEphTrait<i32, String, ChainList<i32, String>, ()>>::createTable(
            hash_fn_gen.clone(),
            4,
        );

    for _ in 0..4 {
        table.table.push(ChainList::new());
    }

    // Insert some values
    StructChainedHashTableStEph::insert(&mut table, 1, "one".to_string());
    StructChainedHashTableStEph::insert(&mut table, 5, "five".to_string());
    StructChainedHashTableStEph::insert(&mut table, 9, "nine".to_string());

    // Resize to larger table
    let resized_table = StructChainedHashTableStEph::resize(&table, 8);

    // Verify values still accessible in resized table
    assert_eq!(StructChainedHashTableStEph::lookup(&resized_table, &1), Some("one".to_string()));
    assert_eq!(StructChainedHashTableStEph::lookup(&resized_table, &5), Some("five".to_string()));
    assert_eq!(StructChainedHashTableStEph::lookup(&resized_table, &9), Some("nine".to_string()));
}

#[test]
fn test_node_clone() {
    let node = Node {
        key: 1,
        value: "one".to_string(),
        next: None,
    };
    let cloned = node.clone();
    assert_eq!(cloned.key, 1);
    assert_eq!(cloned.value, "one");
    assert!(cloned.next.is_none());
}
