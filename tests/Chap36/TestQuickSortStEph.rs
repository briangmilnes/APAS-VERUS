// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap36::QuickSortStEph::QuickSortStEph::*;
use apas_verus::Types::Types::*;

trait ToVec<T: StT> {
    fn to_vec(&self) -> Vec<T>;
}
impl<T: StT> ToVec<T> for ArraySeqStEphS<T> {
    fn to_vec(&self) -> Vec<T> { (0..self.length()).map(|i| self.nth(i).clone()).collect() }
}

#[test]
fn quick_sort_variants_produce_sorted_output() {
    let base = ArraySeqStEphSLit![5, 3, 1, 4, 2, 2, 3];
    let mut first = base.clone();
    ArraySeqStEphS::quick_sort_first(&mut first);
    let mut median3 = base.clone();
    ArraySeqStEphS::quick_sort_median3(&mut median3);
    let mut random = base.clone();
    ArraySeqStEphS::quick_sort_random(&mut random);
    let expected = ArraySeqStEphSLit![1, 2, 2, 3, 3, 4, 5];

    assert_eq!(first, expected);
    assert_eq!(median3, expected);
    assert_eq!(random, expected);
}

#[test]
fn quick_sort_handles_edge_cases() {
    let mut empty: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    ArraySeqStEphS::quick_sort_first(&mut empty);
    assert_eq!(empty.to_vec(), vec![]);

    let mut single = ArraySeqStEphSLit![42];
    ArraySeqStEphS::quick_sort_median3(&mut single);
    assert_eq!(single.to_vec(), vec![42]);

    let mut sorted = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    ArraySeqStEphS::quick_sort_random(&mut sorted);
    assert_eq!(sorted.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut reversed = ArraySeqStEphSLit![5, 4, 3, 2, 1];
    ArraySeqStEphS::quick_sort_first(&mut reversed);
    assert_eq!(reversed.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut pair = ArraySeqStEphSLit![2, 1];
    ArraySeqStEphS::quick_sort_median3(&mut pair);
    assert_eq!(pair.to_vec(), vec![1, 2]);
}

#[test]
fn quick_sort_all_same_elements() {
    let mut data = ArraySeqStEphSLit![7, 7, 7, 7, 7];
    ArraySeqStEphS::quick_sort_first(&mut data);
    assert_eq!(data.to_vec(), vec![7, 7, 7, 7, 7]);
}

#[test]
fn quick_sort_large_descending() {
    let values: Vec<i32> = (0..500).rev().collect();
    let mut data = ArraySeqStEphS::from_vec(values);
    ArraySeqStEphS::quick_sort_random(&mut data);
    let result = data.to_vec();
    for i in 1..result.len() {
        assert!(result[i - 1] <= result[i], "Not sorted at index {i}");
    }
}

#[test]
fn quick_sort_large_ascending() {
    let values: Vec<i32> = (0..500).collect();
    let mut data = ArraySeqStEphS::from_vec(values.clone());
    ArraySeqStEphS::quick_sort_median3(&mut data);
    assert_eq!(data.to_vec(), values);
}

#[test]
fn quick_sort_preserves_multiset() {
    let input = vec![5, 3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut data = ArraySeqStEphS::from_vec(input.clone());
    ArraySeqStEphS::quick_sort_first(&mut data);
    let result = data.to_vec();

    // Same length.
    assert_eq!(result.len(), input.len());

    // Same elements (sorted).
    let mut expected = input;
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn quick_sort_negative_values() {
    let mut data = ArraySeqStEphSLit![-5, 3, -1, 0, -3, 2];
    ArraySeqStEphS::quick_sort_random(&mut data);
    assert_eq!(data.to_vec(), vec![-5, -3, -1, 0, 2, 3]);
}

#[test]
fn quick_sort_two_elements_all_variants() {
    for (a, b) in [(1, 2), (2, 1), (1, 1)] {
        let expected = if a <= b { vec![a, b] } else { vec![b, a] };

        let mut d1 = ArraySeqStEphSLit![a, b];
        ArraySeqStEphS::quick_sort_first(&mut d1);
        assert_eq!(d1.to_vec(), expected);

        let mut d2 = ArraySeqStEphSLit![a, b];
        ArraySeqStEphS::quick_sort_median3(&mut d2);
        assert_eq!(d2.to_vec(), expected);

        let mut d3 = ArraySeqStEphSLit![a, b];
        ArraySeqStEphS::quick_sort_random(&mut d3);
        assert_eq!(d3.to_vec(), expected);
    }
}

#[test]
fn quick_sort_three_elements_all_permutations() {
    let perms = [
        [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1],
    ];
    for perm in &perms {
        let mut data = ArraySeqStEphS::from_vec(perm.to_vec());
        ArraySeqStEphS::quick_sort_first(&mut data);
        assert_eq!(data.to_vec(), vec![1, 2, 3], "Failed for perm {:?}", perm);
    }
}

#[test]
fn quick_sort_alternating_high_low() {
    let mut data = ArraySeqStEphSLit![100, 1, 99, 2, 98, 3, 97, 4];
    ArraySeqStEphS::quick_sort_random(&mut data);
    assert_eq!(data.to_vec(), vec![1, 2, 3, 4, 97, 98, 99, 100]);
}

#[test]
fn quick_sort_all_same_large() {
    let mut data = ArraySeqStEphS::from_vec(vec![42; 200]);
    ArraySeqStEphS::quick_sort_first(&mut data);
    assert!(data.to_vec().iter().all(|&x| x == 42));
}

#[test]
fn quick_sort_sorted_then_one_wrong() {
    let mut data = ArraySeqStEphSLit![1, 2, 3, 4, 0, 5, 6, 7];
    ArraySeqStEphS::quick_sort_median3(&mut data);
    assert_eq!(data.to_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn quick_sort_organ_pipe() {
    let mut data = ArraySeqStEphSLit![1, 3, 5, 7, 9, 8, 6, 4, 2];
    ArraySeqStEphS::quick_sort_first(&mut data);
    assert_eq!(data.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn quick_sort_plateau() {
    let mut data = ArraySeqStEphSLit![5, 5, 5, 1, 1, 1, 9, 9, 9];
    ArraySeqStEphS::quick_sort_random(&mut data);
    assert_eq!(data.to_vec(), vec![1, 1, 1, 5, 5, 5, 9, 9, 9]);
}

#[test]
fn quick_sort_min_max_values() {
    let mut data = ArraySeqStEphSLit![i32::MAX, i32::MIN, 0, i32::MAX, i32::MIN];
    ArraySeqStEphS::quick_sort_median3(&mut data);
    assert_eq!(data.to_vec(), vec![i32::MIN, i32::MIN, 0, i32::MAX, i32::MAX]);
}
