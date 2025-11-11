use vstd::prelude::*;
use crate::experiments::verus_iterator::*;
use crate::experiments::verus_vec_iterator::*;

verus! {
/*

// Spec: sum of sequence up to index n exclusive.
pub open spec fn sum_seq_upto(s: Seq<int>, n: nat) -> int
    decreases n,
{
    if s.len() == 0 { 
        0 
    } else if n > s.len() {
        sum_seq_upto(s, (n - 1) as nat)
    } else if n == 0 {
        0
    } else {
        sum_seq_upto(s, (n - 1) as nat) + (s[n - 1 as nat] as int)
    }
}

// Sum a vector using a while loop.
pub fn sum_vec_while(v: &Vec<u64>) -> (result: Option<u64>)
//    requires
//        sum_seq_upto(v as Seq<int>, v.len() as nat) < 0x1_000_000_000,
//    ensures
//        result as int == sum_seq_upto(v@, v.len() as nat),
{
    let mut sum: u64   = 0;
    let mut i  : usize = 0;
//    assume(sum_seq_upto(v as Seq<int>, v.len() as nat) < 0x1_000_000);
    while i < v.len()
        invariant
            i <= v.len(),
//            sum as int == sum_seq_upto(v@, i as nat),
//            sum_seq_upto(v@, v.len() as nat) < 0x1_0000_0000,
        decreases v.len() - i,
    {

        match sum.checked_add(v[i]) {
            Some (i) => {}
            None     => break
        }

        sum = sum.checked_add(v[i]);
        i = i + 1;
    }
    sum
*/
}


