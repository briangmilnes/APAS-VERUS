//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap43 OrderedTableStEph.

use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_ordered_table_st_eph_new() {
    let table = OrderedTableStEph::<i32, String>::empty();
    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_ordered_table_st_eph_insert_and_lookup() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), Some("two".to_string()));
    assert_eq!(table.lookup(&3), None);
}

#[test]
fn test_ordered_table_st_eph_delete() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let deleted = table.delete(&1);
    assert_eq!(deleted, Some("one".to_string()));
    assert_eq!(table.size(), 1);
    assert_eq!(table.lookup(&1), None);
    assert_eq!(table.lookup(&2), Some("two".to_string()));
}

#[test]
fn test_ordered_table_st_eph_first_key() {
    let mut table = OrderedTableStEph::empty();
    assert_eq!(table.first_key(), None);

    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    assert_eq!(table.first_key(), Some(1));
}

#[test]
fn test_ordered_table_st_eph_last_key() {
    let mut table = OrderedTableStEph::empty();
    assert_eq!(table.last_key(), None);

    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    assert_eq!(table.last_key(), Some(3));
}

#[test]
fn test_ordered_table_st_eph_previous_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());

    assert_eq!(table.previous_key(&1), None);
    assert_eq!(table.previous_key(&3), Some(1));
    assert_eq!(table.previous_key(&5), Some(3));
    assert_eq!(table.previous_key(&4), Some(3));
}

#[test]
fn test_ordered_table_st_eph_next_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());

    assert_eq!(table.next_key(&1), Some(3));
    assert_eq!(table.next_key(&3), Some(5));
    assert_eq!(table.next_key(&5), None);
    assert_eq!(table.next_key(&2), Some(3));
}

#[test]
fn test_ordered_table_st_eph_split_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let (left, right) = table.split_key(&3);

    // Left should contain keys < 3
    assert_eq!(left.size(), 2);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));
    assert_eq!(left.lookup(&3), None);

    // Right should contain keys >= 3
    assert_eq!(right.size(), 2);
    assert_eq!(right.lookup(&3), Some("three".to_string()));
    assert_eq!(right.lookup(&4), Some("four".to_string()));
    assert_eq!(right.lookup(&1), None);
}

#[test]
fn test_ordered_table_st_eph_join_key() {
    let mut left = OrderedTableStEph::empty();
    left.insert(1, "one".to_string(), |_old, new| new.clone());
    left.insert(2, "two".to_string(), |_old, new| new.clone());

    let mut right = OrderedTableStEph::empty();
    right.insert(3, "three".to_string(), |_old, new| new.clone());
    right.insert(4, "four".to_string(), |_old, new| new.clone());

    left.join_key(right);

    assert_eq!(left.size(), 4);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));
    assert_eq!(left.lookup(&3), Some("three".to_string()));
    assert_eq!(left.lookup(&4), Some("four".to_string()));

    // Note: right is consumed by join_key, so we can't test it afterward
}

#[test]
fn test_ordered_table_st_eph_get_key_range() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());
    table.insert(5, "five".to_string(), |_old, new| new.clone());

    let range = table.get_key_range(&2, &4);

    assert_eq!(range.size(), 3);
    assert_eq!(range.lookup(&2), Some("two".to_string()));
    assert_eq!(range.lookup(&3), Some("three".to_string()));
    assert_eq!(range.lookup(&4), Some("four".to_string()));
    assert_eq!(range.lookup(&1), None);
    assert_eq!(range.lookup(&5), None);
}

#[test]
fn test_ordered_table_st_eph_rank_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(10, "ten".to_string(), |_old, new| new.clone());
    table.insert(20, "twenty".to_string(), |_old, new| new.clone());
    table.insert(30, "thirty".to_string(), |_old, new| new.clone());

    assert_eq!(table.rank_key(&10), 0);
    assert_eq!(table.rank_key(&20), 1);
    assert_eq!(table.rank_key(&30), 2);
    assert_eq!(table.rank_key(&15), 1); // Between 10 and 20
}

#[test]
fn test_ordered_table_st_eph_select_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(10, "ten".to_string(), |_old, new| new.clone());
    table.insert(20, "twenty".to_string(), |_old, new| new.clone());
    table.insert(30, "thirty".to_string(), |_old, new| new.clone());

    assert_eq!(table.select_key(0), Some(10));
    assert_eq!(table.select_key(1), Some(20));
    assert_eq!(table.select_key(2), Some(30));
    assert_eq!(table.select_key(3), None);
}

#[test]
fn test_ordered_table_st_eph_split_rank_key() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let (left, right) = table.split_rank_key(2);

    // Left should contain first 2 elements
    assert_eq!(left.size(), 2);
    assert_eq!(left.lookup(&1), Some("one".to_string()));
    assert_eq!(left.lookup(&2), Some("two".to_string()));

    // Right should contain remaining elements
    assert_eq!(right.size(), 2);
    assert_eq!(right.lookup(&3), Some("three".to_string()));
    assert_eq!(right.lookup(&4), Some("four".to_string()));
}

