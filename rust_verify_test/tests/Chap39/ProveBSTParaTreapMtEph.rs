//! Proof tests for BSTParaTreapMtEph (ParamTreap) iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!
//! BSTParaTreapMtEph exposes the ParamTreap<T> type with snapshot-based
//! iteration (owned T values). Only IntoIterator is implemented.
//! into_iter requires obeys_cmp and view_ord_consistent for T.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] bstparatreapmteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;

        fn test_loop_borrow_into()
            requires
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let a: ParamTreap<u64> = ParamTreap::new();

            let mut it: ParamTreapIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant_paramtreap(&it),
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

// for-borrow-into: `for x in iter: (&a).into_iter()`
test_verify_one_file! {
    #[test] bstparatreapmteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;

        fn test_for_borrow_into()
            requires
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let a: ParamTreap<u64> = ParamTreap::new();

            let it: ParamTreapIter<u64> = (&a).into_iter();
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
