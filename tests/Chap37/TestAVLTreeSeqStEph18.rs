//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSeq Eph (Chap18-style operations built using base APIs).

use apas_verus::{AVLTreeSeqStEphLit, ArraySeqStEphSLit};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
use apas_verus::Chap37::*;
use apas_verus::Types::Types::*; // macro import

#[test]
fn test_tabulate_inorder() {
    let t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![0, 1, 2, 3, 4];
    assert_eq!(t.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2, 3, 4]);
}

#[test]
fn test_map_increment() {
    let base: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![0, 1, 2, 3];
    let mapped_v = base.iter().map(|x| *x + 1).collect::<Vec<N>>();
    let mapped = AVLTreeSeqStEphS::<N>::from_vec(mapped_v);
    assert_eq!(mapped.to_arrayseq(), ArraySeqStEphSLit![1, 2, 3, 4]);
}

#[test]
fn test_append_union() {
    let a: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![0, 1, 2, 3];
    let b: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![2, 3, 4, 5];
    let mut vals = a.iter().copied().collect::<Vec<N>>();
    for x in b.iter() {
        if !vals.contains(x) {
            vals.push(*x);
        }
    }
    let u = AVLTreeSeqStEphS::from_vec(vals);
    assert_eq!(u.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2, 3, 4, 5]);
}

#[test]
fn test_filter_even() {
    let base: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphLit![0, 1, 2, 3, 4, 5];
    let kept = base.iter().filter(|x| **x % 2 == 0).copied().collect::<Vec<N>>();
    let evens = AVLTreeSeqStEphS::<N>::from_vec(kept);
    assert_eq!(evens.to_arrayseq(), ArraySeqStEphSLit![0, 2, 4]);
}
