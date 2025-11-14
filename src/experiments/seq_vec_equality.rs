#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[verifier::external_body] // The proof fails!
pub fn vec_copy_generic_equality<T: Copy>(s: &Vec<T>) -> (result: Vec<T>)
    ensures
        result@ == s@,
        result@ =~= s@,
{
    s.clone()
}

pub fn vec_clone_usize_equality(s: &Vec<usize>) -> (result: Vec<usize>)
    ensures
        result@ == s@,
        result@ =~= s@,
{
    s.clone()
}

}
