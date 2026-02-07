//! Proof tests for LabUnDirGraphMtEph iterators
//!
//! Loop patterns tested (see docs/APAS-VERUSIterators.rs):
//!   - loop-borrow-iter (vertices): `loop { ... g.iter_vertices() ... }`
//!   - loop-borrow-iter (edges):    `loop { ... g.iter_edges() ... }`
//!   - for-borrow-iter  (vertices): `for x in iter: g.iter_vertices()`
//!   - for-borrow-iter  (edges):    `for x in iter: g.iter_edges()`
//!
//! IntoIterator is n/a for graphs (ambiguous: vertices vs edges).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter (vertices)
test_verify_one_file! {
    #[test] labundirgraphmteph_loop_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;

        fn test_loop_borrow_iter_vertices()
            requires valid_key_type_for_lab_graph::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(1u64, 2u64, 100u64));

            let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(verts, edges);

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

// loop-borrow-iter (edges)
test_verify_one_file! {
    #[test] labundirgraphmteph_loop_borrow_iter_edges verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;

        fn test_loop_borrow_iter_edges()
            requires valid_key_type_for_lab_graph::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(1u64, 2u64, 100u64));

            let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(verts, edges);

            let mut it: SetStEphIter<LabEdge<u64, u64>> = g.iter_edges();
            let ghost iter_seq: Seq<LabEdge<u64, u64>> = it@.1;
            let ghost mut items: Seq<LabEdge<u64, u64>> = Seq::empty();

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
    #[test] labundirgraphmteph_for_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;

        fn test_for_borrow_iter_vertices()
            requires valid_key_type_for_lab_graph::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(10u64, 20u64, 99u64));

            let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(verts, edges);

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

// for-borrow-iter (edges)
test_verify_one_file! {
    #[test] labundirgraphmteph_for_borrow_iter_edges verus_code! {
        use vstd::prelude::*;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;

        fn test_for_borrow_iter_edges()
            requires valid_key_type_for_lab_graph::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(10u64, 20u64, 99u64));

            let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(verts, edges);

            let it: SetStEphIter<LabEdge<u64, u64>> = g.iter_edges();
            let ghost iter_seq: Seq<LabEdge<u64, u64>> = it@.1;
            let ghost mut items: Seq<LabEdge<u64, u64>> = Seq::empty();

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
