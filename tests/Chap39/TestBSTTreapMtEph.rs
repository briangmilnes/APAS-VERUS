//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTTreapMtEph.

use std::time::Duration;

use rand::{Rng, RngExt};

use apas_verus::BSTTreapMtEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;

fn rand_priority() -> u64 { rand::rng().random() }

#[test]
fn test_macro_empty() {
    let empty: BSTTreapMtEph<i32> = BSTTreapMtEphLit![];
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());
}

#[test]
fn test_macro_with_elements() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEphLit![5, 3, 7, 1, 9];
    assert_eq!(tree.size(), 5);
    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
}

#[test]
fn test_new_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_default() {
    let tree: BSTTreapMtEph<i32> = Default::default();
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn test_insert_and_size() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    assert_eq!(tree.size(), 1);
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());
    assert_eq!(tree.size(), 3);
}

#[test]
fn test_find() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());

    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&7), Some(7));
    assert_eq!(tree.find(&10), None);
}

#[test]
fn test_contains() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());

    assert!(tree.contains(&5));
    assert!(tree.contains(&3));
    assert!(tree.contains(&7));
    assert!(!tree.contains(&10));
    assert!(!tree.contains(&0));
}

#[test]
fn test_is_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert!(tree.is_empty());
    tree.insert(5, rand_priority());
    assert!(!tree.is_empty());
}

#[test]
fn test_minimum() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.minimum(), None);

    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());
    tree.insert(1, rand_priority());
    tree.insert(9, rand_priority());

    assert_eq!(tree.minimum(), Some(1));
}

#[test]
fn test_maximum() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.maximum(), None);

    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());
    tree.insert(1, rand_priority());
    tree.insert(9, rand_priority());

    assert_eq!(tree.maximum(), Some(9));
}

#[test]
fn test_height() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    assert_eq!(tree.height(), 0);

    tree.insert(5, rand_priority());
    assert!(tree.height() >= 1);

    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());
    assert!(tree.height() >= 1);
}

#[test]
fn test_height_balanced() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    for i in 0..64 {
        tree.insert(i, rand_priority());
    }
    let height = tree.height();
    assert!(height < 20, "Height {height} is too large for 64 elements");
}

#[test]
fn test_in_order() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());
    tree.insert(1, rand_priority());
    tree.insert(9, rand_priority());

    let seq = tree.in_order();
    assert_eq!(seq.length(), 5);
    let expected = [1, 3, 5, 7, 9];
    for (i, &exp) in expected.iter().enumerate() {
        assert_eq!(*seq.nth(i), exp);
    }
}

#[test]
fn test_pre_order() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(7, rand_priority());

    let seq = tree.pre_order();
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_duplicate_insert() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(5, rand_priority());
    tree.insert(5, rand_priority());

    assert!(tree.size() >= 1);
    assert!(tree.contains(&5));
}

#[test]
fn test_single_element() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(42, rand_priority());

    assert_eq!(tree.size(), 1);
    assert_eq!(tree.minimum(), Some(42));
    assert_eq!(tree.maximum(), Some(42));
    assert_eq!(tree.height(), 1);
}

#[test]
fn test_large_tree() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    for i in 0..100 {
        tree.insert(i, rand_priority());
    }
    assert_eq!(tree.size(), 100);
    assert_eq!(tree.minimum(), Some(0));
    assert_eq!(tree.maximum(), Some(99));
}

#[test]
fn test_negative_numbers() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(-5, rand_priority());
    tree.insert(-10, rand_priority());
    tree.insert(-3, rand_priority());
    tree.insert(-1, rand_priority());

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.minimum(), Some(-10));
    assert_eq!(tree.maximum(), Some(-1));
    assert!(tree.contains(&-5));
}

#[test]
fn test_mixed_positive_negative() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(-3, rand_priority());
    tree.insert(0, rand_priority());
    tree.insert(-10, rand_priority());
    tree.insert(15, rand_priority());

    assert_eq!(tree.minimum(), Some(-10));
    assert_eq!(tree.maximum(), Some(15));
    assert!(tree.contains(&0));
}

