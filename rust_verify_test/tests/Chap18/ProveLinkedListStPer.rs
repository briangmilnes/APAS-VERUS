//! Proof tests for LinkedListStPer iterator
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... list.iter() ... }`
//!   - for-borrow-iter:    `for x in iter: list.iter()`
//!
//! IntoIterator impls have no ensures (no iter_invariant, no it@.0 == 0),
//! so borrow-into and consume patterns are not provable.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-loop: Manual iteration with loop + match/if-let
test_verify_one_file! {
    #[test] linkedliststper_loop_loop verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap18::LinkedListStPer::LinkedListStPer::*;

        fn test_loop_loop() {
            let list: LinkedListStPerS<u64> = LinkedListStPerS::new(3, 42);

            let ghost orig: Seq<u64> = list.seq@;
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::slice::Iter<'_, u64> = list.iter();
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

// for-iter: `for x in iter: it` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] linkedliststper_for_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap18::LinkedListStPer::LinkedListStPer::*;

        fn test_for_iter() {
            let list: LinkedListStPerS<u64> = LinkedListStPerS::new(3, 99);

            let ghost orig: Seq<u64> = list.seq@;
            let mut collected: Vec<u64> = Vec::new();
            for x in it: list.iter()
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
