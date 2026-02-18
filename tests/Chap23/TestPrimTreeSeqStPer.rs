//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;
use apas_verus::Types::Types::*;
use vstd::prelude::Ghost;

#[test]
fn expose_zero_returns_zero() {
    let seq = PrimTreeSeqStS::<N>::empty();
    assert!(matches!(seq.expose(), PrimTreeSeqStTree::Zero));
}

#[test]
fn expose_one_returns_one() {
    let seq = PrimTreeSeqStS::singleton(7);
    match seq.expose() {
        | PrimTreeSeqStTree::One(value) => assert_eq!(value, 7),
        | other => panic!("expected One variant, got {other:?}"),
    }
}

#[test]
fn expose_two_splits_sequence() {
    let seq = PrimTreeSeqStS::from_vec((0..6).collect());
    match seq.expose() {
        | PrimTreeSeqStTree::Two(left, right) => {
            assert!(left.length() > 0);
            assert!(right.length() > 0);
            assert_eq!(left.length() + right.length(), 6);
            assert_eq!(left.as_slice(), &[0, 1, 2]);
            assert_eq!(right.as_slice(), &[3, 4, 5]);
        }
        | other => panic!("expected Two variant, got {other:?}"),
    }
}

#[test]
fn join_zero_creates_empty_sequence() {
    let seq = PrimTreeSeqStS::<N>::join(PrimTreeSeqStTree::Zero);
    assert_eq!(seq.length(), 0);
}

#[test]
fn join_two_concatenates_sequences() {
    let left = PrimTreeSeqStS::from_vec(vec![1, 2]);
    let right = PrimTreeSeqStS::from_vec(vec![3, 4, 5]);
    let joined = PrimTreeSeqStS::join(PrimTreeSeqStTree::Two(left.clone(), right.clone()));
    assert_eq!(joined.as_slice(), &[1, 2, 3, 4, 5]);
    assert_eq!(joined.length(), left.length() + right.length());
}

#[test]
fn expose_then_join_roundtrip() {
    let original = PrimTreeSeqStS::from_vec((1..=9).collect());
    let exposed = original.expose();
    let reconstructed = PrimTreeSeqStS::join(exposed);
    assert_eq!(original.as_slice(), reconstructed.as_slice());
}

#[test]
fn test_empty_constructor() {
    let empty = PrimTreeSeqStS::<N>::empty();
    assert_eq!(empty.length(), 0);
    assert!(empty.as_slice().is_empty());
}

#[test]
fn test_singleton_constructor() {
    let single = PrimTreeSeqStS::singleton(42);
    assert_eq!(single.length(), 1);
    assert_eq!(single.as_slice(), &[42]);
}

#[test]
fn test_from_vec_constructor() {
    let data = vec![10, 20, 30];
    let seq = PrimTreeSeqStS::from_vec(data.clone());
    assert_eq!(seq.length(), 3);
    assert_eq!(seq.as_slice(), &[10, 20, 30]);
}

#[test]
fn test_into_vec_conversion() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4]);
    let vec_back = seq.into_vec();
    assert_eq!(vec_back, vec![1, 2, 3, 4]);
}

#[test]
fn test_as_slice_view() {
    let seq = PrimTreeSeqStS::from_vec(vec!["a", "b", "c"]);
    let slice = seq.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], "a");
    assert_eq!(slice[1], "b");
    assert_eq!(slice[2], "c");
}

#[test]
fn test_length_method() {
    let empty = PrimTreeSeqStS::<N>::empty();
    assert_eq!(empty.length(), 0);

    let single = PrimTreeSeqStS::singleton(1);
    assert_eq!(single.length(), 1);

    let multi = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(multi.length(), 5);
}

