//! Proof tests for WeightedDirGraphStEphUsize iterators
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter (vertices): `loop { ... g.iter_vertices() ... }`
//!   - loop-borrow-iter (arcs):     `loop { ... g.iter_arcs() ... }`
//!   - for-borrow-iter  (vertices): `for x in iter: g.iter_vertices()`
//!   - for-borrow-iter  (arcs):     `for x in iter: g.iter_arcs()`
//!
//! IntoIterator is n/a for graphs (ambiguous: vertices vs arcs).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter (vertices)
test_verify_one_file! {
    #[test] weighteddirgraphstephusize_loop_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
        use apas_verus::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::*;

        fn test_loop_borrow_iter_vertices()
            requires valid_key_type_LabEdge::<u64, usize>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut arcs: SetStEph<LabEdge<u64, usize>> = SetStEph::empty();
            let _ = arcs.insert(LabEdge(1u64, 2u64, 5usize));

            let g: WeightedDirGraphStEphUsize<u64> =
                LabDirGraphStEph::from_vertices_and_labeled_arcs(verts, arcs);

            let mut it: SetStEphIter<u64> = g.iter_vertices();
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

// loop-borrow-iter (arcs)
test_verify_one_file! {
    #[test] weighteddirgraphstephusize_loop_borrow_iter_arcs verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
        use apas_verus::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::*;

        fn test_loop_borrow_iter_arcs()
            requires valid_key_type_LabEdge::<u64, usize>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut arcs: SetStEph<LabEdge<u64, usize>> = SetStEph::empty();
            let _ = arcs.insert(LabEdge(1u64, 2u64, 5usize));

            let g: WeightedDirGraphStEphUsize<u64> =
                LabDirGraphStEph::from_vertices_and_labeled_arcs(verts, arcs);

            let mut it: SetStEphIter<LabEdge<u64, usize>> = g.iter_arcs();
            let ghost iter_seq: Seq<LabEdge<u64, usize>> = it@.1;
            let ghost mut items: Seq<LabEdge<u64, usize>> = Seq::empty();

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

// for-borrow-iter (vertices)
test_verify_one_file! {
    #[test] weighteddirgraphstephusize_for_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
        use apas_verus::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::*;

        fn test_for_borrow_iter_vertices()
            requires valid_key_type_LabEdge::<u64, usize>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut arcs: SetStEph<LabEdge<u64, usize>> = SetStEph::empty();
            let _ = arcs.insert(LabEdge(10u64, 20u64, 9usize));

            let g: WeightedDirGraphStEphUsize<u64> =
                LabDirGraphStEph::from_vertices_and_labeled_arcs(verts, arcs);

            let it: SetStEphIter<u64> = g.iter_vertices();
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

// for-borrow-iter (arcs)
test_verify_one_file! {
    #[test] weighteddirgraphstephusize_for_borrow_iter_arcs verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
        use apas_verus::Chap06::WeightedDirGraphStEphUsize::WeightedDirGraphStEphUsize::*;

        fn test_for_borrow_iter_arcs()
            requires valid_key_type_LabEdge::<u64, usize>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut arcs: SetStEph<LabEdge<u64, usize>> = SetStEph::empty();
            let _ = arcs.insert(LabEdge(10u64, 20u64, 9usize));

            let g: WeightedDirGraphStEphUsize<u64> =
                LabDirGraphStEph::from_vertices_and_labeled_arcs(verts, arcs);

            let it: SetStEphIter<LabEdge<u64, usize>> = g.iter_arcs();
            let ghost iter_seq: Seq<LabEdge<u64, usize>> = it@.1;
            let ghost mut items: Seq<LabEdge<u64, usize>> = Seq::empty();

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
