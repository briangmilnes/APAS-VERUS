//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 42 single-threaded ephemeral table implementation.

use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap42::TableStEph::TableStEph::*;
use apas_verus::TableStEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_tablestephlit_macro_functionality() {
    // Test empty table creation
    let empty: TableStEph<i32, String> = TableStEphLit![];
    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&42), None);

    // Test table creation with key-value pairs
    let with_data: TableStEph<i32, String> = TableStEphLit![
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
fn test_table_empty() {
    let table = TableStEph::<i32, String>::empty();
    assert_eq!(table.size(), 0);
}

#[test]
fn test_table_singleton() {
    let table = TableStEph::singleton(42, "answer".to_string());
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&42), Some("answer".to_string()));
    assert_eq!(table.find(&0), None);
}

#[test]
fn test_table_insert_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&1), Some("one".to_string()));

    // Insert duplicate key with combine function
    table.insert(1, "ONE".to_string(), |old, new| format!("{old}-{new}"));
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&1), Some("one-ONE".to_string()));

    // Insert new key
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&2), Some("two".to_string()));
}

#[test]
fn test_table_find() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
    assert_eq!(table.find(&4), None);
}

#[test]
fn test_table_delete_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());

    assert_eq!(table.size(), 3);
    table.delete(&2);
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&2), None);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));

    // Delete non-existent key
    table.delete(&99);
    assert_eq!(table.size(), 2);
}

#[test]
fn test_table_domain() {
    let mut table = TableStEph::empty();
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
fn test_table_map_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    table.map(|s| s.to_uppercase());
    assert_eq!(table.find(&1), Some("ONE".to_string()));
    assert_eq!(table.find(&2), Some("TWO".to_string()));
}

#[test]
fn test_table_filter_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    table.filter(|k, _v| *k % 2 == 0);
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&4), Some("four".to_string()));
    assert_eq!(table.find(&1), None);
    assert_eq!(table.find(&3), None);
}

#[test]
fn test_table_intersection_ephemeral() {
    let mut table1 = TableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = TableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(3, "THREE".to_string(), |_old, new| new.clone());
    table2.insert(4, "FOUR".to_string(), |_old, new| new.clone());

    table1.intersection(&table2, |v1, v2| format!("{v1}+{v2}"));
    assert_eq!(table1.size(), 2);
    assert_eq!(table1.find(&2), Some("two+TWO".to_string()));
    assert_eq!(table1.find(&3), Some("three+THREE".to_string()));
    assert_eq!(table1.find(&1), None);
    assert_eq!(table1.find(&4), None);
}

#[test]
fn test_table_union_ephemeral() {
    let mut table1 = TableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());

    let mut table2 = TableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(3, "THREE".to_string(), |_old, new| new.clone());

    table1.union(&table2, |v1, v2| format!("{v1}+{v2}"));
    assert_eq!(table1.size(), 3);
    assert_eq!(table1.find(&1), Some("one".to_string()));
    assert_eq!(table1.find(&2), Some("two+TWO".to_string()));
    assert_eq!(table1.find(&3), Some("THREE".to_string()));
}

#[test]
fn test_table_difference_ephemeral() {
    let mut table1 = TableStEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = TableStEph::empty();
    table2.insert(2, "TWO".to_string(), |_old, new| new.clone());
    table2.insert(4, "FOUR".to_string(), |_old, new| new.clone());

    table1.difference(&table2);
    assert_eq!(table1.size(), 2);
    assert_eq!(table1.find(&1), Some("one".to_string()));
    assert_eq!(table1.find(&3), Some("three".to_string()));
    assert_eq!(table1.find(&2), None);
}

#[test]
fn test_table_restrict_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    table.restrict(&keys);
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&4), Some("four".to_string()));
    assert_eq!(table.find(&1), None);
    assert_eq!(table.find(&3), None);
}

#[test]
fn test_table_subtract_ephemeral() {
    let mut table = TableStEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());
    table.insert(3, "three".to_string(), |_old, new| new.clone());
    table.insert(4, "four".to_string(), |_old, new| new.clone());

    let mut keys = ArraySetStEph::empty();
    keys.insert(2);
    keys.insert(4);

    table.subtract(&keys);
    assert_eq!(table.size(), 2);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));
    assert_eq!(table.find(&2), None);
    assert_eq!(table.find(&4), None);
}

#[test]
fn test_table_tabulate() {
    let mut keys = ArraySetStEph::empty();
    keys.insert(1);
    keys.insert(2);
    keys.insert(3);

    let table = TableStEph::tabulate(|k| k * k, &keys);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some(1));
    assert_eq!(table.find(&2), Some(4));
    assert_eq!(table.find(&3), Some(9));
}

#[test]
fn test_table_ephemeral_semantics() {
    let mut original = TableStEph::empty();
    original.insert(1, "one".to_string(), |_old, new| new.clone());
    original.insert(2, "two".to_string(), |_old, new| new.clone());

    let original_size = original.size();

    // Ephemeral operations modify the original
    original.insert(3, "three".to_string(), |_old, new| new.clone());
    assert_eq!(original.size(), original_size + 1);

    original.delete(&1);
    assert_eq!(original.size(), original_size);
    assert_eq!(original.find(&1), None);
    assert_eq!(original.find(&2), Some("two".to_string()));
    assert_eq!(original.find(&3), Some("three".to_string()));
}

#[test]
fn test_table_steph_lit_macro() {
    let table = TableStEphLit![1 => "one".to_string(), 3 => "three".to_string(), 2 => "two".to_string()];
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));

    let empty_table: TableStEph<i32, String> = TableStEphLit![];
    assert_eq!(empty_table.size(), 0);
}

#[test]
fn test_table_large_operations() {
    let mut table = TableStEph::empty();

    // Insert many elements
    for i in 0..100 {
        table.insert(i, format!("value_{i}"), |_old, new| new.clone());
    }
    assert_eq!(table.size(), 100);

    // Test find on large table
    for i in 0..100 {
        assert_eq!(table.find(&i), Some(format!("value_{i}")));
    }

    // Filter to even numbers
    table.filter(|k, _v| *k % 2 == 0);
    assert_eq!(table.size(), 50);

    for i in 0..100 {
        if i % 2 == 0 {
            assert_eq!(table.find(&i), Some(format!("value_{i}")));
        } else {
            assert_eq!(table.find(&i), None);
        }
    }
}