#[test]
fn test_expose_edge_cases() {
    // Test with exactly 2 elements
    let two_elem = PrimTreeSeqStS::from_vec(vec![1, 2]);
    match two_elem.expose() {
        | PrimTreeSeqStTree::Two(left, right) => {
            assert_eq!(left.length(), 1);
            assert_eq!(right.length(), 1);
            assert_eq!(left.as_slice(), &[1]);
            assert_eq!(right.as_slice(), &[2]);
        }
        | other => panic!("expected Two variant for 2 elements, got {other:?}"),
    }

    // Test with odd number of elements
    let odd_elem = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4, 5]);
    match odd_elem.expose() {
        | PrimTreeSeqStTree::Two(left, right) => {
            assert_eq!(left.length(), 2); // 5/2 = 2
            assert_eq!(right.length(), 3);
            assert_eq!(left.as_slice(), &[1, 2]);
            assert_eq!(right.as_slice(), &[3, 4, 5]);
        }
        | other => panic!("expected Two variant for 5 elements, got {other:?}"),
    }
}

#[test]
fn test_join_one_variant() {
    let joined = PrimTreeSeqStS::join(PrimTreeSeqStTree::One(99));
    assert_eq!(joined.length(), 1);
    assert_eq!(joined.as_slice(), &[99]);
}

#[test]
fn test_join_empty_sequences() {
    let left = PrimTreeSeqStS::<N>::empty();
    let right = PrimTreeSeqStS::<N>::empty();
    let joined = PrimTreeSeqStS::join(PrimTreeSeqStTree::Two(left, right));
    assert_eq!(joined.length(), 0);
    assert!(joined.as_slice().is_empty());
}

#[test]
fn test_join_mixed_sizes() {
    let left = PrimTreeSeqStS::singleton(1);
    let right = PrimTreeSeqStS::from_vec(vec![2, 3, 4]);
    let joined = PrimTreeSeqStS::join(PrimTreeSeqStTree::Two(left, right));
    assert_eq!(joined.length(), 4);
    assert_eq!(joined.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn test_clone_functionality() {
    let original = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let cloned = original.clone();
    assert_eq!(original.as_slice(), cloned.as_slice());
    assert_eq!(original.length(), cloned.length());
}

#[test]
fn test_debug_format() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let debug_str = format!("{seq:?}");
    assert!(debug_str.contains("PrimTreeSeqStS"));
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
}

#[test]
fn test_equality_comparison() {
    let seq1 = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let seq2 = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let seq3 = PrimTreeSeqStS::from_vec(vec![1, 2, 4]);

    assert_eq!(seq1, seq2);
    assert_ne!(seq1, seq3);
}

#[test]
fn test_tree_enum_equality() {
    let tree1 = PrimTreeSeqStTree::<N>::Zero;
    let tree2 = PrimTreeSeqStTree::<N>::Zero;
    assert_eq!(tree1, tree2);

    let tree3 = PrimTreeSeqStTree::One(42);
    let tree4 = PrimTreeSeqStTree::One(42);
    let tree5 = PrimTreeSeqStTree::One(43);
    assert_eq!(tree3, tree4);
    assert_ne!(tree3, tree5);

    let left = PrimTreeSeqStS::from_vec(vec![1, 2]);
    let right = PrimTreeSeqStS::from_vec(vec![3, 4]);
    let tree6 = PrimTreeSeqStTree::Two(left.clone(), right.clone());
    let tree7 = PrimTreeSeqStTree::Two(left, right);
    assert_eq!(tree6, tree7);
}

#[test]
fn test_large_sequence_expose_join() {
    let large_seq = PrimTreeSeqStS::from_vec((0..1000).collect::<Vec<N>>());
    let exposed = large_seq.expose();
    let reconstructed = PrimTreeSeqStS::join(exposed);

    assert_eq!(large_seq.length(), reconstructed.length());
    assert_eq!(large_seq.as_slice(), reconstructed.as_slice());
}

#[test]
fn test_empty() {
    let seq = PrimTreeSeqStS::<i32>::empty();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_singleton() {
    let seq = PrimTreeSeqStS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.as_slice()[0], 42);
}

#[test]
fn test_from_vec() {
    let vec = vec![1, 2, 3];
    let seq = PrimTreeSeqStS::from_vec(vec);
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_into_vec() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let vec = seq.into_vec();
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_as_slice() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    assert_eq!(seq.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_length() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
}

// Tests calling through the trait interface to cover trait impl lines

#[test]
fn test_trait_empty() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::empty();
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_trait_singleton() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.as_slice()[0], 42);
}

