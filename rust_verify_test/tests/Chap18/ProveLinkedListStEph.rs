//! Proof tests for LinkedListStEph iterator
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... list.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&list).into_iter() ... }`
//!   - loop-consume:       `loop { ... list.into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: list.iter()`
//!   - for-borrow-into:    `for x in iter: (&list).into_iter()`
//!   - for-consume:        `for x in iter: list.into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
test_verify_one_file! {
    #[test] linkedliststeph_loop_loop verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;
        
        fn test_loop_loop() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 42);
            
            let mut it: LinkedListStEphIter<u64> = list.iter();
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

// for-iter: `for x in iter: it` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] linkedliststeph_for_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_for_iter() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 99);

            let it: LinkedListStEphIter<u64> = list.iter();
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

// loop-borrow-into: Manual iteration via (&list).into_iter()
test_verify_one_file! {
    #[test] linkedliststeph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_loop_borrow_into() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 55);

            let mut it: LinkedListStEphIter<u64> = (&list).into_iter();
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

// for-borrow-into: `for x in iter: (&list).into_iter()`
test_verify_one_file! {
    #[test] linkedliststeph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_for_borrow_into() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 77);

            let it: LinkedListStEphIter<u64> = (&list).into_iter();
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

// loop-consume: Manual consuming iteration via list.into_iter()
test_verify_one_file! {
    #[test] linkedliststeph_loop_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_loop_consume() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 33);
            let ghost orig_seq: Seq<u64> = list.seq@;

            let mut it = list.into_iter();
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

// for-consume: `for x in iter: list.into_iter()`
test_verify_one_file! {
    #[test] linkedliststeph_for_consume verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

        fn test_for_consume() {
            let list: LinkedListStEphS<u64> = LinkedListStEphS::new(3, 66);
            let ghost orig_seq: Seq<u64> = list.seq@;

            let it = list.into_iter();
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
