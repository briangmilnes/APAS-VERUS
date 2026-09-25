// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_verus::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
use apas_verus::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_verus::Chap37::BSTRBMtEph::BSTRBMtEph::*;
use apas_verus::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_verus::Chap39::BSTTreapStEph::BSTTreapStEph::*;
use apas_verus::Types::Types::*;
use apas_verus::Types::*;

mod bst_balance_check;

type BSTree<T> = BSTPlainMtEph<T>;
type BSTreeAVL<T> = BSTAVLMtEph<T>;
type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;

#[test]
fn mt_plain_basic_ops() {
    let mut bst = BSTree::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value).unwrap();
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&3), Some(3));
    assert!(!bst.contains(&9));
    assert_eq!(bst.minimum(), Some(1));
    assert_eq!(bst.maximum(), Some(7));
}

#[test]
fn mt_avl_basic_ops() {
    let mut bst = BSTreeAVL::new();
    for value in [10, 5, 15, 2, 7, 12, 20] {
        bst.insert(value).unwrap();
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&7), Some(7));
    assert!(!bst.contains(&30));
}

#[test]
fn mt_rb_basic_ops() {
    let mut bst = BSTreeRB::new();
    for value in 0..16 {
        bst.insert(value).unwrap();
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&8), Some(8));
}

#[test]
fn mt_bbalpha_basic_ops() {
    let mut bst = BSTreeBBAlpha::new();
    for value in 0..32 {
        bst.insert(value).unwrap();
    }
    assert_eq!(bst.size(), 32);
    assert_eq!(bst.find(&12), Some(12));
}


#[test]
fn mt_splay_basic_ops() {
    let mut bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value).unwrap();
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&5), Some(5));
}

// Individual variant comprehensive testing for BST*MtEph variants
#[test]
fn mt_plain_comprehensive_operations() {
    let mut bst = BSTree::<i32>::new();

    // Test empty state
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
    assert_eq!(bst.height(), 0);
    assert_eq!(bst.minimum(), None);
    assert_eq!(bst.maximum(), None);
    assert_eq!(bst.find(&42), None);
    assert!(!bst.contains(&42));

    // Test insertions
    let values = [50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43];
    for &val in &values {
        bst.insert(val).unwrap();
    }

    assert_eq!(bst.size(), values.len());
    assert!(!bst.is_empty());
    assert!(bst.height() > 0);
    assert_eq!(bst.minimum(), Some(6));
    assert_eq!(bst.maximum(), Some(87));

    // Test find and contains
    for &val in &values {
        assert_eq!(bst.find(&val), Some(val));
        assert!(bst.contains(&val));
    }
    assert_eq!(bst.find(&99), None);
    assert!(!bst.contains(&99));

    // Note: BSTPlainMtEph doesn't have delete method - test only insertion and search
    // Verify all inserted values are present
    for &val in &values {
        assert!(bst.contains(&val));
    }

    // Test in-order traversal
    let in_order = bst.in_order();
    for i in 1..in_order.length() {
        assert!(*in_order.nth(i - 1) <= *in_order.nth(i));
    }

    // Note: BSTPlainMtEph doesn't have pre_order method - only in_order available
}

#[test]
fn mt_avl_comprehensive_operations() {
    let mut bst = BSTreeAVL::<i32>::new();

    // Test balanced insertion (worst case for unbalanced trees)
    for i in 1..=20 {
        bst.insert(i).unwrap();
    }

    assert_eq!(bst.size(), 20);
    assert_eq!(bst.minimum(), Some(1));
    assert_eq!(bst.maximum(), Some(20));

    // AVL should maintain better balance than plain BST
    let height = bst.height();
    assert!(height <= 6); // log2(20) ≈ 4.3, AVL allows height ≤ 1.44*log2(n)

    // Test that all elements are accessible
    for i in 1..=20 {
        assert!(bst.contains(&i));
        assert_eq!(bst.find(&i), Some(i));
    }

    // Note: BSTAVLMtEph doesn't have delete method - test balance properties only
    // Verify all elements are accessible and tree maintains balance
    for i in 1..=20 {
        assert!(bst.contains(&i));
        assert_eq!(bst.find(&i), Some(i));
    }

    // Test in-order traversal maintains sorted order
    let in_order = bst.in_order();
    for i in 1..in_order.length() {
        assert!(*in_order.nth(i - 1) < *in_order.nth(i));
    }
}

