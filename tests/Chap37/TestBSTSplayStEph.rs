//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_verus::BSTSplayStEphLit;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerBaseTrait, *};
use apas_verus::Chap37::BSTSplayStEph::BSTSplayStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_bstsplaystephlit_macro_functionality() {
    // Test empty tree creation
    let empty: BSTSplayStEph<i32> = BSTSplayStEphLit![];
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: BSTSplayStEph<i32> = BSTSplayStEphLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert!(with_data.contains(&5));
    assert!(with_data.contains(&3));
    assert!(with_data.contains(&7));
    assert!(!with_data.contains(&10));
}

#[test]
fn splay_basic_behaviour() {
    let mut bst = BSTreeSplay::new();
    for value in [4, 2, 6, 1, 3, 5, 7] {
        bst.insert(value);
    }
    assert_eq!(bst.size(), 7);
    assert!(bst.contains(&3));
    assert_eq!(bst.find(&9), None);
    assert_eq!(bst.minimum(), Some(&1));
    assert_eq!(bst.maximum(), Some(&7));
    let inorder = bst.in_order();
    let expected = [1, 2, 3, 4, 5, 6, 7];
    assert_eq!(inorder.length(), expected.len());
    for (exp, value) in expected.iter().zip(inorder.iter()) {
        assert_eq!(*value, *exp);
    }
}

#[test]
fn splay_duplicate_insert_is_idempotent() {
    let mut bst = BSTreeSplay::new();
    bst.insert(10);
    bst.insert(10);
    assert_eq!(bst.size(), 1);
    assert_eq!(bst.find(&10), Some(&10));
}

#[test]
fn test_empty() {
    let bst = BSTreeSplay::<i32>::new();
    assert_eq!(bst.size(), 0);
    assert!(bst.is_empty());
}

#[test]
fn test_large_tree() {
    let mut bst = BSTreeSplay::new();
    for i in 0..100 {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 100);
    assert!(bst.contains(&50));
}

#[test]
fn test_reverse_insert() {
    let mut bst = BSTreeSplay::new();
    for i in (0..50).rev() {
        bst.insert(i);
    }
    assert_eq!(bst.size(), 50);
    assert_eq!(bst.minimum(), Some(&0));
}

#[test]
fn test_negative_numbers() {
    let mut bst = BSTreeSplay::new();
    bst.insert(-5);
    bst.insert(-3);
    bst.insert(-7);
    assert!(bst.contains(&-5));
    assert_eq!(bst.minimum(), Some(&-7));
    assert_eq!(bst.maximum(), Some(&-3));
}

#[test]
fn test_singleton() {
    let mut bst = BSTreeSplay::new();
    bst.insert(42);
    assert_eq!(bst.size(), 1);
    assert!(bst.contains(&42));
}

#[test]
fn test_pre_order() {
    let mut bst = BSTreeSplay::new();
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    let pre = bst.pre_order();
    assert_eq!(pre.length(), 3);
}

#[test]
fn test_height() {
    let mut bst = BSTreeSplay::new();
    for i in 0..64 {
        bst.insert(i);
    }
    let h = bst.height();
    assert!(h >= 6);
}
