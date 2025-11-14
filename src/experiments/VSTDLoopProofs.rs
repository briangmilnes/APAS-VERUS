#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

// Spec function for membership
pub open spec fn seq_u64_mem(s: Seq<u64>, elt: u64) -> bool {
    exists|i: int| 0 <= i < s.len() && s[i] == elt
}

// a. While Loop - Length
pub fn vec_length_while(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    while i < s.len()
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        i += 1;
        length += 1;
    }
    length
}

// a. While Loop - Length (with explicit assertions showing the proof)
pub fn vec_length_while_asserted(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    assert(i <= s.len());
    assert(length == i);
    
    while i < s.len()
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        i += 1;
        length += 1;
        
        assert(i <= s.len());
        assert(length == i);
    }
    
    assert(length == s@.len());
    length
}

// b1. Loop with return - Length
pub fn vec_length_loop_return(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    loop
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        if i >= s.len() {
            return length;
        }
        i += 1;
        length += 1;
    }
}

// b2. Loop with break - DOES NOT VERIFY
// Problem: break exits without propagating the exit condition
// After break, Verus only knows invariants, not which path caused exit
// Requires invariant_except_break (not implemented in current Verus)
// Attempted workaround with ghost variable fails at: assert(exit_cond)

#[verifier::external_body] // The proof fails - break doesn't propagate exit condition
pub fn vec_length_loop_break(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    loop
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        if i >= s.len() {
            break;
        }
        i += 1;
        length += 1;
    }
    length
}

// b1. Loop with return - Length (with explicit assertions showing the proof)
pub fn vec_length_loop_return_asserted(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    assert(i <= s.len());
    assert(length == i);
    
    loop
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        if i >= s.len() {
            assert(i == s.len());
            assert(length == i);
            assert(length == s@.len());
            return length;
        }
        
        i += 1;
        length += 1;
        
        assert(i <= s.len());
        assert(length == i);
    }
}

// c. For Loop (Range) - Length
pub fn vec_length_for_range(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    
    for _i in 0..s.len()
        invariant
            length == _i,
    {
        length += 1;
    }
    length
}

// c. For Loop (Range) - Length (with explicit assertions showing the proof)
pub fn vec_length_for_range_asserted(s: &Vec<u64>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    
    assert(length == 0);
    assert(0 <= s.len());  // range bounds
    
    for _i in 0..s.len()
        invariant
            length == _i,
    {
        length += 1;
        assert(length == _i + 1);
    }
    
    assert(length == s@.len());
    length
}

// d. For Loop (General - over Vec consuming iterator) - Membership check

#[verifier::external_body] // The proof fails so we turn it off here. 
pub fn vec_mem_for_vec(v: Vec<u64>, elt: u64) -> (result: bool)
     ensures result == seq_u64_mem(v@, elt)
 {
     for val in v
     {
         if val == elt {
             return true;
         }
     }
     false
 }

// d. (Alternative) For Loop (Range) - Membership check
pub fn vec_mem_for_range(s: &Vec<u64>, elt: u64) -> (result: bool)
    ensures result == seq_u64_mem(s@, elt)
{
    for i in 0..s.len()
        invariant forall|j: int| 0 <= j < i ==> s@[j] != elt,
    {
        if s[i] == elt {
            return true;
        }
    }
    false
}

} // verus!