#[test]
fn mt_rb_comprehensive_operations() {
    let mut bst = BSTreeRB::<i32>::new();

    // Test with alternating insertions
    let values = [100, 50, 150, 25, 75, 125, 175, 12, 37, 62, 87, 112, 137, 162, 187];
    for &val in &values {
        bst.insert(val).unwrap();
    }

    assert_eq!(bst.size(), values.len());

    // Red-Black tree should maintain good balance
    let height = bst.height();
    assert!(bst_balance_check::height_within_two_lg(bst.size(), height)); // RB tree height <= 2 lg(n + 1)

    // Test comprehensive search
    for &val in &values {
        assert_eq!(bst.find(&val), Some(val));
        assert!(bst.contains(&val));
    }

    // Test edge values
    assert_eq!(bst.minimum(), Some(12));
    assert_eq!(bst.maximum(), Some(187));

    // Note: BSTRBMtEph doesn't have delete method - test comprehensive search only
    // Verify all values are accessible
    for &val in &values {
        assert!(bst.contains(&val));
        assert_eq!(bst.find(&val), Some(val));
    }

    // Verify tree is still sorted
    let in_order = bst.in_order();
    for i in 1..in_order.length() {
        assert!(*in_order.nth(i - 1) < *in_order.nth(i));
    }
}

#[test]
fn mt_bbalpha_comprehensive_operations() {
    let mut bst = BSTreeBBAlpha::<i32>::new();

    // Test worst-case insertion pattern for unbalanced trees
    for i in (1..=30).rev() {
        bst.insert(i).unwrap();
    }

    assert_eq!(bst.size(), 30);

    // BB[α] should maintain balance despite worst-case insertion
    let height = bst.height();
    assert!(height <= 7); // Should be well-balanced

    // Verify all elements in correct order
    let in_order = bst.in_order();
    for i in 0..30 {
        assert_eq!(*in_order.nth(i), (i + 1) as i32);
    }

    // Note: BSTBBAlphaMtEph doesn't have delete method - test balance properties only
    // Verify all elements are accessible
    for i in 1..=30 {
        assert!(bst.contains(&i));
        assert_eq!(bst.find(&i), Some(i));
    }

    // Test pre-order traversal
    let pre_order = bst.pre_order();
    assert_eq!(pre_order.length(), 30);
}


#[test]
fn mt_splay_comprehensive_operations() {
    let mut bst = BSTreeSplay::<i32>::new();

    // Test with access pattern that benefits from splaying
    let values = [50, 25, 75, 12, 37, 62, 87];
    for &val in &values {
        bst.insert(val).unwrap();
    }

    assert_eq!(bst.size(), values.len());

    // Test frequent access (should splay frequently accessed nodes to root)
    let frequent_values = [25, 75];
    for _ in 0..5 {
        for &val in &frequent_values {
            assert!(bst.contains(&val));
        }
    }

    // Test all basic operations
    assert_eq!(bst.minimum(), Some(12));
    assert_eq!(bst.maximum(), Some(87));

    for &val in &values {
        assert_eq!(bst.find(&val), Some(val));
        assert!(bst.contains(&val));
    }

    // Note: BSTSplayMtEph doesn't have delete method - test splay behavior only
    // Verify all values are accessible
    for &val in &values {
        assert!(bst.contains(&val));
        assert_eq!(bst.find(&val), Some(val));
    }

    // Verify tree is still sorted
    let in_order = bst.in_order();
    for i in 1..in_order.length() {
        assert!(*in_order.nth(i - 1) < *in_order.nth(i));
    }
}

#[test]
fn mt_all_variants_empty_operations() {
    // Test that all variants handle empty operations correctly
    macro_rules! test_empty_variant {
        ($variant:ty) => {
            let bst = <$variant>::new();
            assert_eq!(bst.size(), 0);
            assert_eq!(bst.is_empty(), true);
            assert_eq!(bst.height(), 0);
            assert_eq!(bst.minimum(), None);
            assert_eq!(bst.maximum(), None);
            assert_eq!(bst.find(&42), None);
            assert_eq!(bst.contains(&42), false);

            let in_order = bst.in_order();
            assert_eq!(in_order.length(), 0);

            // Note: Not all BST variants have pre_order method
        };
    }

    test_empty_variant!(BSTree<i32>);
    test_empty_variant!(BSTreeAVL<i32>);
    test_empty_variant!(BSTreeRB<i32>);
    test_empty_variant!(BSTreeBBAlpha<i32>);
    test_empty_variant!(BSTreeSplay<i32>);
}

#[test]
fn mt_all_variants_single_element() {
    // Test that all variants handle single element correctly
    macro_rules! test_single_variant {
        ($variant:ty) => {
            let mut bst = <$variant>::new();
            bst.insert(42).unwrap();

            assert_eq!(bst.size(), 1);
            assert_eq!(bst.is_empty(), false);
            assert_eq!(bst.height(), 1);
            assert_eq!(bst.minimum(), Some(42));
            assert_eq!(bst.maximum(), Some(42));
            assert_eq!(bst.find(&42), Some(42));
            assert_eq!(bst.contains(&42), true);
            assert_eq!(bst.contains(&99), false);

            let in_order = bst.in_order();
            assert_eq!(in_order.length(), 1);
            assert_eq!(*in_order.nth(0), 42);

            // Note: Not all BST variants have pre_order method
        };
    }

    test_single_variant!(BSTree<i32>);
    test_single_variant!(BSTreeAVL<i32>);
    test_single_variant!(BSTreeRB<i32>);
    test_single_variant!(BSTreeBBAlpha<i32>);
    test_single_variant!(BSTreeSplay<i32>);
}

