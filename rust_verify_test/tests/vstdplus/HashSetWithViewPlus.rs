//! Proof tests for HashSetWithViewPlus iterator standard.
//!
//! Loop patterns tested:
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in iter: it`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let.
// Uses ghost Seq accumulation to prove full coverage.
test_verify_one_file! {
    #[test] hash_set_wvp_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;

        fn test_loop_loop()
            requires
                vstd::std_specs::hash::obeys_key_model::<u64>(),
                apas_verus::vstdplus::feq::feq::obeys_feq_full::<u64>(),
        {
            let mut s: HashSetWithViewPlus<u64> = HashSetWithViewPlus::new();
            let _ = s.insert(1);
            let _ = s.insert(2);
            let _ = s.insert(3);

            let mut it: HashSetWithViewPlusIter<u64> = s.iter();
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

// for-iter: `for x in iter: it` using ForLoopGhostIterator.
// Proves full coverage via ghost Seq accumulation.
test_verify_one_file! {
    #[test] hash_set_wvp_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;

        fn test_for_iter()
            requires
                vstd::std_specs::hash::obeys_key_model::<u64>(),
                apas_verus::vstdplus::feq::feq::obeys_feq_full::<u64>(),
        {
            let mut s: HashSetWithViewPlus<u64> = HashSetWithViewPlus::new();
            let _ = s.insert(10);
            let _ = s.insert(20);
            let _ = s.insert(30);

            let it: HashSetWithViewPlusIter<u64> = s.iter();
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
            assert(iter_seq.no_duplicates());
        }
    } => Ok(())
}
