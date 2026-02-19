//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use rand::{Rng, RngExt};

use apas_verus::BSTKeyValueStEphLit;

fn rand_priority() -> u64 { rand::rng().random() }
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap40::BSTKeyValueStEph::BSTKeyValueStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn key_value_bst_basic_operations() {
    let mut bst = BSTreeKeyValue::new();
    assert!(bst.is_empty());
    assert_eq!(bst.size(), 0);

    // Insert key-value pairs
    bst.insert(5, "five", rand_priority());
    bst.insert(3, "three", rand_priority());
    bst.insert(7, "seven", rand_priority());
    bst.insert(1, "one", rand_priority());
    bst.insert(9, "nine", rand_priority());

    assert_eq!(bst.size(), 5);
    assert!(!bst.is_empty());

    // Test find/get operations
    assert_eq!(bst.find(&5), Some(&"five"));
    assert_eq!(bst.get(&3), Some(&"three"));
    assert_eq!(bst.find(&10), None);

    // Test contains
    assert!(bst.contains(&7));
    assert!(!bst.contains(&10));

    // Test min/max keys
    assert_eq!(bst.minimum_key(), Some(&1));
    assert_eq!(bst.maximum_key(), Some(&9));
}

#[test]
fn key_value_bst_update_existing_key() {
    let mut bst = BSTreeKeyValue::new();
    bst.insert(5, "five", rand_priority());
    bst.insert(5, "FIVE", rand_priority()); // Update existing key

    assert_eq!(bst.size(), 1); // Size should not change
    assert_eq!(bst.find(&5), Some(&"FIVE")); // Value should be updated
}

#[test]
fn key_value_bst_collections() {
    let mut bst = BSTreeKeyValue::new();
    bst.insert(3, 30, rand_priority());
    bst.insert(1, 10, rand_priority());
    bst.insert(5, 50, rand_priority());
    bst.insert(2, 20, rand_priority());
    bst.insert(4, 40, rand_priority());

    // Test keys (should be in sorted order)
    let keys = bst.keys();
    assert_eq!(keys.length(), 5);
    let expected_keys = [1, 2, 3, 4, 5];
    for (i, expected) in expected_keys.iter().enumerate() {
        assert_eq!(keys.nth(i), expected);
    }

    // Test values (should be in key-sorted order)
    let values = bst.values();
    assert_eq!(values.length(), 5);
    let expected_values = [10, 20, 30, 40, 50];
    for (i, expected) in expected_values.iter().enumerate() {
        assert_eq!(values.nth(i), expected);
    }

    // key_value_pairs method removed due to tuple Display issues
}

#[test]
fn key_value_bst_macro_literal() {
    // Test empty macro
    let empty_bst: BSTKeyValueStEph<i32, &str> = BSTKeyValueStEphLit![];
    assert!(empty_bst.is_empty());

    // Test non-empty macro
    let bst = BSTKeyValueStEphLit![(1, "one"), (3, "three"), (2, "two")];
    assert_eq!(bst.size(), 3);
    assert_eq!(bst.find(&1), Some(&"one"));
    assert_eq!(bst.find(&2), Some(&"two"));
    assert_eq!(bst.find(&3), Some(&"three"));
}

#[test]
fn key_value_bst_height_stays_reasonable() {
    let mut bst = BSTreeKeyValue::new();

    // Insert many elements
    for i in 0..100 {
        bst.insert(i, i * 10, rand_priority());
    }

    assert_eq!(bst.size(), 100);
    let height = bst.height();

    // Treap should keep height logarithmic (allow some slack for randomness)
    assert!(height <= 20, "Height {height} too large for 100 elements");

    // Verify all elements are findable
    for i in 0..100 {
        assert_eq!(bst.find(&i), Some(&(i * 10)));
    }
}

#[test]
fn key_value_bst_string_keys() {
    let mut bst = BSTreeKeyValue::new();
    bst.insert("banana".to_string(), 2, rand_priority());
    bst.insert("apple".to_string(), 1, rand_priority());
    bst.insert("cherry".to_string(), 3, rand_priority());

    assert_eq!(bst.size(), 3);
    assert_eq!(bst.find(&"apple".to_string()), Some(&1));
    assert_eq!(bst.find(&"banana".to_string()), Some(&2));
    assert_eq!(bst.find(&"cherry".to_string()), Some(&3));

    // Keys should be in lexicographic order
    let keys = bst.keys();
    assert_eq!(keys.nth(0), &"apple".to_string());
    assert_eq!(keys.nth(1), &"banana".to_string());
    assert_eq!(keys.nth(2), &"cherry".to_string());
}
