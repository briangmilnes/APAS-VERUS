//! Proof tests for AVLTreeSeqStPer iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!
//! AVLTreeSeqStPer has no IntoIterator for Self, so consume patterns are not applicable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] avltreeseqstper_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;

        fn test_loop_borrow_iter() {
            let a: AVLTreeSeqStPerS<u64> = AVLTreeSeqStPerS::singleton(42u64);

            let it0 = a.iter();
            let ghost orig: Seq<u64> = it0.elts();
            let mut collected: Vec<u64> = Vec::new();
            let mut it: AVLTreeSeqStPerIter<'_, u64> = it0;
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

// r212 form C: module `AVLTreeSeqStPer` defines no `IntoIterator for &AVLTreeSeqStPerS`; the pattern's `IntoIterator` impl is commented out
/*
// loop-borrow-into
test_verify_one_file! {
    #[test] avltreeseqstper_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;

        fn test_loop_borrow_into() {
            let a: AVLTreeSeqStPerS<u64> = AVLTreeSeqStPerS::singleton(42u64);

            let mut it: AVLTreeSeqStPerIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    avltreeseqstper_iter_invariant(&it),
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
*/

// for-borrow-iter
test_verify_one_file! {
    #[test] avltreeseqstper_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;

        fn test_for_borrow_iter() {
            let a: AVLTreeSeqStPerS<u64> = AVLTreeSeqStPerS::singleton(42u64);

            let it0 = a.iter();
            let ghost orig: Seq<u64> = it0.elts();
            let mut collected: Vec<u64> = Vec::new();
            for x in it: it0
                invariant
                    it.seq() == orig.as_ref(),
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

// r212 form C: module `AVLTreeSeqStPer` defines no `IntoIterator for &AVLTreeSeqStPerS`; the pattern's `IntoIterator` impl is commented out
/*
// for-borrow-into
test_verify_one_file! {
    #[test] avltreeseqstper_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;

        fn test_for_borrow_into() {
            let a: AVLTreeSeqStPerS<u64> = AVLTreeSeqStPerS::singleton(42u64);

            let it: AVLTreeSeqStPerIter<u64> = (&a).into_iter();
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
*/
