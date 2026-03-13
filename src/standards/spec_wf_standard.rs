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
//! 6. Ordering: wf predicates come first in requires and ensures.
//!
//!    Put `self.spec_<mod>_wf()`, `other.spec_<mod>_wf()`, and `old(self).spec_<mod>_wf()`
//!    before functional preconditions in requires. Put `self.spec_<mod>_wf()` and
//!    `out.spec_<mod>_wf()` before functional postconditions in ensures. This makes
//!    structural validity immediately visible and separates it from operational constraints.
//!
//!    ```
//!    fn insert(&mut self, key: K, val: V)
//!        requires
//!            old(self).spec_table_wf(),        // wf first
//!            old(self)@.len() < MAX_CAPACITY,  // then functional
//!        ensures
//!            self.spec_table_wf(),             // wf first
//!            self@.contains_key(key),          // then functional
//!    ```
//!
//! 7. Trivially-true wf for Vec-only wrappers.
//!
//!    Some types are thin wrappers around a single `Vec<T>` with no additional
//!    exec fields — e.g., `struct ArraySeqStEphS<T> { pub seq: Vec<T> }`. These
//!    types have no structural invariant beyond what Rust's type system guarantees:
//!    a `Vec<T>` is always a valid sequence. There is no ordering constraint, no
//!    capacity relationship, no index bound to maintain.
//!
//!    For these types, `spec_wf { true }` is the correct body. Do not invent a
//!    synthetic invariant (like `self@.len() <= usize::MAX`) just to avoid `true`.
//!    The wf convention still applies — declare it abstract in the trait, open in
//!    the impl — so that callers uniformly write `requires self.spec_X_wf()`. If
//!    the type later gains a field that introduces a real invariant, the wf body
//!    changes but all call sites already carry the precondition.
//!
//!    Types that DO need a non-trivial wf: any struct with multiple fields whose
//!    relationship must be maintained (e.g., `distances.len() == predecessors.len()`),
//!    types with ordering invariants (heaps, sorted lists, BSTs), or types with
//!    domain constraints (capacity bounds, non-empty guarantees).
//!
//!    The `VecWrapper` example below demonstrates the trivially-true pattern.
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
            requires
                Self::spec_valid_key_type(),
            ensures
                out.spec_container_wf();

        /// &self input: wf first in requires.
        fn len(&self) -> (n: usize)
            requires
                self.spec_container_wf(),
            ensures
                n == self@.len();

        /// &mut self: wf first in both requires and ensures.
        fn push(&mut self, x: T)
            requires
                old(self).spec_container_wf(),
                old(self)@.len() < 999,
            ensures
                self.spec_container_wf();

        /// Self input and output: wf first on both.
        fn merge(&self, other: &Self) -> (out: Self)
            requires
                self.spec_container_wf(),
                other.spec_container_wf(),
            ensures
                out.spec_container_wf();

        /// Non-Self input and output: free function wf first (cycle workaround).
        fn flatten(nested: &Container<Container<T>>) -> (out: Container<Container<T>>)
            requires
                spec_container_wf_generic(nested),
            ensures
                spec_container_wf_generic(&out);
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

    // Trivially-true wf for a Vec-only wrapper (rule 6).

    pub struct VecWrapper<T> {
        pub seq: Vec<T>,
    }

    impl<T> View for VecWrapper<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.seq@ }
    }

    pub trait VecWrapperTrait<T> : View<V = Seq<T>> + Sized {
        /// Abstract wf — trivially true for a Vec-only wrapper.
        spec fn spec_vecwrapper_wf(&self) -> bool;

        fn empty() -> (out: Self)
            ensures
                out.spec_vecwrapper_wf(),
                out@.len() == 0;

        fn len(&self) -> (n: usize)
            requires
                self.spec_vecwrapper_wf(),
            ensures
                n == self@.len();

        fn push(&mut self, x: T)
            requires
                old(self).spec_vecwrapper_wf(),
            ensures
                self.spec_vecwrapper_wf(),
                self@.len() == old(self)@.len() + 1;
    }

    impl<T> VecWrapperTrait<T> for VecWrapper<T> {
        /// Trivially true: Vec<T> has no structural invariant to express.
        open spec fn spec_vecwrapper_wf(&self) -> bool { true }

        fn empty() -> (out: Self) {
            VecWrapper { seq: Vec::new() }
        }

        fn len(&self) -> (n: usize) {
            self.seq.len()
        }

        fn push(&mut self, x: T) {
            self.seq.push(x);
        }
    }

    } // verus!
}
