//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for BSTParaTreapMtEph.

use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
use apas_verus::ParamTreapLit;
use apas_verus::Types::Types::*;

#[test]
fn test_paramtreaplit_macro_functionality() {
    // Test empty tree creation
    let empty: ParamTreap<i32> = ParamTreapLit!();
    assert_eq!(empty.size(), 0);

    // Test tree creation with elements
    let with_data: ParamTreap<i32> = ParamTreapLit![5, 3, 7, 1, 9];
    assert_eq!(with_data.size(), 5);
    assert_eq!(with_data.find(&5), Some(5));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&7), Some(7));
    assert_eq!(with_data.find(&10), None);
}

#[test]
fn test_new() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    assert!(tree.is_empty());
    assert_eq!(tree.size(), 0);
}

#[test]
fn test_insert() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(8);

    assert_eq!(tree.size(), 5);
    assert!(!tree.is_empty());
}

#[test]
fn test_find() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    assert_eq!(tree.find(&10), Some(10));
    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&15), Some(15));
    assert_eq!(tree.find(&20), None);
}

#[test]
fn test_delete() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(8);

    assert_eq!(tree.size(), 5);

    tree.delete(&5);
    assert_eq!(tree.size(), 4);
    assert_eq!(tree.find(&5), None);
    assert_eq!(tree.find(&10), Some(10));
}

#[test]
fn test_split() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(8);
    tree.insert(12);
    tree.insert(20);

    let (left, found, right) = tree.split(&10);

    assert!(found);
    assert!(left.size() > 0);
    assert!(right.size() > 0);

    // Elements < 10 should be in left
    assert_eq!(left.find(&5), Some(5));
    assert_eq!(left.find(&3), Some(3));
    assert_eq!(left.find(&8), Some(8));

    // Elements > 10 should be in right
    assert_eq!(right.find(&15), Some(15));
    assert_eq!(right.find(&12), Some(12));
    assert_eq!(right.find(&20), Some(20));
}

#[test]
fn test_join_pair() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(3);
    tree1.insert(5);
    tree1.insert(7);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(12);
    tree2.insert(15);
    tree2.insert(20);

    let joined = tree1.join_pair(tree2);
    assert_eq!(joined.size(), 6);
    assert_eq!(joined.find(&3), Some(3));
    assert_eq!(joined.find(&20), Some(20));
}

#[test]
fn test_union() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(5);
    tree1.insert(10);
    tree1.insert(15);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(10);
    tree2.insert(12);
    tree2.insert(20);

    let union_tree = tree1.union(&tree2);
    assert_eq!(union_tree.size(), 5); // 5, 10, 12, 15, 20
    assert_eq!(union_tree.find(&5), Some(5));
    assert_eq!(union_tree.find(&20), Some(20));
}

#[test]
fn test_intersect() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(5);
    tree1.insert(10);
    tree1.insert(15);
    tree1.insert(20);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(10);
    tree2.insert(15);
    tree2.insert(25);

    let intersect_tree = tree1.intersect(&tree2);
    assert_eq!(intersect_tree.size(), 2); // 10, 15
    assert_eq!(intersect_tree.find(&10), Some(10));
    assert_eq!(intersect_tree.find(&15), Some(15));
    assert_eq!(intersect_tree.find(&5), None);
}

#[test]
fn test_difference() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(5);
    tree1.insert(10);
    tree1.insert(15);
    tree1.insert(20);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(10);
    tree2.insert(15);

    let diff_tree = tree1.difference(&tree2);
    assert_eq!(diff_tree.size(), 2); // 5, 20
    assert_eq!(diff_tree.find(&5), Some(5));
    assert_eq!(diff_tree.find(&20), Some(20));
    assert_eq!(diff_tree.find(&10), None);
}

#[test]
fn test_filter() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(5);
    tree.insert(10);
    tree.insert(15);
    tree.insert(20);
    tree.insert(25);

    let filtered = tree.filter(|&x| x > 10);
    assert_eq!(filtered.size(), 3); // 15, 20, 25
    assert_eq!(filtered.find(&15), Some(15));
    assert_eq!(filtered.find(&5), None);
}

#[test]
fn test_reduce() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(5);
    tree.insert(10);
    tree.insert(15);
    tree.insert(20);

    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 50);
}

