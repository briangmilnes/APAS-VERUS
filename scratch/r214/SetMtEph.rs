//! Proof tests for SetMtEph (prophetic iterator model, r214).
//!
//! Loop patterns tested (see src/standards/iterator_ptt_standard.rs):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in it: s.iter()`
//!   - from_vec:   construction from a Vec

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: manual iteration, collecting the elements to prove full coverage.
test_verify_one_file! {
    #[test] set_mt_eph_loop_loop verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetMtEph::SetMtEph::*;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let mut s: SetMtEph<u64> = SetMtEph::empty(); let _ = s.insert(1); let _ = s.insert(2); let _ = s.insert(3);
            let it0 = s.iter();
            let ghost orig: Seq<u64> = into_iter_hash_keys(it0);
            let mut collected: Vec<u64> = Vec::new();
            let mut it = it0;
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == *x);
                        }
                        collected.push(*x);
                    },
                    None => {
                        assert(pos == orig.len());
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// for-iter: for loop over s.iter(), collecting the elements to prove full coverage.
test_verify_one_file! {
    #[test] set_mt_eph_for_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetMtEph::SetMtEph::*;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let mut s: SetMtEph<u64> = SetMtEph::empty(); let _ = s.insert(10); let _ = s.insert(20); let _ = s.insert(30);
            let it0 = s.iter();
            let ghost orig: Seq<u64> = into_iter_hash_keys(it0);
            let mut collected: Vec<u64> = Vec::new();
            for x in it: it0
                invariant
                    it.seq().unref() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == *it.seq()[i],
            {
                collected.push(*x);
            }
            assert(collected@ =~= orig);
        }
    } => Ok(())
}

// from_vec: Uses for-iter pattern on Vec's IntoIter
test_verify_one_file! {
    #[test] set_mt_eph_from_vec verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetMtEph::SetMtEph::*;

        fn test_from_vec()
            requires valid_key_type::<u64>()
        {
            let v: Vec<u64> = vec![1, 2, 3];
            let s: SetMtEph<u64> = SetMtEph::from_vec(v);
            
            // from_vec ensures: s@.finite() and correct elements
            assert(s@.finite());
        }
    } => Ok(())
}

