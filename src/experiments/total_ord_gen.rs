//! Experiment: TotalOrdGen - A total ordering trait with full Eq specifications
//!
//! GOAL: Create a trait that specifies:
//! 1. Full equality (reflexive, symmetric, transitive)
//! 2. Total ordering with <
//! 3. Axioms to assume proofs for both
pub mod total_ord_gen {
    use vstd::prelude::*;

verus! {

// Total ordering trait with full Eq specifications
    pub trait TotalOrdGen: PartialEq + Eq + Sized + View + Clone {
        // Equality axioms
        proof fn axiom_eq_reflexive(x: &Self)                      ensures *x == *x;
        proof fn axiom_eq_symmetric(x: &Self, y: &Self)            ensures (*x == *y) == (*y == *x);
        proof fn axiom_eq_transitive(x: &Self, y: &Self, z: &Self) requires *x == *y, *y == *z, ensures *x == *z;
        proof fn axiom_eq_view(x: &Self, y: &Self) requires *x == *y, ensures x@ == y@;

        // Clone axioms
        proof fn axiom_cloned_implies_eq(x: &Self, x_clone: &Self)
            requires cloned(*x, *x_clone)
            ensures *x == *x_clone;

        proof fn axiom_cloned_view_eq(x: &Self, x_clone: Self)
            requires cloned(*x, x_clone)
            ensures x@ == x_clone@, *x == x_clone;

        // Ordering spec
        spec fn le_spec(&self, other: &Self) -> bool;

    // Ordering axioms

        proof fn axiom_le_reflexive(x: &Self) ensures x.le_spec(x);
        proof fn axiom_le_antisymmetric(x: &Self, y: &Self) requires x.le_spec(y), y.le_spec(x), ensures *x == *y;
        proof fn axiom_le_transitive(x: &Self, y: &Self, z: &Self)
            requires x.le_spec(y), y.le_spec(z),
            ensures x.le_spec(z);
        proof fn axiom_le_total(x: &Self, y: &Self) ensures x.le_spec(y) || y.le_spec(x);
    }

// Generic implementation - axioms are assumed.
        impl<T: PartialEq + Eq + Sized + View + Clone> TotalOrdGen for T {
            proof fn axiom_eq_reflexive(x: &Self) { assume(*x == *x); }
            proof fn axiom_eq_symmetric(x: &Self, y: &Self) { assume((*x == *y) == (*y == *x)); }
            proof fn axiom_eq_transitive(x: &Self, y: &Self, z: &Self) { assume(*x == *y && *y == *z ==> *x == *z); }
            proof fn axiom_eq_view(x: &Self, y: &Self) { assume(*x == *y ==> x@ == y@); }

            proof fn axiom_cloned_implies_eq(x: &Self, x_clone: &Self) {
                assume(cloned(*x, *x_clone) ==> *x == *x_clone);
            }

            proof fn axiom_cloned_view_eq(x: &Self, x_clone: Self) {
                Self::axiom_cloned_implies_eq(x, &x_clone);
                Self::axiom_eq_view(x, &x_clone);
            }

            open spec fn le_spec(&self, other: &Self) -> bool { arbitrary() }
            proof fn axiom_le_reflexive(x: &Self) { assume(x.le_spec(x)); }
            proof fn axiom_le_antisymmetric(x: &Self, y: &Self) { assume(x.le_spec(y) && y.le_spec(x) ==> *x == *y); }
            proof fn axiom_le_transitive(x: &Self, y: &Self, z: &Self) 
             { assume(x.le_spec(y) && y.le_spec(z) ==> x.le_spec(z)); }
            proof fn axiom_le_total(x: &Self, y: &Self) { assume(x.le_spec(y) || y.le_spec(x)); }
        }

    // Test function that requires TotalOrdGen
    fn test_equality_axioms<T: TotalOrdGen>(x: T, y: T, z: T) {
        proof {
            assert(x == x); // T::axiom_eq_reflexive(&x);

            if x == y { 
                assert(y == x); // T::axiom_eq_symmetric(&x, &y);
            }

            if x == y && y == z {
                assert(x == z); // T::axiom_eq_transitive(&x, &y, &z);
            }

            if x == y {
                assert(x@ == y@); // T::axiom_eq_view(&x, &y);
            }
        }
    }

    fn test_clone_axioms<T: TotalOrdGen>(x: T) {
        let x_clone = x.clone();
        proof {
            assert(x@ == x_clone@); // T::axiom_cloned_view_eq(&x, x_clone);
            assert(x == x_clone);   // T::axiom_cloned_view_eq(&x, x_clone);
        }
    }

    fn test_ordering_axioms<T: TotalOrdGen>(x: T, y: T, z: T) {
        proof {
            assert(x.le_spec(&x)); // T::axiom_le_reflexive(&x);

            assert(x.le_spec(&y) || y.le_spec(&x)); // T::axiom_le_total(&x, &y);

            if x.le_spec(&y) && y.le_spec(&z) {
                assert(x.le_spec(&z)); // T::axiom_le_transitive(&x, &y, &z);
            }

            if x.le_spec(&y) && y.le_spec(&x) {
                assert(x == y); // T::axiom_le_antisymmetric(&x, &y);
            }
        }
    }

    // Concrete test with u64 to show instantiation works
    fn test_u64_uses_trait() {
        test_equality_axioms(1u64, 2u64, 3u64);
        test_clone_axioms(42u64);
        test_ordering_axioms(5u64, 10u64, 15u64);
    }

        } // verus!
}
