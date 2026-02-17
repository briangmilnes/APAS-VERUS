//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 42 multi-threaded ephemeral table implementation.

use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;
use apas_verus::Chap42::TableMtEph::TableMtEph::*;
use apas_verus::TableMtEphLit;
use apas_verus::Types::Types::*;

#[test]
fn test_tablemtephlit_macro_functionality() {
    // Test empty table creation
    let empty: TableMtEph<i32, String> = TableMtEphLit![];
    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&42), None);

    // Test table creation with key-value pairs
    let with_data: TableMtEph<i32, String> = TableMtEphLit![
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
    let table = TableMtEph::<i32, String>::empty();
    assert_eq!(table.size(), 0);
}

#[test]
fn test_table_singleton() {
    let table = TableMtEph::singleton(42, "answer".to_string());
    assert_eq!(table.size(), 1);
    assert_eq!(table.find(&42), Some("answer".to_string()));
    assert_eq!(table.find(&0), None);
}

#[test]
fn test_table_insert_ephemeral() {
    let mut table = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
    table.insert(1, "one".to_string(), |_old, new| new.clone());
    table.insert(2, "two".to_string(), |_old, new| new.clone());

    table.map(|s| s.to_uppercase());
    assert_eq!(table.find(&1), Some("ONE".to_string()));
    assert_eq!(table.find(&2), Some("TWO".to_string()));
}

#[test]
fn test_table_filter_ephemeral() {
    let mut table = TableMtEph::empty();
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
    let mut table1 = TableMtEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = TableMtEph::empty();
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
    let mut table1 = TableMtEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());

    let mut table2 = TableMtEph::empty();
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
    let mut table1 = TableMtEph::empty();
    table1.insert(1, "one".to_string(), |_old, new| new.clone());
    table1.insert(2, "two".to_string(), |_old, new| new.clone());
    table1.insert(3, "three".to_string(), |_old, new| new.clone());

    let mut table2 = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
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
    let mut table = TableMtEph::empty();
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

    let table = TableMtEph::tabulate(|k| k * k, &keys);
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some(1));
    assert_eq!(table.find(&2), Some(4));
    assert_eq!(table.find(&3), Some(9));
}

#[test]
fn test_table_ephemeral_semantics() {
    let mut original = TableMtEph::empty();
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
fn test_table_mteph_lit_macro() {
    let table = TableMtEphLit![1 => "one".to_string(), 3 => "three".to_string(), 2 => "two".to_string()];
    assert_eq!(table.size(), 3);
    assert_eq!(table.find(&1), Some("one".to_string()));
    assert_eq!(table.find(&2), Some("two".to_string()));
    assert_eq!(table.find(&3), Some("three".to_string()));

    let empty_table: TableMtEph<i32, String> = TableMtEphLit![];
    assert_eq!(empty_table.size(), 0);
}

#[test]
fn test_table_parallel_operations() {
    let mut table = TableMtEph::empty();

    // Insert many elements to test parallel operations
    for i in 0..50 {
        table.insert(i, format!("value_{i}"), |_old, new| new.clone());
    }
    assert_eq!(table.size(), 50);

    // Test parallel map
    table.map(|s| s.to_uppercase());
    for i in 0..50 {
        assert_eq!(table.find(&i), Some(format!("VALUE_{i}")));
    }

    // Test parallel filter
    table.filter(|k, _v| *k % 2 == 0);
    assert_eq!(table.size(), 25);

    for i in 0..50 {
        if i % 2 == 0 {
            assert_eq!(table.find(&i), Some(format!("VALUE_{i}")));
        } else {
            assert_eq!(table.find(&i), None);
        }
    }
}

#[test]
fn test_table_parallel_tabulate() {
    let mut keys = ArraySetStEph::empty();
    for i in 0..20 {
        keys.insert(i);
    }

    // Test parallel tabulation with expensive function
    let table = TableMtEph::tabulate(
        |k| {
            // Simulate some computation
            std::thread::sleep(std::time::Duration::from_millis(1));
            k * k * k
        },
        &keys,
    );

    assert_eq!(table.size(), 20);
    for i in 0..20 {
        assert_eq!(table.find(&i), Some(i * i * i));
    }
}
