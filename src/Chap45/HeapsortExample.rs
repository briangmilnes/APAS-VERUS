//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Heapsort Example - Algorithm 45.2 using all Priority Queue implementations

pub mod HeapsortExample {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap45::BalancedTreePQ::BalancedTreePQ::BalancedTreePQ;
    use crate::Chap45::BalancedTreePQ::BalancedTreePQ::*;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::Chap45::LeftistHeapPQ::LeftistHeapPQ::*;
    use crate::Chap45::SortedListPQ::SortedListPQ::SortedListPQ;
    use crate::Chap45::SortedListPQ::SortedListPQ::*;
    use crate::Chap45::UnsortedListPQ::UnsortedListPQ::UnsortedListPQ;
    use crate::Chap45::UnsortedListPQ::UnsortedListPQ::*;
    use crate::Types::Types::*;

    pub trait HeapsortTrait {
        /// Heapsort using UnsortedListPQ
        /// Claude Work: Θ(n²), Span: Θ(n²) - inefficient due to O(n) deleteMin
        fn heapsort_unsorted_list<T: StT + Ord>(sequence: &[T]) -> Vec<T>;

        /// Heapsort using SortedListPQ  
        /// Claude Work: Θ(n²), Span: Θ(n²) - inefficient due to O(n) insert
        fn heapsort_sorted_list<T: StT + Ord>(sequence: &[T])   -> Vec<T>;

        /// Heapsort using BalancedTreePQ
        /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity
        fn heapsort_balanced_tree<T: StT + Ord>(sequence: &[T]) -> Vec<T>;

        /// Heapsort using BinaryHeapPQ
        /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity
        fn heapsort_binary_heap<T: StT + Ord>(sequence: &[T])   -> Vec<T>;

        /// Heapsort using LeftistHeapPQ
        /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity
        fn heapsort_leftist_heap<T: StT + Ord>(sequence: &[T])  -> Vec<T>;

        /// Compare all heapsort implementations on the same input
        /// Claude Work: Θ(n²), Span: Θ(n²) - dominated by worst implementation
        fn compare_all_heapsorts<T: StT + Ord>(sequence: &[T])  -> HeapsortComparison<T>;
    }

    /// Heapsort using UnsortedListPQ
    /// Claude Work: Θ(n²), Span: Θ(n²) - inefficient due to O(n) deleteMin
    pub fn heapsort_unsorted_list<T: StT + Ord>(sequence: &[T]) -> Vec<T> {
        let mut pq = UnsortedListPQ::empty();
        for element in sequence {
            pq = pq.insert(element.clone());
        }
        let mut result = Vec::new();
        while !pq.is_empty() {
            let (new_pq, min_element) = pq.delete_min();
            if let Some(element) = min_element {
                result.push(element);
            }
            pq = new_pq;
        }
        result
    }

    /// Heapsort using SortedListPQ  
    /// Claude Work: Θ(n²), Span: Θ(n²) - inefficient due to O(n) insert
    pub fn heapsort_sorted_list<T: StT + Ord>(sequence: &[T]) -> Vec<T> {
        let mut pq = SortedListPQ::empty();
        for element in sequence {
            pq = pq.insert(element.clone());
        }
        let mut result = Vec::new();
        while !pq.is_empty() {
            let (new_pq, min_element) = pq.delete_min();
            if let Some(element) = min_element {
                result.push(element);
            }
            pq = new_pq;
        }
        result
    }

    /// Heapsort using BalancedTreePQ
    /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity
    pub fn heapsort_balanced_tree<T: StT + Ord>(sequence: &[T]) -> Vec<T> {
        let mut pq = BalancedTreePQ::empty();
        for element in sequence {
            pq = pq.insert(element.clone());
        }
        let mut result = Vec::new();
        while !pq.is_empty() {
            let (new_pq, min_element) = pq.delete_min();
            if let Some(element) = min_element {
                result.push(element);
            }
            pq = new_pq;
        }
        result
    }

    /// Heapsort using BinaryHeapPQ
    /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity
    pub fn heapsort_binary_heap<T: StT + Ord>(sequence: &[T]) -> Vec<T> {
        let mut pq = BinaryHeapPQ::empty();
        for element in sequence {
            pq = pq.insert(element.clone());
        }
        let mut result = Vec::new();
        while !pq.is_empty() {
            let (new_pq, min_element) = pq.delete_min();
            if let Some(element) = min_element {
                result.push(element);
            }
            pq = new_pq;
        }
        result
    }

