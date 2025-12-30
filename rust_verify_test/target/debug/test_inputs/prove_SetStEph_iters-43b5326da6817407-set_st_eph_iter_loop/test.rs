#![feature(fmt_internals)]

#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(deprecated)]
#![feature(allocator_api)]
#![feature(proc_macro_hygiene)]
#![feature(never_type)]
#![feature(core_intrinsics)]
#![feature(ptr_metadata)]
use verus_builtin::*;
use verus_builtin_macros::*;

::verus_builtin_macros::verus!{
use vstd::prelude::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;

fn test_loop() 
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(1);
    let _ = s.insert(2);
    let _ = s.insert(3);
    let len = s.size();  // len == s@.len() == 3
    
    let mut it = s.iter();
    let ghost iter_seq = it@.1;
    
    // Prove iter_seq.len() == len
    proof {
        // From iter ensures: iter_seq.no_duplicates()
        iter_seq.unique_seq_to_set();
        // Now: iter_seq.len() == iter_seq.to_set().len()
        
        // From iter ensures: iter_seq.map(|i,k| k@).to_set() == s@
        // For u64: k@ == k (View identity), so the mapped seq equals iter_seq
        // Therefore: iter_seq.to_set() == s@
        assert(iter_seq.map(|i: int, k: u64| k@).to_set() =~= s@);
        
        // iter_seq.len() == iter_seq.to_set().len() == s@.len() == len
        assert(iter_seq.len() == len);
    }
    
    let mut count: usize = 0;
    
    #[verifier::loop_isolation(false)]
    loop
        invariant
            count == it@.0,
            count <= len,
            iter_seq.len() == len,
            iter_invariant(&it),
            iter_seq == it@.1,
        decreases iter_seq.len() - it@.0,
    {
        let next = it.next();
        if next.is_none() {
            break;
        }
        count = count + 1;
    }
    assert(count == len);
}
}
