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

fn test_while()
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(10);
    let _ = s.insert(20);
    let _ = s.insert(30);
    
    let len = s.size();
    let mut it = s.iter();
    let ghost iter_seq = it@.1;
    let ghost mut items: Seq<u64> = Seq::empty();
    
    let mut next = it.next();
    let mut count: usize = 0;
    
    #[verifier::loop_isolation(false)]
    while next.is_some()
        invariant
            iter_invariant(&it),
            iter_seq == it@.1,
            next.is_some() ==> it@.0 == count + 1,
            next.is_none() ==> it@.0 == count,
            count <= len,
            it@.0 <= iter_seq.len(),
            items =~= iter_seq.take(count as int),
        decreases iter_seq.len() - count,
    {
        proof {
            items = items.push(*next.unwrap());
        }
        count = count + 1;
        next = it.next();
    }
    
    assert(count == len);
    assert(items =~= iter_seq);
}
}
