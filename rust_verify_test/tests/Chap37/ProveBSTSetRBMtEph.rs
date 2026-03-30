//! Proof tests for BSTSetRBMtEph iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!   - loop-consume:       `loop { ... a.into_iter() ... }`
//!   - for-consume:        `for x in iter: a.into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] bstsetrbmteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_borrow_iter()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let mut it: BSTSetRBMtEphIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    bstsetrbmteph_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof { items = items.push(x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// loop-borrow-into
test_verify_one_file! {
    #[test] bstsetrbmteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_borrow_into()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let mut it: BSTSetRBMtEphIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    bstsetrbmteph_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof { items = items.push(x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-borrow-iter
test_verify_one_file! {
    #[test] bstsetrbmteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_borrow_iter()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let it: BSTSetRBMtEphIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-borrow-into
test_verify_one_file! {
    #[test] bstsetrbmteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_borrow_into()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let it: BSTSetRBMtEphIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// loop-consume
test_verify_one_file! {
    #[test] bstsetrbmteph_loop_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_loop_consume()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let mut it: BSTSetRBMtEphIter<u64> = a.into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    bstsetrbmteph_iter_invariant(&it),
                    iter_seq == it@.1,
                    it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof { items = items.push(x); }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-consume
test_verify_one_file! {
    #[test] bstsetrbmteph_for_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
        use apas_verus::BSTSetRBMtEphLit;
        use apas_verus::vstdplus::feq::feq::*;

        fn test_for_consume()
            requires obeys_feq_clone::<u64>()
        {
            let mut a: BSTSetRBMtEph<u64> = BSTSetRBMtEphLit![1u64, 2u64, 3u64];

            let it: BSTSetRBMtEphIter<u64> = a.into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
