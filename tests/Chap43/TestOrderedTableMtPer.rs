//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap43 OrderedTableMtPer.

use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEphTrait;
use apas_verus::Chap43::OrderedTableMtPer::OrderedTableMtPer::*;
use apas_verus::Types::Types::*;

#[test]
fn test_ordered_table_mt_per_new() {
    let table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    assert_eq!(table.size(), 0);
}

#[test]
fn test_ordered_table_mt_per_insert_and_find() {
    let table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    let table = table.insert(1, "one".to_string());
    let table = table.insert(2, "two".to_string());

    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), None);
}

#[test]
fn test_ordered_table_mt_per_delete() {
    let table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    let table = table.insert(1, "one".to_string());
    let table = table.insert(2, "two".to_string());

    let table = table.delete(&1);
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&1), None);
    assert_eq!(table.find(&2), Some("two".to_string()));
}

#[test]
fn test_ordered_table_mt_per_filter() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();

    for i in 0..20 {
        table = table.insert(i, format!("value_{i}"));
    }

    // filter takes a predicate on Pair<K, V>
    let filtered = table.filter(|pair: &Pair<i32, String>| pair.0 % 2 == 0);
    assert_eq!(filtered.size(), 10);
}

#[test]
fn test_ordered_table_mt_per_map() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();

    for i in 0..10 {
        table = table.insert(i, format!("val_{i}"));
    }

    // map transforms values: append "_mapped" to each value
    let mapped = table.map(|_k: &i32, v: &String| format!("{v}_mapped"));
    assert_eq!(mapped.size(), 10);
    assert_eq!(mapped.find(&3), Some("val_3_mapped".to_string()));
}

#[test]
fn test_ordered_table_mt_per_singleton() {
    let table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::singleton(42, "answer".to_string());
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&42), Some("answer".to_string()));
}

#[test]
fn test_ordered_table_mt_per_domain() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();

    for i in [5, 2, 8] {
        table = table.insert(i, format!("val_{i}"));
    }

    let domain = table.domain();
    assert_eq!(domain.size(), 3);
}

#[test]
fn test_ordered_table_mt_per_persistence() {
    let table1: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    let table2 = table1.insert(1, "one".to_string());
    let table3 = table2.insert(2, "two".to_string());

    assert_eq!(table1.size(), 0);
    assert_eq!(table2.size(), 1);
    assert_eq!(table3.size(), 2);
}
