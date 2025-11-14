#![allow(unused_imports)]
use vstd::prelude::*;

verus! {

pub fn vec_length_while(s: &Vec<usize>) -> (length: usize)
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

} // verus!

