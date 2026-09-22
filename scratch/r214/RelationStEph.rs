//! Proof tests for RelationStEph (prophetic iterator model, r214).
//!
//! Loop patterns tested (see src/standards/iterator_ptt_standard.rs):
//!   - loop-loop:  `loop { match it.next() { ... } }`
//!   - for-iter:   `for x in it: r.iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: manual iteration, collecting the pairs to prove full coverage.
test_verify_one_file! {
    #[test] relation_st_eph_loop_loop verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::RelationStEph::RelationStEph::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::RelationLit;

        fn test_body()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let r: RelationStEph<u64, u64> = RelationLit![(1u64, 10u64), (2u64, 20u64), (3u64, 30u64)];
            let it0 = r.iter();
            let ghost orig: Seq<Pair<u64, u64>> = into_iter_hash_keys(it0);
            let mut collected: Vec<Pair<u64, u64>> = Vec::new();
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

// for-iter: for loop over r.iter(), collecting the pairs to prove full coverage.
test_verify_one_file! {
    #[test] relation_st_eph_for_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::RelationStEph::RelationStEph::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::RelationLit;

        fn test_body()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let r: RelationStEph<u64, u64> = RelationLit![(1u64, 100u64), (2u64, 200u64), (3u64, 300u64)];
            let it0 = r.iter();
            let ghost orig: Seq<Pair<u64, u64>> = into_iter_hash_keys(it0);
            let mut collected: Vec<Pair<u64, u64>> = Vec::new();
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
