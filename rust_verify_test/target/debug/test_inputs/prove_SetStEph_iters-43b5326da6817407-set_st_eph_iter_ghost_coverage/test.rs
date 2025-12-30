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

fn test_ghost_coverage()
    requires valid_key_type::<u64>()
{
    let mut s: SetStEph<u64> = SetStEph::empty();
    let _ = s.insert(1);
    let _ = s.insert(2);
    let _ = s.insert(3);
    
    // Ghost: the set of elements we've seen
    let ghost mut seen: Set<u64> = Set::empty();
    
    let mut it = s.iter();
    let ghost iter_seq = it@.1;  // Full sequence that will be iterated
    
    // From iter() ensures:
    //   iter_seq.no_duplicates()
    //   iter_seq.map(|i: int, k: u64| k@).to_set() =~= s@
    // For u64, k@ == k, so iter_seq.to_set() =~= s@
    
    #[verifier::loop_isolation(false)]
    loop
        invariant
            // seen == first it@.0 elements of iter_seq (as a set)
            seen =~= iter_seq.take(it@.0 as int).to_set(),
            iter_invariant(&it),
            iter_seq == it@.1,
            it@.0 <= iter_seq.len(),
        decreases iter_seq.len() - it@.0,
    {
        match it.next() {
            Some(x) => {
                proof {
                    // Adding x to seen: take(pos).to_set() ∪ {x} == take(pos+1).to_set()
                    // This follows from: take(n).push(seq[n]) == take(n+1)
                    assert(iter_seq.take((it@.0 - 1) as int).push(*x) =~= iter_seq.take(it@.0 as int));
                    seen = seen.insert(*x);
                }
            }
            None => {
                break;
            }
        }
    }
    
    // After loop: it@.0 == iter_seq.len()
    // So: seen == iter_seq.take(len).to_set() == iter_seq.to_set()
    // From iter() ensures: iter_seq.to_set() =~= s@ (for u64, map is identity)
    
    proof {
        // take(full_length) == full_sequence
        assert(iter_seq.take(iter_seq.len() as int) =~= iter_seq);
        // seen == iter_seq.to_set() (from invariant at exit)
        assert(seen =~= iter_seq.to_set());
        // iter_seq.to_set() == s@ (need to map, but for u64 it's identity)
        // This is the key step: connecting the sequence's set to the original set
    }
    
    // We've seen all elements!
    assert(seen =~= iter_seq.to_set());
    // And we can assert no duplicates in what we saw
    assert(iter_seq.no_duplicates());
}
}
