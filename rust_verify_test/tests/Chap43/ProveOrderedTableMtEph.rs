//! Proof tests for OrderedTableMtEph iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... t.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&t).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: t.iter()`
//!   - for-borrow-into:    `for x in iter: (&t).into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] chap43_orderedtablemteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
        use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::spec_pair_key_determines_order;

        fn test_loop_borrow_iter()
            requires
                vstd::laws_cmp::obeys_cmp::<Pair<u64, u64>>(),
                view_ord_consistent::<Pair<u64, u64>>(),
                spec_pair_key_determines_order::<u64, u64>(),
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let t = OrderedTableMtEph::singleton(1u64, 10u64);

            let mut it: OrderedTableMtEphIter<u64, u64> = t.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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

// loop-borrow-into
test_verify_one_file! {
    #[test] chap43_orderedtablemteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
        use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::spec_pair_key_determines_order;

        fn test_loop_borrow_into()
            requires
                vstd::laws_cmp::obeys_cmp::<Pair<u64, u64>>(),
                view_ord_consistent::<Pair<u64, u64>>(),
                spec_pair_key_determines_order::<u64, u64>(),
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let t = OrderedTableMtEph::singleton(1u64, 10u64);

            let mut it: OrderedTableMtEphIter<u64, u64> = (&t).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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
    #[test] chap43_orderedtablemteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
        use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::spec_pair_key_determines_order;

        fn test_for_borrow_iter()
            requires
                vstd::laws_cmp::obeys_cmp::<Pair<u64, u64>>(),
                view_ord_consistent::<Pair<u64, u64>>(),
                spec_pair_key_determines_order::<u64, u64>(),
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let t = OrderedTableMtEph::singleton(1u64, 10u64);

            let it: OrderedTableMtEphIter<u64, u64> = t.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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
    #[test] chap43_orderedtablemteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
        use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::spec_pair_key_determines_order;

        fn test_for_borrow_into()
            requires
                vstd::laws_cmp::obeys_cmp::<Pair<u64, u64>>(),
                view_ord_consistent::<Pair<u64, u64>>(),
                spec_pair_key_determines_order::<u64, u64>(),
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let t = OrderedTableMtEph::singleton(1u64, 10u64);

            let it: OrderedTableMtEphIter<u64, u64> = (&t).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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
