//! Experiment: Trait hierarchy for equality relations
//!
//! GOAL: Build a trait hierarchy that guarantees:
//! 1. EqRel: Basic equality (reflexive, symmetric, transitive)
//! 2. EqRelWithView: Adds view equality
//! 3. EqRelWithViewClone: Adds clone with view equality

pub mod eq_rel {
    use vstd::prelude::*;

    verus! {

    pub trait EqRel: PartialEq + Eq + Sized {
        proof fn eq_reflexive(x: &Self)           ensures x == x;
        proof fn eq_symmetric(x: &Self, y: &Self) ensures (x == y) == (y == x);
        proof fn eq_transitive(x: &Self, y: &Self, z: &Self)
            requires x == y, y == z              ensures x == z;
    }

    pub trait EqRelWithView: EqRel + View {
        proof fn eq_view(x: &Self, y: &Self)
            requires x == y
            ensures x@ == y@;
    }

    // Subtrait: Adds Clone and guarantees clone view equality.
    pub trait EqRelWithViewClone: EqRelWithView + Clone {
        // Axiom: cloned implies executable equality for this type
        proof fn axiom_cloned_implies_eq(x: &Self, x_clone: &Self)
            requires cloned(*x, *x_clone)
            ensures *x == *x_clone;

        // Clone gives equal view: x@ == x_clone@.
        proof fn clone_view_eq(x: &Self, x_clone: Self)
            requires cloned(*x, x_clone)
            ensures x@ == x_clone@,
                    x  == x_clone;
    }

/*
    // These conflict with the generic impl - generic impl instantiates for u64,
    // but prove otherwise. 
    impl EqRel for u64 {
        proof fn eq_reflexive(x: &Self) {}
        proof fn eq_symmetric(x: &Self, y: &Self) {}
        proof fn eq_transitive(x: &Self, y: &Self, z: &Self) {}
    }
    impl EqRelWithView for u64 { proof fn eq_view(x: &Self, y: &Self) {} }
    impl EqRelWithViewClone for u64 { 
        proof fn axiom_cloned_implies_eq(x: &Self, x_clone: &Self) {}
        proof fn clone_view_eq(x: &Self, x_clone: Self) {}
    }
*/

    // Trait that uses EqRelWithViewClone to provide test operations
    pub trait EqRelTest<T: EqRelWithViewClone> {
        fn test_eq_rel(x: T, y: T, z: T);
        fn test_eq_rel_view(x: T, y: T);
        fn test_eq_rel_clone(x: T);
    }

    // Implementation of EqRelTest directly for u64
    impl EqRelTest<u64> for u64 {
        fn test_eq_rel(x: u64, y: u64, z: u64) {
            proof {
// These are not even needed, it just proves.
//            u64::eq_reflexive(&x);
//            u64::eq_symmetric(&x, &y);
//            u64::eq_transitive(&x, &y, &z);
                assert(x == x);
                if x == y           { assert(y == x); }
                if x == y && y == z { assert(x == z);}
            }
        }

        fn test_eq_rel_view(x: u64, y: u64) {
//                u64::eq_view(&x, &y); Not needed.
            proof { if x == y { assert(x@ == y@); } }
        }

        fn test_eq_rel_clone(x: u64) {
 //            u64::clone_view_eq(&x, x_clone); Not needed.
            let x_clone = x.clone();
            proof { assert(x@ == x_clone@); }
        }
    }

     impl<T: PartialEq + Eq + Sized> EqRel for T {
         proof fn eq_reflexive(x: &Self) {}
         proof fn eq_symmetric(x: &Self, y: &Self) {}
         proof fn eq_transitive(x: &Self, y: &Self, z: &Self) {}
     }

     impl<T: EqRel + View> EqRelWithView for T {
         proof fn eq_view(x: &Self, y: &Self) {}
     }

     impl<T: EqRelWithView + Clone> EqRelWithViewClone for T {
         proof fn axiom_cloned_implies_eq(x: &Self, x_clone: &Self) {
             // This is an AXIOM for generic T - we assume it
             assume(cloned(*x, *x_clone) ==> *x == *x_clone);
         }

         proof fn clone_view_eq(x: &Self, x_clone: Self) {
             // Use the axiom to get x == x_clone
             Self::axiom_cloned_implies_eq(x, &x_clone);
             // Now we have x == x_clone, use eq_view to get x@ == x_clone@
             Self::eq_view(x, &x_clone);
         }
     }

     // Generic implementation of EqRelTest
     struct GenericEqRelTest;

     impl<T: EqRelWithViewClone> EqRelTest<T> for GenericEqRelTest {
         fn test_eq_rel(x: T, y: T, z: T) {
             proof {
                 assert(x == x);
                 if x == y           { assert(y == x); }
                 if x == y && y == z { assert(x == z);}
             }
         }

         fn test_eq_rel_view(x: T, y: T) {
             proof { if x == y { assert(x@ == y@); } }
         }

         fn test_eq_rel_clone(x: T) {
             let x_clone = x.clone();
             proof { 
                 T::clone_view_eq(&x, x_clone);
                 assert(x@ == x_clone@); 
             }
         }
     }

    } // verus!
}

