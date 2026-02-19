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

#[test]
fn test_ordered_table_mt_per_first_last_key() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    assert_eq!(table.first_key(), None);
    assert_eq!(table.last_key(), None);

    table = table.insert(5, "five".into());
    table = table.insert(2, "two".into());
    table = table.insert(8, "eight".into());

    assert_eq!(table.first_key(), Some(2));
    assert_eq!(table.last_key(), Some(8));
}

#[test]
fn test_ordered_table_mt_per_previous_next_key() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    for i in [10, 20, 30, 40, 50] {
        table = table.insert(i, format!("v{i}"));
    }

    assert_eq!(table.previous_key(&30), Some(20));
    assert_eq!(table.previous_key(&10), None);
    assert_eq!(table.previous_key(&25), Some(20));
    assert_eq!(table.next_key(&30), Some(40));
    assert_eq!(table.next_key(&50), None);
    assert_eq!(table.next_key(&35), Some(40));
}

#[test]
fn test_ordered_table_mt_per_split_key() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    for i in [1, 3, 5, 7] {
        table = table.insert(i, format!("v{i}"));
    }

    let (left, found, right) = table.split_key(&5);
    assert_eq!(found, Some("v5".to_string()));
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 1);
    assert_eq!(left.find(&1), Some("v1".to_string()));
    assert_eq!(left.find(&3), Some("v3".to_string()));
    assert_eq!(right.find(&7), Some("v7".to_string()));

    let (left2, found2, right2) = table.split_key(&4);
    assert_eq!(found2, None);
    assert_eq!(left2.size(), 2);
    assert_eq!(right2.size(), 2);
}

#[test]
fn test_ordered_table_mt_per_join_key() {
    let mut t1: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    t1 = t1.insert(1, "one".into());
    t1 = t1.insert(2, "two".into());

    let mut t2: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    t2 = t2.insert(5, "five".into());
    t2 = t2.insert(6, "six".into());

    let joined = t1.join_key(&t2);
    assert_eq!(joined.size(), 4);
    assert_eq!(joined.find(&1), Some("one".to_string()));
    assert_eq!(joined.find(&6), Some("six".to_string()));
}

#[test]
fn test_ordered_table_mt_per_get_key_range() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    for i in [10, 20, 30, 40, 50] {
        table = table.insert(i, format!("v{i}"));
    }

    let range = table.get_key_range(&20, &40);
    assert_eq!(range.size(), 3);
    assert_eq!(range.find(&20), Some("v20".to_string()));
    assert_eq!(range.find(&30), Some("v30".to_string()));
    assert_eq!(range.find(&40), Some("v40".to_string()));
    assert_eq!(range.find(&10), None);
}

#[test]
fn test_ordered_table_mt_per_rank_select_key() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    for i in [10, 20, 30, 40, 50] {
        table = table.insert(i, format!("v{i}"));
    }

    assert_eq!(table.rank_key(&30), 2);
    assert_eq!(table.rank_key(&10), 0);
    assert_eq!(table.rank_key(&25), 2);
    assert_eq!(table.rank_key(&55), 5);

    assert_eq!(table.select_key(0), Some(10));
    assert_eq!(table.select_key(2), Some(30));
    assert_eq!(table.select_key(4), Some(50));
    assert_eq!(table.select_key(5), None);
}

#[test]
fn test_ordered_table_mt_per_split_rank_key() {
    let mut table: OrderedTableMtPer<i32, String> = OrderedTableMtPerTrait::empty();
    for i in [10, 20, 30, 40, 50] {
        table = table.insert(i, format!("v{i}"));
    }

    let (left, right) = table.split_rank_key(2);
    assert_eq!(left.size(), 2);
    assert_eq!(right.size(), 3);
    assert_eq!(left.find(&10), Some("v10".to_string()));
    assert_eq!(left.find(&20), Some("v20".to_string()));
    assert_eq!(right.find(&30), Some("v30".to_string()));
}
