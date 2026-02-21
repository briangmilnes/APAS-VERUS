// Copyright (c) 2025 Brian G. Milnes
//! Hypothesis: Can Verus verify for loops with bounds and array access?

pub mod ForLoops {

use vstd::prelude::*;
use vstd::relations::sorted_by;

verus! {

    fn loop_bounds() -> ()
    {
// You need to allow it to step over the bound.
      for i in 1..7
            invariant 1 <= i,
                      i <= 7,
        {
        }
    }

    fn loop_on_array(a: &mut [u64]) -> ()
    {
        for i in 0..a.len()
         invariant i <= a.len()
        {
          let key = a[i];
        }
    }
  }
}

fn main(){}
