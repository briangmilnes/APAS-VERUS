// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
#![allow(clippy::unnecessary_mut_passed)]

use rand::*;

use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
use apas_verus::Chap36::QuickSortMtEphSlice::QuickSortMtEphSlice::*;
use apas_verus::Types::Types::*;

fn to_vec<T: StT + Send + Sync + 'static>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }

fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }

fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec()) }

#[test]
fn quick_sort_slice_variants_produce_sorted_output() {
    let base = mk_seq(&[5, 3, 1, 4, 2, 2, 3]);
    let expected = vec![1, 2, 2, 3, 3, 4, 5];

    let mut first = base.clone();
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut first);
    assert_eq!(to_vec(&first), expected);

    let mut median3 = base.clone();
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut median3);
    assert_eq!(to_vec(&median3), expected);

    let mut random = base.clone();
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut random);
    assert_eq!(to_vec(&random), expected);
}

#[test]
fn quick_sort_slice_edge_cases() {
    let mut empty = ArraySeqMtEphSliceS::from_vec(Vec::new());
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut empty);
    assert!(to_vec(&empty).is_empty());

    let mut single = mk_seq(&[42]);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut single);
    assert_eq!(to_vec(&single), vec![42]);

    let mut sorted = mk_seq(&[1, 2, 3, 4, 5]);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut sorted);
    assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);

    let mut reversed = mk_seq(&[5, 4, 3, 2, 1]);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut reversed);
    assert_eq!(to_vec(&reversed), vec![1, 2, 3, 4, 5]);

    let mut pair = mk_seq(&[2, 1]);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut pair);
    assert_eq!(to_vec(&pair), vec![1, 2]);
}

#[test]
fn quick_sort_slice_large_inputs() {
    let mut descending = ArraySeqMtEphSliceS::from_vec((0..230).rev().collect());
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut descending);
    assert!(is_sorted(&to_vec(&descending)));

    let mut rng = rng();
    let random_data = (0..230).map(|_| rng.random_range(-10_000..10_000)).collect::<Vec<i32>>();
    let mut random_seq = ArraySeqMtEphSliceS::from_vec(random_data);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut random_seq);
    assert!(is_sorted(&to_vec(&random_seq)));
}

fn median3_pivot(a: &ArraySeqMtEphSliceS<i32>) -> i32 {
    let idx = <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::median3_pivot_idx(a, a.length());
    a.nth_cloned(idx)
}

#[test]
fn slice_pivot_strategies_match_expectations() {
    type Q = ArraySeqMtEphSliceS<i32>;
    assert_eq!(<Q as QuickSortMtEphSliceTrait<i32>>::median_of_three(9, 1, 5), 5);
    assert_eq!(<Q as QuickSortMtEphSliceTrait<i32>>::median_of_three(1, 9, 5), 5);
    assert_eq!(<Q as QuickSortMtEphSliceTrait<i32>>::median_of_three(5, 5, 1), 5);

    // Median of a[0] = 3, a[n/2] = 5, a[n-1] = 7.
    let median_case = mk_seq(&[3, 8, 5, 6, 7]);
    assert_eq!(median3_pivot(&median_case), 5);

    // Two elements: a[0] = 20, a[1] = 10, a[1] = 10.
    let pair = mk_seq(&[20, 10]);
    assert_eq!(median3_pivot(&pair), 10);
}

#[test]
fn quick_sort_slice_small_inputs_use_shared_pivots() {
    let mut seq = mk_seq(&[4, 1, 3]);
    assert_eq!(seq.nth_cloned(0), 4);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut seq);
    assert_eq!(to_vec(&seq), vec![1, 3, 4]);

    // Median of a[0] = 8, a[n/2] = 7, a[n-1] = 5.
    let mut seq_med = mk_seq(&[8, 2, 7, 1, 5]);
    assert_eq!(median3_pivot(&seq_med), 7);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut seq_med);
    assert_eq!(to_vec(&seq_med), vec![1, 2, 5, 7, 8]);
}

#[test]
fn slice_length_method() {
    let empty = ArraySeqMtEphSliceS::<i32>::from_vec(vec![]);
    assert_eq!(empty.length(), 0);

    let single = mk_seq(&[42]);
    assert_eq!(single.length(), 1);

    let multi = mk_seq(&[1, 2, 3, 4, 5]);
    assert_eq!(multi.length(), 5);
}

#[test]
fn slice_nth_cloned_method() {
    let seq = mk_seq(&[10, 20, 30, 40, 50]);

    assert_eq!(seq.nth_cloned(0), 10);
    assert_eq!(seq.nth_cloned(2), 30);
    assert_eq!(seq.nth_cloned(4), 50);
}

#[test]
fn slice_to_vec_method() {
    let seq = mk_seq(&[1, 2, 3, 4]);
    let vec_result = seq.to_vec();
    assert_eq!(vec_result, vec![1, 2, 3, 4]);

    let empty = ArraySeqMtEphSliceS::<i32>::from_vec(vec![]);
    let empty_vec = empty.to_vec();
    assert_eq!(empty_vec, Vec::<i32>::new());
}

