//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use rand::{Rng, RngExt};

use apas_verus::BSTReducedStEphLit;

fn rand_priority() -> u64 { rand::rng().random() }
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap40::BSTReducedStEph::BSTReducedStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn reduced_bst_sum_operations() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    assert!(bst.is_empty());
    assert_eq!(bst.size(), 0);
    assert_eq!(bst.reduced_value(), 0); // Sum identity

    // Insert key-value pairs
    bst.insert(5, 50, rand_priority());
    bst.insert(3, 30, rand_priority());
    bst.insert(7, 70, rand_priority());
    bst.insert(1, 10, rand_priority());
    bst.insert(9, 90, rand_priority());

    assert_eq!(bst.size(), 5);
    assert_eq!(bst.reduced_value(), 250); // Sum: 50+30+70+10+90 = 250

    // Test find operations
    assert_eq!(bst.find(&5), Some(&50));
    assert_eq!(bst.find(&10), None);

    // Test range reduction
    assert_eq!(bst.range_reduce(&3, &7), 150); // Sum of values for keys 3,5,7: 30+50+70 = 150
    assert_eq!(bst.range_reduce(&1, &5), 90); // Sum of values for keys 1,3,5: 10+30+50 = 90
    assert_eq!(bst.range_reduce(&8, &10), 90); // Sum of values for keys 9: 90
}

// BSTMaxStEph test removed due to Option<T> not implementing Display

#[test]
fn reduced_bst_count_operations() {
    let mut bst: BSTCountStEph<i32, &str> = BSTreeReduced::new();
    assert!(bst.is_empty());
    assert_eq!(bst.reduced_value(), 0); // Count identity

    // Insert key-value pairs
    bst.insert(5, "five", rand_priority());
    bst.insert(3, "three", rand_priority());
    bst.insert(7, "seven", rand_priority());
    bst.insert(1, "one", rand_priority());
    bst.insert(9, "nine", rand_priority());

    assert_eq!(bst.size(), 5);
    assert_eq!(bst.reduced_value(), 5); // Count: 5 elements

    // Test range reduction
    assert_eq!(bst.range_reduce(&3, &7), 3); // Count of keys 3,5,7: 3
    assert_eq!(bst.range_reduce(&1, &5), 3); // Count of keys 1,3,5: 3
    assert_eq!(bst.range_reduce(&8, &10), 1); // Count of keys 9: 1
    assert_eq!(bst.range_reduce(&10, &15), 0); // No keys in range
}

#[test]
fn reduced_bst_update_existing_key() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    bst.insert(5, 50, rand_priority());
    bst.insert(3, 30, rand_priority());
    assert_eq!(bst.reduced_value(), 80); // 50 + 30

    // Update existing key
    bst.insert(5, 100, rand_priority()); // Update 5 -> 100

    assert_eq!(bst.size(), 2); // Size should not change
    assert_eq!(bst.find(&5), Some(&100)); // Value should be updated
    assert_eq!(bst.reduced_value(), 130); // 100 + 30
}

#[test]
fn reduced_bst_collections() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    bst.insert(3, 300, rand_priority());
    bst.insert(1, 100, rand_priority());
    bst.insert(5, 500, rand_priority());
    bst.insert(2, 200, rand_priority());
    bst.insert(4, 400, rand_priority());

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
    let expected_values = [100, 200, 300, 400, 500];
    for (i, expected) in expected_values.iter().enumerate() {
        assert_eq!(values.nth(i), expected);
    }

    // key_value_pairs method removed due to tuple Display issues

    // Test total sum
    assert_eq!(bst.reduced_value(), 1500); // 100+200+300+400+500
}

#[test]
fn reduced_bst_macro_literal() {
    // Test empty macro
    let empty_bst: BSTSumStEph<i32, i32> = BSTReducedStEphLit![];
    assert!(empty_bst.is_empty());
    assert_eq!(empty_bst.reduced_value(), 0);

    // Test non-empty macro
    let bst: BSTSumStEph<i32, i32> = BSTReducedStEphLit![(1, 10), (3, 30), (2, 20)];
    assert_eq!(bst.size(), 3);
    assert_eq!(bst.find(&1), Some(&10));
    assert_eq!(bst.find(&2), Some(&20));
    assert_eq!(bst.find(&3), Some(&30));
    assert_eq!(bst.reduced_value(), 60); // 10+20+30
}

#[test]
fn reduced_bst_height_stays_reasonable() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    // Insert many elements
    let mut expected_sum = 0;
    for i in 0..100 {
        let value = i * 10;
        bst.insert(i, value, rand_priority());
        expected_sum += value;
    }

    assert_eq!(bst.size(), 100);
    assert_eq!(bst.reduced_value(), expected_sum);

    let height = bst.height();

    // Treap should keep height logarithmic (allow some slack for randomness)
    assert!(height <= 20, "Height {height} too large for 100 elements");

    // Verify all elements are findable
    for i in 0..100 {
        assert_eq!(bst.find(&i), Some(&(i * 10)));
    }
}

