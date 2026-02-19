//! Proof tests for Chap19 ArraySeqMtEphSlice.
//!
//! Tests cover:
//!   - Operations: empty, singleton, new, from_vec, length, nth_cloned, slice, subseq_copy
//!   - Iterators: loop-borrow-iter, for-borrow-iter, for-borrow-into
//!
//! The iterator uses vstd's std::slice::Iter directly (no custom iterator type).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// Operations: construction + access
test_verify_one_file! {
    #[test] slice_operations verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

        fn test_empty() {
            let e: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::empty();
            assert(e.slice_wf());
            assert(e.spec_len() == 0);
            let len = e.length();
            assert(len == 0);
        }

        fn test_singleton() {
            let s: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::singleton(7);
            assert(s.slice_wf());
            assert(s.spec_len() == 1);
            assert(s.spec_index(0) == 7u64);
            let len = s.length();
            assert(len == 1);
            let v = s.nth_cloned(0);
            assert(v == 7u64);
        }

        fn test_new() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(5, 42);
            assert(a.slice_wf());
            assert(a.spec_len() == 5);
            assert(a.spec_index(0) == 42u64);
            assert(a.spec_index(4) == 42u64);
            let len = a.length();
            assert(len == 5);
            let v = a.nth_cloned(2);
            assert(v == 42u64);
        }

        fn test_from_vec() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let ghost v_view = v@;
            let fv: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::from_vec(v);
            assert(fv.slice_wf());
            assert(fv.spec_len() == 3);
            assert(fv.spec_index(0) == v_view[0]);
            assert(fv.spec_index(1) == v_view[1]);
            assert(fv.spec_index(2) == v_view[2]);
        }

        fn test_slice() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(5, 42);
            let sl = a.slice(1, 3);
            assert(sl.slice_wf());
            assert(sl.spec_len() == 3);
            assert(sl.spec_index(0) == a.spec_index(1));
            assert(sl.spec_index(0) == 42u64);
            assert(sl.spec_index(2) == 42u64);
            let len = sl.length();
            assert(len == 3);
            let v = sl.nth_cloned(1);
            assert(v == 42u64);
        }

        fn test_subseq_copy() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(5, 42);
            let sub = a.subseq_copy(2, 2);
            assert(sub.slice_wf());
            assert(sub.spec_len() == 2);
            assert(sub.spec_index(0) == a.spec_index(2));
            assert(sub.spec_index(0) == 42u64);
        }

        fn test_nested_slice() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(10, 99);
            let s1 = a.slice(2, 6);
            assert(s1.spec_len() == 6);
            let s2 = s1.slice(1, 3);
            assert(s2.spec_len() == 3);
            assert(s2.spec_index(0) == s1.spec_index(1));
            assert(s2.spec_index(0) == a.spec_index(3));
            assert(s2.spec_index(0) == 99u64);
        }
    } => Ok(())
}

// Iterator: loop-borrow-iter (manual loop + match on std::slice::Iter)
test_verify_one_file! {
    #[test] slice_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

        fn test_loop_borrow_iter() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(3, 42);

            let mut it = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            assert(iter_seq.len() == a.spec_len());
            assert(a.spec_len() == 3);

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    items =~= iter_seq.take(it@.0 as int),
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
            assert(items.len() == 3);
        }
    } => Ok(())
}

// Iterator: for-borrow-iter using vstd's ForLoopGhostIterator for slices
test_verify_one_file! {
    #[test] slice_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

        fn test_for_borrow_iter() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(3, 42);

            let it = a.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            assert(iter_seq.len() == 3);

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(*x); }
            }

            assert(items =~= iter_seq);
            assert(items.len() == 3);
        }
    } => Ok(())
}

// Iterator: for-borrow-into using IntoIterator for &ArraySeqMtEphSliceS
test_verify_one_file! {
    #[test] slice_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

        fn test_for_borrow_into() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(3, 77);

            let it = (&a).into_iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            let ghost mut items: Seq<u64> = Seq::empty();

            assert(iter_seq.len() == 3);

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    items =~= iter_seq.take(iter.pos),
                    iter.pos <= iter_seq.len(),
            {
                proof { items = items.push(*x); }
            }

            assert(items =~= iter_seq);
            assert(items.len() == 3);
        }
    } => Ok(())
}

// Iterator over a slice (subrange) — proves the chain through slice + iter
test_verify_one_file! {
    #[test] slice_iter_over_subslice verus_code! {
        use vstd::prelude::*;
        use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;

        fn test_iter_over_subslice() {
            let a: ArraySeqMtEphSliceS<u64> = ArraySeqMtEphSliceS::new(10, 55);
            let sl = a.slice(3, 4);
            assert(sl.spec_len() == 4);

            let it = sl.iter();
            let ghost iter_seq: Seq<u64> = it@.1;
            assert(iter_seq.len() == 4);

            let ghost mut count: int = 0;

            for x in iter: it
                invariant
                    iter.elements == iter_seq,
                    count == iter.pos,
                    iter.pos <= iter_seq.len(),
                    forall|j: int| #![trigger iter_seq[j]]
                        0 <= j < iter_seq.len() ==> iter_seq[j] == 55u64,
            {
                assert(*x == 55u64);
                proof { count = count + 1; }
            }

            assert(count == 4);
        }
    } => Ok(())
}
