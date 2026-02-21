// Copyright (c) 2025 Brian G. Milnes
//! Hypothesis: Can Verus verify nested for loops on arrays?

pub mod ForFor {

    use vstd::prelude::*;
    use vstd::relations::sorted_by;

    verus! {

      fn provable_array_for(a: &mut [u64]) -> (r: &[u64])
            ensures old(a).len() == r.len()
      {
          if a.len() == 0 { 
              a 
          } else {
              let l = a.len();
              for i in 0..l
                  invariant l == a.len()
              {
                  a[i] = a[i];
              }
              a
          }
      }

      fn unprovable_array_for(a: &mut [u64]) -> (r: &[u64])
      {
          if a.len() == 0 {
              a 
          } else {
              for i in 0..a.len()
                  invariant old(a).len() == a.len()
              {
//                 assert(old(a).len() == a.len());  // unprovable
                   assume(old(a).len() == a.len()); 
                   a[i] = a[i];
//                 assert(old(a).len() == a.len());  // unprovable
                   assume(old(a).len() == a.len()); 
              }
              a
          }
      }

      fn provable_for_for(a: &mut [u64]) -> (r: &[u64])
            ensures old(a).len() == r.len()
      {
          if a.len() == 0 { 
              a 
          } else {
              for i in 0..a.len()
                  invariant old(a).len() == a.len()
              {
                  let mut j :usize = i;
                  while j > 0
                      invariant old(a).len() == a.len(),
                                j < a.len(),
                      decreases j
                  {
                      a[j] = a[j];
                      j -= 1;
                  }
              }
              a
          }
       }

    }
}

fn main() {}

