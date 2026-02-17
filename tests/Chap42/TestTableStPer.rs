//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 42 single-threaded persistent table implementation.

use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap42::TableStPer::TableStPer::*;
use apas_verus::TableStPerLit;
use apas_verus::Types::Types::*;

#[test]
fn test_tablestperlit_macro_functionality() {
    // Test empty table creation
    let empty: TableStPer<i32, String> = TableStPerLit![];
    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&42), None);

    // Test table creation with key-value pairs
    let with_data: TableStPer<i32, String> = TableStPerLit![
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string()
    ];
    assert_eq!(with_data.size(), 3);
    assert_eq!(with_data.find(&1), Some("one".to_string()));
    assert_eq!(with_data.find(&2), Some("two".to_string()));
    assert_eq!(with_data.find(&3), Some("three".to_string()));
    assert_eq!(with_data.find(&4), None);
}

#[test]
fn test_table_empty_and_size() {
    let table = TableStPer::<i32, String>::empty();
    assert_eq!(table.size(), 0);
}

#[test]
fn test_table_singleton() {
    let table = TableStPer::singleton(42, "hello".to_string());
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&42), Some("hello".to_string()));
    assert_eq!(table.find(&99), None);
}

#[test]
fn test_table_insert_and_find() {
    let table = TableStPer::empty();
    let table = table.insert(1, "one".to_string(), |_old, new| new.clone());
    let table = table.insert(2, "two".to_string(), |_old, new| new.clone());
    let table = table.insert(3, "three".to_string(), |_old, new| new.clone());

    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
    assert_eq!(table.find(&4), None);
}

#[test]
fn test_table_insert_with_combine() {
    let table = TableStPer::empty();
    let table = table.insert(1, 10, |old, new| old + new);
    let table = table.insert(1, 5, |old, new| old + new); // Should combine: 10 + 5 = 15

    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&1), Some(15));
}

#[test]
fn test_table_delete() {
    let table = TableStPer::empty();
    let table = table.insert(1, "one".to_string(), |_old, new| new.clone());
    let table = table.insert(2, "two".to_string(), |_old, new| new.clone());
    let table = table.insert(3, "three".to_string(), |_old, new| new.clone());

    let table = table.delete(&2);
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), None);
    assert_eq!(table.find(&3), Some("three".to_string()));
}

#[test]
fn test_table_domain() {
    let table = TableStPer::empty();
    let table = table.insert(3, "three".to_string(), |_old, new| new.clone());
    let table = table.insert(1, "one".to_string(), |_old, new| new.clone());
    let table = table.insert(2, "two".to_string(), |_old, new| new.clone());

    let domain = table.domain();
    assert_eq!(domain.size(), 3);
    assert!(domain.find(&1));
    assert!(domain.find(&2));
    assert!(domain.find(&3));
    assert!(!domain.find(&4));
}

#[test]
fn test_table_map() {
    let table = TableStPer::empty();
    let table = table.insert(1, 10, |_old, new| *new);
    let table = table.insert(2, 20, |_old, new| *new);
    let table = table.insert(3, 30, |_old, new| *new);

    let mapped = table.map(|v| v * 2);
    assert_eq!(mapped.size(), 3);
    assert_eq!(mapped.find(&1), Some(20));
    assert_eq!(mapped.find(&2), Some(40));
    assert_eq!(mapped.find(&3), Some(60));
}

#[test]
fn test_table_filter() {
    let table = TableStPer::empty();
    let table = table.insert(1, 10, |_old, new| *new);
    let table = table.insert(2, 25, |_old, new| *new);
    let table = table.insert(3, 30, |_old, new| *new);
    let table = table.insert(4, 15, |_old, new| *new);

    let filtered = table.filter(|_k, v| *v > 20);
    assert_eq!(filtered.size(), 2);
    assert_eq!(filtered.find(&2), Some(25));
    assert_eq!(filtered.find(&3), Some(30));
    assert_eq!(filtered.find(&1), None);
    assert_eq!(filtered.find(&4), None);
}

#[test]
fn test_table_intersection() {
    let table1 = TableStPer::empty();
    let table1 = table1.insert(1, 10, |_old, new| *new);
    let table1 = table1.insert(2, 20, |_old, new| *new);
    let table1 = table1.insert(3, 30, |_old, new| *new);

    let table2 = TableStPer::empty();
    let table2 = table2.insert(2, 200, |_old, new| *new);
    let table2 = table2.insert(3, 300, |_old, new| *new);
    let table2 = table2.insert(4, 400, |_old, new| *new);

    let intersection = table1.intersection(&table2, |v1, v2| v1 + v2);
    assert_eq!(intersection.size(), 2);
    assert_eq!(intersection.find(&2), Some(220)); // 20 + 200
    assert_eq!(intersection.find(&3), Some(330)); // 30 + 300
    assert_eq!(intersection.find(&1), None);
    assert_eq!(intersection.find(&4), None);
}

