// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 45: Example 45.2 - Heapsort Algorithm Demonstrations

//  Table of Contents
//  1. module
//  2. imports
//  4. type definitions
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  13. derive impls outside verus!

pub mod Example45_2 {

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap45::HeapsortExample::HeapsortExample::*;

    verus! {

// 4. type definitions
        pub struct Example45_2;

// 7. proof fns
        proof fn _example_45_2_verified() {}

// 8. traits
        /// Heapsort demonstrations comparing all five PQ implementations on various inputs.
        pub trait Example45_2Trait {
            fn example_45_2_textbook_example()         -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_reverse_sorted()           -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_already_sorted()           -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_duplicates()               -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_single_element()           -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_empty()                    -> (comparison: HeapsortComparison<i32>);
            fn example_45_2_efficiency_demonstration() -> (results: Vec<(String, Vec<i32>)>);
            fn run_example_45_2()                      -> (output: String);
        }

// 9. impls
        #[verifier::external]
        impl Example45_2Trait for Example45_2 {
            fn example_45_2_textbook_example()         -> HeapsortComparison<i32> { textbook_example() }
            fn example_45_2_reverse_sorted()           -> HeapsortComparison<i32> { reverse_sorted_example() }
            fn example_45_2_already_sorted()           -> HeapsortComparison<i32> { already_sorted_example() }
            fn example_45_2_duplicates()               -> HeapsortComparison<i32> { duplicates_example() }
            fn example_45_2_single_element()           -> HeapsortComparison<i32> { single_element_example() }
            fn example_45_2_empty()                    -> HeapsortComparison<i32> { empty_example() }
            fn example_45_2_efficiency_demonstration() -> Vec<(String, Vec<i32>)> { efficiency_demonstration() }
            fn run_example_45_2()                      -> String { run_example_45_2() }
        }

    }

// 13. derive impls outside verus!
    impl Debug for Example45_2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Example45_2")
        }
    }

    impl Display for Example45_2 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Example45_2")
        }
    }

    /// Example 45.2: Textbook heapsort demonstration.
    pub fn example_45_2_textbook_example() -> HeapsortComparison<i32> { textbook_example() }

    /// Example 45.2: Reverse-sorted input demonstration.
    pub fn example_45_2_reverse_sorted() -> HeapsortComparison<i32> { reverse_sorted_example() }

    /// Example 45.2: Already-sorted input demonstration.
    pub fn example_45_2_already_sorted() -> HeapsortComparison<i32> { already_sorted_example() }

    /// Example 45.2: Input with duplicates demonstration.
    pub fn example_45_2_duplicates() -> HeapsortComparison<i32> { duplicates_example() }

    /// Example 45.2: Single element demonstration.
    pub fn example_45_2_single_element() -> HeapsortComparison<i32> { single_element_example() }

    /// Example 45.2: Empty input demonstration.
    pub fn example_45_2_empty() -> HeapsortComparison<i32> { empty_example() }

    /// Example 45.2: Efficiency comparison demonstration.
    pub fn example_45_2_efficiency_demonstration() -> Vec<(String, Vec<i32>)> { efficiency_demonstration() }

    /// Run comprehensive demonstration of Example 45.2.
    pub fn run_example_45_2() -> String {
        let mut output = String::new();

        output.push_str("=== Example 45.2: Heapsort Algorithm Demonstrations ===\n\n");

        // Textbook Example
        output.push_str("PART 1: Textbook Example Dataset\n");
        let textbook = example_45_2_textbook_example();
        output.push_str(&format!("Input: {:?}\n", textbook.input));
        output.push_str(&format!("All results match: {}\n", textbook.all_results_match()));
        output.push_str(&format!("All results sorted: {}\n", textbook.all_results_sorted()));
        output.push_str(&format!("Binary heap result: {:?}\n\n", textbook.binary_heap_result));

        // Reverse-Sorted Example
        output.push_str("PART 2: Reverse-Sorted Input\n");
        let reverse = example_45_2_reverse_sorted();
        output.push_str(&format!("Input: {:?}\n", reverse.input));
        output.push_str(&format!("All results match: {}\n", reverse.all_results_match()));
        output.push_str(&format!("Binary heap result: {:?}\n\n", reverse.binary_heap_result));

        // Already-Sorted Example
        output.push_str("PART 3: Already-Sorted Input\n");
        let sorted = example_45_2_already_sorted();
        output.push_str(&format!("Input: {:?}\n", sorted.input));
        output.push_str(&format!("All results match: {}\n", sorted.all_results_match()));
        output.push_str(&format!("Binary heap result: {:?}\n\n", sorted.binary_heap_result));

        // Duplicates Example
        output.push_str("PART 4: Input with Duplicates\n");
        let duplicates = example_45_2_duplicates();
        output.push_str(&format!("Input: {:?}\n", duplicates.input));
        output.push_str(&format!("All results match: {}\n", duplicates.all_results_match()));
        output.push_str(&format!("Binary heap result: {:?}\n\n", duplicates.binary_heap_result));

        // Edge Cases
        output.push_str("PART 5: Edge Cases\n");
        let single = example_45_2_single_element();
        let empty = example_45_2_empty();
        output.push_str(&format!("Single element - All match: {}\n", single.all_results_match()));
        output.push_str(&format!("Empty input - All match: {}\n\n", empty.all_results_match()));

        // Efficiency Analysis
        output.push_str("PART 6: Efficiency Comparison\n");
        let efficiency = example_45_2_efficiency_demonstration();
        for (name, data) in efficiency {
            output.push_str(&format!("{}: {} elements\n", name, data.len()));
        }
        output.push('\n');

        // Complexity Analysis
        output.push_str("PART 7: Complexity Analysis\n");
        let complexity = complexity_analysis();
        for (impl_name, complexity_str, notes) in complexity {
            output.push_str(&format!("{impl_name}: {complexity_str} - {notes}\n"));
        }
        output.push('\n');

        // Correctness Verification
        output.push_str("PART 8: Correctness Verification\n");
        let all_correct = correctness_verification();
        output.push_str(&format!("All implementations correct: {all_correct}\n"));

        output.push_str("\nSummary:\n");
        output.push_str("- Binary heap and leftist heap provide optimal Θ(n log n) performance\n");
        output.push_str("- Balanced tree also achieves Θ(n log n) with good constants\n");
        output.push_str("- Unsorted and sorted list implementations are Θ(n²) and impractical\n");
        output.push_str("- All implementations produce identical, correctly sorted results\n");
        output.push_str("- Heapsort is stable across different input patterns\n");

        output
    }
}
