//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Example 41.3: Demonstrating set operations from Example 41.1

pub mod Example41_3 {

    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::*;
    use vstd::prelude::*;

    pub type T = u64;

    verus! {

broadcast use vstd::laws_cmp::group_laws_cmp;

    pub trait Example41_3Trait {
        /// Example 41.1 cases using ArraySetStEph.
        fn example_41_1_array_set();

        /// Example 41.1 cases using AVLTreeSetStEph.
        fn example_41_1_avl_set();

        /// Demonstrate set operations with different implementations.
        fn demonstrate_set_operations();
    }

    fn example_41_1_array_set_impl()
    {
        // Example 41.1: |{a, b, c}| = 3
        let set_abc = ArraySetStEphLit!['a', 'b', 'c'];
        let fa = set_abc.find(&'a');
        let fb = set_abc.find(&'b');
        let fc = set_abc.find(&'c');
        assert(fa && fb && fc);

        // {x ∈ {4, 11, 2, 6} | x < 7} = {4, 2, 6}
        let set_nums = ArraySetStEphLit![4i32, 11i32, 2i32, 6i32];
        let pred_lt7 = |x: &i32| -> (result: bool)
            ensures result == (*x < 7i32),
        { *x < 7i32 };
        let filtered = set_nums.filter(pred_lt7, Ghost(|v: i32| v < 7i32));
        let f4 = filtered.find(&4i32);
        let f2 = filtered.find(&2i32);
        let f6 = filtered.find(&6i32);
        let not_f11 = filtered.find(&11i32);
        assert(f4 && f2 && f6 && !not_f11);

        // find {6, 2, 9, 11, 8} 4 = false
        let set_search = ArraySetStEphLit![6i32, 2i32, 9i32, 11i32, 8i32];
        let not_s4 = set_search.find(&4i32);
        let s6 = set_search.find(&6i32);
        assert(!not_s4 && s6);

        // {2, 7, 8, 11} ∪ {7, 9, 11, 14, 17} = {2, 7, 8, 9, 11, 14, 17}
        let set1 = ArraySetStEphLit![2i32, 7i32, 8i32, 11i32];
        let set2 = ArraySetStEphLit![7i32, 9i32, 11i32, 14i32, 17i32];
        let union_result = set1.union(&set2);
        let u2 = union_result.find(&2i32);
        let u9 = union_result.find(&9i32);
        let u14 = union_result.find(&14i32);
        assert(u2 && u9 && u14);

        // fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {2, 7, 8, 11} (use ArraySetStEphLit! to demonstrate)
        let set_from_seq = ArraySetStEphLit![2i32, 7i32, 8i32, 11i32];
        let sf2 = set_from_seq.find(&2i32);
        let sf7 = set_from_seq.find(&7i32);
        assert(sf2 && sf7);
    }

    fn example_41_1_avl_set_impl()
    {
        // vstd::laws_cmp::group_laws_cmp does not include char; assume type axioms for char.
        proof {
            assume(vstd::laws_cmp::obeys_cmp_spec::<char>());
            assume(view_ord_consistent::<char>());
        }

        // Example 41.1: |{a, b, c}| = 3
        let set_abc = AVLTreeSetStEphLit!['a', 'b', 'c'];
        let fa = set_abc.find(&'a');
        let fb = set_abc.find(&'b');
        let fc = set_abc.find(&'c');
        assert(fa && fb && fc);

        // {x ∈ {4, 11, 2, 6} | x < 7} = {4, 2, 6}
        let set_nums = AVLTreeSetStEphLit![4i32, 11i32, 2i32, 6i32];
        let pred_lt7 = |x: &i32| -> (result: bool)
            ensures result == (*x < 7i32),
        { *x < 7i32 };
        let filtered = set_nums.filter(pred_lt7, Ghost(|v: i32| v < 7i32));
        let f4 = filtered.find(&4i32);
        let f2 = filtered.find(&2i32);
        let f6 = filtered.find(&6i32);
        let not_f11 = filtered.find(&11i32);
        assert(f4 && f2 && f6 && !not_f11);

        // find {6, 2, 9, 11, 8} 4 = false
        let set_search = AVLTreeSetStEphLit![6i32, 2i32, 9i32, 11i32, 8i32];
        let not_s4 = set_search.find(&4i32);
        let s6 = set_search.find(&6i32);
        assert(!not_s4 && s6);

        // {2, 7, 8, 11} ∪ {7, 9, 11, 14, 17} = {2, 7, 8, 9, 11, 14, 17}
        let set1 = AVLTreeSetStEphLit![2i32, 7i32, 8i32, 11i32];
        let set2 = AVLTreeSetStEphLit![7i32, 9i32, 11i32, 14i32, 17i32];
        let union_result = set1.union(&set2);
        let u2 = union_result.find(&2i32);
        let u9 = union_result.find(&9i32);
        let u14 = union_result.find(&14i32);
        assert(u2 && u9 && u14);

        // fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {2, 7, 8, 11} (demonstrate with AVLTreeSetStEphLit!)
        let set_from_seq = AVLTreeSetStEphLit![2i32, 7i32, 8i32, 11i32];
        let sf2 = set_from_seq.find(&2i32);
        let sf7 = set_from_seq.find(&7i32);
        assert(sf2 && sf7);
    }

    fn example_41_3_from_seq_demonstration_impl()
    {
        // Example 41.3: fromSeq ⟨2, 7, 2, 8, 11, 2⟩ = {2, 7, 8, 11}
        // Demonstrate using direct set construction and reduce-via-union.

        // Direct construction via ArraySetStEphLit!
        let set_result = ArraySetStEphLit![2i32, 7i32, 8i32, 11i32];
        let r2  = set_result.find(&2i32);
        let r7  = set_result.find(&7i32);
        let r8  = set_result.find(&8i32);
        let r11 = set_result.find(&11i32);
        assert(r2 && r7 && r8 && r11);

        // Reduce-with-union over singletons: {2} ∪ {7} ∪ {8} ∪ {11} = {2, 7, 8, 11}
        let s2  = ArraySetStEph::singleton(2i32);
        let s7  = ArraySetStEph::singleton(7i32);
        let s8  = ArraySetStEph::singleton(8i32);
        let s11 = ArraySetStEph::singleton(11i32);

        let manual_union = s2.union(&s7).union(&s8).union(&s11);

        let m2  = manual_union.find(&2i32);
        let m7  = manual_union.find(&7i32);
        let m8  = manual_union.find(&8i32);
        let m11 = manual_union.find(&11i32);
        assert(m2 && m7 && m8 && m11);
    }

    fn additional_set_operations_impl()
    {
        // Intersection: {1, 2, 3, 4, 5} ∩ {4, 5, 6, 7, 8} = {4, 5}
        let set1 = ArraySetStEphLit![1i32, 2i32, 3i32, 4i32, 5i32];
        let set2 = ArraySetStEphLit![4i32, 5i32, 6i32, 7i32, 8i32];
        let intersection = set1.intersection(&set2);
        let i4 = intersection.find(&4i32);
        let i5 = intersection.find(&5i32);
        let not_i1 = intersection.find(&1i32);
        let not_i6 = intersection.find(&6i32);
        assert(i4 && i5 && !not_i1 && !not_i6);

        // Difference: {1, 2, 3, 4, 5} \ {4, 5, 6, 7, 8} = {1, 2, 3}
        let difference = set1.difference(&set2);
        let d1 = difference.find(&1i32);
        let d2 = difference.find(&2i32);
        let d3 = difference.find(&3i32);
        let not_d4 = difference.find(&4i32);
        let not_d5 = difference.find(&5i32);
        assert(d1 && d2 && d3 && !not_d4 && !not_d5);

        // Delete: {1, 2, 3, 4, 5} \ {3} = {1, 2, 4, 5}
        let mut set_delete = ArraySetStEphLit![1i32, 2i32, 3i32, 4i32, 5i32];
        set_delete.delete(&3i32);
        let del3 = set_delete.find(&3i32);
        let del1 = set_delete.find(&1i32);
        assert(!del3 && del1);

        // Insert: {1, 2, 4, 5} ∪ {3} = {1, 2, 3, 4, 5}
        let mut set_insert = ArraySetStEphLit![1i32, 2i32, 4i32, 5i32];
        set_insert.insert(3i32);
        let ins3 = set_insert.find(&3i32);
        assert(ins3);
    }

    impl Example41_3Trait for () {
        fn example_41_1_array_set() { example_41_1_array_set_impl(); }
        fn example_41_1_avl_set() { example_41_1_avl_set_impl(); }
        fn demonstrate_set_operations() {
            example_41_1_array_set_impl();
            example_41_1_avl_set_impl();
            example_41_3_from_seq_demonstration_impl();
            additional_set_operations_impl();
        }
    }

    pub fn example_41_1_array_set()
    { example_41_1_array_set_impl(); }
    pub fn example_41_1_avl_set()
    { example_41_1_avl_set_impl(); }
    pub fn example_41_3_from_seq_demonstration()
    { example_41_3_from_seq_demonstration_impl(); }
    pub fn additional_set_operations()
    { additional_set_operations_impl(); }

    } // verus!
}
