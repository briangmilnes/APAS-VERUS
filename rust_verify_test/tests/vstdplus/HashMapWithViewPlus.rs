//! Proof tests for HashMapWithViewPlus iterator standard.
//!
//! Loop patterns tested:
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for (k, v) in iter: it`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let.
// Uses ghost Seq accumulation to prove full coverage.
test_verify_one_file! {
    #[test] hash_map_wvp_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

        fn test_loop_loop()
            requires
                vstd::std_specs::hash::obeys_key_model::<u64>(),
                apas_verus::vstdplus::feq::feq::obeys_feq_full::<u64>(),
        {
            let mut m: HashMapWithViewPlus<u64, u64> = HashMapWithViewPlus::new();
            m.insert(1, 10);
            m.insert(2, 20);
            m.insert(3, 30);

            let mut it: HashMapWithViewPlusIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<(u64, u64)> = it@.1;
            let ghost mut items: Seq<(u64, u64)> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some((k, v)) = it.next() {
                    proof {
                        items = items.push((*k, *v));
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

// for-iter: `for (k, v) in iter: it` using ForLoopGhostIterator.
// Proves full coverage via ghost Seq accumulation.
test_verify_one_file! {
    #[test] hash_map_wvp_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

        fn test_for_iter()
            requires
                vstd::std_specs::hash::obeys_key_model::<u64>(),
                apas_verus::vstdplus::feq::feq::obeys_feq_full::<u64>(),
        {
            let mut m: HashMapWithViewPlus<u64, u64> = HashMapWithViewPlus::new();
            m.insert(10, 100);
            m.insert(20, 200);
            m.insert(30, 300);

            let it: HashMapWithViewPlusIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<(u64, u64)> = it@.1;
            let ghost mut items: Seq<(u64, u64)> = Seq::empty();

            for pair in iter: it
                invariant
                    iter.kv_pairs == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push((*pair.0, *pair.1));
                }
            }

            assert(items =~= iter_seq);
            assert(iter_seq.no_duplicates());
        }
    } => Ok(())
}
