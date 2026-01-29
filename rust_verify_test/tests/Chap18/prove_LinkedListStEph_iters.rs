//! Proof tests for LinkedListStEph iterator
//!
//! Loop patterns tested (see docs/APASLoops.md):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in iter: it`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
test_verify_one_file! {
    #[test] linkedliststeph_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;
        
        fn test_loop_loop() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 42);
            
            let mut it: LinkedListStEphIter<u64> = list.iter();
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
        }
    } => Ok(())
}

// for-iter: `for x in iter: it` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] linkedliststeph_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_for_iter() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 99);
            
            let it: LinkedListStEphIter<u64> = list.iter();
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
            
            assert(items =~= iter_seq);
        }
    } => Ok(())
}
