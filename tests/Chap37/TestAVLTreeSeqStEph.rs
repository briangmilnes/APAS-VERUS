use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphTrait, *};
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let tree = AVLTreeSeqStEphS::<i32>::empty();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_new() {
    let tree = AVLTreeSeqStEphS::<i32>::new();
    assert_eq!(tree.length(), 0);
    assert!(tree.isEmpty());
}

#[test]
fn test_singleton() {
    let tree = AVLTreeSeqStEphS::singleton(42);
    assert_eq!(tree.length(), 1);
    assert!(tree.isSingleton());
    assert_eq!(*tree.nth(0), 42);
}

#[test]
fn test_from_vec() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(tree.length(), 5);
    assert_eq!(*tree.nth(0), 1);
    assert_eq!(*tree.nth(4), 5);
}

#[test]
fn test_nth() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![10, 20, 30]);
    assert_eq!(*tree.nth(0), 10);
    assert_eq!(*tree.nth(1), 20);
    assert_eq!(*tree.nth(2), 30);
}

#[test]
fn test_set() {
    let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    assert!(tree.set(1, 99).is_ok());
    assert_eq!(*tree.nth(1), 99);
}

#[test]
fn test_update() {
    let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    tree.update((1, 99));
    assert_eq!(*tree.nth(1), 99);
}

#[test]
fn test_length() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4]);
    assert_eq!(tree.length(), 4);
}

#[test]
fn test_isEmpty() {
    let empty = AVLTreeSeqStEphS::<i32>::empty();
    assert!(empty.isEmpty());
    let non_empty = AVLTreeSeqStEphS::singleton(1);
    assert!(!non_empty.isEmpty());
}

#[test]
fn test_isSingleton() {
    let single = AVLTreeSeqStEphS::singleton(1);
    assert!(single.isSingleton());
    let empty = AVLTreeSeqStEphS::<i32>::empty();
    assert!(!empty.isSingleton());
    let multi = AVLTreeSeqStEphS::from_vec(vec![1, 2]);
    assert!(!multi.isSingleton());
}

#[test]
fn test_subseq_copy() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3, 4, 5]);
    let sub = tree.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_new_root() {
    let tree = AVLTreeSeqStEphS::<i32>::new_root();
    assert_eq!(tree.length(), 0);
}

#[test]
fn test_to_arrayseq() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let arr = tree.to_arrayseq();
    assert_eq!(arr.length(), 3);
    assert_eq!(*arr.nth(1), 2);
}

#[test]
fn test_iter() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let collected: Vec<i32> = tree.iter().map(|x| *x).collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_push_back() {
    let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2]);
    tree.push_back(3);
    assert_eq!(tree.length(), 3);
    assert_eq!(*tree.nth(2), 3);
}

#[test]
fn test_contains_value() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    assert!(tree.contains_value(&2));
    assert!(!tree.contains_value(&99));
}

#[test]
fn test_insert_value() {
    let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 3]);
    tree.insert_value(2);
    assert_eq!(tree.length(), 3);
    assert!(tree.contains_value(&2));
}

#[test]
fn test_delete_value() {
    let mut tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let deleted = tree.delete_value(&2);
    assert!(deleted);
    assert_eq!(tree.length(), 2);
    assert!(!tree.contains_value(&2));
    
    let not_deleted = tree.delete_value(&99);
    assert!(!not_deleted);
}

#[test]
fn test_default_impl() {
    let tree: AVLTreeSeqStEphS<i32> = Default::default();
    assert_eq!(tree.length(), 0);
}

#[test]
fn test_clone_impl() {
    let tree = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let cloned = tree.clone();
    assert_eq!(cloned.length(), 3);
    assert_eq!(*cloned.nth(1), 2);
}

#[test]
fn test_partialeq_impl() {
    let tree1 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let tree2 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 3]);
    let tree3 = AVLTreeSeqStEphS::from_vec(vec![1, 2, 4]);
    assert!(tree1 == tree2);
    assert!(tree1 != tree3);
}

#[test]
fn test_eq_impl() {
    let tree1 = AVLTreeSeqStEphS::from_vec(vec![1, 2]);
    let tree2 = AVLTreeSeqStEphS::from_vec(vec![1, 2]);
    assert!(tree1 == tree2);
}

