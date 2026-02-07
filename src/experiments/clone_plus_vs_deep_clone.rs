// Experiment: Can DeepViewClone replace clone_plus for Vec-backed types?
//
// HYPOTHESIS: DeepViewClone gives res.deep_view() == self.deep_view(),
// which is Seq<T::V> — the same mapped type our ArraySeq View uses.
// If we define View in terms of deep_view, clone inside verus! should work.
//
// Also test: can vstd's laws_eq replace our feq equality hierarchy?
//
// RESULT: Tests 1-3, 5 PASS. Test 4 FAILS — obeys_view_eq is opaque,
// generic equality requires reveal(obeys_view_eq) in the function body.
// vstd laws_eq works for concrete types; generic needs reveal or a wrapper.
//
// RESULT: DeepViewClone CANNOT replace clone_plus in generic code.
// deep_clone ensures res.deep_view() == self.deep_view(), but existing
// specs use res@ (i.e., res.view()). For generic V, the SMT solver cannot
// connect deep_view() to view() — they are two unrelated spec functions.
// The old clone_plus ensures cloned(*self, res) which implies res == self,
// giving res@ == self@ trivially. deep_clone only works when View is
// defined in terms of deep_view (as in TestSeq above), which would require
// rewriting all existing View impls — not feasible.

pub mod clone_plus_vs_deep_clone {

    use vstd::prelude::*;
    use vstd::view::DeepView;
    use vstd::contrib::exec_spec::DeepViewClone;

    verus! {
    //!	2. broadcast use
    //!	3. type definitions
    //!	4. view impls
    //!	9. exec fns
    //!	10. derive impls

    //!		2. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;


    //!		3. type definitions

    // A Vec-backed sequence with mapped View, like ArraySeqS
    #[verifier::ext_equal]
    #[verifier::reject_recursive_types(T)]
    pub struct TestSeq<T> {
        pub seq: Vec<T>,
    }


    //!		4. view impls

    // View defined via deep_view — same result type (Seq<T::V>) as ArraySeqS
    impl<T: DeepView> View for TestSeq<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq.deep_view()
        }
    }


    //!		9. exec fns

    // Test 2: Clone a concrete TestSeq<u64>
    fn test_clone_u64() {
        let v1: TestSeq<u64> = TestSeq { seq: vec![1u64, 2u64, 3u64] };
        let v2 = v1.clone();
        assert(v2@ == v1@);
    }

    // Test 3: Can we use vstd's obeys_view_eq instead of feq?
    // The feq() exec function does: x == y with ensures eq == (x@ == y@)
    // vstd gives: obeys_view_eq::<T>() ==> (x.eq_spec(&y) <==> x@ == y@)
    // Combined with obeys_eq_spec::<T>() ==> (x == y <==> x.eq_spec(&y)):
    // We get: x == y <==> x@ == y@
    fn test_vstd_equality(x: &u64, y: &u64) -> (eq: bool)
        ensures eq == (x@ == y@)
    {
        broadcast use vstd::laws_eq::group_laws_eq;
        *x == *y
    }

    // Test 4: FAILS — obeys_view_eq is opaque, needs reveal(obeys_view_eq)
    // fn test_vstd_equality_generic<T: PartialEq + View + Copy>(x: &T, y: &T) -> (eq: bool)
    //     requires vstd::laws_eq::obeys_view_eq::<T>()
    //     ensures eq == (x@ == y@)
    // {
    //     broadcast use vstd::laws_eq::group_laws_eq;
    //     *x == *y
    // }

    // Test 5: Element clone in a loop — replacing clone_plus usage
    // This is the pattern used in SetStEph, graph code, etc.
    fn test_element_deep_clone() {
        let v: Vec<u64> = vec![10u64, 20u64, 30u64];
        let elem = v[1].deep_clone();
        // deep_clone ensures: elem.deep_view() == v[1].deep_view()
        // For u64: deep_view is just the value, so elem@ == v[1]@
        assert(elem@ == v@[1]@);
    }


    //!		10. derive impls

    // Test 1: Clone impl using deep_clone — no external_body
    impl<T: DeepViewClone> Clone for TestSeq<T> {
        fn clone(&self) -> (res: Self)
            ensures res@ == self@
        {
            TestSeq { seq: self.seq.deep_clone() }
        }
    }

} // verus!
}
