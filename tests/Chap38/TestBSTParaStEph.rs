//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use vstd::prelude::Ghost;

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::*;
use apas_verus::ParamBSTLit;
use apas_verus::Types::Types::*;

#[test]
fn test_parambstlit_macro_functionality() {
    // Test empty BST creation
    let empty: ParamBST<i32> = ParamBSTLit![];
    assert_eq!(empty.size(), 0);
    assert_eq!(empty.find(&42), None);

    // Test BST creation with elements
    let with_data: ParamBST<i32> = ParamBSTLit![2, 1, 3];
    assert_eq!(with_data.size(), 3);
    assert_eq!(with_data.find(&1), Some(1));
    assert_eq!(with_data.find(&2), Some(2));
    assert_eq!(with_data.find(&3), Some(3));
    assert_eq!(with_data.find(&4), None);
}

fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {
    let mut tree = ParamBST::new();
    for value in start..end {
        tree.insert(value);
    }
    tree
}

fn make_tree(values: &[i32]) -> ParamBST<i32> {
    let mut tree = ParamBST::new();
    for &value in values {
        tree.insert(value);
    }
    tree
}

#[test]
fn para_basic_insert_find() {
    let tree = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(tree.size(), 7);
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&8), None);
    assert!(!tree.is_empty());
    assert_eq!(tree.in_order(), ArraySeqStPerS::from_vec(vec![1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn para_split_and_join_pair() {
    let tree = make_tree(&[0, 1, 2, 3, 4, 5]);
    let (less, present, greater) = tree.split(&3);
    assert!(present);
    assert_eq!(less.in_order(), ArraySeqStPerS::from_vec(vec![0, 1, 2]));
    assert_eq!(greater.in_order(), ArraySeqStPerS::from_vec(vec![4, 5]));

    let rejoined = less.join_pair(greater);
    assert_eq!(rejoined.in_order(), ArraySeqStPerS::from_vec(vec![0, 1, 2, 4, 5]));
}

#[test]
fn para_union_and_delete() {
    let a = make_tree(&[1, 3, 5, 7]);
    let b = make_tree(&[0, 2, 4, 6, 8]);
    let mut union = a.union(&b);
    assert_eq!(union.in_order(), ArraySeqStPerS::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));

    union.delete(&4);
    union.delete(&7);
    assert_eq!(union.find(&4), None);
    assert_eq!(union.find(&7), None);
    assert_eq!(union.in_order(), ArraySeqStPerS::from_vec(vec![0, 1, 2, 3, 5, 6, 8]));
}

#[test]
fn para_join_mid_expose_roundtrip() {
    let empty = ParamBST::<i32>::join_mid(Exposed::Leaf);
    match empty.expose() {
        | Exposed::Leaf => {}
        | Exposed::Node(..) => panic!("expected leaf"),
    }

    let left = ParamBST::join_mid(Exposed::Leaf);
    let right = ParamBST::join_mid(Exposed::Leaf);
    let combined = ParamBST::join_mid(Exposed::Node(left, 10, right));

    match combined.expose() {
        | Exposed::Leaf => panic!("expected node"),
        | Exposed::Node(l, key, r) => {
            assert_eq!(key, 10);
            assert_eq!(l.size(), 0);
            assert_eq!(r.size(), 0);
        }
    }
}

#[test]
fn para_singleton() {
    let tree = ParamBST::singleton(42);
    assert_eq!(tree.size(), 1);
    assert_eq!(tree.find(&42), Some(42));
    assert_eq!(tree.find(&0), None);
    assert_eq!(tree.in_order(), ArraySeqStPerS::from_vec(vec![42]));
}

#[test]
fn para_intersect_and_difference() {
    let a = make_tree(&[1, 2, 3, 4, 5, 6]);
    let b = make_tree(&[4, 5, 6, 7, 8]);

    let intersection = a.intersect(&b);
    assert_eq!(intersection.in_order(), ArraySeqStPerS::from_vec(vec![4, 5, 6]));

    let difference = a.difference(&b);
    assert_eq!(difference.in_order(), ArraySeqStPerS::from_vec(vec![1, 2, 3]));
}

#[test]
fn para_filter_and_reduce() {
    let tree = make_tree(&[1, 2, 3, 4, 5, 6]);

    let evens = tree.filter(|v| v % 2 == 0, Ghost::assume_new());
    assert_eq!(evens.in_order(), ArraySeqStPerS::from_vec(vec![2, 4, 6]));

    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 21);

    let empty_sum = ParamBST::new().reduce(|a, b| a + b, 0);
    assert_eq!(empty_sum, 0);
}

#[test]
fn para_intersect_and_difference_large() {
    let a = make_range_tree(0, 256);
    let b = make_range_tree(128, 384);

    let intersection = a.intersect(&b);
    let intersect_values = intersection.in_order().iter().copied().collect::<Vec<_>>();
    let expected_intersection = (128..256).collect::<Vec<_>>();
    assert_eq!(intersect_values, expected_intersection);

    let difference = a.difference(&b);
    let diff_values = difference.in_order().iter().copied().collect::<Vec<_>>();
    let expected_difference = (0..128).collect::<Vec<_>>();
    assert_eq!(diff_values, expected_difference);
}

#[test]
fn para_filter_and_reduce_edge_cases() {
    let tree = make_range_tree(0, 64);

    let odds = tree.filter(|v| v % 2 == 1, Ghost::assume_new());
    let odd_values = odds.in_order().iter().copied().collect::<Vec<_>>();
    let expected_odds = (0..64).filter(|v| v % 2 == 1).collect::<Vec<_>>();
    assert_eq!(odd_values, expected_odds);

    let sum_squares = tree.reduce(|acc, v| acc + v * v, 0);
    let expected_sum_squares = (63 * 64 * 127) / 6;
    assert_eq!(sum_squares, expected_sum_squares);

    let single = make_tree(&[42]);
    let filtered_single = single.filter(|v| *v == 42, Ghost::assume_new());
    assert_eq!(filtered_single.in_order().iter().copied().collect::<Vec<_>>(), vec![42]);
    let reduced_single = single.reduce(|a, b| a + b, 0);
    assert_eq!(reduced_single, 42);
}

#[test]
fn para_split_nonexistent_key() {
    let tree = make_tree(&[1, 3, 5, 7, 9]);
    let (less, present, greater) = tree.split(&4);
    assert!(!present);
    assert_eq!(less.in_order(), ArraySeqStPerS::from_vec(vec![1, 3]));
    assert_eq!(greater.in_order(), ArraySeqStPerS::from_vec(vec![5, 7, 9]));
}

#[test]
fn para_ops_on_empty_tree() {
    let empty = ParamBST::<i32>::new();
    assert_eq!(empty.find(&1), None);
    assert_eq!(empty.size(), 0);
    assert!(empty.is_empty());

    let other = make_tree(&[1, 2, 3]);
    assert_eq!(empty.union(&other).in_order(), ArraySeqStPerS::from_vec(vec![1, 2, 3]));
    assert_eq!(other.union(&empty).in_order(), ArraySeqStPerS::from_vec(vec![1, 2, 3]));
    assert_eq!(empty.intersect(&other).size(), 0);
    assert_eq!(empty.difference(&other).size(), 0);
    assert_eq!(other.difference(&empty).in_order(), ArraySeqStPerS::from_vec(vec![1, 2, 3]));
}

#[test]
fn para_delete_nonexistent_key() {
    let mut tree = make_tree(&[1, 3, 5]);
    tree.delete(&2);
    assert_eq!(tree.in_order(), ArraySeqStPerS::from_vec(vec![1, 3, 5]));
    assert_eq!(tree.size(), 3);
}

#[test]
fn para_large_union() {
    let a = make_range_tree(0, 100);
    let b = make_range_tree(50, 150);
    let union = a.union(&b);
    assert_eq!(union.size(), 150);
    let order = union.in_order();
    for i in 0..150 {
        assert_eq!(*order.nth(i), i as i32, "Mismatch at index {i}");
    }
}

#[test]
fn para_delete_all_elements() {
    let mut tree = make_tree(&[3, 1, 4, 1, 5, 9]);
    for &v in &[1, 3, 4, 5, 9] {
        tree.delete(&v);
    }
    assert_eq!(tree.size(), 0);
    assert!(tree.is_empty());
}

#[test]
fn para_insert_descending_order() {
    let tree = make_range_tree(0, 50);
    assert_eq!(tree.size(), 50);
    for i in 0..50 {
        assert_eq!(tree.find(&i), Some(i));
    }
}

#[test]
fn para_split_at_min_and_max() {
    let tree = make_tree(&[10, 20, 30, 40, 50]);

    let (less, present, greater) = tree.split(&10);
    assert!(present);
    assert_eq!(less.size(), 0);
    assert_eq!(greater.size(), 4);

    let (less, present, greater) = tree.split(&50);
    assert!(present);
    assert_eq!(less.size(), 4);
    assert_eq!(greater.size(), 0);
}

#[test]
fn para_intersect_with_self() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let inter = tree.intersect(&tree);
    assert_eq!(inter.in_order(), tree.in_order());
}

