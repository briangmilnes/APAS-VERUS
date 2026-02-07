// Copyright (c) 2025 Brian G. Milnes
//! Experiment: TotalOrdGenAxioms - Using broadcast axioms instead of trait methods
//!
//! GOAL: Make axioms fire automatically via broadcast instead of explicit calls

pub mod total_ord_gen_axioms {
    use vstd::prelude::*;

    verus! {
    //!	5. spec fns
    //!	6. proof fns/broadcast groups
    //!	7. traits
    //!	8. impls
    //!	9. exec fns

    //!		5. spec fns

    // Marker spec to trigger broadcast axioms
    spec fn is_total_ord_gen<T: TotalOrdGenAxioms>() -> bool { true }


    //!		6. proof fns/broadcast groups

    // Broadcast axioms for equality
    broadcast proof fn axiom_eq_transitive<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, y: T, z: T| x == y && y == z ==> #[trigger] (x == z)
    {
        admit(); // Cannot prove for generic T
    }

    broadcast proof fn axiom_eq_view<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, y: T| x == y ==> #[trigger] (x@ == y@)
    {
        admit(); // Cannot prove for generic T
    }

    // Broadcast axioms for clone
    broadcast proof fn axiom_cloned_implies_eq<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, x_clone: T| cloned(x, x_clone) ==> #[trigger] (x == x_clone)
    {
        admit(); // Cannot prove for generic T
    }

    // Broadcast axioms for ordering
    broadcast proof fn axiom_le_reflexive<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T| #[trigger] x.le_spec(&x)
    {
        admit(); // le_spec is admit()
    }

    broadcast proof fn axiom_le_antisymmetric<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, y: T| x.le_spec(&y) && y.le_spec(&x) ==> #[trigger] (x == y)
    {
        admit(); // Cannot prove for arbitrary le_spec
    }

    broadcast proof fn axiom_le_transitive<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, y: T, z: T| x.le_spec(&y) && #[trigger] y.le_spec(&z) ==> x.le_spec(&z)
    {
        admit(); // Cannot prove for arbitrary le_spec
    }

    broadcast proof fn axiom_le_total<T: TotalOrdGenAxioms>()
        requires #[trigger] is_total_ord_gen::<T>()
        ensures forall|x: T, y: T| #[trigger] x.le_spec(&y) || y.le_spec(&x)
    {
        admit(); // Cannot prove for arbitrary le_spec
    }

    // Broadcast group
    broadcast group group_total_ord_axioms {
        axiom_eq_transitive,
        axiom_eq_view,
        axiom_cloned_implies_eq,
        axiom_le_reflexive,
        axiom_le_antisymmetric,
        axiom_le_transitive,
        axiom_le_total,
    }


    //!		7. traits

    // Trait without axiom methods - just the specs
    pub trait TotalOrdGenAxioms: PartialEq + Eq + Sized + View + Clone {
        spec fn le_spec(&self, other: &Self) -> bool;
    }


    //!		8. impls

    // Generic implementation - just the spec
    impl<T: PartialEq + Eq + Sized + View + Clone> TotalOrdGenAxioms for T {
        open spec fn le_spec(&self, other: &Self) -> bool {
            true
        }
    }


    //!		9. exec fns

    // Test functions - no explicit axiom calls needed
    fn test_equality_axioms<T: TotalOrdGenAxioms>(x: T, y: T, z: T) {
        broadcast use group_total_ord_axioms;
        proof {
            assert(x == x);
            if x == y { 
                assert(y == x);
            }
            if x == y && y == z {
                assert(x == z);
            }
            if x == y {
                assert(x@ == y@);
            }
        }
    }

    fn test_clone_axioms<T: TotalOrdGenAxioms>(x: T) {
        broadcast use group_total_ord_axioms;
        let x_clone = x.clone();
        proof {
            assert(x@ == x_clone@);
            assert(x == x_clone);
        }
    }

    fn test_ordering_axioms<T: TotalOrdGenAxioms>(x: T, y: T, z: T) {
        broadcast use group_total_ord_axioms;
        proof {
            assert(x.le_spec(&x));
            assert(x.le_spec(&y) || y.le_spec(&x));
            if x.le_spec(&y) && y.le_spec(&z) {
                assert(x.le_spec(&z));
            }
            if x.le_spec(&y) && y.le_spec(&x) {
                assert(x == y);
            }
        }
    }

    // Concrete test with u64
    fn test_u64_uses_broadcast() {
        test_equality_axioms(1u64, 2u64, 3u64);
        test_clone_axioms(42u64);
        test_ordering_axioms(5u64, 10u64, 15u64);
    }

} // verus!
}