#[test]
fn test_expose() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    #[allow(clippy::assertions_on_constants)]
    match tree.expose() {
        | Exposed::Leaf => assert!(true),
        | _ => panic!("Empty tree should expose as Leaf"),
    }

    tree.insert(10);
    match tree.expose() {
        | Exposed::Node(_, key, _) => assert_eq!(key, 10),
        | Exposed::Leaf => panic!("Non-empty tree should expose as Node"),
    }
}

#[test]
fn test_join_mid() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(5);
    tree.insert(15);

    let exposed = tree.expose();
    let reconstructed: ParamTreap<i32> = ParamTreapTrait::join_mid(exposed);
    assert_eq!(reconstructed.size(), tree.size());
}

#[test]
fn test_duplicate_insert() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(10);

    // Treap may handle duplicates by replacement or ignore
    assert!(tree.size() > 0);
    assert_eq!(tree.find(&10), Some(10));
}

#[test]
fn test_delete_nonexistent() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);

    let size_before = tree.size();
    tree.delete(&20); // Not in tree
    assert_eq!(tree.size(), size_before);
}

#[test]
fn test_large_tree() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();

    for i in 0..100 {
        tree.insert(i);
    }

    assert_eq!(tree.size(), 100);

    for i in 0..100 {
        assert_eq!(tree.find(&i), Some(i));
    }
}

#[test]
fn test_sequential_operations() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();

    // Insert
    for i in [5, 3, 7, 1, 9, 4, 6, 8, 2] {
        tree.insert(i);
    }
    assert_eq!(tree.size(), 9);

    // Delete some
    tree.delete(&3);
    tree.delete(&7);
    assert_eq!(tree.size(), 7);

    // Find remaining
    assert_eq!(tree.find(&5), Some(5));
    assert_eq!(tree.find(&1), Some(1));
    assert_eq!(tree.find(&3), None);
}

#[test]
fn test_union_disjoint() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(1);
    tree1.insert(2);
    tree1.insert(3);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(4);
    tree2.insert(5);
    tree2.insert(6);

    let union_tree = tree1.union(&tree2);
    assert_eq!(union_tree.size(), 6);
}

#[test]
fn test_intersect_empty() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(1);
    tree1.insert(2);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(5);
    tree2.insert(6);

    let intersect_tree = tree1.intersect(&tree2);
    assert_eq!(intersect_tree.size(), 0);
}

#[test]
fn test_difference_complete() {
    let tree1: ParamTreap<i32> = ParamTreapTrait::new();
    tree1.insert(1);
    tree1.insert(2);
    tree1.insert(3);

    let tree2: ParamTreap<i32> = ParamTreapTrait::new();
    tree2.insert(1);
    tree2.insert(2);
    tree2.insert(3);

    let diff_tree = tree1.difference(&tree2);
    assert_eq!(diff_tree.size(), 0);
}

#[test]
fn test_filter_all() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(5);
    tree.insert(10);
    tree.insert(15);

    let filtered = tree.filter(|&x| x > 0);
    assert_eq!(filtered.size(), tree.size());
}

#[test]
fn test_filter_none() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(5);
    tree.insert(10);
    tree.insert(15);

    let filtered = tree.filter(|&x| x > 100);
    assert_eq!(filtered.size(), 0);
}

#[test]
fn test_string_keys() {
    let tree: ParamTreap<String> = ParamTreapTrait::new();
    tree.insert("banana".to_string());
    tree.insert("apple".to_string());
    tree.insert("cherry".to_string());

    assert_eq!(tree.size(), 3);
    assert_eq!(tree.find(&"apple".to_string()), Some("apple".to_string()));
}

#[test]
fn test_split_empty() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    let (left, found, right) = tree.split(&10);

    assert!(!found);
    assert_eq!(left.size(), 0);
    assert_eq!(right.size(), 0);
}

#[test]
fn test_split_boundaries() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    let (left, found, _right) = tree.split(&1);
    assert!(!found);
    assert_eq!(left.size(), 0);

    let (_left, found, right) = tree.split(&20);
    assert!(!found);
    assert_eq!(right.size(), 0);
}

#[test]
fn test_expose_with_priority() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);

    let maybe_exposed = tree.expose_with_priority();
    assert!(maybe_exposed.is_some());
}

#[test]
fn test_reduce_empty() {
    let tree: ParamTreap<i32> = ParamTreapTrait::new();
    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 0);
}
