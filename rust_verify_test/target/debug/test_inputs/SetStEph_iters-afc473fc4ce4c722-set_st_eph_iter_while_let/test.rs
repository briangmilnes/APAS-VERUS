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

fn test_while_let()
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(10);
    let _ = s.insert(20);
    let _ = s.insert(30);
    
    let mut it = s.iter();
    let ghost iter_seq = it@.1;
    let ghost mut items: Seq<u64> = Seq::empty();
    
    #[verifier::loop_isolation(false)]
    while let Some(x) = it.next()
        invariant
            items =~= iter_seq.take(it@.0 as int),
            iter_invariant(&it),
            iter_seq == it@.1,
            it@.0 <= iter_seq.len(),
        decreases iter_seq.len() - it@.0,
    {
        proof {
            items = items.push(*x);
        }
    }
    
    assert(it@.0 == iter_seq.len());
    assert(items =~= iter_seq);
    assert(iter_seq.no_duplicates());
}
}