#[test]
fn test_ordered_table_st_eph_ephemeral_semantics() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());

    let original_size = table.size();

    // Ephemeral operations should modify the original table
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    assert_eq!(table.size(), original_size + 1);

    table.delete(&1);
    assert_eq!(table.size(), original_size);
    assert_eq!(table.lookup(&1), None);
}

#[test]
fn test_ordered_table_st_eph_filter() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let filtered = table.filter(|k, _v| k % 2 == 0);

    assert_eq!(filtered.size(), 2);
    assert_eq!(filtered.lookup(&2), Some("two".to_string()));
    assert_eq!(filtered.lookup(&4), Some("four".to_string()));
    assert_eq!(filtered.lookup(&1), None);
    assert_eq!(filtered.lookup(&3), None);
}

#[test]
fn test_ordered_table_st_eph_map() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let mapped = table.map(|k, v| format!("{k}:{v}"));

    assert_eq!(mapped.size(), 2);
    assert_eq!(mapped.lookup(&1), Some("1:one".to_string()));
    assert_eq!(mapped.lookup(&2), Some("2:two".to_string()));
}

#[test]
fn test_ordered_table_st_eph_reduce() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    let sum = table.reduce(0, |acc, _k, v| acc + v);
    assert_eq!(sum, 60);
}

#[test]
fn test_ordered_table_st_eph_collect() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let collected = table.collect();
    assert_eq!(collected.length(), 2);
}

#[test]
fn test_ordered_table_st_eph_from_sorted_entries() {
    let entries = vec![
        Pair(1, "one".to_string()),
        Pair(2, "two".to_string()),
        Pair(3, "three".to_string()),
    ];

    let seq = AVLTreeSeqStPerS::from_vec(entries);
    let table = from_sorted_entries(seq);

    assert_eq!(table.size(), 3);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), Some("two".to_string()));
    assert_eq!(table.lookup(&3), Some("three".to_string()));
}

#[test]
fn test_ordered_table_st_eph_empty_operations() {
    let mut table = OrderedTableStEph::<i32, String>::empty();

    assert_eq!(table.first_key(), None);
    assert_eq!(table.last_key(), None);
    assert_eq!(table.previous_key(&1), None);
    assert_eq!(table.next_key(&1), None);
    assert_eq!(table.select_key(0), None);

    let (left, right) = table.split_key(&1);
    assert_eq!(left.size(), 0);
    assert_eq!(right.size(), 0);

    let range = table.get_key_range(&1, &5);
    assert_eq!(range.size(), 0);
}

#[test]
fn test_ordered_table_st_eph_singleton() {
    let table = OrderedTableStEph::singleton(42, "answer".to_string());
    assert_eq!(table.size(), 1);
    assert!(!table.is_empty());
    assert_eq!(table.lookup(&42), Some("answer".to_string()));
    assert_eq!(table.find(&42), Some("answer".to_string()));
    assert_eq!(table.first_key(), Some(42));
    assert_eq!(table.last_key(), Some(42));
}

#[test]
fn test_ordered_table_st_eph_find() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());

    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
    assert_eq!(table.find(&4), None);
    assert_eq!(table.find(&0), None);
}

#[test]
fn test_ordered_table_st_eph_domain() {
    let mut table = OrderedTableStEph::empty();
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let domain = table.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.find(&1));
    assert!(domain.find(&2));
    assert!(domain.find(&3));
    assert!(!domain.find(&4));
}

#[test]
fn test_ordered_table_st_eph_domain_empty() {
    let table = OrderedTableStEph::<i32, String>::empty();
    let domain = table.domain();
    assert_eq!(domain.size(), 0);
}

#[test]
fn test_ordered_table_st_eph_tabulate() {
    let mut keys = ArraySetStEph::empty();
    keys.insert(1);
    keys.insert(2);
    keys.insert(3);

    let table = OrderedTableStEph::tabulate(|k| format!("value_{k}"), &keys);

    assert_eq!(table.size(), 3);
    assert_eq!(table.lookup(&1), Some("value_1".to_string()));
    assert_eq!(table.lookup(&2), Some("value_2".to_string()));
    assert_eq!(table.lookup(&3), Some("value_3".to_string()));
}

#[test]
fn test_ordered_table_st_eph_tabulate_empty() {
    let keys = ArraySetStEph::<i32>::empty();
    let table = OrderedTableStEph::tabulate(|k| format!("value_{k}"), &keys);

    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_ordered_table_st_eph_intersection() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = OrderedTableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(3, "THREE".to_string(), |_old, new| new.clone());
    table2.insert(4, "FOUR".to_string(), |_old, new| new.clone());

    table1.intersection(&table2, |v1, _v2| v1.clone());

    assert_eq!(table1.size(), 2);
    assert_eq!(table1.lookup(&1), None);
    assert_eq!(table1.lookup(&2), Some("two".to_string()));
    assert_eq!(table1.lookup(&3), Some("three".to_string()));
    assert_eq!(table1.lookup(&4), None);
}

