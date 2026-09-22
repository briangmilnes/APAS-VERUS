//! Proof tests for PrimTreeSeqStPer iterator
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - loop-consume:       `loop { ... a.into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!   - for-consume:        `for x in iter: a.into_iter()`

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration with loop + a.iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let ghost orig: Seq<u64> = a.seq@;
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

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

            let ghost orig: Seq<u64> = a.seq@;
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

// loop-consume: Manual consuming iteration via a.into_iter()
test_verify_one_file! {
    #[test] primtreeseq_loop_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_loop_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);
            let ghost orig: Seq<u64> = a.seq@;
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

// for-borrow-iter: `for x in iter: a.iter()` using ForLoopGhostIterator
test_verify_one_file! {
    #[test] primtreeseq_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_borrow_iter() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

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

// for-borrow-into: `(&a).into_iter()` using IntoIterator for &PrimTreeSeqStS
test_verify_one_file! {
    #[test] primtreeseq_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_borrow_into() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);

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

// for-consume: `for x in iter: a.into_iter()` consuming via ForLoopGhostIterator
test_verify_one_file! {
    #[test] primtreeseq_for_consume verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

        fn test_for_consume() {
            let mut v: Vec<u64> = Vec::new();
            v.push(10); v.push(20); v.push(30);
            let a: PrimTreeSeqStS<u64> = PrimTreeSeqStS::from_vec(v);
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
