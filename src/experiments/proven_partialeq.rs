// Copyright (c) 2025 Brian G. Milnes
//! Experiment: Proven PartialEq with forall axiom proofs
//!
//! GOAL: Create ProvenPartialEq requiring proofs of:
//!   - reflexivity: forall x. eq(x, x)
//!   - symmetry: forall x, y. eq(x, y) ==> eq(y, x)
//!   - transitivity: forall x, y, z. eq(x,y) && eq(y,z) ==> eq(x,z)
//!   - consistency: ne(a,b) <==> !eq(a,b) (proven by construction via ensures)
//! RESULT: Yes - all proofs auto-verified for i32 equality (reflexivity, symmetry, transitivity).

pub mod proven_partialeq {
    use vstd::prelude::*;

verus! {

    // TEST 1: Trait with explicit axioms.
    pub trait ProvenPartialEq: View + Sized {
        spec fn spec_eq(a: Self::V, b: Self::V) -> bool;

        fn eq(&self, other: &Self) -> (result: bool)
            ensures result == Self::spec_eq(self@, other@);
        
        fn ne(&self, other: &Self) -> (result: bool)
            ensures result == !Self::spec_eq(self@, other@);  // Consistency by construction
        
        proof fn proof_reflexivity()
            ensures forall |x: Self::V| Self::spec_eq(x, x);
        
        proof fn proof_symmetry()
            ensures forall |x: Self::V, y: Self::V| 
                Self::spec_eq(x, y) ==> Self::spec_eq(y, x);
        
        proof fn proof_transitivity()
            ensures forall |x: Self::V, y: Self::V, z: Self::V|
                (Self::spec_eq(x, y) && Self::spec_eq(y, z)) ==> Self::spec_eq(x, z);
    }

    // TEST 2: Implement directly for i32
    impl ProvenPartialEq for i32 {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            *self == *other
        }
         
        fn ne(&self, other: &Self) -> (result: bool) {
            *self != *other
        }
        
        proof fn proof_reflexivity() {
            // Verus should prove: forall x. x == x
        }
        
        proof fn proof_symmetry() {
            // Verus should prove: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus should prove: x == y && y == z ==> x == z
        }
    }

    // TEST 3: Use i32 impl (disambiguate from std PartialEq)
    fn test_use_i32(a: i32, b: i32) -> (result: bool)
        ensures result == (a@ == b@)
    {
        ProvenPartialEq::eq(&a, &b)
    }

    // TEST 4: Implement for a wrapper struct type
    pub struct MyInt { pub val: i32 }
    
    impl View for MyInt {
        type V = i32;
        open spec fn view(&self) -> i32 { self.val as i32 }
    }
    
    impl ProvenPartialEq for MyInt {
        open spec fn spec_eq(a: i32, b: i32) -> bool { a == b }
        
        fn eq(&self, other: &Self) -> (result: bool) {
            self.val == other.val
        }
        
        fn ne(&self, other: &Self) -> (result: bool) {
            self.val != other.val
        }
        
        proof fn proof_reflexivity() {
            // Verus should prove: forall x. x == x
        }
        
        proof fn proof_symmetry() {
            // Verus should prove: x == y ==> y == x
        }
        
        proof fn proof_transitivity() {
            // Verus should prove: x == y && y == z ==> x == z
        }
    }

    // TEST 5: Use MyInt impl
    fn test_use_myint(a: MyInt, b: MyInt) -> (result: bool)
        ensures result == (a@ == b@)
    {
        a.eq(&b)
    }

} // verus!
}

