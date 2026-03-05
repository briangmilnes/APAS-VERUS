//! Proof tests for standards::view_standard.
//!
//! Tests View trait patterns and all 6 iterator loop forms.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

test_verify_one_file! {
    #[test] view_standard_simple verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_simple_view() {
            let s = SimpleS::new(42);
            assert(s@ == 42);
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] view_standard_collection verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_collection_view() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            let c = CollectionS { seq: v };
            assert(c@.len() == 2);
            assert(c@[0] == 10);
            assert(c@[1] == 20);
        }
    } => Ok(())
}

// loop-borrow-iter
test_verify_one_file! {
    #[test] view_standard_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };

            let mut it: CollectionIter<u64> = a.iter();
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

// loop-borrow-into
test_verify_one_file! {
    #[test] view_standard_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };

            let mut it: CollectionIter<u64> = (&a).into_iter();
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

// loop-consume
test_verify_one_file! {
    #[test] view_standard_loop_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
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

// for-borrow-iter
test_verify_one_file! {
    #[test] view_standard_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };

            let it: CollectionIter<u64> = a.iter();
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
    #[test] view_standard_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };

            let it: CollectionIter<u64> = (&a).into_iter();
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

// for-consume
test_verify_one_file! {
    #[test] view_standard_for_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
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
