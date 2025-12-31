// Copyright (c) 2025 Brian G. Milnes
#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

/* Fails as it should.
#[verifier::external_body] does not stop this even.
pub fn int_variable_fails() -> (result: int)
    ensures result == 5
{
    let i: int = 5 as int;
    i
}
*/

// This function SUCCEEDS even though it uses Vec<int>:
#[verifier::external_body]
pub fn vec_int_succeeds(s: Vec<int>) -> (result: usize)
    ensures result == s@.len()
{
    s.len()
}

// BUG: This function signature is ALLOWED (Vec<int> and int parameters),
// but the body FAILS because you cannot compare int values in executable code.
// Error: "The Verus types 'nat' and 'int' can only be used in ghost code"
// Uncomment to see the error:

pub fn vec_int_mem_verifies(s: Vec<int>, elt: int) -> (result: bool)
{
    for i in 0..s.len() {
        if s[i] == elt {
            return true;
        }
    }
    false
}

}
