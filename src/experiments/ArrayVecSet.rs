pub mod ArrayVecSet {

use vstd::prelude::*;
use vstd::relations::sorted_by;

verus! {
    fn for_on_array_with_use(a: &mut [usize]) -> usize
    {
        let mut last : usize = 0;
        for i in 0..a.len()
        {
            last = a[i];
        }
        last
    }

/// You have to assume a's length does not change. 
    fn for_on_array_with_sum(a: &mut [usize]) -> ()
    {
        if a.len() == 0 {
          ()
        }
        else
        {
            let l = a.len();
            for i in iter: 0..l
            {
                assume(a.len() == l);
                assert(i < a.len());
                a[i] = i;
                assert(a.len() == l);
            }
        }
    }

    fn while_on_array_with_sum(a: &mut [usize]) -> ()
    {
        let l = a.len();
        let mut i = 0;
        while i < a.len() 
         decreases a.len() - i,
        {
//            assume(a.len() == l);
//            assert(i < a.len());
            a[i] = i;
//            assert(a.len() == l);
            i += 1;
        }
    }
  }
}

fn main() {}
