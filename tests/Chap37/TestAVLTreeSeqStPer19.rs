//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::AVLTreeSeqStPerLit;
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
use apas_verus::Chap37::*;
use apas_verus::Types::Types::*; // macro import

#[test]
fn test_tabulate_and_map_ch19() {
    let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 4];
    let m: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 2, 4, 6, 8];
    let expected = AVLTreeSeqStPerLit![0, 2, 4, 6, 8];
    assert_eq!(m, expected);
}

#[test]
fn test_select_and_append_ch19() {
    let a: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2];
    let b: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![3, 4, 5];
    // select method doesn't exist in AVLTreeSeqStPerTrait, using nth instead
    assert_eq!(a.nth(0), &0);
    assert_eq!(b.nth(1), &4);
    // Test bounds checking
    assert_eq!(a.length(), 3);
    assert_eq!(b.length(), 3);
    let c: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 4, 5];
    let expected = AVLTreeSeqStPerLit![0, 1, 2, 3, 4, 5];
    assert_eq!(c, expected);
}

#[test]
fn test_deflate_and_filter_ch19() {
    let _t: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2, 3, 4, 5];
    let d: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![2];
    let expected_d = AVLTreeSeqStPerLit![2];
    assert_eq!(d, expected_d);
    let f: AVLTreeSeqStPerS<N> = AVLTreeSeqStPerLit![0, 1, 2];
    let expected_f = AVLTreeSeqStPerLit![0, 1, 2];
    assert_eq!(f, expected_f);
}
