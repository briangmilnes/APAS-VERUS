/// Can I make a while over a while on arrays prove?

pub mod WhileWhile {

    use vstd::prelude::*;
    use vstd::relations::sorted_by;

    verus! {

      fn provable_array_while(a: &mut [u64]) -> (r: &[u64])
            ensures old(a).len() == r.len()
      {
          if a.len() == 0 { 
              a 
          } else {
              let mut i = 0;
              while i < a.len()
                  invariant old(a).len() == a.len(),
                  decreases a.len() - i
              {
                  a[i] = a[i];
                  i += 1;
              }
              a
          }
      }

      fn unprovable_array_while(a: &mut [u64]) -> (r: &[u64])
      {
          if a.len() == 0 { 
              a 
          } else {
              let l = a.len();
              let mut i = 0;
              while i < l
                  decreases a.len() - i
              {
//                  assert(a.len() == l); // unprovable
                   assume(a.len() == l);
                  a[i] = a[i];
//                  assert(a.len() == l); // unprovable wo the assume
                  i += 1;
              }
              a
          }
      }

      fn provable_while_while(a: &mut [u64]) -> (r: &[u64])
            ensures old(a).len() == r.len()
      {
          if a.len() == 0 { 
              a 
          } else {
              let mut i  = 0;
              while i < a.len()
                  invariant old(a).len() == a.len(),
                  decreases a.len() - i
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
                  i += 1;
              }
              a
          }
       }

    }
}

fn main() {}
