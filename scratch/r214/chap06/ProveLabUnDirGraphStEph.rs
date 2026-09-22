//! Proof tests for LabUnDirGraphStEph iterators (prophetic model, r214)
//!
//! Loop patterns tested (see src/standards/iterator_ptt_standard.rs):
//!   - loop-borrow-iter (vertices): `loop { ... g.vertices().iter() ... }`
//!   - loop-borrow-iter (edges):    `loop { ... g.labeled_edges().iter() ... }`
//!   - for-borrow-iter  (vertices): `for x in it: g.vertices().iter()`
//!   - for-borrow-iter  (edges):    `for x in it: g.labeled_edges().iter()`
//!
//! IntoIterator is n/a for graphs (ambiguous: vertices vs edges).

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// loop-borrow-iter (vertices)
test_verify_one_file! {
    #[test] labundirgraphsteph_loop_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;

        fn test_loop_borrow_iter_vertices()
            requires valid_key_type_LabEdge::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(1u64, 2u64, 100u64));

            let g = LabUnDirGraphStEph::from_vertices_and_labeled_edges(verts, edges);

            let it0 = g.vertices().iter();
            let ghost orig: Seq<u64> = into_iter_hash_keys(it0);
            let mut collected: Vec<u64> = Vec::new();
            let mut it = it0;
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

// loop-borrow-iter (edges)
test_verify_one_file! {
    #[test] labundirgraphsteph_loop_borrow_iter_edges verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;

        fn test_loop_borrow_iter_edges()
            requires valid_key_type_LabEdge::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(1);
            let _ = verts.insert(2);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(1u64, 2u64, 100u64));

            let g = LabUnDirGraphStEph::from_vertices_and_labeled_edges(verts, edges);

            let it0 = g.labeled_edges().iter();
            let ghost orig: Seq<LabEdge<u64, u64>> = into_iter_hash_keys(it0);
            let mut collected: Vec<LabEdge<u64, u64>> = Vec::new();
            let mut it = it0;
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

// for-borrow-iter (vertices)
test_verify_one_file! {
    #[test] labundirgraphsteph_for_borrow_iter_vertices verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;

        fn test_for_borrow_iter_vertices()
            requires valid_key_type_LabEdge::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(10u64, 20u64, 99u64));

            let g = LabUnDirGraphStEph::from_vertices_and_labeled_edges(verts, edges);

            let it0 = g.vertices().iter();
            let ghost orig: Seq<u64> = into_iter_hash_keys(it0);
            let mut collected: Vec<u64> = Vec::new();
            for x in it: it0
                invariant
                    it.seq().unref() == orig,
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

// for-borrow-iter (edges)
test_verify_one_file! {
    #[test] labundirgraphsteph_for_borrow_iter_edges verus_code! {
        use vstd::prelude::*;
        use vstd::std_specs::iter::*;
        use vstd::std_specs::hash::into_iter_hash_keys;
        use apas_verus::Types::Types::*;
        use apas_verus::Chap05::SetStEph::SetStEph::*;
        use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;

        fn test_for_borrow_iter_edges()
            requires valid_key_type_LabEdge::<u64, u64>()
        {
            let mut verts: SetStEph<u64> = SetStEph::empty();
            let _ = verts.insert(10);
            let _ = verts.insert(20);

            let mut edges: SetStEph<LabEdge<u64, u64>> = SetStEph::empty();
            let _ = edges.insert(LabEdge(10u64, 20u64, 99u64));

            let g = LabUnDirGraphStEph::from_vertices_and_labeled_edges(verts, edges);

            let it0 = g.labeled_edges().iter();
            let ghost orig: Seq<LabEdge<u64, u64>> = into_iter_hash_keys(it0);
            let mut collected: Vec<LabEdge<u64, u64>> = Vec::new();
            for x in it: it0
                invariant
                    it.seq().unref() == orig,
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
