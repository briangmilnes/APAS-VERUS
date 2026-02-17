//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Example 41.3: Demonstrating set operations from Example 41.1

pub mod Example41_3 {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::*;
    pub type T = N;

    pub trait Example41_3Trait {
        /// Example 41.1 cases using ArraySetStEph
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn example_41_1_array_set();

        /// Example 41.1 cases using AVLTreeSetStEph
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn example_41_1_avl_set();

        /// Demonstrate set operations with different implementations
        /// APAS: Work Θ(n log n), Span Θ(log n)
        fn demonstrate_set_operations();
    }

    pub fn example_41_1_array_set() {
        // Example 41.1 cases using ArraySetStEph

        // |{a, b, c}| = 3
        let set_abc = ArraySetStEphLit!['a', 'b', 'c'];
        assert_eq!(set_abc.size(), 3);

        // {x ∈ {4, 11, 2, 6} | x < 7} = {4, 2, 6}
        let set_nums = ArraySetStEphLit![4, 11, 2, 6];
        let filtered = set_nums.filter(|&x| x < 7);
        assert_eq!(filtered.size(), 3);
        assert!(filtered.find(&4));
        assert!(filtered.find(&2));
        assert!(filtered.find(&6));
        assert!(!filtered.find(&11));

        // find {6, 2, 9, 11, 8} 4 = false
        let set_search = ArraySetStEphLit![6, 2, 9, 11, 8];
        assert!(!set_search.find(&4));
        assert!(set_search.find(&6));

        // {2, 7, 8, 11} ∪ {7, 9, 11, 14, 17} = {2, 7, 8, 9, 11, 14, 17}
        let set1 = ArraySetStEphLit![2, 7, 8, 11];
        let set2 = ArraySetStEphLit![7, 9, 11, 14, 17];
        let union_result = set1.union(&set2);
        assert_eq!(union_result.size(), 7);
        assert!(union_result.find(&2));
        assert!(union_result.find(&7));
        assert!(union_result.find(&8));
        assert!(union_result.find(&9));
        assert!(union_result.find(&11));
        assert!(union_result.find(&14));
        assert!(union_result.find(&17));

        // toSeq {2, 7, 8, 11} = ⟨8, 11, 2, 7⟩ (order may vary)
        let set_to_seq = ArraySetStEphLit![2, 7, 8, 11];
        let seq_result = set_to_seq.to_seq();
        assert_eq!(seq_result.length(), 4);

        // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
        // fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {8, 2, 11, 7}
        let seq_with_dups = ArraySeqStEphSLit![2, 7, 2, 8, 11, 2];
        let set_from_seq = ArraySetStEph::from_seq(seq_with_dups);
        assert_eq!(set_from_seq.size(), 4);
        assert!(set_from_seq.find(&2));
        assert!(set_from_seq.find(&7));
        assert!(set_from_seq.find(&8));
        assert!(set_from_seq.find(&11));
    }

    pub fn example_41_1_avl_set() {
        // Example 41.1 cases using AVLTreeSetStEph

        // |{a, b, c}| = 3
        let set_abc = AVLTreeSetStEphLit!['a', 'b', 'c'];
        assert_eq!(set_abc.size(), 3);

        // {x ∈ {4, 11, 2, 6} | x < 7} = {4, 2, 6}
        let set_nums = AVLTreeSetStEphLit![4, 11, 2, 6];
        let filtered = set_nums.filter(|&x| x < 7);
        assert_eq!(filtered.size(), 3);
        assert!(filtered.find(&4));
        assert!(filtered.find(&2));
        assert!(filtered.find(&6));
        assert!(!filtered.find(&11));

        // find {6, 2, 9, 11, 8} 4 = false
        let set_search = AVLTreeSetStEphLit![6, 2, 9, 11, 8];
        assert!(!set_search.find(&4));
        assert!(set_search.find(&6));

        // {2, 7, 8, 11} ∪ {7, 9, 11, 14, 17} = {2, 7, 8, 9, 11, 14, 17}
        let set1 = AVLTreeSetStEphLit![2, 7, 8, 11];
        let set2 = AVLTreeSetStEphLit![7, 9, 11, 14, 17];
        let union_result = set1.union(&set2);
        assert_eq!(union_result.size(), 7);
        assert!(union_result.find(&2));
        assert!(union_result.find(&7));
        assert!(union_result.find(&8));
        assert!(union_result.find(&9));
        assert!(union_result.find(&11));
        assert!(union_result.find(&14));
        assert!(union_result.find(&17));

        // toSeq {2, 7, 8, 11} = ⟨8, 11, 2, 7⟩ (order may vary)
        let set_to_seq = AVLTreeSetStEphLit![2, 7, 8, 11];
        let seq_result = set_to_seq.to_seq();
        assert_eq!(seq_result.length(), 4);

        // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
        // fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {8, 2, 11, 7}
        let seq_with_dups = AVLTreeSeqStEphLit![2, 7, 2, 8, 11, 2];
        let set_from_seq = AVLTreeSetStEph::from_seq(seq_with_dups);
        assert_eq!(set_from_seq.size(), 4);
        assert!(set_from_seq.find(&2));
        assert!(set_from_seq.find(&7));
        assert!(set_from_seq.find(&8));
        assert!(set_from_seq.find(&11));
    }

