//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap28::MaxContigSubSumBruteStEph::MaxContigSubSumBruteStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_empty() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, None); // -∞
}

#[test]
fn test_single_positive() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![5];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(5));
}

#[test]
fn test_single_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-3];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(-3));
}

#[test]
fn test_example_from_book() {
    // Example 28.3: a = [1, -2, 0, 3, -1, 0, 2, -3], max sum = 4 from [3, -1, 0, 2]
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, -2, 0, 3, -1, 0, 2, -3];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(4));
}

#[test]
fn test_all_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-5, -2, -8, -1];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(-1)); // Best is single element [-1]
}

#[test]
fn test_all_positive() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![1, 2, 3, 4];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(10)); // Sum of all elements
}

#[test]
fn test_starts_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![-5, 3, 4];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(7)); // [3, 4]
}

#[test]
fn test_ends_negative() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![3, 4, -5];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(7)); // [3, 4]
}

#[test]
fn test_zero_elements() {
    let a: ArraySeqStEphS<i32> = ArraySeqStEphSLit![0, 0, 0];
    let result = ArraySeqStEphS::max_contig_sub_sum_brute(&a);
    assert_eq!(result, Some(0));
}