#[test]
fn test_trait_from_vec() {
    let vec = vec![1, 2, 3];
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::from_vec(vec);
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_trait_into_vec() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::from_vec(vec![1, 2, 3]);
    // into_vec is a struct method, not on the trait
    let vec = seq.into_vec();
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_trait_as_slice() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::from_vec(vec![1, 2, 3]);
    assert_eq!(seq.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_trait_length() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(seq.length(), 5);
}

#[test]
fn test_trait_expose_zero() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::empty();
    assert!(matches!(seq.expose(), PrimTreeSeqStTree::Zero));
}

#[test]
fn test_trait_expose_one() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::singleton(7);
    match seq.expose() {
        | PrimTreeSeqStTree::One(value) => assert_eq!(value, 7),
        | other => panic!("expected One variant, got {other:?}"),
    }
}

#[test]
fn test_trait_expose_two() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::from_vec((0..6).collect());
    match seq.expose() {
        | PrimTreeSeqStTree::Two(left, right) => {
            assert_eq!(left.length() + right.length(), 6);
            assert_eq!(left.as_slice(), &[0, 1, 2]);
            assert_eq!(right.as_slice(), &[3, 4, 5]);
        }
        | other => panic!("expected Two variant, got {other:?}"),
    }
}

#[test]
fn test_trait_join_zero() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::join(PrimTreeSeqStTree::Zero);
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_trait_join_one() {
    let seq = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::join(PrimTreeSeqStTree::One(99));
    assert_eq!(seq.length(), 1);
    assert_eq!(seq.as_slice(), &[99]);
}

#[test]
fn test_trait_join_two() {
    let left = PrimTreeSeqStS::from_vec(vec![1, 2]);
    let right = PrimTreeSeqStS::from_vec(vec![3, 4, 5]);
    let joined = <PrimTreeSeqStS<i32> as PrimTreeSeqStTrait<i32>>::join(PrimTreeSeqStTree::Two(left, right));
    assert_eq!(joined.length(), 5);
    assert_eq!(joined.as_slice(), &[1, 2, 3, 4, 5]);
}

// append, subseq, update, map, tabulate, filter

#[test]
fn test_append() {
    let a = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let b = PrimTreeSeqStS::from_vec(vec![4, 5]);
    let ab = PrimTreeSeqStS::append(&a, &b);
    assert_eq!(ab.as_slice(), &[1, 2, 3, 4, 5]);
    assert_eq!(ab.length(), 5);
}

#[test]
fn test_append_empty_left() {
    let a = PrimTreeSeqStS::<i32>::empty();
    let b = PrimTreeSeqStS::from_vec(vec![1, 2]);
    let ab = PrimTreeSeqStS::append(&a, &b);
    assert_eq!(ab.as_slice(), &[1, 2]);
}

#[test]
fn test_append_empty_right() {
    let a = PrimTreeSeqStS::from_vec(vec![1, 2]);
    let b = PrimTreeSeqStS::<i32>::empty();
    let ab = PrimTreeSeqStS::append(&a, &b);
    assert_eq!(ab.as_slice(), &[1, 2]);
}

#[test]
fn test_subseq() {
    let seq = PrimTreeSeqStS::from_vec(vec![10, 20, 30, 40, 50]);
    let sub = seq.subseq(1, 3);
    assert_eq!(sub.as_slice(), &[20, 30, 40]);
}

