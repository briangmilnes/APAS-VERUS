//! Proof tests for standards::wrapping_iterators_standard.
//!
//! Tests all 6 loop forms for OuterS (the wrapping layer).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] wrapping_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test() {
            let a: OuterS<u64> = OuterS::new(3, 42);

            let mut it: OuterIter<u64> = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_seq == it@.1,
                    0 <= it@.0 <= iter_seq.len(),
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

// loop-borrow-into
test_verify_one_file! {
    #[test] wrapping_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test() {
            let a: OuterS<u64> = OuterS::new(3, 55);

            let mut it: OuterIter<u64> = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
                    iter_seq == it@.1,
                    0 <= it@.0 <= iter_seq.len(),
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

// for-borrow-iter
test_verify_one_file! {
    #[test] wrapping_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test() {
            let a: OuterS<u64> = OuterS::new(3, 99);

            let it: OuterIter<u64> = a.iter();
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

// for-borrow-into
test_verify_one_file! {
    #[test] wrapping_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test() {
            let a: OuterS<u64> = OuterS::new(3, 77);

            let it: OuterIter<u64> = (&a).into_iter();
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
