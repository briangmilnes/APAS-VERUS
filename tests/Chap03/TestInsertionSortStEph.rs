// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

use apas_verus::Chap03::InsertionSortStEph::InsertionSortStEph::*;

fn sort_and_assert(mut data: Vec<u64>, expected: &[u64]) {
    let result = insertion_sort(&mut data);
    assert_eq!(result, expected);
}

#[test]
fn insertion_sort_handles_empty() {
    let mut data = Vec::<u64>::new();
    let result = insertion_sort(&mut data);
    assert!(result.is_empty());
}

#[test]
fn insertion_sort_single_element() { sort_and_assert(vec![42], &[42]); }

#[test]
fn insertion_sort_already_sorted() { sort_and_assert(vec![1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]); }

#[test]
fn insertion_sort_reverse_order() { sort_and_assert(vec![5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]); }

#[test]
fn insertion_sort_with_duplicates() { sort_and_assert(vec![3, 1, 2, 3, 1], &[1, 1, 2, 3, 3]); }

#[test]
fn insertion_sort_random_slice() {
    let mut data = vec![10, 1, 7, 3, 3, 9, 0, 5];
    let mut expected = data.clone();
    expected.sort();
    let result = insertion_sort(&mut data);
    assert_eq!(result, expected.as_slice());
}

#[test]
fn insertion_sort_large_input_stress_test() {
    // Generate a large vector with 10,000+ elements
    let mut data = (0..10_000).rev().collect::<Vec<u64>>(); // Reverse sorted - worst case
    let mut expected = data.clone();
    expected.sort();

    let result = insertion_sort(&mut data);
    assert_eq!(result, expected.as_slice());
    assert_eq!(result.len(), 10_000);

    // Verify it's actually sorted
    for i in 1..result.len() {
        assert!(result[i - 1] <= result[i], "Array not properly sorted at index {i}");
    }
}

#[test]
fn insertion_sort_all_same_elements() {
    sort_and_assert(vec![7, 7, 7, 7, 7], &[7, 7, 7, 7, 7]);
}

#[test]
fn insertion_sort_two_elements() {
    sort_and_assert(vec![2, 1], &[1, 2]);
}

#[test]
fn insertion_sort_two_elements_sorted() {
    sort_and_assert(vec![1, 2], &[1, 2]);
}

#[test]
fn insertion_sort_alternating_pattern() {
    sort_and_assert(vec![1, 100, 2, 99, 3, 98], &[1, 2, 3, 98, 99, 100]);
}

#[test]
fn insertion_sort_with_zeros() {
    sort_and_assert(vec![0, 0, 1, 0, 0], &[0, 0, 0, 0, 1]);
}

#[test]
fn insertion_sort_max_values() {
    sort_and_assert(vec![u64::MAX, 0, u64::MAX, 1], &[0, 1, u64::MAX, u64::MAX]);
}

#[test]
fn insertion_sort_preserves_length() {
    for len in 0..20 {
        let mut data: Vec<u64> = (0..len).rev().collect();
        let result = insertion_sort(&mut data);
        assert_eq!(result.len(), len as usize);
    }
}

#[test]
fn insertion_sort_stability_like_behavior() {
    // All elements same — sorted output should equal input.
    let mut data = vec![42u64; 50];
    let result = insertion_sort(&mut data);
    assert!(result.iter().all(|&x| x == 42));
    assert_eq!(result.len(), 50);
}

