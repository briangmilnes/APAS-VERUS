// Copyright (c) 2025 Brian G. Milnes
//! Experiment: Can we create assert-like comments that type check but don't prove?
//! And can such constructs be used to prove false?

pub mod pervasives {
    use vstd::prelude::*;

    verus! {
    //!	5. spec fns
    //!	6. proof fns/broadcast groups
    //!	9. exec fns

    //!		5. spec fns

    // Approach 3: spec_comment - doesn't prove, just documents
    
    pub open spec fn spec_comment(description: &str, claim: bool) -> bool {
        true  // Always returns true, ignoring the claim
    }


    //!		6. proof fns/broadcast groups

    // Approach 1: External body proof function
    
    #[verifier::external_body]
    pub proof fn assert_unchecked(b: bool)
        ensures b,  // Claims b is true without proof!
    {
        // Body not checked by verifier
    }

    // Test: Can assert_unchecked prove false?
    pub proof fn test_assert_unchecked_proves_false()
        ensures false,
    {
        assert_unchecked(false);  // This should make false true!
    }

    // Approach 2: external_body on claim_anything
    
    #[verifier::external_body]
    pub proof fn claim_anything<A>(a: A, b: A)
        ensures a == b,
    {
    }

    pub proof fn test_claim_anything_proves_false()
        ensures false,
    {
        claim_anything(1int, 2int);  // Now 1 == 2
        assert(1int == 2int);        // From ensures
        assert(false);               // 1 != 2, contradiction
    }

    pub proof fn example_with_spec_comments() {
        assert(spec_comment("x should be positive", true));
        assert(spec_comment("this claim is false but passes", false));
    }

    // Approach 4: assume(false) - the known cheat
    
    pub proof fn using_assume_proves_false()
        ensures false,
    {
        assume(false);
    }


    //!		9. exec fns

    // Test vstd::pervasive::unreached
    
    // unreached<A>() has requires false, returns A
    // It's in vstd::prelude so should be available
    
    pub fn test_unreached_in_match() -> u32 {
        let x: Option<u32> = Some(42);
        match x {
            Some(v) => v,
            None => {
                // This branch is unreachable since x = Some(42)
                // But Verus doesn't know that without proof
                // unreached()  // Would need `requires false` to be satisfied
                0  // Use dummy for now
            }
        }
    }

    // Using unreached properly - must prove the branch is impossible
    pub fn test_unreached_with_proof(x: u32) -> u32
        requires x > 0,
    {
        if x > 0 {
            x
        } else {
            // This branch is impossible given requires x > 0
            proof { assert(false); }  // Proves this is unreachable
            unreached()
        }
    }

} // verus!
}
