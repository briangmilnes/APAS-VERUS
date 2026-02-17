//! Proof tests for Chap19 ArraySeqStEph iterators
//!
//! Chap19 re-exports the Chap18 ArraySeqStEphS struct and adds algorithmic
//! methods (tabulate, flatten, filter, iterate). Iterator infrastructure
//! is inherited from Chap18.
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
    #[test] chap19_arrayseqsteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_loop_borrow_iter() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 42);

            let mut it: ArraySeqStEphIter<u64> = a.iter();
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
    #[test] chap19_arrayseqsteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_loop_borrow_into() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 55);

            let mut it: ArraySeqStEphIter<u64> = (&a).into_iter();
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

// loop-consume: Manual consuming iteration via a.into_iter()
test_verify_one_file! {
    #[test] chap19_arrayseqsteph_loop_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_loop_consume() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 33);
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

// for-borrow-iter: `for x in iter: a.iter()`
test_verify_one_file! {
    #[test] chap19_arrayseqsteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_for_borrow_iter() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 99);

            let it: ArraySeqStEphIter<u64> = a.iter();
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

// for-borrow-into: `for x in iter: (&a).into_iter()`
test_verify_one_file! {
    #[test] chap19_arrayseqsteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_for_borrow_into() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 77);

            let it: ArraySeqStEphIter<u64> = (&a).into_iter();
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

// for-consume: `for x in iter: a.into_iter()`
test_verify_one_file! {
    #[test] chap19_arrayseqsteph_for_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

        fn test_for_consume() {
            let a: ArraySeqStEphS<u64> = ArraySeqStEphS::new(3, 66);
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
                proof { items = items.push(x); }
            }

            assert(items =~= iter_seq);
        }
    } => Ok(())
}
