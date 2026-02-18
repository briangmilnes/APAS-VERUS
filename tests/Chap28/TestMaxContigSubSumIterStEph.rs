//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap28::MaxContigSubSumIterStEph::MaxContigSubSumIterStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, None);
}

#[test]
fn test_single_positive() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![5];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(5));
}

#[test]
fn test_single_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-3];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(-3));
}

#[test]
fn test_example_from_book() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, -2, 0, 3, -1, 0, 2, -3];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(4));
}

#[test]
fn test_all_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-5, -2, -8, -1];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(-1));
}

#[test]
fn test_all_positive() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, 2, 3, 4];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(10));
}

#[test]
fn test_starts_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-5, 3, 4];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(7));
}

#[test]
fn test_ends_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![3, 4, -5];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(7));
}

#[test]
fn test_larger_example() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = ArraySeqStEphS::max_contig_sub_sum_iter(&a);
    assert_eq!(result, Some(6));
}
