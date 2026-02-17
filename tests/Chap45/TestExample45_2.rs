//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Example 45.2.

use apas_verus::Chap45::Example45_2::Example45_2::*;

#[test]
fn test_textbook_example() {
    let result = example_45_2_textbook_example();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_reverse_sorted() {
    let result = example_45_2_reverse_sorted();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_already_sorted() {
    let result = example_45_2_already_sorted();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_duplicates() {
    let result = example_45_2_duplicates();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_single_element() {
    let result = example_45_2_single_element();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_empty() {
    let result = example_45_2_empty();
    assert!(result.all_results_match());
    assert!(result.all_results_sorted());
}

#[test]
fn test_efficiency_demonstration() {
    let result = example_45_2_efficiency_demonstration();
    assert!(!result.is_empty());
}

#[test]
fn test_run_example_45_2() {
    let output = run_example_45_2();
    assert!(!output.is_empty());
    assert!(output.contains("Example 45.2"));
    assert!(output.contains("Heapsort"));
}
