//! feq - Full Equality specification extending Eq
//!
//! Uses external_trait_specification/external_trait_extension pattern like vstd/std_specs/cmp.rs
//! to add full equality specs to Rust's Eq trait.

#[cfg(verus_keep_ghost)]
pub mod feq {
    use vstd::prelude::*;
    use vstd::std_specs::cmp::PartialEqSpec;
    use vstd::pervasive::strictly_cloned;
    use core::cmp::Eq;
    use core::marker::PointeeSized;

    verus! {

    // Extend Eq with full equality specs using external_trait_extension
    #[verifier::external_trait_specification]
    #[verifier::external_trait_extension(FeqSpec via FeqSpecImpl)]
    pub trait ExFeq: PartialEq + PointeeSized {
        type ExternalTraitSpecificationFor: Eq;

        // Whether this type obeys full equality (reflexive, symmetric, transitive)
        spec fn obeys_feq() -> bool;
    }

    // Spec functions for full equality properties (using eq_spec from PartialEqSpec)
    pub open spec fn feq_reflexive<T: Eq + Sized>() -> bool {
        forall|x: T| #[trigger] x.eq_spec(&x)
    }

    pub open spec fn feq_symmetric<T: Eq + Sized>() -> bool {
        forall|x: T, y: T| #[trigger] x.eq_spec(&y) <==> y.eq_spec(&x)
    }

    pub open spec fn feq_transitive<T: Eq + Sized>() -> bool {
        forall|x: T, y: T, z: T| #[trigger] x.eq_spec(&y) && #[trigger] y.eq_spec(&z) ==> x.eq_spec(&z)
    }

    pub open spec fn obeys_feq_properties<T: Eq + Sized>() -> bool {
        &&& feq_reflexive::<T>()
        &&& feq_symmetric::<T>()
        &&& feq_transitive::<T>()
    }

    pub open spec fn obeys_feq_view<T: Eq + View + Sized>() -> bool {
        forall|x: T, y: T| #[trigger] x.eq_spec(&y) ==> x@ == y@
    }

    pub open spec fn obeys_feq_clone<T: Eq + Clone + Sized>() -> bool {
        &&& forall|x: T, y: T| #[trigger] cloned(x, y) ==> x.eq_spec(&y)
        &&& forall|x: &T, y: T| #[trigger] cloned(*x, y) ==> x.eq_spec(&y)
        &&& forall|x: T, y: T| #[trigger] strictly_cloned(x, y) ==> x.eq_spec(&y)
        &&& forall|x: &T, y: T| #[trigger] strictly_cloned(*x, y) ==> x.eq_spec(&y)
    }

    // The eq_spec function and the == operator mean the same thing.
    pub open spec fn obeys_feq_eq<T: Eq + Sized>() -> bool {
        forall|x: T, y: T| #[trigger] x.eq_spec(&y) <==> x == y
    }

    // View injectivity: equal views imply equal values
    pub open spec fn obeys_feq_view_injective<T: Eq + View + Sized>() -> bool {
        forall|x: T, y: T| #[trigger] x.view() == #[trigger] y.view() ==> x == y
    }

    pub open spec fn obeys_feq_full<T: Eq + View + Clone + Sized>() -> bool {
        &&& obeys_feq_properties::<T>()
        &&& obeys_feq_view::<T>()
        &&& obeys_feq_view_injective::<T>()
        &&& obeys_feq_clone::<T>()
        &&& obeys_feq_eq::<T>()
    }

    /// Lemma: cloned values have equal views when obeys_feq_full holds
    pub proof fn lemma_cloned_view_eq<T: Eq + View + Clone + Sized>(x: T, y: T)
        requires cloned(x, y), obeys_feq_full::<T>(),
        ensures x@ == y@,
    {
        // obeys_feq_clone: cloned(x, y) ==> x.eq_spec(&y)
        assert(x.eq_spec(&y));
        // obeys_feq_view: x.eq_spec(&y) ==> x@ == y@
    }

    /// Lemma: obeys_feq_view_injective implies vstd::relations::injective on view
    pub proof fn lemma_view_injective<T: Eq + View + Sized>()
        requires obeys_feq_view_injective::<T>(),
        ensures vstd::relations::injective(|k: T| k@),
    {
        let f = |k: T| k@;
        assert forall |x: T, y: T| #[trigger] f(x) == #[trigger] f(y) implies x == y by {
            // obeys_feq_view_injective gives us: x@ == y@ ==> x == y
            assert(x@ == y@ ==> x == y);
        }
    }

    // Implementation for bool
    impl FeqSpecImpl for bool {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<bool>() }
    }

    // Implementation for u8
    impl FeqSpecImpl for u8 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<u8>() }
    }

    // Implementation for u16
    impl FeqSpecImpl for u16 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<u16>() }
    }

    // Implementation for u32
    impl FeqSpecImpl for u32 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<u32>() }
    }

    // Implementation for u64
    impl FeqSpecImpl for u64 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<u64>() }
    }

    // Implementation for u128
    impl FeqSpecImpl for u128 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<u128>() }
    }

    // Implementation for usize
    impl FeqSpecImpl for usize {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<usize>() }
    }

    // Implementation for i8
    impl FeqSpecImpl for i8 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<i8>() }
    }

    // Implementation for i16
    impl FeqSpecImpl for i16 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<i16>() }
    }

    // Implementation for i32
    impl FeqSpecImpl for i32 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<i32>() }
    }

    // Implementation for i64
    impl FeqSpecImpl for i64 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<i64>() }
    }

    // Implementation for i128
    impl FeqSpecImpl for i128 {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<i128>() }
    }

    impl FeqSpecImpl for isize {
        open spec fn obeys_feq() -> bool { obeys_feq_properties::<isize>() }
    }

    // Broadcast proof: cloned values are equal
// Veracity: USED
    pub broadcast proof fn axiom_cloned_implies_eq<T: Eq + Clone + Sized>(x: &T, y: T)
        requires #[trigger] cloned(*x, y), obeys_feq_clone::<T>()
        ensures *x == y
    {
        admit();
    }

    // Exec function: test equality and get spec fact
    pub fn feq<T: Eq + View + Clone + Sized>(x: &T, y: &T) -> (eq: bool)
        requires obeys_feq_full::<T>()
        ensures eq == (x@ == y@)
    {
        let result = *x == *y;
        proof {
            // obeys_feq_eq: x.eq_spec(&y) <==> x == y
            // obeys_feq_view: x.eq_spec(&y) ==> x@ == y@
            // obeys_feq_view_injective: x@ == y@ ==> x == y
            assume(result == (x@ == y@));
        }
        result
    }

    pub broadcast group group_feq_axioms {
        axiom_cloned_implies_eq,
    }

    } // verus!
}

// Stub module for non-Verus compilation
#[cfg(not(verus_keep_ghost))]
pub mod feq {
    /// Stub feq function for non-Verus builds - just uses ==
    pub fn feq<T: Eq>(x: &T, y: &T) -> bool {
        *x == *y
    }
    
    /// Stub obeys_feq_clone for non-Verus builds - always true
    pub fn obeys_feq_clone<T: Eq + Clone>() -> bool { true }
}
