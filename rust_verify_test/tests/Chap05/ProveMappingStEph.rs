//! Proof tests for MappingStEph iterators
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter:   `loop { ... a.iter() ... }`
//!   - loop-borrow-into:   `loop { ... (&a).into_iter() ... }`
//!   - for-borrow-iter:    `for x in iter: a.iter()`
//!   - for-borrow-into:    `for x in iter: (&a).into_iter()`
//!
//! IntoIterator for Self is n/a (HashSet-backed, no vstd spec for consuming).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter: Manual iteration with loop + a.iter()
test_verify_one_file! {
    #[test] mappingsteph_loop_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::MappingLit;

        fn test_loop_borrow_iter()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let m: MappingStEph<u64, u64> = MappingLit![(1u64, 10u64), (2u64, 20u64)];

            let mut it: MappingStEphIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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

// loop-borrow-into: Manual iteration via (&a).into_iter()
test_verify_one_file! {
    #[test] mappingsteph_loop_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::MappingLit;

        fn test_loop_borrow_into()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let m: MappingStEph<u64, u64> = MappingLit![(1u64, 10u64), (2u64, 20u64)];

            let mut it: MappingStEphIter<u64, u64> = (&m).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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

// for-borrow-iter: `for x in iter: a.iter()`
test_verify_one_file! {
    #[test] mappingsteph_for_borrow_iter verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::MappingLit;

        fn test_for_borrow_iter()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let m: MappingStEph<u64, u64> = MappingLit![(10u64, 100u64), (20u64, 200u64)];

            let it: MappingStEphIter<u64, u64> = m.iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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

// for-borrow-into: `for x in iter: (&a).into_iter()`
test_verify_one_file! {
    #[test] mappingsteph_for_borrow_into verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::{Pair, valid_key_type_Pair};
        use apas_verus::Chap05::MappingStEph::MappingStEph::*;
        use apas_verus::MappingLit;

        fn test_for_borrow_into()
            requires valid_key_type_Pair::<u64, u64>()
        {
            let m: MappingStEph<u64, u64> = MappingLit![(10u64, 100u64), (20u64, 200u64)];

            let it: MappingStEphIter<u64, u64> = (&m).into_iter();
            let ghost iter_seq: Seq<Pair<u64, u64>> = it@.1;
            let ghost mut items: Seq<Pair<u64, u64>> = Seq::empty();

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
