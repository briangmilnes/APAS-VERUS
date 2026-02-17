//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 42 Example 42.1.

use apas_verus::Chap42::Example42_1::Example42_1::*;
use apas_verus::Types::Types::*;

#[test]
fn test_example_42_1_runs() {
    // Test that the example runs without panicking
    example_42_1();
}

#[test]
fn test_performance_comparison_runs() {
    // Test that the performance comparison runs without panicking
    performance_comparison();
}
