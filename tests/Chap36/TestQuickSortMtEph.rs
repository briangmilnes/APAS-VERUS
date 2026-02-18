//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::ArraySeqMtEphChap19SLit;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap36::QuickSortMtEph::Chapter36Mt::*;
use apas_verus::Types::Types::*;

fn to_vec<T: StTInMtT>(a: &ArraySeqMtEphS<T>) -> Vec<T> { (0..a.length()).map(|i| a.nth(i).clone()).collect() }

fn is_sorted<T: StTInMtT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }

#[test]
fn quick_sort_mt_variants_produce_sorted_output() {
    let base = ArraySeqMtEphChap19SLit![5, 3, 1, 4, 2, 2, 3];
    let expected = vec![1, 2, 2, 3, 3, 4, 5];

    let mut first = base.clone();
    quick_sort_first(&mut first);
    assert_eq!(to_vec(&first), expected);

    let mut median3 = base.clone();
    quick_sort_median3(&mut median3);
    assert_eq!(to_vec(&median3), expected);

    let mut random = base.clone();
    quick_sort_random(&mut random);
    assert_eq!(to_vec(&random), expected);
}

#[test]
fn quick_sort_mt_edge_cases() {
    let mut empty: ArraySeqMtEphS<i32> = ArraySeqMtEphChap19SLit![];
    quick_sort_first(&mut empty);
    assert!(to_vec(&empty).is_empty());

    let mut single = ArraySeqMtEphChap19SLit![42];
    quick_sort_median3(&mut single);
    assert_eq!(to_vec(&single), vec![42]);

    let mut sorted = ArraySeqMtEphChap19SLit![1, 2, 3, 4, 5];
    quick_sort_random(&mut sorted);
    assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);

    let mut reversed = ArraySeqMtEphChap19SLit![5, 4, 3, 2, 1];
    quick_sort_first(&mut reversed);
    assert_eq!(to_vec(&reversed), vec![1, 2, 3, 4, 5]);

    let mut pair = ArraySeqMtEphChap19SLit![2, 1];
    quick_sort_median3(&mut pair);
    assert_eq!(to_vec(&pair), vec![1, 2]);
}

#[test]
fn quick_sort_mt_large_inputs() {
    let mut descending = ArraySeqMtEphS::from_vec((0..230).rev().collect());
    quick_sort_first(&mut descending);
    assert!(is_sorted(&to_vec(&descending)));
}

#[test]
fn quick_sort_mt_concurrent_execution() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let test_data = Arc::new(ArraySeqMtEphChap19SLit![9, 3, 7, 1, 5, 8, 2, 6, 4]);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let barrier = Arc::new(Barrier::new(3));

    let mut handles = vec![];

    let data1 = Arc::clone(&test_data);
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        let mut seq = (*data1).clone();
        quick_sort_first(&mut seq);
        to_vec(&seq)
    }));

    let data2 = Arc::clone(&test_data);
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        let mut seq = (*data2).clone();
        quick_sort_median3(&mut seq);
        to_vec(&seq)
    }));

    let data3 = Arc::clone(&test_data);
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let mut seq = (*data3).clone();
        quick_sort_random(&mut seq);
        to_vec(&seq)
    }));

    for handle in handles {
        let result = handle.join().unwrap();
        assert_eq!(result, expected);
    }
}
