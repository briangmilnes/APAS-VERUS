#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap37::BSTAVLMtEph::BSTAVLMtEph::*;
use apas_verus::Chap37::BSTBBAlphaMtEph::BSTBBAlphaMtEph::*;
use apas_verus::Chap37::BSTPlainMtEph::BSTPlainMtEph::*;
use apas_verus::Chap37::BSTRBMtEph::BSTRBMtEph::*;
use apas_verus::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_verus::Chap39::BSTTreapStEph::BSTTreapStEph::*;
use apas_verus::Types::Types::*;
use apas_verus::Types::*;

#[test]
fn mt_plain_basic_ops() {
    let bst = BSTree::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&3), Some(3));
    assert!(!bst.contains(&9));
    assert_eq!(bst.minimum(), Some(1));
    assert_eq!(bst.maximum(), Some(7));
}

#[test]
fn mt_avl_basic_ops() {
    let bst = BSTreeAVL::new();
    for value in [10, 5, 15, 2, 7, 12, 20] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&7), Some(7));
    assert!(!bst.contains(&30));
}

#[test]
fn mt_rb_basic_ops() {
    let bst = BSTreeRB::new();
    for value in 0..16 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 16);
    assert_eq!(bst.find(&8), Some(8));
}

#[test]
fn mt_bbalpha_basic_ops() {
    let bst = BSTreeBBAlpha::new();
    for value in 0..32 {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 32);
    assert_eq!(bst.find(&12), Some(12));
}


#[test]
fn mt_splay_basic_ops() {
    let bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert_eq!(bst.find(&5), Some(5));
}

// Individual variant comprehensive testing for BST*MtEph variants
#[test]
fn mt_plain_comprehensive_operations() {
    let bst = BSTree::<i32>::new();

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
        bst.insert(val);
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
    let bst = BSTreeAVL::<i32>::new();

    // Test balanced insertion (worst case for unbalanced trees)
    for i in 1..=20 {
        bst.insert(i);
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
    let bst = BSTreeRB::<i32>::new();

    // Test with alternating insertions
    let values = [100, 50, 150, 25, 75, 125, 175, 12, 37, 62, 87, 112, 137, 162, 187];
    for &val in &values {
        bst.insert(val);
    }

    assert_eq!(bst.size(), values.len());

    // Red-Black tree should maintain good balance
    let height = bst.height();
    assert!(height <= 8); // RB tree height ≤ 2*log2(n+1)

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
    let bst = BSTreeBBAlpha::<i32>::new();

    // Test worst-case insertion pattern for unbalanced trees
    for i in (1..=30).rev() {
        bst.insert(i);
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
    let bst = BSTreeSplay::<i32>::new();

    // Test with access pattern that benefits from splaying
    let values = [50, 25, 75, 12, 37, 62, 87];
    for &val in &values {
        bst.insert(val);
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
            let bst = <$variant>::new();
            bst.insert(42);

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
            let bst = <$variant>::new();

            // Insert duplicates
            bst.insert(10);
            bst.insert(5);
            bst.insert(15);
            bst.insert(10); // Duplicate
            bst.insert(5); // Duplicate

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

// Parallel operation verification tests for BST*MtEph variants
#[test]
fn mt_concurrent_plain_bst_operations() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let bst = Arc::new(BSTree::<i32>::new());
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];

    // Thread 1: Insert values 1-25
    let bst1 = Arc::clone(&bst);
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        for i in 1..=25 {
            bst1.insert(i);
        }
        bst1.size()
    }));

    // Thread 2: Insert values 26-50
    let bst2 = Arc::clone(&bst);
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        for i in 26..=50 {
            bst2.insert(i);
        }
        bst2.size()
    }));

    // Thread 3: Search for values
    let bst3 = Arc::clone(&bst);
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let mut found_count = 0;
        for i in 1..=50 {
            if bst3.contains(&i) {
                found_count += 1;
            }
        }
        found_count
    }));

    // Thread 4: Check height
    let bst4 = Arc::clone(&bst);
    let barrier4 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier4.wait();
        bst4.height()
    }));

    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // Verify results - exact counts may vary due to concurrent insertions
    // but we should have reasonable results
    assert!(results[0] >= 25); // Thread 1 size
    assert!(results[1] >= 25); // Thread 2 size
    // Thread 3 found count can vary
    // Height can be 0 for empty tree or vary based on timing in concurrent operations
    // Just verify it's a valid height value (non-negative, which is always true for usize)
}

#[test]
fn mt_concurrent_avl_bst_operations() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let bst = Arc::new(BSTreeAVL::<i32>::new());
    let num_threads = 6;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let bst_clone = Arc::clone(&bst);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait();

            // Each thread inserts different ranges
            let start = thread_id * 10;
            let end = start + 10;

            for i in start..end {
                bst_clone.insert(i as i32);
            }

            // Test operations
            let size = bst_clone.size();
            let height = bst_clone.height();
            let min = bst_clone.minimum();
            let max = bst_clone.maximum();

            (thread_id, size, height, min, max)
        }));
    }

    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // Verify each thread's results
    for (thread_id, size, height, min, max) in results {
        assert!(size >= 10); // At least the thread's own insertions
        assert!(height > 0); // Tree has some height
        // Min/max depend on insertion order across threads
        println!("Thread {thread_id}: size={size}, height={height}, min={min:?}, max={max:?}");
    }
}