    pub fn example_41_3_from_seq_demonstration() {
        // Example 41.3: Two implementations of fromSeq

        // Sequential version: fromseq a = Seq.iterate Set.insert ∅ a
        // (This would be implemented as a simple loop with insert)

        // Parallel version: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
        // This is what our implementation uses

        let seq = ArraySeqStEphSLit![1, 3, 2, 3, 1, 4, 2];
        let set_result = ArraySetStEph::from_seq(seq);

        // Should contain unique elements: {1, 2, 3, 4}
        assert_eq!(set_result.size(), 4);
        assert!(set_result.find(&1));
        assert!(set_result.find(&2));
        assert!(set_result.find(&3));
        assert!(set_result.find(&4));

        // Demonstrate the conceptual steps:
        // 1. Create singleton sets: ⟨{1}, {3}, {2}, {3}, {1}, {4}, {2}⟩
        // 2. Reduce with union: {1} ∪ {3} ∪ {2} ∪ {3} ∪ {1} ∪ {4} ∪ {2} = {1, 2, 3, 4}

        let singleton1 = ArraySetStEph::singleton(1);
        let singleton3 = ArraySetStEph::singleton(3);
        let singleton2 = ArraySetStEph::singleton(2);
        let singleton4 = ArraySetStEph::singleton(4);

        let manual_union = singleton1
            .union(&singleton3)
            .union(&singleton2)
            .union(&singleton3) // duplicate, should not change result
            .union(&singleton1) // duplicate, should not change result
            .union(&singleton4)
            .union(&singleton2); // duplicate, should not change result

        assert_eq!(manual_union.size(), 4);
        assert!(manual_union.find(&1));
        assert!(manual_union.find(&2));
        assert!(manual_union.find(&3));
        assert!(manual_union.find(&4));

        // Both approaches should yield the same result
        assert_eq!(set_result.size(), manual_union.size());
    }

    pub fn additional_set_operations() {
        // Additional set operations testing
        let set1 = ArraySetStEphLit![1, 2, 3, 4, 5];
        let set2 = ArraySetStEphLit![4, 5, 6, 7, 8];

        // Intersection: {1, 2, 3, 4, 5} ∩ {4, 5, 6, 7, 8} = {4, 5}
        let intersection = set1.intersection(&set2);
        assert_eq!(intersection.size(), 2);
        assert!(intersection.find(&4));
        assert!(intersection.find(&5));
        assert!(!intersection.find(&1));
        assert!(!intersection.find(&6));

        // Difference: {1, 2, 3, 4, 5} \ {4, 5, 6, 7, 8} = {1, 2, 3}
        let difference = set1.difference(&set2);
        assert_eq!(difference.size(), 3);
        assert!(difference.find(&1));
        assert!(difference.find(&2));
        assert!(difference.find(&3));
        assert!(!difference.find(&4));
        assert!(!difference.find(&5));

        // Delete operation
        let mut set_delete = ArraySetStEphLit![1, 2, 3, 4, 5];
        set_delete.delete(&3);
        assert_eq!(set_delete.size(), 4);
        assert!(!set_delete.find(&3));
        assert!(set_delete.find(&1));
        assert!(set_delete.find(&2));
        assert!(set_delete.find(&4));
        assert!(set_delete.find(&5));

        // Insert operation
        let mut set_insert = ArraySetStEphLit![1, 2, 4, 5];
        set_insert.insert(3);
        assert_eq!(set_insert.size(), 5);
        assert!(set_insert.find(&3));

        // Insert duplicate (should not change size)
        set_insert.insert(3);
        assert_eq!(set_insert.size(), 5);
    }
}
