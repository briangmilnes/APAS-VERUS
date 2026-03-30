//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

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
