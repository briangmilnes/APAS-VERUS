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

fn test_for_borrowing()
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(1000);
    let _ = s.insert(2000);
    let _ = s.insert(3000);
    
    let mut count: usize = 0;
    for _x in &s  // Borrowing - s still usable
    {
        count = count + 1;
    }
    // Would assert count == s.size() if this worked
}
}