#[test]
fn test_subseq_full() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let sub = seq.subseq(0, 3);
    assert_eq!(sub.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_subseq_empty() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let sub = seq.subseq(1, 0);
    assert_eq!(sub.length(), 0);
}

#[test]
fn test_update() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let updated = PrimTreeSeqStS::update(&seq, 1, 99);
    assert_eq!(updated.as_slice(), &[1, 99, 3]);
}

#[test]
fn test_update_first() {
    let seq = PrimTreeSeqStS::from_vec(vec![10, 20, 30]);
    let updated = PrimTreeSeqStS::update(&seq, 0, 42);
    assert_eq!(updated.as_slice(), &[42, 20, 30]);
}

#[test]
fn test_update_last() {
    let seq = PrimTreeSeqStS::from_vec(vec![10, 20, 30]);
    let updated = PrimTreeSeqStS::update(&seq, 2, 42);
    assert_eq!(updated.as_slice(), &[10, 20, 42]);
}

#[test]
fn test_map() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let doubled = PrimTreeSeqStS::map(&seq, &|x: &i32| x * 2);
    assert_eq!(doubled.as_slice(), &[2, 4, 6]);
}

#[test]
fn test_map_to_string() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3]);
    let strings = PrimTreeSeqStS::map(&seq, &|x: &i32| format!("{x}"));
    assert_eq!(strings.as_slice(), &["1", "2", "3"]);
}

#[test]
fn test_tabulate() {
    let seq = PrimTreeSeqStS::tabulate(&|i: usize| i * i, 5);
    assert_eq!(seq.as_slice(), &[0, 1, 4, 9, 16]);
}

#[test]
fn test_tabulate_empty() {
    let seq = PrimTreeSeqStS::<i32>::tabulate(&|_: usize| 0, 0);
    assert_eq!(seq.length(), 0);
}

#[test]
fn test_filter() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4, 5, 6]);
    let evens = PrimTreeSeqStS::filter(&seq, &|x: &i32| x % 2 == 0, Ghost::assume_new());
    assert_eq!(evens.as_slice(), &[2, 4, 6]);
}

#[test]
fn test_filter_all() {
    let seq = PrimTreeSeqStS::from_vec(vec![2, 4, 6]);
    let evens = PrimTreeSeqStS::filter(&seq, &|x: &i32| x % 2 == 0, Ghost::assume_new());
    assert_eq!(evens.as_slice(), &[2, 4, 6]);
}

#[test]
fn test_filter_none() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 3, 5]);
    let evens = PrimTreeSeqStS::filter(&seq, &|x: &i32| x % 2 == 0, Ghost::assume_new());
    assert_eq!(evens.length(), 0);
}

fn generic_length<T: StT, S: PrimTreeSeqStTrait<T>>(seq: &S) -> N { seq.length() }

fn generic_expose<T: StT, S: PrimTreeSeqStTrait<T>>(seq: &S) -> PrimTreeSeqStTree<T> { seq.expose() }

#[test]
fn test_generic_length() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(generic_length(&seq), 5);
}

#[test]
fn test_generic_expose_empty() {
    let seq = PrimTreeSeqStS::<i32>::empty();
    assert!(matches!(generic_expose(&seq), PrimTreeSeqStTree::Zero));
}

#[test]
fn test_generic_expose_one() {
    let seq = PrimTreeSeqStS::singleton(42);
    match generic_expose(&seq) {
        | PrimTreeSeqStTree::One(value) => assert_eq!(value, 42),
        | other => panic!("expected One, got {other:?}"),
    }
}

#[test]
fn test_generic_expose_two() {
    let seq = PrimTreeSeqStS::from_vec(vec![1, 2, 3, 4]);
    match generic_expose(&seq) {
        | PrimTreeSeqStTree::Two(left, right) => {
            assert_eq!(left.length(), 2);
            assert_eq!(right.length(), 2);
        }
        | other => panic!("expected Two, got {other:?}"),
    }
}
