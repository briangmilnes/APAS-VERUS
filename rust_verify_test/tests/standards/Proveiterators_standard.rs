//! Proof tests for standards::iterators_standard (delegated iteration under
//! the verus 0.2026.09.13 prophetic iterator model).
//!
//! Loop patterns tested:
//!   - loop-borrow-iter:   `loop` over `a.iter()` on its own `next()`
//!   - loop-borrow-into:   `loop` over `(&a).into_iter()` on its own `next()`
//!   - loop-consume:       `loop` over `a.into_iter()` on its own `next()`
//!   - for-borrow-iter:    `for x in it: a.iter()`
//!   - for-borrow-into:    `for x in it: (&a).into_iter()`
//!   - for-consume:        `for x in it: a.into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter
test_verify_one_file! {
    #[test] iterators_standard_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_loop_borrow_iter() {
            let a: ExampleS<u64> = ExampleS::new(3, 42);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::slice::Iter<'_, u64> = a.iter();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == *x);
                        }
                        collected.push(*x);
                    },
                    None => {
                        assert(pos == orig.len());
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
    #[test] iterators_standard_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_loop_borrow_into() {
            let a: ExampleS<u64> = ExampleS::new(3, 55);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::slice::Iter<'_, u64> = (&a).into_iter();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == *x);
                        }
                        collected.push(*x);
                    },
                    None => {
                        assert(pos == orig.len());
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
    #[test] iterators_standard_loop_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_loop_consume() {
            let a: ExampleS<u64> = ExampleS::new(3, 33);
            let ghost orig: Seq<u64> = a@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = a.into_iter();
            let ghost mut pos: int = 0;
            loop
                invariant
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= orig.len(),
                    IteratorSpec::remaining(&it).len() == orig.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> #[trigger] IteratorSpec::remaining(&it)[i] == orig[pos + i],
                    collected.len() == pos,
                    forall|i: int| 0 <= i < collected.len()
                        ==> #[trigger] collected@[i] == orig[i],
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    Some(x) => {
                        proof {
                            pos = pos + 1;
                            assert(orig[old_pos] == x);
                        }
                        collected.push(x);
                    },
                    None => {
                        assert(pos == orig.len());
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
    #[test] iterators_standard_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_for_borrow_iter() {
            let a: ExampleS<u64> = ExampleS::new(3, 99);
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
    #[test] iterators_standard_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_for_borrow_into() {
            let a: ExampleS<u64> = ExampleS::new(3, 77);
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

// for-consume
test_verify_one_file! {
    #[test] iterators_standard_for_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::standards::iterators_standard::iterators_standard::*;

        fn test_for_consume() {
            let a: ExampleS<u64> = ExampleS::new(3, 66);
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
