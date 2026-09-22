//! Proof tests for standards::wrapping_iterators_standard (verus 0.2026.09.13
//! prophetic iterator model).
//!
//! Pattern A (re-expose the inner std iterator) over OuterS:
//!   - loop-borrow-iter:   `loop` over `VerusForLoopWrapper::new(a.iter())`
//!   - loop-borrow-into:   `loop` over `VerusForLoopWrapper::new((&a).into_iter())`
//!   - for-borrow-iter:    `for x in it: a.iter()`
//!   - for-borrow-into:    `for x in it: (&a).into_iter()`
//! Pattern B (adaptor by delegation) over OuterS:
//!   - loop-adapted:       `loop` over `VerusForLoopWrapper::new(a.iter_adapted())`
//!   - for-adapted:        `for x in it: a.iter_adapted()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] wrapping_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_loop_borrow_iter() {
            let a: OuterS<u64> = OuterS::new(3, 42);
            let ghost orig: Seq<u64> = a@;
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
    #[test] wrapping_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_loop_borrow_into() {
            let a: OuterS<u64> = OuterS::new(3, 55);
            let ghost orig: Seq<u64> = a@;
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

// for-borrow-iter
test_verify_one_file! {
    #[test] wrapping_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_for_borrow_iter() {
            let a: OuterS<u64> = OuterS::new(3, 99);
            let ghost orig: Seq<u64> = a@;
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
    #[test] wrapping_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_for_borrow_into() {
            let a: OuterS<u64> = OuterS::new(3, 77);
            let ghost orig: Seq<u64> = a@;
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

// loop-adapted (Pattern B)
test_verify_one_file! {
    #[test] wrapping_loop_adapted verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_loop_adapted() {
            let a: OuterS<u64> = OuterS::new(3, 11);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<OuterIter<'_, u64>> =
                VerusForLoopWrapper::new(a.iter_adapted());
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

// for-adapted (Pattern B)
test_verify_one_file! {
    #[test] wrapping_for_adapted verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::wrapping_iterators_standard::wrapping_iterators_standard::*;

        fn test_for_adapted() {
            let a: OuterS<u64> = OuterS::new(3, 22);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: a.iter_adapted()
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
