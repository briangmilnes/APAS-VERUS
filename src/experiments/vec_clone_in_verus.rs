// Experiment: Can Vec clone work inside verus! with ensures on view?
//
// RESULT: PARTIALLY — Vec::clone alone cannot prove view equality for generic T.
// DeepViewClone solves it. See clone_plus_vs_deep_clone.rs for the full solution.
//
// FINDINGS:
// - Vec::clone gives cloned(old[i], new[i]) per element but generic Clone has no spec
// - cloned(a, b) does NOT imply a == b for generic T (Clone trait has no postcondition)
// - Therefore self.seq@ =~= res.seq@ cannot be proven from Vec::clone alone
// - ext_equal on the wrapper does not help (problem is at Seq<T> level)
// - DeepViewClone bypasses the issue entirely with deep_view equality

use vstd::prelude::*;
use vstd::view::DeepView;
use vstd::contrib::exec_spec::DeepViewClone;

verus! {

broadcast use vstd::std_specs::vec::group_vec_axioms;

// ATTEMPT 1: Vec::clone with all four postconditions — PASSES
// but cannot prove self.seq@ =~= res.seq@ (commented out, fails)
//
// #[verifier::ext_equal]
// #[verifier::reject_recursive_types(T)]
// pub struct SimpleWrapper<T> {
//     pub seq: Vec<T>,
// }
//
// impl<T> View for SimpleWrapper<T> {
//     type V = Seq<T>;
//     open spec fn view(&self) -> Seq<T> { self.seq@ }
// }
//
// impl<T: Clone> Clone for SimpleWrapper<T> {
//     fn clone(&self) -> (res: Self)
//         ensures
//             res.seq@.len() == self.seq@.len(),
//             forall|i| #![all_triggers] 0 <= i < self.seq@.len()
//                 ==> cloned::<T>(self.seq@[i], res.seq@[i]),
//             vstd::std_specs::vec::vec_clone_trigger(self.seq, res.seq),
//             self.seq@ =~= res.seq@ ==> self.seq@ == res.seq@,
//             // FAILS: self.seq@ =~= res.seq@,
//     {
//         SimpleWrapper { seq: self.seq.clone() }
//     }
// }

// ATTEMPT 4: DeepViewClone on a mapped-view wrapper
#[verifier::reject_recursive_types(T)]
pub struct DeepCloneWrapper<T> {
    pub seq: Vec<T>,
}

impl<T: DeepView> View for DeepCloneWrapper<T> {
    type V = Seq<T::V>;

    open spec fn view(&self) -> Seq<T::V> {
        self.seq.deep_view()
    }
}

impl<T: DeepViewClone> DeepCloneWrapper<T> {
    fn deep_clone_wrapper(&self) -> (res: Self)
        ensures res@ == self@
    {
        DeepCloneWrapper { seq: self.seq.deep_clone() }
    }
}

fn test_deep_clone() {
    let v1: DeepCloneWrapper<u64> = DeepCloneWrapper { seq: vec![1u64, 2u64, 3u64] };
    let v2 = v1.deep_clone_wrapper();
    assert(v2@ == v1@);
}

} // verus!
