//! Proof tests for SetStEph
//!
//! Loop patterns tested (see docs/APASLoops.md):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in set.iter()`
//!
//! Comparing loop-loop vs for-iter:
//!   - from_vec:     loop-loop on Vec index
//!   - from_vec_for: for-iter on Vec (uses Vec's IntoIter)

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
// Uses ghost Seq accumulation to prove full coverage
test_verify_one_file! {
    #[test] set_st_eph_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        
        fn test_loop_loop() 
            requires valid_key_type::<u64>()
        {
            let mut s: SetStEph<u64> = SetStEph::empty();
            let _ = s.insert(1);
            let _ = s.insert(2);
            let _ = s.insert(3);
            
            let mut it: SetStEphIter<u64> = s.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(*x);
                    }
                } else {
                    break;
                }
            }
            
            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
            assert(iter_seq.no_duplicates());
        }
    } => Ok(())
}

// for-iter: `for x in set.iter()` using ForLoopGhostIterator
// Proves full coverage via ghost Seq accumulation
test_verify_one_file! {
    #[test] set_st_eph_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;

        fn test_for_iter()
            requires valid_key_type::<u64>()
        {
            let mut s: SetStEph<u64> = SetStEph::empty();
            let _ = s.insert(10);
            let _ = s.insert(20);
            let _ = s.insert(30);
            
            let it: SetStEphIter<u64> = s.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();
            
            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(*x);
                }
            }
            
            // After loop: iter.pos == iter.elements.len() (from ghost_ensures)
            // So items == iter_seq.take(len) == iter_seq
            assert(items =~= iter_seq);
            assert(iter_seq.no_duplicates());
        }
    } => Ok(())
}

// from_vec_for: Uses for-iter pattern on Vec's IntoIter
test_verify_one_file! {
    #[test] set_st_eph_from_vec_for verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;

        fn test_from_vec_for()
            requires valid_key_type::<u64>()
        {
            let v: Vec<u64> = vec![1, 2, 3];
            let s: SetStEph<u64> = SetStEph::from_vec_for(v);
            
            // from_vec_for ensures: s@.finite() and correct elements
            assert(s@.finite());
        }
    } => Ok(())
}

