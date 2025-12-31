//! Proof tests for MappingStEph
//!
//! Loop patterns tested (see docs/APASLoops.md):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in mapping.iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
// Uses ghost Seq accumulation to prove full coverage
test_verify_one_file! {
    #[test] mapping_st_eph_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::MappingLit;
        
        fn test_loop_loop() 
            requires valid_key_type_Pair::<u64, u64>()
        {
            // Create a functional mapping (each key maps to exactly one value)
            let m: MappingStEph<u64, u64> = MappingLit![(1u64, 10u64), (2u64, 20u64), (3u64, 30u64)];
            
            let mut it: MappingStEphIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
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

// for-iter: `for x in mapping.iter()` using ForLoopGhostIterator
// Proves full coverage via ghost Seq accumulation
test_verify_one_file! {
    #[test] mapping_st_eph_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::MappingLit;

        fn test_for_iter()
            requires valid_key_type_Pair::<u64, u64>()
        {
            // Create a functional mapping (each key maps to exactly one value)
            let m: MappingStEph<u64, u64> = MappingLit![(1u64, 100u64), (2u64, 200u64), (3u64, 300u64)];
            
            let it: MappingStEphIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();
            
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
