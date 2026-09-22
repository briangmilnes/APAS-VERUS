//! Proof tests for BSTParaTreapMtEph (ParamTreap) iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!
//! BSTParaTreapMtEph exposes the ParamTreap<T> type with snapshot-based
//! iteration (owned T values) through `iter()`; its `IntoIterator for
//! &ParamTreap` is commented out (r212 form C). `iter` requires obeys_cmp
//! and view_ord_consistent for T.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration via a.iter()
test_verify_one_file! {
    #[test] bstparatreapmteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;

        fn test_loop_borrow_iter()
            requires
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let a: ParamTreap<u64> = ParamTreap::new();

            let it0 = a.iter();
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

// for-borrow-iter: `for x in iter: a.iter()`
test_verify_one_file! {
    #[test] bstparatreapmteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;

        fn test_for_borrow_iter()
            requires
                vstd::laws_cmp::obeys_cmp::<u64>(),
                view_ord_consistent::<u64>(),
        {
            let a: ParamTreap<u64> = ParamTreap::new();

            let it0 = a.iter();
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
