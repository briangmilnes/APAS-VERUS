// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: can we call spec_wf on a concrete non-Self type in a trait?
//!
//! Problem: calling `spec_wf()` on `Wrapper<Wrapper<T>>` (concrete, not Self)
//! inside a trait definition causes a 3-node cycle:
//!   method → impl body → trait decl → method.
//! Verus must resolve the impl for the concrete type, which depends on the
//! trait, which contains the method that triggered the lookup.
//!
//! Four approaches tested on Pattern 4 (concrete non-Self input and output):
//!
//! - 4a: Generic free spec fn.  PASSES — no trait dispatch, no cycle.
//! - 4b: Trait method on concrete type.  CYCLES — impl resolution.
//! - 4c: Generic spec fn in trait (`spec fn wf_of<V>`).  PARSE ERROR —
//!       Verus does not support type parameters on spec fns in traits.
//! - 4d: Type param on spec_wrapper_wf<V>.  CYCLES — impl resolution still
//!       triggered by `nested` being concrete, regardless of turbofish.
//!
//! Conclusion: only a free function (4a) avoids the cycle.
//!
//! RESULT: FAILS (4c is the active code; 4a works but is commented out)

pub mod generic_specs_to_prevent_cycles {

use vstd::prelude::*;

verus! {

    //  4. type definitions

    pub struct Wrapper<T> {
        pub data: Vec<T>,
    }

    //  5. view impls

    impl<T> View for Wrapper<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

    //  6. spec fns

    /// Generic free spec fn — no trait resolution needed.
    pub open spec fn spec_wrapper_wf_generic<V>(s: &Wrapper<V>) -> bool {
        s@.len() < 1000
    }

    //  8. traits

    pub trait WrapperTrait<T> : View<V = Seq<T>> + Sized {
        spec fn spec_wrapper_wf(&self) -> bool;

        /// 4c: Verus cannot parse type params on trait spec fns.
        spec fn spec_wf_of<V>(&self, s: &Wrapper<V>) -> bool;

        /// Pattern 1: &self -> Self.
        fn dup(&self) -> (out: Self)
            requires self.spec_wrapper_wf()
            ensures out.spec_wrapper_wf();

        /// Pattern 2: &self -> bool.
        fn check(&self) -> (b: bool)
            requires self.spec_wrapper_wf();

        /// Pattern 3: () -> Self.
        fn empty() -> (out: Self)
            ensures out.spec_wrapper_wf();

        /// Pattern 4a: free fn — PASSES.
        // fn flatten(nested: &Wrapper<Wrapper<T>>) -> (out: Wrapper<Wrapper<T>>)
        //     requires spec_wrapper_wf_generic(nested)
        //     ensures spec_wrapper_wf_generic(&out);

        /// Pattern 4b: non-generic trait method on concrete type — CYCLES.
        // fn flatten(nested: &Wrapper<Wrapper<T>>) -> (out: Wrapper<Wrapper<T>>)
        //     requires nested.spec_wrapper_wf()
        //     ensures out.spec_wrapper_wf();

        /// Pattern 4c: generic spec fn in trait — PARSE ERROR.
        fn flatten(nested: &Wrapper<Wrapper<T>>) -> (out: Wrapper<Wrapper<T>>)
            requires spec_wrapper_wf_generic(nested)
            ensures spec_wrapper_wf_generic(&out);

        /// Pattern 4d: type param on spec_wrapper_wf<V> — CYCLES.
        // fn flatten(nested: &Wrapper<Wrapper<T>>) -> (out: Wrapper<Wrapper<T>>)
        //     requires nested.spec_wrapper_wf::<Wrapper<T>>()
        //     ensures out.spec_wrapper_wf::<Wrapper<T>>();
    }

    //  9. impls

    impl<T> WrapperTrait<T> for Wrapper<T> {

        open spec fn spec_wrapper_wf(&self) -> bool {
            self@.len() < 1000
        }

        open spec fn spec_wf_of<V>(&self, s: &Wrapper<V>) -> bool {
            s@.len() < 1000
        }

        fn dup(&self) -> (out: Self) {
            Wrapper { data: Vec::new() }
        }

        fn check(&self) -> (b: bool) {
            self.data.len() < 500
        }

        fn empty() -> (out: Self) {
            Wrapper { data: Vec::new() }
        }

        fn flatten(nested: &Wrapper<Wrapper<T>>) -> (out: Wrapper<Wrapper<T>>) {
            Wrapper { data: Vec::new() }
        }
    }

} // verus!
}
