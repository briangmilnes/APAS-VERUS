// Copyright (c) 2025 Brian G. Milnes
//! Experiment: Can we use broadcast axioms to avoid explicit assert(cloned(...)) calls?
//!
//! CONCLUSION: We still need explicit assert(cloned(*x, x_clone)) calls.
//! The clone() postcondition establishes cloned(x, x_clone), but we need to assert it
//! to bring it into the SMT solver's context.
//!
//! Similar to obeys_view_eq, we could potentially create a broadcast lemma, but it would
//! require revealing something or having the right triggers.

pub mod clone {
    use vstd::prelude::*;

    verus! {

    // Broadcast axiom attempt: cloned + obeys_view_eq implies view equality
    // Key insight from bst_map_generic: For concrete types like u64, clone just works
    // For generic types, we need BOTH cloned AND equality reasoning (obeys_view_eq)
    //
    // cloned(a, b) means: strictly_cloned(a, b) || a == b
    // If a == b, then we need obeys_view_eq to get a@ == b@
    
    broadcast proof fn axiom_cloned_view_eq<T: Clone + View + PartialEq>()
        requires #[trigger] vstd::laws_eq::obeys_view_eq::<T>()
        ensures
            forall|a: T, b: T| #![trigger cloned(a, b)] cloned(a, b) && a == b ==> a@ == b@
    {
        reveal(vstd::laws_eq::obeys_view_eq);
    }

    broadcast group group_clone_axioms { axiom_cloned_view_eq,}

    // These are what we want at the function level:

    #[verifier::external_body]
    fn test_clone_no_assert_return_eq<T: Clone + View + PartialEq>(x: &T) -> (x_clone: T)
        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures
            x == x_clone // FAILS
    {
//        broadcast use group_clone_axioms;

        x.clone()
    }

    #[verifier::external_body]
    fn test_clone_no_assert_return_eq_view<T: Clone + View + PartialEq>(x: &T) -> (x_clone: T)
        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures
            x@ == x_clone@  // FAILS
    {
//        broadcast use group_clone_axioms;

        x.clone()
    }


    #[verifier::external_body]
    fn test_clone_assert_return<T: Clone + View + PartialEq>(x: T) -> (eq_x_clone: (bool,T))
        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures eq_x_clone.0 <==> x == eq_x_clone.1, // FAILS
    {
//        broadcast use group_clone_axioms;
        
        let x_clone = x.clone();
        if x == x_clone {
            assert(x@ == x_clone@); // FAILS
            return (true,x_clone)
        } else {
            assert(x@ != x_clone@); // FAILS
            return (false,x);
        }
    }

   // These are what we want in executable code:

    #[verifier::external_body]
    fn test_clone_with_assert<T: Clone + View + PartialEq>(x: &T)
        requires vstd::laws_eq::obeys_view_eq::<T>()
    {
        let x_clone = x.clone();
        proof {
            assert(x@ == x_clone@);  // FAILS.
        }
    }

    fn test_1_cloned_automatic<T: Clone + View>(x: &T)
        where T::V: PartialEq
    {
        let x_clone = x.clone();
        proof {
            assert(cloned(*x, x_clone)); // RESULT: We knew this.
        }
    }

    #[verifier::external_body]
    fn test_2_concrete_eq<T: Clone + View + Eq>(x: &T)
        requires vstd::laws_eq::obeys_concrete_eq::<T>()
    {
        use vstd::laws_eq::*;
        let x_clone = x.clone();
        proof {
//            reveal(obeys_concrete_eq);
            assert(cloned(*x, x_clone));  // Automatic from clone
            assert(x@ == x_clone@);       // FAILS.
        }
    }

    #[verifier::external_body]
    fn test_3_extensional_equality<T: Clone + View>(x: &T)
    {
        let x_clone = x.clone();
        proof {
            assert(cloned(*x, x_clone));  // Automatic from clone
            assert(x@ =~= x_clone@);      // FAILS. 
        }
    }

    // These are all concrete and now work.

    fn test_clone_no_assert_return_eq_u64(x: &u64) -> (x_clone: u64)
//        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures
            x == x_clone // SUCCEEDS
    {
        broadcast use group_clone_axioms;
        x.clone()
    }

    fn test_clone_no_assert_return_eq_view_u64(x: &u64) -> (x_clone: u64)
//        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures
            x@ == x_clone@  // SUCCEEDS
    {
        broadcast use group_clone_axioms;
        x.clone()
    }

    fn test_clone_assert_return_u64(x:u64) -> (eq_x_clone: (bool,u64))
//        requires vstd::laws_eq::obeys_view_eq::<T>()
        ensures eq_x_clone.0 <==> x == eq_x_clone.1, // SUCCEEDS
    {
//        broadcast use group_clone_axioms;
        
        let x_clone = x.clone();
        if x == x_clone {
            assert(x@ == x_clone@); // SUCCEEDS
            return (true,x_clone)
        } else {
            assert(x@ != x_clone@); // SUCCEEDS
            return (false,x);
        }
    }

    } // verus!
}
