//! Proof tests for standards::prophetic_iterators_standard.
//!
//! Exercises the verus 0.2026.05.21 prophetic iterator model (PR #2163) across
//! every loop form an APAS collection must support. The Vec-backed `ExampleS`
//! covers delegated iteration; the from-scratch `CountIter` covers custom
//! iteration.
//!
//! Loop forms tested:
//!   - for-borrow-iter:  `for x in it: a.iter()`
//!   - for-borrow-into:  `for x in it: &a`
//!   - for-consume:      `for x in it: a.into_iter()`
//!   - loop-borrow:      manual `loop` over `a.iter()`
//!   - loop-consume:     manual `loop` over `a.into_iter()`
//!   - for-custom:       `for x in it: count_iter(..)`   (custom iteration)
//!   - loop-custom:      manual `loop` over `count_iter(..)`  (custom iteration)

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// for-borrow-iter: `for x in it: a.iter()`
test_verify_one_file! {
    #[test] prophetic_iterators_standard_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_for_borrow_iter() {
            let a: ExampleS<u64> = ExampleS::new(4, 42);
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

// for-borrow-into: `for x in it: &a`
test_verify_one_file! {
    #[test] prophetic_iterators_standard_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_for_borrow_into() {
            let a: ExampleS<u64> = ExampleS::new(4, 55);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: &a
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

// for-consume: `for x in it: a.into_iter()`
test_verify_one_file! {
    #[test] prophetic_iterators_standard_for_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_for_consume() {
            let a: ExampleS<u64> = ExampleS::new(4, 66);
            let ghost orig: Seq<u64> = a@;
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

// loop-borrow: manual `loop` over `a.iter()`. The measure is the underlying
// iterator's non-prophetic `decrease()`; the conclusion is drawn before break.
test_verify_one_file! {
    #[test] prophetic_iterators_standard_loop_borrow verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_loop_borrow() {
            let a: ExampleS<u64> = ExampleS::new(4, 77);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<'_, std::slice::Iter<'_, u64>> =
                VerusForLoopWrapper::new(a.iter(), Ghost(None));
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

// loop-consume: manual `loop` over `a.into_iter()`.
test_verify_one_file! {
    #[test] prophetic_iterators_standard_loop_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_loop_consume() {
            let a: ExampleS<u64> = ExampleS::new(4, 88);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<'_, std::vec::IntoIter<u64>> =
                VerusForLoopWrapper::new(a.into_iter(), Ghost(None));
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

// for-custom: `for x in it: count_iter(..)` over the from-scratch custom
// iterator. Collected vector equals 0..n.
test_verify_one_file! {
    #[test] prophetic_iterators_standard_for_custom verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_for_custom(n: u64) {
            let mut collected: Vec<u64> = Vec::new();
            for x in it: count_iter(0, n)
                invariant
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
            {
                collected.push(x);
            }
            assert(collected@ =~= Seq::new(n as nat, |i: int| i as u64));
        }
    } => Ok(())
}

// loop-custom: manual `loop` over the from-scratch custom iterator.
test_verify_one_file! {
    #[test] prophetic_iterators_standard_loop_custom verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::prophetic_iterators_standard::prophetic_iterators_standard::*;

        fn test_loop_custom(n: u64) {
            let mut collected: Vec<u64> = Vec::new();
            let mut it: VerusForLoopWrapper<'_, CountIter> =
                VerusForLoopWrapper::new(count_iter(0, n), Ghost(None));
            loop
                invariant
                    it.wf(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it.iter),
                    IteratorSpec::decrease(&it.iter) is Some,
                    it.seq() =~= Seq::new(n as nat, |i: int| i as u64),
                    collected.len() == it.index(),
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == it.seq()[i],
                decreases IteratorSpec::decrease(&it.iter)->0,
            {
                match it.next() {
                    Some(x) => { collected.push(x); },
                    None => {
                        assert(collected@ =~= Seq::new(n as nat, |i: int| i as u64));
                        break;
                    },
                }
            }
        }
    } => Ok(())
}
