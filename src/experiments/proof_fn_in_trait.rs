// Copyright (c) 2025 Brian G. Milnes
//! Experiment: Can we put proof fn in a trait?
//!
//! GOAL: Test if Verus allows proof functions as trait methods
//! RESULT: Yes.

pub mod proof_fn_in_trait {
    use vstd::prelude::*;

    verus! {

    // TEST 1: Simple trait with a proof function
    pub trait SimpleTrait {
        proof fn simple_proof_method(&self)
            ensures true;
    }

    // TEST 2: Implement it for u64
    impl SimpleTrait for u64 {
        proof fn simple_proof_method(&self)
            ensures true
        {
            // Empty proof body
        }
    }

    // TEST 3: Trait with spec fn and proof fn
    pub trait ViewTrait: View {
        spec fn some_property(&self) -> bool;
        
        proof fn lemma_property_holds(&self)
            requires self.some_property()
            ensures self@ == self@;
    }

    // TEST 4: Implement for u64
    impl ViewTrait for u64 {
        open spec fn some_property(&self) -> bool {
            true
        }
        
        proof fn lemma_property_holds(&self)
        {
            // requires/ensures come from trait
            // Should prove trivially
        }
    }

    // TEST 5: Can we call the proof function?
    fn test_call_proof_fn(x: u64) {
        proof {
            x.simple_proof_method();
            x.lemma_property_holds();
        }
    }

    } // verus!
}