    /// Heapsort using LeftistHeapPQ
    /// Claude Work: Θ(n log n), Span: Θ(n log n) - optimal complexity with superior meld
    pub fn heapsort_leftist_heap<T: StT + Ord>(sequence: &[T]) -> Vec<T> {
        let mut pq = LeftistHeapPQ::empty();
        for element in sequence {
            pq = pq.insert(element.clone());
        }
        let mut result = Vec::new();
        while !pq.is_empty() {
            let (new_pq, min_element) = pq.delete_min();
            if let Some(element) = min_element {
                result.push(element);
            }
            pq = new_pq;
        }
        result
    }

    /// Demonstrate all heapsort variants on the same input
    pub fn compare_all_heapsorts<T: StT + Ord>(sequence: &[T]) -> HeapsortComparison<T> {
        HeapsortComparison {
            input: sequence.to_vec(),
            unsorted_list_result: heapsort_unsorted_list(sequence),
            sorted_list_result: heapsort_sorted_list(sequence),
            balanced_tree_result: heapsort_balanced_tree(sequence),
            binary_heap_result: heapsort_binary_heap(sequence),
            leftist_heap_result: heapsort_leftist_heap(sequence),
        }
    }

    /// Results of comparing all heapsort implementations
    #[derive(Debug, Clone, PartialEq)]
    pub struct HeapsortComparison<T: StT + Ord> {
        pub input: Vec<T>,
        pub unsorted_list_result: Vec<T>,
        pub sorted_list_result: Vec<T>,
        pub balanced_tree_result: Vec<T>,
        pub binary_heap_result: Vec<T>,
        pub leftist_heap_result: Vec<T>,
    }

    pub trait HeapsortExamplesTrait {
        /// Example from textbook - demonstrate heapsort on a small dataset
        fn textbook_example()         -> HeapsortComparison<i32>;
        /// Demonstrate heapsort on reverse-sorted input (worst case for some algorithms)
        fn reverse_sorted_example()   -> HeapsortComparison<i32>;
        /// Demonstrate heapsort on already-sorted input
        fn already_sorted_example()   -> HeapsortComparison<i32>;
        /// Demonstrate heapsort on input with duplicates
        fn duplicates_example()       -> HeapsortComparison<i32>;
        /// Demonstrate heapsort on single element
        fn single_element_example()   -> HeapsortComparison<i32>;
        /// Demonstrate heapsort on empty input
        fn empty_example()            -> HeapsortComparison<i32>;
        /// Generate large example for performance testing
        fn large_example(size: usize) -> Vec<i32>;
        /// Demonstrate the efficiency difference between implementations
        fn efficiency_demonstration() -> Vec<(String, Vec<i32>)>;
    }

    pub trait HeapsortAnalysisTrait {
        /// Analyze the theoretical complexity of each heapsort variant
        fn complexity_analysis()      -> Vec<(String, String, String)>;
        /// Verify that all heapsort implementations produce correct results
        fn correctness_verification() -> bool;
    }

    pub trait SequenceUtilsTrait {
        /// Convert Vec to ArraySeqStPerS for use with APAS sequence types
        fn vec_to_array_seq<T: StT>(vec: &[T])  -> ArraySeqStPerS<T>;
        /// Convert Vec to AVLTreeSeqStPerS for use with balanced tree operations
        fn vec_to_avl_seq<T: StT>(vec: &[T])    -> AVLTreeSeqStPerS<T>;
        /// Check if a sequence is sorted
        fn is_sorted<T: Ord>(vec: &[T])         -> bool;
        /// Generate test sequences of various patterns
        fn generate_test_sequences(size: usize) -> Vec<(String, Vec<i32>)>;
    }

    impl<T: StT + Ord> HeapsortComparison<T> {
        /// Verify that all implementations produce the same sorted result
        pub fn all_results_match(&self) -> bool {
            let expected = &self.binary_heap_result; // Use binary heap as reference
            self.unsorted_list_result == *expected
                && self.sorted_list_result == *expected
                && self.balanced_tree_result == *expected
                && self.leftist_heap_result == *expected
        }

        /// Check if all results are properly sorted
        pub fn all_results_sorted(&self) -> bool {
            fn is_sorted<T: Ord>(vec: &[T]) -> bool { vec.windows(2).all(|w| w[0] <= w[1]) }

            is_sorted(&self.unsorted_list_result)
                && is_sorted(&self.sorted_list_result)
                && is_sorted(&self.balanced_tree_result)
                && is_sorted(&self.binary_heap_result)
                && is_sorted(&self.leftist_heap_result)
        }
    }

    /// Example from textbook - demonstrate heapsort on a small dataset
    pub fn textbook_example() -> HeapsortComparison<i32> {
        let input = vec![64, 34, 25, 12, 22, 11, 90];
        compare_all_heapsorts(&input)
    }

