//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqStEphSLit;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap36::QuickSortStEph::Chapter36St::*;
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
    quick_sort_first(&mut first);
    let mut median3 = base.clone();
    quick_sort_median3(&mut median3);
    let mut random = base.clone();
    quick_sort_random(&mut random);
    let expected = ArraySeqStEphSLit![1, 2, 2, 3, 3, 4, 5];

    assert_eq!(first, expected);
    assert_eq!(median3, expected);
    assert_eq!(random, expected);
}

#[test]
fn quick_sort_handles_edge_cases() {
    let mut empty: ArraySeqStEphS<i32> = ArraySeqStEphSLit![];
    quick_sort_first(&mut empty);
    assert_eq!(empty.to_vec(), vec![]);

    let mut single = ArraySeqStEphSLit![42];
    quick_sort_median3(&mut single);
    assert_eq!(single.to_vec(), vec![42]);

    let mut sorted = ArraySeqStEphSLit![1, 2, 3, 4, 5];
    quick_sort_random(&mut sorted);
    assert_eq!(sorted.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut reversed = ArraySeqStEphSLit![5, 4, 3, 2, 1];
    quick_sort_first(&mut reversed);
    assert_eq!(reversed.to_vec(), vec![1, 2, 3, 4, 5]);

    let mut pair = ArraySeqStEphSLit![2, 1];
    quick_sort_median3(&mut pair);
    assert_eq!(pair.to_vec(), vec![1, 2]);
}