#[test]
fn reduced_bst_range_queries() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    // Insert elements: keys 1,2,3,4,5 with values 10,20,30,40,50
    for i in 1..=5 {
        bst.insert(i, i * 10, rand_priority());
    }

    // Test various range queries
    assert_eq!(bst.range_reduce(&1, &5), 150); // All elements: 10+20+30+40+50 = 150
    assert_eq!(bst.range_reduce(&2, &4), 90); // Middle elements: 20+30+40 = 90
    assert_eq!(bst.range_reduce(&1, &1), 10); // Single element: 10
    assert_eq!(bst.range_reduce(&5, &5), 50); // Single element: 50
    assert_eq!(bst.range_reduce(&0, &0), 0); // No elements in range
    assert_eq!(bst.range_reduce(&6, &10), 0); // No elements in range
    assert_eq!(bst.range_reduce(&3, &3), 30); // Single element: 30
}

#[test]
fn reduced_bst_string_keys() {
    let mut bst: BSTSumStEph<String, i32> = BSTreeReduced::new();
    bst.insert("banana".to_string(), 2, rand_priority());
    bst.insert("apple".to_string(), 1, rand_priority());
    bst.insert("cherry".to_string(), 3, rand_priority());

    assert_eq!(bst.size(), 3);
    assert_eq!(bst.reduced_value(), 6); // 1+2+3
    assert_eq!(bst.find(&"apple".to_string()), Some(&1));
    assert_eq!(bst.find(&"banana".to_string()), Some(&2));
    assert_eq!(bst.find(&"cherry".to_string()), Some(&3));

    // Test range query with string keys
    assert_eq!(bst.range_reduce(&"apple".to_string(), &"cherry".to_string()), 6); // All elements
    assert_eq!(bst.range_reduce(&"apple".to_string(), &"banana".to_string()), 3);
    // apple + banana: 1+2
}

#[test]
fn test_contains_method() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    bst.insert(5, 50, rand_priority());
    bst.insert(3, 30, rand_priority());
    bst.insert(7, 70, rand_priority());

    assert!(bst.contains(&5));
    assert!(bst.contains(&3));
    assert!(bst.contains(&7));
    assert!(!bst.contains(&1));
    assert!(!bst.contains(&10));
}

#[test]
fn test_get_method() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    bst.insert(5, 50, rand_priority());
    bst.insert(3, 30, rand_priority());
    bst.insert(7, 70, rand_priority());

    assert_eq!(bst.get(&5), Some(&50));
    assert_eq!(bst.get(&3), Some(&30));
    assert_eq!(bst.get(&7), Some(&70));
    assert_eq!(bst.get(&1), None);
    assert_eq!(bst.get(&10), None);
}

#[test]
fn test_minimum_maximum_keys() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    // Empty tree
    assert_eq!(bst.minimum_key(), None);
    assert_eq!(bst.maximum_key(), None);

    // Insert elements in random order
    bst.insert(5, 50, rand_priority());
    assert_eq!(bst.minimum_key(), Some(&5));
    assert_eq!(bst.maximum_key(), Some(&5));

    bst.insert(3, 30, rand_priority());
    assert_eq!(bst.minimum_key(), Some(&3));
    assert_eq!(bst.maximum_key(), Some(&5));

    bst.insert(7, 70, rand_priority());
    assert_eq!(bst.minimum_key(), Some(&3));
    assert_eq!(bst.maximum_key(), Some(&7));

    bst.insert(1, 10, rand_priority());
    assert_eq!(bst.minimum_key(), Some(&1));
    assert_eq!(bst.maximum_key(), Some(&7));

    bst.insert(9, 90, rand_priority());
    assert_eq!(bst.minimum_key(), Some(&1));
    assert_eq!(bst.maximum_key(), Some(&9));
}

#[test]
fn test_count_reducer_all_operations() {
    let mut bst: BSTCountStEph<i32, &str> = BSTreeReduced::new();

    // Insert and verify count updates
    bst.insert(5, "five", rand_priority());
    assert_eq!(bst.reduced_value(), 1);
    assert_eq!(bst.size(), 1);

    bst.insert(3, "three", rand_priority());
    bst.insert(7, "seven", rand_priority());
    assert_eq!(bst.reduced_value(), 3);
    assert_eq!(bst.size(), 3);

    // Test min/max with string values
    assert_eq!(bst.minimum_key(), Some(&3));
    assert_eq!(bst.maximum_key(), Some(&7));

    // Test contains and get
    assert!(bst.contains(&5));
    assert_eq!(bst.get(&5), Some(&"five"));
    assert!(!bst.contains(&10));
}

#[test]
fn test_empty_tree_operations() {
    let bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    assert!(bst.is_empty());
    assert_eq!(bst.size(), 0);
    assert_eq!(bst.height(), 0);
    assert_eq!(bst.reduced_value(), 0);
    assert_eq!(bst.minimum_key(), None);
    assert_eq!(bst.maximum_key(), None);
    assert!(!bst.contains(&0));
    assert_eq!(bst.find(&0), None);
    assert_eq!(bst.get(&0), None);
    assert_eq!(bst.keys().length(), 0);
    assert_eq!(bst.values().length(), 0);
    assert_eq!(bst.range_reduce(&0, &10), 0);
}