    /// Demonstrate heapsort on reverse-sorted input (worst case for some algorithms)
    pub fn reverse_sorted_example() -> HeapsortComparison<i32> {
        let input = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        compare_all_heapsorts(&input)
    }

    /// Demonstrate heapsort on already-sorted input
    pub fn already_sorted_example() -> HeapsortComparison<i32> {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        compare_all_heapsorts(&input)
    }

    /// Demonstrate heapsort on input with duplicates
    pub fn duplicates_example() -> HeapsortComparison<i32> {
        let input = vec![5, 2, 8, 2, 9, 1, 5, 5];
        compare_all_heapsorts(&input)
    }

    /// Demonstrate heapsort on single element
    pub fn single_element_example() -> HeapsortComparison<i32> {
        let input = vec![42];
        compare_all_heapsorts(&input)
    }

    /// Demonstrate heapsort on empty input
    pub fn empty_example() -> HeapsortComparison<i32> {
        let input = vec![];
        compare_all_heapsorts(&input)
    }

    /// Generate a large random-like sequence for performance testing
    pub fn large_example(size: usize) -> Vec<i32> {
        // Generate pseudo-random sequence for testing
        let mut result = Vec::with_capacity(size);
        let mut x: i64 = 1;
        for _ in 0..size {
            x = (x * 1103515245 + 12345) % (1i64 << 31); // Linear congruential generator
            result.push((x % 1000) as i32);
        }
        result
    }

    /// Demonstrate the efficiency difference between implementations
    pub fn efficiency_demonstration() -> Vec<(String, Vec<i32>)> {
        let inputs = vec![
            ("Small (7 elements)".to_string(), vec![64, 34, 25, 12, 22, 11, 90]),
            ("Medium (20 elements)".to_string(), large_example(20)),
            ("Large (100 elements)".to_string(), large_example(100)),
        ];

        inputs
    }

    /// Analyze the theoretical complexity of each heapsort variant
    pub fn complexity_analysis() -> Vec<(String, String, String)> {
        vec![
            (
                "UnsortedListPQ".to_string(),
                "Θ(n²)".to_string(),
                "O(n) deleteMin dominates".to_string(),
            ),
            (
                "SortedListPQ".to_string(),
                "Θ(n²)".to_string(),
                "O(n) insert dominates".to_string(),
            ),
            (
                "BalancedTreePQ".to_string(),
                "Θ(n log n)".to_string(),
                "O(log n) insert and deleteMin".to_string(),
            ),
            (
                "BinaryHeapPQ".to_string(),
                "Θ(n log n)".to_string(),
                "O(log n) insert and deleteMin".to_string(),
            ),
            (
                "LeftistHeapPQ".to_string(),
                "Θ(n log n)".to_string(),
                "O(log n) insert and deleteMin + superior meld".to_string(),
            ),
        ]
    }

    /// Verify that all heapsort implementations produce correct results
    pub fn correctness_verification() -> bool {
        let test_cases = vec![
            textbook_example(),
            reverse_sorted_example(),
            already_sorted_example(),
            duplicates_example(),
            single_element_example(),
            empty_example(),
        ];

        test_cases
            .iter()
            .all(|comparison| comparison.all_results_match() && comparison.all_results_sorted())
    }

    /// Convert Vec to ArraySeqStPerS for use with APAS sequence types
    pub fn vec_to_array_seq<T: StT>(vec: &[T]) -> ArraySeqStPerS<T> {
        let mut result = ArraySeqStPerS::empty();
        for element in vec {
            let single_seq = ArraySeqStPerS::singleton(element.clone());
            result = ArraySeqStPerS::append(&result, &single_seq);
        }
        result
    }

    /// Convert Vec to AVLTreeSeqStPerS for use with balanced tree operations
    pub fn vec_to_avl_seq<T: StT>(vec: &[T]) -> AVLTreeSeqStPerS<T> { AVLTreeSeqStPerS::from_vec(vec.to_vec()) }

    /// Check if a sequence is sorted
    pub fn is_sorted<T: Ord>(vec: &[T]) -> bool { vec.windows(2).all(|w| w[0] <= w[1]) }

    /// Generate test sequences of various patterns
    pub fn generate_test_sequences(size: usize) -> Vec<(String, Vec<i32>)> {
        vec![
            ("Random".to_string(), large_example(size)),
            ("Sorted".to_string(), (1..=size as i32).collect()),
            ("Reverse".to_string(), (1..=size as i32).rev().collect()),
            ("AllSame".to_string(), vec![42; size]),
            ("AlmostSorted".to_string(), {
                let mut seq: Vec<i32> = (1..=size as i32).collect();
                if size > 2 {
                    seq.swap(0, size - 1); // Swap first and last
                }
                seq
            }),
        ]
    }
}