#[test]
fn test_empty_operations() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();

    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.minimum(), None);
    assert_eq!(tree.maximum(), None);
    assert_eq!(tree.find(&5), None);
    assert!(!tree.contains(&5));

    let seq = tree.in_order();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_clone_shares_state() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    let cloned = tree.clone();

    // Clone shares the Arc, so inserts on one are visible on the other.
    tree.insert(10, rand_priority());
    assert!(cloned.contains(&10));
}

#[test]
fn test_string_keys() {
    let tree: BSTTreapMtEph<String> = BSTTreapMtEph::new();
    tree.insert("dog".to_string(), rand_priority());
    tree.insert("cat".to_string(), rand_priority());
    tree.insert("elephant".to_string(), rand_priority());
    tree.insert("ant".to_string(), rand_priority());

    assert_eq!(tree.size(), 4);
    assert_eq!(tree.minimum(), Some("ant".to_string()));
    assert_eq!(tree.maximum(), Some("elephant".to_string()));
    assert!(tree.contains(&"dog".to_string()));
}

#[test]
fn test_extremes() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(i32::MAX, rand_priority());
    tree.insert(i32::MIN, rand_priority());
    tree.insert(0, rand_priority());

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.minimum(), Some(i32::MIN));
    assert_eq!(tree.maximum(), Some(i32::MAX));
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_concurrent_readers() {
    use std::thread;

    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    for i in 0..50 {
        tree.insert(i, rand_priority());
    }

    let handles: Vec<_> = (0..4).map(|_| {
        let t = tree.clone();
        thread::spawn(move || {
            for i in 0..50 {
                assert!(t.contains(&i));
            }
            assert_eq!(t.size(), 50);
        })
    }).collect();

    for h in handles {
        h.join().expect("reader thread panicked");
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_concurrent_writer_and_readers() {
    use std::sync::Arc;
    use std::thread;

    let tree: BSTTreapMtEph<i64> = BSTTreapMtEph::new();
    let tree_arc = Arc::new(tree);

    let writer = {
        let t = tree_arc.clone();
        thread::spawn(move || {
            for i in 0..100i64 {
                t.insert(i, rand_priority());
                thread::sleep(Duration::from_micros(10));
            }
        })
    };

    let readers: Vec<_> = (0..3).map(|_| {
        let t = tree_arc.clone();
        thread::spawn(move || {
            for _ in 0..50 {
                let _sz = t.size();
                let _ = t.minimum();
                let _ = t.maximum();
                thread::sleep(Duration::from_micros(5));
            }
        })
    }).collect();

    writer.join().expect("writer thread panicked");
    for r in readers {
        r.join().expect("reader thread panicked");
    }

    assert_eq!(tree_arc.size(), 100);
}

#[test]
fn test_traversal_empty() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();

    let in_seq = tree.in_order();
    assert_eq!(in_seq.length(), 0);

    let pre_seq = tree.pre_order();
    assert_eq!(pre_seq.length(), 0);
}

#[test]
fn test_singleton_traversal() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(42, rand_priority());

    let in_seq = tree.in_order();
    assert_eq!(in_seq.length(), 1);
    assert_eq!(*in_seq.nth(0), 42);

    let pre_seq = tree.pre_order();
    assert_eq!(pre_seq.length(), 1);
    assert_eq!(*pre_seq.nth(0), 42);
}

#[test]
fn test_reverse_order_insert() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    tree.insert(5, rand_priority());
    tree.insert(4, rand_priority());
    tree.insert(3, rand_priority());
    tree.insert(2, rand_priority());
    tree.insert(1, rand_priority());

    assert_eq!(tree.size(), 5);
    assert_eq!(tree.minimum(), Some(1));
    assert_eq!(tree.maximum(), Some(5));
}

#[test]
fn test_contains_step_pattern() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    for i in (0..100).step_by(3) {
        tree.insert(i, rand_priority());
    }

    for i in (0..100).step_by(3) {
        assert!(tree.contains(&i));
    }

    assert!(!tree.contains(&1));
    assert!(!tree.contains(&2));
    assert!(!tree.contains(&100));
}

#[test]
fn test_debug_format() {
    let tree: BSTTreapMtEph<i32> = BSTTreapMtEph::new();
    let debug = format!("{:?}", tree);
    assert!(debug.contains("BSTTreapMtEph"));
}
