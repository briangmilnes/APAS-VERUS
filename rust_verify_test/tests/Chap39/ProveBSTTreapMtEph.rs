//! Proof tests for BSTTreapMtEph iterators.
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!
//! BSTTreapMtEph uses snapshot-based iteration (owned T values).
//! Only IntoIterator is implemented (no separate .iter() method).
//! Note: BSTTreapMtEphLit! uses hashing and cannot be used in PTTs.
//! Instead, construct trees with new() + insert().

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] bsttreapmteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;

        fn test_loop_borrow_into() {
            let mut a: BSTTreapMtEph<u64> = BSTTreapMtEph::new();
            a.insert(1u64, 100u64);
            a.insert(2u64, 200u64);
            a.insert(3u64, 300u64);

            let it0 = (&a).into_iter();
            let ghost orig: Seq<u64> = vstd::std_specs::vec::into_iter_elts(it0);
            let mut collected: Vec<u64> = Vec::new();
            let mut it: std::vec::IntoIter<u64> = it0;
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

// for-borrow-into: `for x in iter: (&a).into_iter()`
test_verify_one_file! {
    #[test] bsttreapmteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap39::BSTTreapMtEph::BSTTreapMtEph::*;

        fn test_for_borrow_into() {
            let mut a: BSTTreapMtEph<u64> = BSTTreapMtEph::new();
            a.insert(1u64, 100u64);
            a.insert(2u64, 200u64);
            a.insert(3u64, 300u64);

            let it0 = (&a).into_iter();
            let ghost orig: Seq<u64> = vstd::std_specs::vec::into_iter_elts(it0);
            let mut collected: Vec<u64> = Vec::new();
            for x in it: it0
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