#[test]
fn slice_from_vec_constructor() {
    let data = vec![5, 10, 15, 20];
    let seq = ArraySeqMtEphSliceS::from_vec(data.clone());
    assert_eq!(seq.to_vec(), data);
    assert_eq!(seq.length(), 4);
}

#[test]
fn slice_clone_functionality() {
    let original = mk_seq(&[1, 2, 3]);
    let cloned = original.clone();

    assert_eq!(original.to_vec(), cloned.to_vec());
    assert_eq!(original.length(), cloned.length());

    // Verify they're independent copies
    assert_eq!(original.nth_cloned(1), cloned.nth_cloned(1));
}

#[test]
fn slice_pivot_first_edge_cases() {
    // The first-element pivot of a range is the element at the range start.
    let seq = mk_seq(&[1, 2, 3, 4, 5]);
    assert_eq!(seq.slice(2, 3).nth_cloned(0), 3);
    assert_eq!(seq.slice(0, 3).nth_cloned(0), 1);

    let mut sub = seq.slice(2, 3);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut sub);
    assert_eq!(to_vec(&sub), vec![3, 4, 5]);
}

#[test]
fn slice_pivot_median3_edge_cases() {
    // Exactly three elements: median of 3, 1, 2 is 2.
    let three = mk_seq(&[3, 1, 2]);
    assert_eq!(median3_pivot(&three), 2);

    // Range [1, 6) is [1, 9, 3, 7, 2]: median of first 1, middle 3, last 2 is 2.
    let seq = mk_seq(&[5, 1, 9, 3, 7, 2, 8]);
    assert_eq!(median3_pivot(&seq.slice(1, 5)), 2);
}

#[test]
fn slice_random_pivot_sorts_every_range() {
    let seq = mk_seq(&[60, 10, 50, 20, 40, 30]);
    for start in 0..6 {
        for end in (start + 1)..=6 {
            let mut sub = seq.slice(start, end - start);
            <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut sub);
            let mut expected: Vec<i32> = (start..end).map(|i| seq.nth_cloned(i)).collect();
            expected.sort();
            assert_eq!(to_vec(&sub), expected, "range [{start}..{end})");
        }
    }
}

#[test]
fn slice_concurrent_sorting_stress_test() {
    use std::sync::Arc;
    use std::thread;

    // Test concurrent sorting operations
    let test_data = vec![9, 3, 7, 1, 5, 8, 2, 6, 4, 0];
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut handles = vec![];

    // Spawn multiple threads doing different sorts
    for thread_id in 0..6 {
        let data = test_data.clone();
        handles.push(thread::spawn(move || {
            let mut seq = ArraySeqMtEphSliceS::from_vec(data);

            match thread_id % 3 {
                | 0 => <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_first(&mut seq),
                | 1 => <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut seq),
                | _ => <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut seq),
            }

            seq.to_vec()
        }));
    }

    // Verify all threads produce correct results
    for handle in handles {
        let result = handle.join().unwrap();
        assert_eq!(result, expected);
        assert!(is_sorted(&result));
    }
}

#[test]
fn slice_pivot_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let seq = Arc::new(mk_seq(&[15, 3, 9, 1, 12, 7, 20, 5, 18, 11]));
    let mut handles = vec![];

    // Test concurrent pivot operations
    for _ in 0..4 {
        let seq_clone = Arc::clone(&seq);
        handles.push(thread::spawn(move || {
            let first_pivot = seq_clone.nth_cloned(0);
            let median_pivot = median3_pivot(&seq_clone.slice(2, 6));
            let mut random_sorted = seq_clone.slice(1, 8);
            <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_random(&mut random_sorted);

            (first_pivot, median_pivot, to_vec(&random_sorted))
        }));
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Range [2, 8) is [9, 1, 12, 7, 20, 5]: median of first 9, middle 7, last 5 is 7.
    let mut expected_sorted: Vec<i32> = (1..9).map(|i| seq.nth_cloned(i)).collect();
    expected_sorted.sort();
    for result in &results {
        assert_eq!(result.0, 15);
        assert_eq!(result.1, 7);
        assert_eq!(result.2, expected_sorted);
    }
    // The shared sequence is unchanged by the readers.
    assert_eq!(to_vec(&seq), vec![15, 3, 9, 1, 12, 7, 20, 5, 18, 11]);
}

#[test]
fn slice_large_data_handling() {
    // Test with larger datasets to verify scalability
    let large_size = 10_000;
    let mut large_data = (0..large_size).collect::<Vec<i32>>();
    large_data.reverse(); // Make it reverse sorted (worst case)

    let mut seq = ArraySeqMtEphSliceS::from_vec(large_data);
    <ArraySeqMtEphSliceS<i32> as QuickSortMtEphSliceTrait<i32>>::quick_sort_median3(&mut seq);

    let result = seq.to_vec();
    assert_eq!(result.len(), large_size as usize);
    assert!(is_sorted(&result));
    assert_eq!(result[0], 0);
    assert_eq!(result[large_size as usize - 1], large_size - 1);
}
