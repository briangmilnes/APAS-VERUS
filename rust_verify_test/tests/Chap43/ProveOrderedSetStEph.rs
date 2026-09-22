//! Proof tests for Chap43 OrderedSetStEph iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... s.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&s).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: s.iter()`
//!   - for-borrow-into:    `for x in iter: (&s).into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] chap43_orderedsetsteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap43::OrderedSetStEph::OrderedSetStEph::*;
        use vstd::laws_cmp::obeys_cmp;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_loop_borrow_iter()
            requires obeys_cmp::<u64>(), view_ord_consistent::<u64>(),
        {
            let mut s = OrderedSetStEph::singleton(1u64);
            s.insert(2u64);
            s.insert(3u64);

            let it0 = s.iter();
            let ghost orig: Seq<u64> = vstd::std_specs::vec::into_iter_elts(it0);
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = it0;
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
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
                            assert(orig[old_pos] == x);
                        }
                        collected.push(x);
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
// r212 form C: module `OrderedSetStEph` defines no `IntoIterator for &OrderedSetStEph`; the pattern's `IntoIterator` impl is commented out
// (src/experiments/intoiter_form_c_no_impl.rs), so this test is too.
/*
test_verify_one_file! {
    #[test] chap43_orderedsetsteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap43::OrderedSetStEph::OrderedSetStEph::*;
        use vstd::laws_cmp::obeys_cmp;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_loop_borrow_into()
            requires obeys_cmp::<u64>(), view_ord_consistent::<u64>(),
        {
            let mut s = OrderedSetStEph::singleton(1u64);
            s.insert(2u64);
            s.insert(3u64);

            let mut it: OrderedSetStEphIter<u64> = (&s).into_iter();
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
*/

// for-borrow-iter
test_verify_one_file! {
    #[test] chap43_orderedsetsteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap43::OrderedSetStEph::OrderedSetStEph::*;
        use vstd::laws_cmp::obeys_cmp;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_for_borrow_iter()
            requires obeys_cmp::<u64>(), view_ord_consistent::<u64>(),
        {
            let mut s = OrderedSetStEph::singleton(1u64);
            s.insert(2u64);
            s.insert(3u64);

            let it0 = s.iter();
            let ghost orig: Seq<u64> = vstd::std_specs::vec::into_iter_elts(it0);
            let mut collected: Vec<u64> = Vec::new();
            for x in it: it0
                invariant
                    it.seq() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
            {
                collected.push(x);
            }
            assert(collected@ =~= orig);
        }
    } => Ok(())
}

// for-borrow-into
// r212 form C: module `OrderedSetStEph` defines no `IntoIterator for &OrderedSetStEph`; the pattern's `IntoIterator` impl is commented out
// (src/experiments/intoiter_form_c_no_impl.rs), so this test is too.
/*
test_verify_one_file! {
    #[test] chap43_orderedsetsteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap43::OrderedSetStEph::OrderedSetStEph::*;
        use vstd::laws_cmp::obeys_cmp;
        use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;

        fn test_for_borrow_into()
            requires obeys_cmp::<u64>(), view_ord_consistent::<u64>(),
        {
            let mut s = OrderedSetStEph::singleton(1u64);
            s.insert(2u64);
            s.insert(3u64);

            let it: OrderedSetStEphIter<u64> = (&s).into_iter();
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
*/
