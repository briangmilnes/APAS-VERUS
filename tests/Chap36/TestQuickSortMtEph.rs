//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use rand::*;

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
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut first);
    assert_eq!(to_vec(&first), expected);

    let mut median3 = base.clone();
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut median3);
    assert_eq!(to_vec(&median3), expected);

    let mut random = base.clone();
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_random(&mut random);
    assert_eq!(to_vec(&random), expected);
}

#[test]
fn quick_sort_mt_edge_cases() {
    let mut empty: ArraySeqMtEphS<i32> = ArraySeqMtEphChap19SLit![];
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut empty);
    assert!(to_vec(&empty).is_empty());

    let mut single = ArraySeqMtEphChap19SLit![42];
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut single);
    assert_eq!(to_vec(&single), vec![42]);

    let mut sorted = ArraySeqMtEphChap19SLit![1, 2, 3, 4, 5];
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_random(&mut sorted);
    assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);

    let mut reversed = ArraySeqMtEphChap19SLit![5, 4, 3, 2, 1];
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut reversed);
    assert_eq!(to_vec(&reversed), vec![1, 2, 3, 4, 5]);

    let mut pair = ArraySeqMtEphChap19SLit![2, 1];
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut pair);
    assert_eq!(to_vec(&pair), vec![1, 2]);
}

#[test]
fn pivot_mt_strategies_match_expectations() {
    let seq = ArraySeqMtEphChap19SLit![9, 1, 5, 3, 7];
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 9);
    assert_eq!(seq.pivot_mt_median3(0, seq.length()), 7);

    let random_seq = ArraySeqMtEphChap19SLit![10, 20, 30, 40, 50];
    let pivot = random_seq.pivot_mt_random(1, 4);
    let mut within = false;
    for idx in 1..4 {
        if random_seq.nth(idx).clone() == pivot {
            within = true;
            break;
        }
    }
    assert!(within, "random pivot should be drawn from the requested sub-range");

    let median_case = ArraySeqMtEphChap19SLit![3, 8, 5, 6, 7];
    assert_eq!(median_case.pivot_mt_median3(0, median_case.length()), 5);
}

#[test]
fn quick_sort_mt_large_inputs() {
    let mut descending = ArraySeqMtEphS::from_vec((0..230).rev().collect());
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut descending);
    assert!(is_sorted(&to_vec(&descending)));

    let mut rng = rng();
    let random_data = (0..230).map(|_| rng.random_range(-10_000..10_000)).collect::<Vec<i32>>();
    let mut random_seq = ArraySeqMtEphS::from_vec(random_data);
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_random(&mut random_seq);
    assert!(is_sorted(&to_vec(&random_seq)));
}

#[test]
fn quick_sort_mt_small_inputs_use_shared_pivots() {
    let mut seq = ArraySeqMtEphChap19SLit![4, 1, 3];
    assert_eq!(seq.pivot_mt_first(0, seq.length()), 4);
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut seq);
    assert_eq!(to_vec(&seq), vec![1, 3, 4]);

    let mut seq_med = ArraySeqMtEphChap19SLit![8, 2, 7, 1, 5];
    assert_eq!(seq_med.pivot_mt_median3(0, seq_med.length()), 7);
    <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut seq_med);
    assert_eq!(to_vec(&seq_med), vec![1, 2, 5, 7, 8]);
}

#[test]
fn quick_sort_mt_concurrent_execution_verification() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    // Test concurrent execution of QuickSort Mt variants
    let test_data = Arc::new(ArraySeqMtEphChap19SLit![9, 3, 7, 1, 5, 8, 2, 6, 4]);
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let barrier = Arc::new(Barrier::new(3));

    let mut handles = vec![];

    // Thread 1: quick_sort_mt_first
    let data1 = Arc::clone(&test_data);
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        let mut seq = (*data1).clone();
        <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut seq);
        to_vec(&seq)
    }));

    // Thread 2: quick_sort_mt_median3
    let data2 = Arc::clone(&test_data);
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        let mut seq = (*data2).clone();
        <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut seq);
        to_vec(&seq)
    }));

    // Thread 3: quick_sort_mt_random
    let data3 = Arc::clone(&test_data);
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let mut seq = (*data3).clone();
        <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_random(&mut seq);
        to_vec(&seq)
    }));

    // Collect results from all threads
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // All threads should produce the same sorted result
    for result in results {
        assert_eq!(result, expected);
        assert!(is_sorted(&result));
    }
}

#[test]
fn quick_sort_mt_thread_safety_stress_test() {
    use std::sync::Arc;
    use std::thread;

    // Stress test with multiple threads sorting different data concurrently
    let num_threads = 4;
    let data_size: usize = 1000;

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        handles.push(thread::spawn(move || {
            // Each thread gets different data based on thread_id
            let mut data = (0..data_size).map(|i| ((i * thread_id + 1) % 100) as i32).collect::<Vec<i32>>();
            data.reverse(); // Make it unsorted

            let mut seq = ArraySeqMtEphS::from_vec(data);

            // Use different sorting variants based on thread_id
            match thread_id % 3 {
                | 0 => <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_first(&mut seq),
                | 1 => <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_median3(&mut seq),
                | _ => <ArraySeqMtEphS<i32> as Chapter36MtTrait<i32>>::quick_sort_mt_random(&mut seq),
            }

            let result = to_vec(&seq);
            (thread_id, is_sorted(&result), result.len())
        }));
    }

    // Verify all threads completed successfully
    for handle in handles {
        let (thread_id, is_sorted_result, length) = handle.join().unwrap();
        assert!(is_sorted_result, "Thread {thread_id} failed to sort correctly");
        assert_eq!(length, data_size, "Thread {thread_id} lost data during sorting");
    }
}

#[test]
fn quick_sort_mt_pivot_strategies_concurrent() {
    use std::sync::Arc;
    use std::thread;

    // Test pivot selection strategies concurrently
    let test_seq = Arc::new(ArraySeqMtEphChap19SLit![15, 3, 9, 1, 12, 7, 20, 5, 18, 11]);
    let mut handles = vec![];

    // Test pivot_mt_first concurrently
    for _ in 0..3 {
        let seq = Arc::clone(&test_seq);
        handles.push(thread::spawn(move || seq.pivot_mt_first(0, seq.length())));
    }

    // Test pivot_mt_median3 concurrently
    for _ in 0..3 {
        let seq = Arc::clone(&test_seq);
        handles.push(thread::spawn(move || {
            seq.pivot_mt_median3(2, 8) // Test with subrange
        }));
    }

    // Test pivot_mt_random concurrently
    for _ in 0..3 {
        let seq = Arc::clone(&test_seq);
        handles.push(thread::spawn(move || {
            seq.pivot_mt_random(1, 9) // Test with subrange
        }));
    }

    // Collect all results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify pivot_mt_first results (should be consistent)
    assert_eq!(results[0], results[1]);
    assert_eq!(results[1], results[2]);
    assert_eq!(results[0], 15); // First element

    // Verify pivot_mt_median3 results (should be consistent for same range)
    assert_eq!(results[3], results[4]);
    assert_eq!(results[4], results[5]);

    // pivot_mt_random results may vary, but should be within valid range
    for &pivot in results.iter().skip(6).take(3) {
        let mut found = false;
        for j in 1..9 {
            if test_seq.nth(j).clone() == pivot {
                found = true;
                break;
            }
        }
        assert!(found, "Random pivot {pivot} not found in expected range");
    }
}
