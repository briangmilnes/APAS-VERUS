
pub mod ArrayVal {

use vstd::prelude::*;
use vstd::relations::sorted_by;

verus! {

    fn array_lit() -> (r: [u64;1]) 
       ensures r[0] == 0,
    {
     let r = [0];
     r
    }

    fn array_mod() -> (r: [u64;1]) 
       ensures r[0] == 1,
    {
     let mut r = [0];
     r[0] = 1;
     r
    }

/*  No clone.
    fn mut_array_ref_in_out(a: &mut [u64]) -> (r: &[u64]) 
       ensures r[0] == 1,
    {
     let mut r = [0u64];
     r[0] = 1u64;
     &r.clone()
    }
*/

// This will type an insertion sort, but will it prove?
    fn mut_array_ref_in_array_ref_out(a: &mut [u64]) -> (r: &[u64]) 
       requires old(a).len() > 0,
       ensures r.len() == old(a).len(),
               r[0] == 1,
    {
     a[0] = 1;
     a
    }
 }
}

fn main(){}
