//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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

fn make_tree(values: &[i32]) -> ParamBST<i32> {
    let tree = ParamBST::new();
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
    let union = a.union(&b);
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