#[test]
fn test_ordered_table_st_eph_intersection_disjoint() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());

    let mut table2 = OrderedTableStEph::empty();
    table2.insert(3, "three".to_string(), |_old, new| new.clone());
    table2.insert(4, "four".to_string(), |_old, new| new.clone());

    table1.intersection(&table2, |v1, _v2| v1.clone());

    assert_eq!(table1.size(), 0);
    assert!(table1.is_empty());
}

#[test]
fn test_ordered_table_st_eph_union() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());

    let mut table2 = OrderedTableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(3, "three".to_string(), |_old, new| new.clone());

    table1.union(&table2, |v1, _v2| v1.clone());

    assert_eq!(table1.size(), 3);
    assert_eq!(table1.lookup(&1), Some("one".to_string()));
    assert_eq!(table1.lookup(&2), Some("two".to_string()));
    assert_eq!(table1.lookup(&3), Some("three".to_string()));
}

#[test]
fn test_ordered_table_st_eph_union_empty() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());

    let table2 = OrderedTableStEph::<i32, String>::empty();
    table1.union(&table2, |v1, _v2| v1.clone());

    assert_eq!(table1.size(), 1);
    assert_eq!(table1.lookup(&1), Some("one".to_string()));
}

#[test]
fn test_ordered_table_st_eph_difference() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = OrderedTableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(4, "FOUR".to_string(), |_old, new| new.clone());

    table1.difference(&table2);

    assert_eq!(table1.size(), 2);
    assert_eq!(table1.lookup(&1), Some("one".to_string()));
    assert_eq!(table1.lookup(&2), None);
    assert_eq!(table1.lookup(&3), Some("three".to_string()));
}

#[test]
fn test_ordered_table_st_eph_difference_empty() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());

    let table2 = OrderedTableStEph::<i32, String>::empty();
    table1.difference(&table2);

    assert_eq!(table1.size(), 1);
    assert_eq!(table1.lookup(&1), Some("one".to_string()));
}

#[test]
fn test_ordered_table_st_eph_restrict() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    table.restrict(&keys);

    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&1), None);
    assert_eq!(table.lookup(&2), Some("two".to_string()));
    assert_eq!(table.lookup(&3), None);
    assert_eq!(table.lookup(&4), Some("four".to_string()));
}

#[test]
fn test_ordered_table_st_eph_restrict_empty_keys() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let keys = ArraySetStEph::<i32>::empty();
    table.restrict(&keys);

    assert_eq!(table.size(), 0);
    assert!(table.is_empty());
}

#[test]
fn test_ordered_table_st_eph_subtract() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    table.subtract(&keys);

    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), None);
    assert_eq!(table.lookup(&3), Some("three".to_string()));
    assert_eq!(table.lookup(&4), None);
}

#[test]
fn test_ordered_table_st_eph_subtract_empty_keys() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    let keys = ArraySetStEph::<i32>::empty();
    table.subtract(&keys);

    assert_eq!(table.size(), 2);
    assert_eq!(table.lookup(&1), Some("one".to_string()));
    assert_eq!(table.lookup(&2), Some("two".to_string()));
}

#[test]
fn test_ordered_table_st_eph_clone() {
    let mut table1 = OrderedTableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());

    let table2 = table1.clone();

    assert_eq!(table2.size(), 2);
    assert_eq!(table2.lookup(&1), Some("one".to_string()));
    assert_eq!(table2.lookup(&2), Some("two".to_string()));
}

#[test]
fn test_ordered_table_st_eph_reduce_sum() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, 10, |_old, new| *new);
    table.insert(2, 20, |_old, new| *new);
    table.insert(3, 30, |_old, new| *new);

    let sum = table.reduce(0, |acc, _k, v| acc + v);
    assert_eq!(sum, 60);
}

#[test]
fn test_ordered_table_st_eph_reduce_empty() {
    let table = OrderedTableStEph::<i32, i32>::empty();
    let sum = table.reduce(42, |acc, _k, v| acc + v);
    assert_eq!(sum, 42);
}

#[test]
fn test_ordered_table_st_eph_large_dataset() {
    let mut table = OrderedTableStEph::empty();

    // Insert a dataset (200 for consistency)
    for i in 0..200 {
        table.insert(i, format!("value_{i}"), |_old, new| new.clone());
    }

    // Test filter operation
    let even_filtered = table.filter(|k, _v| k % 2 == 0);
    assert_eq!(even_filtered.size(), 100);

    // Test map operation
    let mapped = table.map(|k, v| format!("mapped_{k}:{v}"));
    assert_eq!(mapped.size(), 200);

    // Test ordering operations
    assert_eq!(table.first_key(), Some(0));
    assert_eq!(table.last_key(), Some(199));
    assert_eq!(table.select_key(100), Some(100));
    assert_eq!(table.rank_key(&100), 100);
}

#[test]
fn test_delete_nonexistent() {
    let mut table = OrderedTableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.delete(&2);
    assert_eq!(table.size(), 2);
}
