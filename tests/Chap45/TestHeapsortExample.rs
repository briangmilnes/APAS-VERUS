//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap45 HeapsortExample.

use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerTrait, *};
use apas_verus::Chap45::HeapsortExample::HeapsortExample::*;
use apas_verus::Types::Types::*;

#[test]
fn test_heapsort_unsorted_list_basic() {
    let input = vec![5, 3, 8, 1, 9, 2];
    let result = heapsort_unsorted_list(&input);
    assert_eq!(result, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_heapsort_sorted_list_basic() {
    let input = vec![5, 3, 8, 1, 9, 2];
    let result = heapsort_sorted_list(&input);
    assert_eq!(result, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_heapsort_balanced_tree_basic() {
    let input = vec![5, 3, 8, 1, 9, 2];
    let result = heapsort_balanced_tree(&input);
    assert_eq!(result, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_heapsort_binary_heap_basic() {
    let input = vec![5, 3, 8, 1, 9, 2];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_heapsort_leftist_heap_basic() {
    let input = vec![5, 3, 8, 1, 9, 2];
    let result = heapsort_leftist_heap(&input);
    assert_eq!(result, vec![1, 2, 3, 5, 8, 9]);
}

#[test]
fn test_heapsort_empty() {
    let input: Vec<i32> = vec![];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![]);
}

#[test]
fn test_heapsort_single_element() {
    let input = vec![42];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![42]);
}

#[test]
fn test_heapsort_already_sorted() {
    let input = vec![1, 2, 3, 4, 5];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_heapsort_reverse_sorted() {
    let input = vec![5, 4, 3, 2, 1];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_heapsort_duplicates() {
    let input = vec![5, 2, 8, 2, 9, 1, 5, 5];
    let result = heapsort_binary_heap(&input);
    assert_eq!(result, vec![1, 2, 2, 5, 5, 5, 8, 9]);
}

#[test]
fn test_compare_all_heapsorts() {
    let input = vec![64, 34, 25, 12, 22, 11, 90];
    let comparison = compare_all_heapsorts(&input);

    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
    assert_eq!(comparison.input, input);
}

#[test]
fn test_textbook_example() {
    let comparison = textbook_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
    assert_eq!(comparison.input, vec![64, 34, 25, 12, 22, 11, 90]);
}

#[test]
fn test_reverse_sorted_example() {
    let comparison = reverse_sorted_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
}

#[test]
fn test_already_sorted_example() {
    let comparison = already_sorted_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
}

#[test]
fn test_duplicates_example() {
    let comparison = duplicates_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
}

#[test]
fn test_single_element_example() {
    let comparison = single_element_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
}

#[test]
fn test_empty_example() {
    let comparison = empty_example();
    assert!(comparison.all_results_match());
    assert!(comparison.all_results_sorted());
}

#[test]
fn test_large_example() {
    let result = large_example(100);
    assert_eq!(result.len(), 100);
}

#[test]
fn test_efficiency_demonstration() {
    let demos = efficiency_demonstration();
    assert!(demos.len() >= 3);
    for (name, input) in demos {
        assert!(!name.is_empty());
        assert!(!input.is_empty());
    }
}

#[test]
fn test_complexity_analysis() {
    let analysis = complexity_analysis();
    assert_eq!(analysis.len(), 5);
    for (name, complexity, reason) in analysis {
        assert!(!name.is_empty());
        assert!(!complexity.is_empty());
        assert!(!reason.is_empty());
    }
}

#[test]
fn test_correctness_verification() {
    assert!(correctness_verification());
}

#[test]
fn test_is_sorted_utility() {
    assert!(is_sorted(&[1, 2, 3, 4, 5]));
    assert!(!is_sorted(&[5, 4, 3, 2, 1]));
    assert!(is_sorted(&[1, 1, 2, 2, 3]));
    assert!(is_sorted(&[42]));
    let empty: Vec<i32> = vec![];
    assert!(is_sorted(&empty));
}

#[test]
fn test_vec_to_array_seq() {
    let vec = vec![1, 2, 3];
    let seq = vec_to_array_seq(&vec);
    // ArraySeqStPer traits already imported at module level
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_vec_to_avl_seq() {
    let vec = vec![1, 2, 3];
    let seq = vec_to_avl_seq(&vec);
    use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    assert_eq!(seq.length(), 3);
}

#[test]
fn test_generate_test_sequences() {
    let sequences = generate_test_sequences(10);
    assert!(sequences.len() >= 5);
    for (name, seq) in sequences {
        assert!(!name.is_empty());
        assert_eq!(seq.len(), 10);
    }
}
