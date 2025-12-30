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

fn test_for_iter()
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(100);
    let _ = s.insert(200);
    let _ = s.insert(300);
    
    let mut count: usize = 0;
    for _x in s.iter()
    {
        count = count + 1;
    }
    // Would assert count == s.size() if this worked
}
}