#[test]
fn mt_all_variants_duplicate_handling() {
    // Test that all variants handle duplicates correctly
    macro_rules! test_duplicate_variant {
        ($variant:ty) => {
            let mut bst = <$variant>::new();

            // Insert duplicates
            bst.insert(10).unwrap();
            bst.insert(5).unwrap();
            bst.insert(15).unwrap();
            bst.insert(10).unwrap(); // Duplicate
            bst.insert(5).unwrap(); // Duplicate

            // Size should reflect actual unique elements (behavior may vary by implementation)
            assert!(bst.size() >= 3); // At least the unique elements
            assert_eq!(bst.contains(&10), true);
            assert_eq!(bst.contains(&5), true);
            assert_eq!(bst.contains(&15), true);

            // Tree should still be sorted
            let in_order = bst.in_order();
            for i in 1..in_order.length() {
                assert!(*in_order.nth(i - 1) <= *in_order.nth(i));
            }
        };
    }

    test_duplicate_variant!(BSTree<i32>);
    test_duplicate_variant!(BSTreeAVL<i32>);
    test_duplicate_variant!(BSTreeRB<i32>);
    test_duplicate_variant!(BSTreeBBAlpha<i32>);
    test_duplicate_variant!(BSTreeSplay<i32>);
}

// Concurrent readers. Insert takes &mut self, so one owner builds the tree;
// the tree is then shared through Arc and queried by several threads at once.
// Every reader must observe exactly the contents the owner inserted.
macro_rules! concurrent_readers {
    ($variant:ty, $values:expr, $readers:expr) => {{
        use std::sync::{Arc, Barrier};
        use std::thread;

        let values: Vec<i32> = $values;
        let mut bst = <$variant>::new();
        for &v in &values {
            bst.insert(v).unwrap();
        }
        let mut expected = values.clone();
        expected.sort();
        expected.dedup();
        let expected_height = bst.height();

        let bst = Arc::new(bst);
        let barrier = Arc::new(Barrier::new($readers));
        let handles: Vec<_> = (0..$readers)
            .map(|_| {
                let bst = Arc::clone(&bst);
                let barrier = Arc::clone(&barrier);
                let values = values.clone();
                thread::spawn(move || {
                    barrier.wait();
                    let found = values.iter().filter(|&&v| bst.contains(&v)).count();
                    let absent_found = bst.contains(&-1);
                    let in_order = bst.in_order();
                    let seq: Vec<i32> = (0..in_order.length()).map(|i| *in_order.nth(i)).collect();
                    (bst.size(), bst.height(), bst.minimum(), bst.maximum(), found, absent_found, seq)
                })
            })
            .collect();

        for h in handles {
            let (size, height, min, max, found, absent_found, seq) = h.join().unwrap();
            assert_eq!(size, expected.len());
            assert_eq!(height, expected_height);
            assert_eq!(min, expected.first().copied());
            assert_eq!(max, expected.last().copied());
            assert_eq!(found, values.len());
            assert!(!absent_found);
            assert_eq!(seq, expected);
        }
    }};
}

#[test]
fn mt_concurrent_plain_bst_operations() {
    concurrent_readers!(BSTree<i32>, (1..=50).collect(), 4);
}

#[test]
fn mt_concurrent_avl_bst_operations() {
    concurrent_readers!(BSTreeAVL<i32>, (0..60).collect(), 6);
}

#[test]
fn mt_concurrent_rb_bst_stress() {
    concurrent_readers!(BSTreeRB<i32>, (0..800).collect(), 8);
}

#[test]
fn mt_concurrent_bbalpha_operations() {
    let values: Vec<i32> = (1..=50).chain((51..=100).rev()).chain([125, 112, 137, 106, 118, 131, 143]).collect();
    concurrent_readers!(BSTreeBBAlpha<i32>, values, 3);
}

#[test]
fn mt_concurrent_splay_access_patterns() {
    let values: Vec<i32> = (1..=20).chain([100, 50, 150, 25, 75, 125, 175]).collect();
    concurrent_readers!(BSTreeSplay<i32>, values, 2);
}

#[test]
fn mt_all_variants_concurrent_stress() {
    concurrent_readers!(BSTree<i32>, (1..=30).collect(), 3);
    concurrent_readers!(BSTreeAVL<i32>, (1..=30).collect(), 3);
    concurrent_readers!(BSTreeRB<i32>, (1..=30).collect(), 3);
    concurrent_readers!(BSTreeBBAlpha<i32>, (1..=30).collect(), 3);
    concurrent_readers!(BSTreeSplay<i32>, (1..=30).collect(), 3);
}