#[test]
fn para_difference_with_self() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let diff = tree.difference(&tree);
    assert_eq!(diff.size(), 0);
}

#[test]
fn para_union_with_self() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let union = tree.union(&tree);
    assert_eq!(union.in_order(), tree.in_order());
}

#[test]
fn para_filter_none_match() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let filtered = tree.filter(|v| *v > 100, Ghost::assume_new());
    assert_eq!(filtered.size(), 0);
}

#[test]
fn para_filter_all_match() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let filtered = tree.filter(|v| *v > 0, Ghost::assume_new());
    assert_eq!(filtered.size(), 5);
}

#[test]
fn para_reduce_product() {
    let tree = make_tree(&[1, 2, 3, 4, 5]);
    let product = tree.reduce(|a, b| a * b, 1);
    assert_eq!(product, 120);
}

#[test]
fn para_in_order_sorted() {
    let tree = make_range_tree(1, 20);
    let seq = tree.in_order();
    assert_eq!(seq.length(), 19);
    for i in 0..seq.length() - 1 {
        assert!(seq.nth(i) < seq.nth(i + 1), "not sorted at {i}");
    }
}

#[test]
fn para_duplicate_insert() {
    let mut tree = ParamBST::<i32>::new();
    tree.insert(5);
    tree.insert(5);
    assert_eq!(tree.size(), 1);
}

#[test]
fn para_delete_root() {
    let mut tree = make_tree(&[5, 3, 7]);
    tree.delete(&5);
    assert_eq!(tree.size(), 2);
    assert_eq!(tree.find(&5), None);
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&7), Some(7));
}

#[test]
fn para_join_pair_disjoint() {
    let left = make_range_tree(1, 5);
    let right = make_range_tree(10, 15);
    let joined = left.join_pair(right);
    assert_eq!(joined.size(), 9);
    for i in 1..5 {
        assert_eq!(joined.find(&i), Some(i));
    }
    for i in 10..15 {
        assert_eq!(joined.find(&i), Some(i));
    }
}

#[test]
fn para_clone_independence() {
    let mut tree = make_tree(&[1, 2, 3]);
    let cloned = tree.clone();
    tree.insert(4);
    assert_eq!(tree.size(), 4);
    assert_eq!(cloned.size(), 3);
}
