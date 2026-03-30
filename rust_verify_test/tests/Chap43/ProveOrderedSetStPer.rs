//! Proof tests for OrderedSetStPer iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... s.iter() ... }`
//!   - for-borrow-iter:    `for x in iter: s.iter()`
//!
//! OrderedSetStPer has no IntoIterator impl, so borrow-into and consume
//! patterns are not applicable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] chap43_orderedsetstper_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap43::OrderedSetStPer::OrderedSetStPer::*;
        use vstd::laws_cmp::obeys_cmp_spec;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_loop_borrow_iter()
            requires obeys_cmp_spec::<u64>(), view_ord_consistent::<u64>(),
        {
            let s = OrderedSetStPer::singleton(1u64);
            let s = s.insert(2u64);
            let s = s.insert(3u64);

            let mut it: OrderedSetStPerIter<u64> = s.iter();
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
    #[test] chap43_orderedsetstper_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap43::OrderedSetStPer::OrderedSetStPer::*;
        use vstd::laws_cmp::obeys_cmp_spec;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_for_borrow_iter()
            requires obeys_cmp_spec::<u64>(), view_ord_consistent::<u64>(),
        {
            let s = OrderedSetStPer::singleton(1u64);
            let s = s.insert(2u64);
            let s = s.insert(3u64);

            let it: OrderedSetStPerIter<u64> = s.iter();
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
