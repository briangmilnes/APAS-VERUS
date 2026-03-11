//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Spec WF Standard: well-formedness predicates in trait-impl modules.
//!
//! This standard covers how to attach `spec_<module>_wf` predicates to traits,
//! including the Verus cycle workaround for concrete non-Self types.
//!
//! 1. Abstract in the trait, open in the impl.
//!
//!    The trait declares `spec fn spec_<module>_wf(&self) -> bool;` (abstract).
//!    The impl provides `open spec fn spec_<module>_wf(&self) -> bool { ... }`.
//!    This follows the multi-struct standard's abstract/open pattern.
//!
//! 2. `spec_valid_key_type` follows the same pattern.
//!
//!    Abstract in the trait: `spec fn spec_valid_key_type() -> bool;`
//!    Open in the impl: `open spec fn spec_valid_key_type() -> bool { valid_key_type::<T>() }`
//!    Constructors (`empty`, `singleton`, `from_vec`) use `Self::spec_valid_key_type()`
//!    in requires since they have no `&self` to call wf on.
//!
//! 3. Every same-type `&self`/`&Self` input gets wf in requires.
//!    Every `Self` output gets wf in ensures.
//!
//!    | Position | Self type                    | Non-Self type                  |
//!    |----------|------------------------------|--------------------------------|
//!    | Requires | `self.spec_<mod>_wf()`       | `spec_<mod>_wf_generic(x)`     |
//!    | Ensures  | `out.spec_<mod>_wf()`        | `spec_<mod>_wf_generic(&out)`  |
//!
//! 4. Non-Self concrete types need a free function (Verus cycle workaround).
//!
//!    Calling a trait method on a concrete non-Self type inside a trait definition
//!    triggers a 3-node cycle in Verus: method → impl body → trait declaration.
//!    This happens because Verus must resolve which impl provides the method for
//!    the concrete type, but that impl depends on the trait being fully defined.
//!
//!    `Self` avoids this because Verus uses the abstract declaration directly
//!    without resolving an impl.
//!
//!    The workaround is a generic free spec fn at module level:
//!
//!    ```
//!    pub open spec fn spec_<mod>_wf_generic<V: Bounds>(s: &MyType<V>) -> bool {
//!        s@.finite() && valid_key_type::<V>()
//!    }
//!    ```
//!
//!    This bypasses trait dispatch entirely — no impl resolution, no cycle.
//!
//!    Alternatives that do NOT work (see `experiments/generic_specs_to_prevent_cycles.rs`):
//!    - Trait method on concrete type: cycles.
//!    - Generic static spec fn in trait (`spec fn wf_of<V>`): Verus parse error.
//!    - Type parameter on wf (`spec fn wf<V>(&self)`): still cycles.
//!
//! 5. `&mut self` methods use `old(self).spec_<mod>_wf()` in requires,
//!    `self.spec_<mod>_wf()` in ensures.
//!
//! Reference implementation: `src/Chap05/SetStEph.rs`.
//! Experiment: `src/experiments/generic_specs_to_prevent_cycles.rs`.

pub mod spec_wf_standard {

    use vstd::prelude::*;

    verus! {

    //  4. type definitions

    pub struct Container<T> {
        pub data: Vec<T>,
    }

    //  5. view impls

    impl<T> View for Container<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.data@ }
    }

    //  6. spec fns

    /// Generic free spec fn for non-Self types (cycle workaround).
    pub open spec fn spec_container_wf_generic<V>(s: &Container<V>) -> bool {
        s@.len() < 1000
    }

    //  8. traits

    pub trait ContainerTrait<T> : View<V = Seq<T>> + Sized {

        /// Abstract wf — open in impl.
        spec fn spec_container_wf(&self) -> bool;

        /// Abstract valid key type — open in impl.
        spec fn spec_valid_key_type() -> bool;

        /// Constructor: no &self, uses spec_valid_key_type.
        fn empty() -> (out: Self)
            requires Self::spec_valid_key_type()
            ensures out.spec_container_wf();

        /// &self input: wf in requires.
        fn len(&self) -> (n: usize)
            requires self.spec_container_wf()
            ensures n == self@.len();

        /// &mut self: old(self) wf in requires, self wf in ensures.
        fn push(&mut self, x: T)
            requires old(self).spec_container_wf(), old(self)@.len() < 999
            ensures self.spec_container_wf();

        /// Self input and output: wf on both.
        fn merge(&self, other: &Self) -> (out: Self)
            requires self.spec_container_wf(), other.spec_container_wf()
            ensures out.spec_container_wf();

        /// Non-Self input and output: free function wf (cycle workaround).
        fn flatten(nested: &Container<Container<T>>) -> (out: Container<Container<T>>)
            requires spec_container_wf_generic(nested)
            ensures spec_container_wf_generic(&out);
    }

    //  9. impls

    impl<T> ContainerTrait<T> for Container<T> {

        /// Open wf: the real predicate body.
        open spec fn spec_container_wf(&self) -> bool {
            self@.len() < 1000
        }

        open spec fn spec_valid_key_type() -> bool {
            true
        }

        fn empty() -> (out: Self) {
            Container { data: Vec::new() }
        }

        fn len(&self) -> (n: usize) {
            self.data.len()
        }

        fn push(&mut self, x: T) {
            self.data.push(x);
        }

        fn merge(&self, other: &Self) -> (out: Self) {
            Container { data: Vec::new() }
        }

        fn flatten(nested: &Container<Container<T>>) -> (out: Container<Container<T>>) {
            Container { data: Vec::new() }
        }
    }

    } // verus!
}
