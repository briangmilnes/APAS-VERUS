//! Proof tests for PrimTreeSeqStPer iterator
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - loop-consume:       `loop { ... a.into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!   - for-consume:        `for x in iter: a.into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration with loop + a.iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let mut it: PrimTreeSeqStIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    prim_tree_seq_iter_invariant(&it),
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

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let mut it: PrimTreeSeqStIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    prim_tree_seq_iter_invariant(&it),
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

// loop-consume: Manual consuming iteration via a.into_iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);
            let ghost orig_seq: Seq<u64> = a.seq@;

            let mut it = a.into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            assert(iter_seq == orig_seq);

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_seq == it@.1,
                    0 <= it@.0 <= iter_seq.len(),
                decreases iter_seq.len() - it@.0,
            {
                if let Some(x) = it.next() {
                    proof {
                        items = items.push(x);
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
    #[test] primtreeseq_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let it: PrimTreeSeqStIter<u64> = a.iter();
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

// for-borrow-into: `(&a).into_iter()` using IntoIterator for &PrimTreeSeqStS
test_verify_one_file! {
    #[test] primtreeseq_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let it: PrimTreeSeqStIter<u64> = (&a).into_iter();
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

// for-consume: `for x in iter: a.into_iter()` consuming via ForLoopGhostIterator
test_verify_one_file! {
    #[test] primtreeseq_for_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);
            let ghost orig_seq: Seq<u64> = a.seq@;

            let it = a.into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            assert(iter_seq == orig_seq);

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof {
                    items = items.push(x);
                }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