#[test]
fn test_table_union() {
    let table1 = TableStPer::empty();
    let table1 = table1.insert(1, 10, |_old, new| *new);
    let table1 = table1.insert(2, 20, |_old, new| *new);

    let table2 = TableStPer::empty();
    let table2 = table2.insert(2, 200, |_old, new| *new);
    let table2 = table2.insert(3, 300, |_old, new| *new);

    let union = table1.union(&table2, |v1, v2| v1 + v2);
    assert_eq!(union.size(), 3);
    assert_eq!(union.find(&1), Some(10)); // Only in table1
    assert_eq!(union.find(&2), Some(220)); // Combined: 20 + 200
    assert_eq!(union.find(&3), Some(300)); // Only in table2
}

#[test]
fn test_table_difference() {
    let table1 = TableStPer::empty();
    let table1 = table1.insert(1, 10, |_old, new| *new);
    let table1 = table1.insert(2, 20, |_old, new| *new);
    let table1 = table1.insert(3, 30, |_old, new| *new);

    let table2 = TableStPer::empty();
    let table2 = table2.insert(2, 200, |_old, new| *new);
    let table2 = table2.insert(4, 400, |_old, new| *new);

    let difference = table1.difference(&table2);
    assert_eq!(difference.size(), 2);
    assert_eq!(difference.find(&1), Some(10));
    assert_eq!(difference.find(&3), Some(30));
    assert_eq!(difference.find(&2), None);
    assert_eq!(difference.find(&4), None);
}

#[test]
fn test_table_restrict() {
    let table = TableStPer::empty();
    let table = table.insert(1, "one".to_string(), |_old, new| new.clone());
    let table = table.insert(2, "two".to_string(), |_old, new| new.clone());
    let table = table.insert(3, "three".to_string(), |_old, new| new.clone());
    let table = table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    let restricted = table.restrict(&keys);
    assert_eq!(restricted.size(), 2);
    assert_eq!(restricted.find(&2), Some("two".to_string()));
    assert_eq!(restricted.find(&4), Some("four".to_string()));
    assert_eq!(restricted.find(&1), None);
    assert_eq!(restricted.find(&3), None);
}

#[test]
fn test_table_subtract() {
    let table = TableStPer::empty();
    let table = table.insert(1, "one".to_string(), |_old, new| new.clone());
    let table = table.insert(2, "two".to_string(), |_old, new| new.clone());
    let table = table.insert(3, "three".to_string(), |_old, new| new.clone());
    let table = table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    let subtracted = table.subtract(&keys);
    assert_eq!(subtracted.size(), 2);
    assert_eq!(subtracted.find(&1), Some("one".to_string()));
    assert_eq!(subtracted.find(&3), Some("three".to_string()));
    assert_eq!(subtracted.find(&2), None);
    assert_eq!(subtracted.find(&4), None);
}

#[test]
fn test_table_tabulate() {
    let mut keys = ArraySetStEph::empty();
    keys.insert(1);
    keys.insert(2);
    keys.insert(3);

    let table = TableStPer::tabulate(|k| k * k, &keys);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some(1));
    assert_eq!(table.find(&2), Some(4));
    assert_eq!(table.find(&3), Some(9));
}

#[test]
fn test_table_macro() {
    let table = TableStPerLit![
        1 => "one".to_string(),
        3 => "three".to_string(),
        2 => "two".to_string()
    ];

    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
}

#[test]
fn test_table_empty_operations() {
    let empty = TableStPer::<i32, String>::empty();

    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&1), None);

    let domain = empty.domain();
    assert_eq!(domain.size(), 0);

    let mapped = empty.map(|s| s.to_uppercase());
    assert_eq!(mapped.size(), 0);

    let filtered = empty.filter(|_k, _v| true);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_table_persistence() {
    let table1 = TableStPer::empty();
    let table2 = table1.insert(1, "one".to_string(), |_old, new| new.clone());
    let table3 = table2.insert(2, "two".to_string(), |_old, new| new.clone());

    // Original tables should be unchanged
    assert_eq!(table1.size(), 0);
    assert_eq!(table2.size(), 1);
    assert_eq!(table3.size(), 2);

    assert_eq!(table2.find(&1), Some("one".to_string()));
    assert_eq!(table2.find(&2), None);

    assert_eq!(table3.find(&1), Some("one".to_string()));
    assert_eq!(table3.find(&2), Some("two".to_string()));
}
