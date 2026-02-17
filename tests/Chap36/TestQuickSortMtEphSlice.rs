#![cfg(feature = "all_chapters")]
//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
#![allow(clippy::unnecessary_mut_passed)]

use rand::*;

use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
use apas_verus::Chap36::QuickSortMtEphSlice::Chapter36MtEphSlice::*;
use apas_verus::Types::Types::*;

fn to_vec<T: StT + Send + Sync + 'static>(a: &ArraySeqMtEphSliceS<T>) -> Vec<T> { a.to_vec() }

fn is_sorted<T: StT + Ord>(values: &[T]) -> bool { values.windows(2).all(|w| w[0] <= w[1]) }

fn mk_seq(data: &[i32]) -> ArraySeqMtEphSliceS<i32> { ArraySeqMtEphSliceS::from_vec(data.to_vec()) }

#[test]
fn quick_sort_slice_variants_produce_sorted_output() {
    let base = mk_seq(&[5, 3, 1, 4, 2, 2, 3]);
    let expected = vec![1, 2, 2, 3, 3, 4, 5];

    let mut first = base.clone();
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut first);
    assert_eq!(to_vec(&first), expected);

    let mut median3 = base.clone();
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut median3);
    assert_eq!(to_vec(&median3), expected);

    let mut random = base.clone();
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_random(&mut random);
    assert_eq!(to_vec(&random), expected);
}

#[test]
fn quick_sort_slice_edge_cases() {
    let mut empty = ArraySeqMtEphSliceS::from_vec(Vec::new());
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut empty);
    assert!(to_vec(&empty).is_empty());

    let mut single = mk_seq(&[42]);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut single);
    assert_eq!(to_vec(&single), vec![42]);

    let mut sorted = mk_seq(&[1, 2, 3, 4, 5]);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_random(&mut sorted);
    assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);

    let mut reversed = mk_seq(&[5, 4, 3, 2, 1]);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut reversed);
    assert_eq!(to_vec(&reversed), vec![1, 2, 3, 4, 5]);

    let mut pair = mk_seq(&[2, 1]);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut pair);
    assert_eq!(to_vec(&pair), vec![1, 2]);
}

#[test]
fn quick_sort_slice_large_inputs() {
    let mut descending = ArraySeqMtEphSliceS::from_vec((0..230).rev().collect());
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut descending);
    assert!(is_sorted(&to_vec(&descending)));

    let mut rng = rng();
    let random_data = (0..230).map(|_| rng.random_range(-10_000..10_000)).collect::<Vec<i32>>();
    let mut random_seq = ArraySeqMtEphSliceS::from_vec(random_data);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_random(&mut random_seq);
    assert!(is_sorted(&to_vec(&random_seq)));
}

#[test]
fn slice_pivot_strategies_match_expectations() {
    let seq = mk_seq(&[9, 1, 5, 3, 7]);
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 9);

    let median_case = mk_seq(&[3, 8, 5, 6, 7]);
    assert_eq!(median_case.pivot_mt_median3(0, median_case.length()), 5);

    let random_seq = mk_seq(&[10, 20, 30, 40, 50]);
    let pivot = random_seq.pivot_mt_random(1, 4);
    let mut within = false;
    for idx in 1..4 {
        if random_seq.nth_cloned(idx) == pivot {
            within = true;
            break;
        }
    }
    assert!(within, "random pivot should be drawn from the requested sub-range");
}

#[test]
fn quick_sort_slice_small_inputs_use_shared_pivots() {
    let mut seq = mk_seq(&[4, 1, 3]);
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 4);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut seq);
    assert_eq!(to_vec(&seq), vec![1, 3, 4]);

    let mut seq_med = mk_seq(&[8, 2, 7, 1, 5]);
    assert_eq!(seq_med.pivot_mt_median3(0, seq_med.length()), 7);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut seq_med);
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
fn slice_pivot_mt_first_edge_cases() {
    let single = mk_seq(&[42]);
    assert_eq!(single.pivot_mt_first(0, 1), 42);

    let seq = mk_seq(&[1, 2, 3, 4, 5]);
    assert_eq!(seq.pivot_mt_first(2, 5), 3); // Should return element at start index
    assert_eq!(seq.pivot_mt_first(0, 3), 1); // Should return first element in range
}

#[test]
fn slice_pivot_mt_median3_edge_cases() {
    // Test with exactly 3 elements
    let three = mk_seq(&[3, 1, 2]);
    let median = three.pivot_mt_median3(0, 3);
    // Median of [3, 1, 2] should be 2
    assert_eq!(median, 2);

    // Test with larger range
    let seq = mk_seq(&[5, 1, 9, 3, 7, 2, 8]);
    let median_large = seq.pivot_mt_median3(1, 6); // Range [1, 9, 3, 7, 2]
    // Should pick median of first, middle, last: [1, 3, 2] -> median is 2
    assert_eq!(median_large, 2);
}

#[test]
fn slice_pivot_mt_random_range_validation() {
    let seq = mk_seq(&[10, 20, 30, 40, 50, 60]);

    // Test various ranges
    for start in 0..4 {
        for end in (start + 1)..6 {
            let pivot = seq.pivot_mt_random(start, end);

            // Verify pivot is within the specified range
            let mut found = false;
            for i in start..end {
                if seq.nth_cloned(i) == pivot {
                    found = true;
                    break;
                }
            }
            assert!(found, "Random pivot {pivot} not found in range [{start}..{end})");
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
                | 0 => <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_first(&mut seq),
                | 1 => <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut seq),
                | _ => <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_random(&mut seq),
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
            let first_pivot = seq_clone.pivot_mt_first(0, seq_clone.length());
            let median_pivot = seq_clone.pivot_mt_median3(2, 8);
            let random_pivot = seq_clone.pivot_mt_random(1, 9);

            (first_pivot, median_pivot, random_pivot)
        }));
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify consistency for deterministic pivots
    for result in &results {
        assert_eq!(result.0, 15); // First pivot should always be the same
        assert_eq!(result.1, results[0].1); // Median3 should be consistent for same range
    }

    // Random pivots should be within valid range
    for result in &results {
        let random_pivot = result.2;
        let mut found = false;
        for i in 1..9 {
            if seq.nth_cloned(i) == random_pivot {
                found = true;
                break;
            }
        }
        assert!(found, "Random pivot {random_pivot} not in expected range");
    }
}

#[test]
fn slice_large_data_handling() {
    // Test with larger datasets to verify scalability
    let large_size = 10_000;
    let mut large_data = (0..large_size).collect::<Vec<i32>>();
    large_data.reverse(); // Make it reverse sorted (worst case)

    let mut seq = ArraySeqMtEphSliceS::from_vec(large_data);
    <ArraySeqMtEphSliceS<i32> as Chapter36MtSliceTrait<i32>>::quick_sort_mt_median3(&mut seq);

    let result = seq.to_vec();
    assert_eq!(result.len(), large_size as usize);
    assert!(is_sorted(&result));
    assert_eq!(result[0], 0);
    assert_eq!(result[large_size as usize - 1], large_size - 1);
}
