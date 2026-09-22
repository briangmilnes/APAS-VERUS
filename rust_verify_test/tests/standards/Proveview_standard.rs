//! Proof tests for standards::view_standard.
//!
//! Tests View trait patterns and all 6 iterator loop forms (delegated
//! iteration under the verus 0.2026.09.13 prophetic iterator model; see
//! src/standards/iterators_standard.rs).

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
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<std::slice::Iter<'_, u64>> =
                VerusForLoopWrapper::new(a.iter());
            loop
                invariant
                    it.wf(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
                    IteratorSpec::decrease(&it.iter) is Some,
                    it.seq() == orig.as_ref(),
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == *it.seq()[i],
                decreases IteratorSpec::decrease(&it.iter)->0,
            {
                match it.next() {
                    Some(x) => { collected.push(*x); },
                    None => {
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// loop-borrow-into
test_verify_one_file! {
    #[test] view_standard_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<std::slice::Iter<'_, u64>> =
                VerusForLoopWrapper::new((&a).into_iter());
            loop
                invariant
                    it.wf(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
                    IteratorSpec::decrease(&it.iter) is Some,
                    it.seq() == orig.as_ref(),
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == *it.seq()[i],
                decreases IteratorSpec::decrease(&it.iter)->0,
            {
                match it.next() {
                    Some(x) => { collected.push(*x); },
                    None => {
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// loop-consume
test_verify_one_file! {
    #[test] view_standard_loop_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_loop_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<std::vec::IntoIter<u64>> =
                VerusForLoopWrapper::new(a.into_iter());
            loop
                invariant
                    it.wf(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
                    IteratorSpec::decrease(&it.iter) is Some,
                    it.seq() == orig,
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
                decreases IteratorSpec::decrease(&it.iter)->0,
            {
                match it.next() {
                    Some(x) => { collected.push(x); },
                    None => {
                        assert(collected@ =~= orig);
                        break;
                    },
                }
            }
        }
    } => Ok(())
}

// for-borrow-iter
test_verify_one_file! {
    #[test] view_standard_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: a.iter()
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

// for-borrow-into
test_verify_one_file! {
    #[test] view_standard_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: (&a).into_iter()
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

// for-consume
test_verify_one_file! {
    #[test] view_standard_for_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::view_standard::view_standard::*;

        fn test_for_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10);
            v.push(20);
            v.push(30);
            let a = CollectionS { seq: v };
            let ghost orig: Seq<u64> = a.seq@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: a.into_iter()
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
