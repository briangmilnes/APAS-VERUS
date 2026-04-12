//! Proof tests for TableStPer iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!
//! TableStPer wraps ArraySeqStPer<Pair<K,V>>, yielding &'a Pair<K,V> references.
//! Has both .iter() and IntoIterator for &'a Self.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration via a.iter()
test_verify_one_file! {
    #[test] tablestper_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap42::TableStPer::TableStPer::*;
        use apas_verus::Types::Types::*;

        fn test_loop_borrow_iter() {
            let a: TableStPer<u64, u64> = TableStPer::empty();

            let mut it: TableStPerIter<u64, u64> = a.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant_tablestper(&it),
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

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] tablestper_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap42::TableStPer::TableStPer::*;
        use apas_verus::Types::Types::*;

        fn test_loop_borrow_into() {
            let a: TableStPer<u64, u64> = TableStPer::empty();

            let mut it: TableStPerIter<u64, u64> = (&a).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_invariant_tablestper(&it),
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

// for-borrow-iter: `for x in iter: a.iter()`
test_verify_one_file! {
    #[test] tablestper_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap42::TableStPer::TableStPer::*;
        use apas_verus::Types::Types::*;

        fn test_for_borrow_iter() {
            let a: TableStPer<u64, u64> = TableStPer::empty();

            let it: TableStPerIter<u64, u64> = a.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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

// for-borrow-into: `for x in iter: (&a).into_iter()`
test_verify_one_file! {
    #[test] tablestper_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap42::TableStPer::TableStPer::*;
        use apas_verus::Types::Types::*;

        fn test_for_borrow_into() {
            let a: TableStPer<u64, u64> = TableStPer::empty();

            let it: TableStPerIter<u64, u64> = (&a).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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