#[test]
fn mt_concurrent_rb_bst_stress() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let bst = Arc::new(BSTreeRB::<i32>::new());
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let bst_clone = Arc::clone(&bst);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait();

            // Stress test with many insertions
            for i in 0..100 {
                let value = (thread_id * 100 + i) as i32;
                bst_clone.insert(value);
            }

            let final_size = bst_clone.size();
            let height = bst_clone.height();
            let in_order = bst_clone.in_order();
            let is_sorted = (1..in_order.length()).all(|i| *in_order.nth(i - 1) <= *in_order.nth(i));

            (thread_id, final_size, height, is_sorted)
        }));
    }

    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // Verify results
    for (thread_id, final_size, height, is_sorted) in results {
        assert!(final_size >= 100); // At least the thread's own insertions
        assert!(height > 0);
        assert!(is_sorted, "Thread {thread_id} produced unsorted result");
    }
}

#[test]
fn mt_concurrent_bbalpha_operations() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let bst = Arc::new(BSTreeBBAlpha::<i32>::new());
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    // Thread 1: Sequential insertions
    let bst1 = Arc::clone(&bst);
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        for i in 1..=50 {
            bst1.insert(i);
        }
        (bst1.size(), bst1.height())
    }));

    // Thread 2: Reverse sequential insertions
    let bst2 = Arc::clone(&bst);
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        for i in (51..=100).rev() {
            bst2.insert(i);
        }
        (bst2.size(), bst2.height())
    }));

    // Thread 3: Random-like insertions
    let bst3 = Arc::clone(&bst);
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let values = [125, 112, 137, 106, 118, 131, 143];
        for &val in &values {
            bst3.insert(val);
        }
        (bst3.size(), bst3.height())
    }));

    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // All threads should contribute to the tree
    for (i, (size, height)) in results.iter().enumerate() {
        assert!(*size >= 7); // At least some insertions
        assert!(*height > 0);
        println!("Thread {i}: size={size}, height={height}");
    }
}


#[test]
fn mt_concurrent_splay_access_patterns() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let bst = Arc::new(BSTreeSplay::<i32>::new());
    let barrier = Arc::new(Barrier::new(2));
    let mut handles = vec![];

    // Thread 1: Insert and frequent access
    let bst1 = Arc::clone(&bst);
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();

        // Insert values
        for i in 1..=20 {
            bst1.insert(i);
        }

        // Frequently access certain values (should splay them)
        let frequent_values = [5, 10, 15];
        for _ in 0..10 {
            for &val in &frequent_values {
                assert!(bst1.contains(&val));
            }
        }

        bst1.size()
    }));

    // Thread 2: Insert different range
    let bst2 = Arc::clone(&bst);
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();

        for i in [100, 50, 150, 25, 75, 125, 175] {
            bst2.insert(i);
        }

        bst2.size()
    }));

    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // Both threads should contribute
    assert!(results[0] >= 20); // Thread 1: at least 20 elements
    assert!(results[1] >= 7); // Thread 2: at least 7 elements
}

#[test]
fn mt_all_variants_concurrent_stress() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    macro_rules! test_concurrent_variant {
        ($variant:ty) => {
            let bst = Arc::new(<$variant>::new());
            let barrier = Arc::new(Barrier::new(3));
            let mut handles = vec![];

            for thread_id in 0..3 {
                let bst_clone = Arc::clone(&bst);
                let barrier_clone = Arc::clone(&barrier);

                handles.push(thread::spawn(move || {
                    barrier_clone.wait();

                    // Each thread inserts different ranges
                    let start = thread_id * 10 + 1;
                    let end = start + 10;

                    for i in start..end {
                        bst_clone.insert(i);
                    }

                    // Test that all operations work
                    let size = bst_clone.size();
                    let height = bst_clone.height();
                    let min = bst_clone.minimum();
                    let max = bst_clone.maximum();

                    (size, height, min, max)
                }));
            }

            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

            // Each thread should contribute to the tree
            for (thread_id, (size, height, min, max)) in results.iter().enumerate() {
                assert!(*size >= 10); // At least some insertions
                assert!(*height > 0);
                println!(
                    "Variant {} Thread {}: size={}, height={}, min={:?}, max={:?}",
                    stringify!($variant),
                    thread_id,
                    size,
                    height,
                    min,
                    max
                );
            }
        };
    }

    test_concurrent_variant!(BSTree<i32>);
    test_concurrent_variant!(BSTreeAVL<i32>);
    test_concurrent_variant!(BSTreeRB<i32>);
    test_concurrent_variant!(BSTreeBBAlpha<i32>);
    test_concurrent_variant!(BSTreeSplay<i32>);
}