#[test]
fn test_single_element_operations() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();
    bst.insert(42, 100, rand_priority());

    assert!(!bst.is_empty());
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.height(), 1);
    assert_eq!(bst.reduced_value(), 100);
    assert_eq!(bst.minimum_key(), Some(&42));
    assert_eq!(bst.maximum_key(), Some(&42));
    assert!(bst.contains(&42));
    assert_eq!(bst.find(&42), Some(&100));
    assert_eq!(bst.get(&42), Some(&100));

    let keys = bst.keys();
    assert_eq!(keys.length(), 1);
    assert_eq!(keys.nth(0), &42);

    let values = bst.values();
    assert_eq!(values.length(), 1);
    assert_eq!(values.nth(0), &100);

    assert_eq!(bst.range_reduce(&42, &42), 100);
    assert_eq!(bst.range_reduce(&0, &50), 100);
    assert_eq!(bst.range_reduce(&50, &100), 0);
}

#[test]
fn test_large_tree_with_all_operations() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    // Insert 50 elements
    let mut expected_sum = 0;
    for i in 0..50 {
        let value = i * 2;
        bst.insert(i, value, rand_priority());
        expected_sum += value;
    }

    // Verify basic properties
    assert_eq!(bst.size(), 50);
    assert_eq!(bst.reduced_value(), expected_sum);
    assert_eq!(bst.minimum_key(), Some(&0));
    assert_eq!(bst.maximum_key(), Some(&49));

    // Verify all elements are present
    for i in 0..50 {
        assert!(bst.contains(&i));
        assert_eq!(bst.get(&i), Some(&(i * 2)));
        assert_eq!(bst.find(&i), Some(&(i * 2)));
    }

    // Verify keys and values collections
    let keys = bst.keys();
    let values = bst.values();
    assert_eq!(keys.length(), 50);
    assert_eq!(values.length(), 50);

    for i in 0..50 {
        assert_eq!(keys.nth(i), &(i as i32));
        assert_eq!(values.nth(i), &((i as i32) * 2));
    }

    // Test range reductions
    let range_sum_0_10 = (0..=10).map(|i| i * 2).sum::<i32>();
    assert_eq!(bst.range_reduce(&0, &10), range_sum_0_10);

    let range_sum_20_30 = (20..=30).map(|i| i * 2).sum::<i32>();
    assert_eq!(bst.range_reduce(&20, &30), range_sum_20_30);
}

#[test]
fn test_duplicate_keys_overwrite() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    bst.insert(5, 50, rand_priority());
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.reduced_value(), 50);

    // Insert same key with different value
    bst.insert(5, 100, rand_priority());
    assert_eq!(bst.size(), 1); // Size should not change
    assert_eq!(bst.get(&5), Some(&100)); // Value should be updated
    assert_eq!(bst.reduced_value(), 100); // Reduced value should reflect new value

    // Insert again
    bst.insert(5, 200, rand_priority());
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.get(&5), Some(&200));
    assert_eq!(bst.reduced_value(), 200);
}

#[test]
fn test_edge_case_ranges() {
    let mut bst: BSTSumStEph<i32, i32> = BSTreeReduced::new();

    for i in 1..=5 {
        bst.insert(i, i * 10, rand_priority());
    }

    // Range with no elements
    assert_eq!(bst.range_reduce(&10, &20), 0);
    assert_eq!(bst.range_reduce(&-10, &0), 0);

    // Range with single element
    assert_eq!(bst.range_reduce(&3, &3), 30);

    // Range covering all elements
    assert_eq!(bst.range_reduce(&1, &5), 150); // 10+20+30+40+50

    // Range with partial coverage
    assert_eq!(bst.range_reduce(&2, &4), 90); // 20+30+40

    // Range extending beyond tree bounds
    assert_eq!(bst.range_reduce(&-10, &10), 150); // All elements
    assert_eq!(bst.range_reduce(&3, &100), 120); // 30+40+50
}

#[test]
fn test_sumop_trait() {
    assert_eq!(<SumOp<i32> as ReduceOp<i32, i32>>::identity(), 0);
    assert_eq!(<SumOp<i32> as ReduceOp<i32, i32>>::combine(5, 10), 15);
    assert_eq!(<SumOp<i32> as ReduceOp<i32, i32>>::lift(&42), 42);
}

#[test]
fn test_countop_trait() {
    assert_eq!(<CountOp<i32> as ReduceOp<i32, N>>::identity(), 0);
    assert_eq!(<CountOp<i32> as ReduceOp<i32, N>>::combine(3, 7), 10);
    assert_eq!(<CountOp<i32> as ReduceOp<i32, N>>::lift(&42), 1);
}
