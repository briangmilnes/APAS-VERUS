//! Proof tests for AVLTreeSeq iterator.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!
//! Note: AVLTreeSeq has no IntoIterator impl, so borrow-into and consume
//! patterns are not applicable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration with loop + a.iter()
test_verify_one_file! {
    #[test] avltreeseq_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::AVLTreeSeq::AVLTreeSeq::*;

        fn test_loop_borrow_iter() {
            let a: AVLTreeS<u64> = AVLTreeS::singleton(42u64);

            let mut it: AVLTreeSeqIter<u64> = a.iter();
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
                    proof {
                        items = items.push(*x);
                    }
                } else {
                    break;
                }
            }

            assert(it@.0 == iter_seq.len());
            assert(items =~= iter_seq);
        }
    } => Ok(())
}

// for-borrow-iter: `for x in iter: a.iter()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] avltreeseq_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap37::AVLTreeSeq::AVLTreeSeq::*;

        fn test_for_borrow_iter() {
            let a: AVLTreeS<u64> = AVLTreeS::singleton(42u64);

            let it: AVLTreeSeqIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(*x);
                }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
