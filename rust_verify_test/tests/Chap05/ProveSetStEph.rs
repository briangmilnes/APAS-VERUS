//! Proof tests for SetStEph iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... s.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&s).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: s.iter()`
//!   - for-borrow-into:    `for x in iter: (&s).into_iter()`
//!
//! SetStEph has no IntoIterator for Self, so consume patterns are not applicable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration with loop + s.iter()
test_verify_one_file! {
    #[test] setsteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_loop_borrow_iter()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];

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
                    proof { items = items.push(*x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// loop-borrow-into: Manual iteration via (&s).into_iter()
test_verify_one_file! {
    #[test] setsteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_loop_borrow_into()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];

            let mut it: SetStEphIter<u64> = (&s).into_iter();
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
                    proof { items = items.push(*x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-borrow-iter: `for x in iter: s.iter()`
test_verify_one_file! {
    #[test] setsteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_for_borrow_iter()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];

            let it: SetStEphIter<u64> = s.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(*x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-borrow-into: `for x in iter: (&s).into_iter()`
test_verify_one_file! {
    #[test] setsteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::SetLit;

        fn test_for_borrow_into()
            requires valid_key_type::<u64>()
        {
            let s: SetStEph<u64> = SetLit![1u64, 2u64, 3u64];

            let it: SetStEphIter<u64> = (&s).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(*x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
