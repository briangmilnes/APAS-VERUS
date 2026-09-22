//! Proof tests for SetStEph iterators (prophetic model, r214).
//!
//! Loop patterns tested (see src/standards/iterator_ptt_standard.rs):
//!   - loop-borrow-iter:   `loop { ... s.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&s).into_iter() ... }`
//!   - for-borrow-iter:    `for x in it: s.iter()`
//!   - for-borrow-into:    `for x in it: (&s).into_iter()`
//!
//! The iterator is `std::collections::hash_set::Iter`; `orig` is its
//! non-prophetic contents `into_iter_hash_keys`. SetStEph has no IntoIterator
//! for Self, so consume patterns are not applicable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] setsteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
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

// loop-borrow-into
test_verify_one_file! {
    #[test] setsteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
            let it0 = (&s).into_iter();
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

// for-borrow-iter
test_verify_one_file! {
    #[test] setsteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
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

// for-borrow-into
test_verify_one_file! {
    #[test] setsteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_body()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];
            let it0 = (&s).into_iter();
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