#[test]
fn insertion_sort_sorted_pairs() {
    sort_and_assert(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn insertion_sort_sawtooth_pattern() {
    sort_and_assert(vec![1, 5, 2, 5, 3, 5, 4, 5], &[1, 2, 3, 4, 5, 5, 5, 5]);
}

#[test]
fn insertion_sort_powers_of_two() {
    sort_and_assert(vec![64, 1, 32, 2, 16, 4, 8], &[1, 2, 4, 8, 16, 32, 64]);
}

#[test]
fn insertion_sort_nearly_sorted() {
    // Only last element is out of place.
    sort_and_assert(vec![1, 2, 3, 4, 5, 6, 7, 0], &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn insertion_sort_nearly_sorted_one_swap() {
    // Adjacent elements swapped in middle.
    sort_and_assert(vec![1, 2, 4, 3, 5, 6], &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn insertion_sort_three_elements() {
    let perms: [[u64; 3]; 6] = [
        [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1],
    ];
    for perm in &perms {
        let mut data = perm.to_vec();
        let result = insertion_sort(&mut data);
        assert_eq!(result, &[1, 2, 3], "Failed for perm {:?}", perm);
    }
}

#[test]
fn insertion_sort_large_values_spread() {
    sort_and_assert(
        vec![u64::MAX, u64::MAX / 2, 0, u64::MAX / 4, u64::MAX / 3],
        &[0, u64::MAX / 4, u64::MAX / 3, u64::MAX / 2, u64::MAX],
    );
}

#[test]
fn insertion_sort_many_duplicates() {
    sort_and_assert(
        vec![3, 1, 3, 1, 3, 1, 3, 1],
        &[1, 1, 1, 1, 3, 3, 3, 3],
    );
}

#[test]
fn insertion_sort_descending_10() {
    sort_and_assert(
        vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
}

#[test]
fn insertion_sort_one_distinct_rest_same() {
    sort_and_assert(vec![5, 5, 5, 1, 5, 5], &[1, 5, 5, 5, 5, 5]);
}

#[test]
fn insertion_sort_consecutive_integers() {
    let mut data: Vec<u64> = (1..=100).rev().collect();
    let expected: Vec<u64> = (1..=100).collect();
    let result = insertion_sort(&mut data);
    assert_eq!(result, expected.as_slice());
}

#[test]
fn insertion_sort_head_is_minimum() {
    let mut data = vec![0, 99, 88, 77, 66, 55];
    let result = insertion_sort(&mut data);
    assert_eq!(result[0], 0);
    assert_eq!(result[result.len() - 1], 99);
}

#[test]
fn insertion_sort_tail_is_maximum() {
    let mut data = vec![50, 30, 10, 99, 20, 40];
    let result = insertion_sort(&mut data);
    assert_eq!(result[result.len() - 1], 99);
}

#[test]
fn insertion_sort_even_length() {
    sort_and_assert(vec![4, 2, 6, 8], &[2, 4, 6, 8]);
}

#[test]
fn insertion_sort_odd_length() {
    sort_and_assert(vec![9, 1, 5, 3, 7], &[1, 3, 5, 7, 9]);
}

#[test]
fn insertion_sort_interleaved_high_low() {
    sort_and_assert(
        vec![100, 1, 99, 2, 98, 3, 97, 4],
        &[1, 2, 3, 4, 97, 98, 99, 100],
    );
}

#[test]
fn insertion_sort_repeated_min_max() {
    sort_and_assert(
        vec![0, u64::MAX, 0, u64::MAX, 0],
        &[0, 0, 0, u64::MAX, u64::MAX],
    );
}

#[test]
fn insertion_sort_fibonacci_values() {
    sort_and_assert(
        vec![13, 8, 21, 1, 5, 3, 2, 1],
        &[1, 1, 2, 3, 5, 8, 13, 21],
    );
}

#[test]
fn insertion_sort_single_repeated() {
    sort_and_assert(vec![42, 42], &[42, 42]);
}

#[test]
fn insertion_sort_four_elements_all_permutations() {
    let expected = [1u64, 2, 3, 4];
    // Test a selection of permutations.
    let perms: [[u64; 4]; 6] = [
        [4, 3, 2, 1],
        [2, 4, 1, 3],
        [3, 1, 4, 2],
        [1, 4, 3, 2],
        [4, 1, 2, 3],
        [2, 3, 4, 1],
    ];
    for perm in &perms {
        sort_and_assert(perm.to_vec(), &expected);
    }
}

#[test]
fn insertion_sort_plateau_then_drop() {
    sort_and_assert(vec![10, 10, 10, 1, 1, 1], &[1, 1, 1, 10, 10, 10]);
}

#[test]
fn insertion_sort_organ_pipe() {
    // Ascending then descending.
    sort_and_assert(vec![1, 3, 5, 7, 6, 4, 2], &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn insertion_sort_mod_3_pattern() {
    sort_and_assert(vec![0, 2, 1, 0, 2, 1, 0, 2, 1], &[0, 0, 0, 1, 1, 1, 2, 2, 2]);
}

#[test]
fn insertion_sort_returns_same_slice() {
    // Verify the returned slice is a reference into the same data.
    let mut data = vec![3, 1, 2];
    let result = insertion_sort(&mut data);
    assert_eq!(result, &[1, 2, 3]);
    assert_eq!(data, vec![1, 2, 3]);
}

#[test]
fn insertion_sort_medium_random_stress() {
    // 500 elements in a pseudo-random order.
    let mut data: Vec<u64> = (0..500).collect();
    // Simple LCG shuffle.
    let mut rng: u64 = 54321;
    for i in (1..500usize).rev() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (rng >> 33) as usize % (i + 1);
        data.swap(i, j);
    }
    let mut expected = data.clone();
    expected.sort();
    let result = insertion_sort(&mut data);
    assert_eq!(result, expected.as_slice());
}
