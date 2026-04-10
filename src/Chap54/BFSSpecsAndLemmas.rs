//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Shared BFS specifications and lemmas over abstract Seq types.
//! Used by BFSStEph, BFSStPer, BFSMtEph, BFSMtPer to avoid duplicating
//! the same graph-wf, distances-bounded, and parents-bounded predicates.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups

//		Section 1. module

pub mod BFSSpecsAndLemmas {


    //		Section 2. imports

    use vstd::prelude::*;

    verus! {

    //		Section 6. spec fns


    /// Sentinel value: distance not yet assigned.
    pub const UNREACHABLE: usize = usize::MAX;

    /// Sentinel value: no parent in BFS tree.
    pub const NO_PARENT: usize = usize::MAX;

    /// Well-formed adjacency list: every neighbor index is a valid vertex.
    pub open spec fn spec_bfs_graph_wf(graph: Seq<Seq<usize>>) -> bool {
        forall|u: int, i: int|
            0 <= u < graph.len() && 0 <= i < graph[u].len()
            ==> #[trigger] graph[u][i] < graph.len()
    }

    /// Every distance entry is either UNREACHABLE or a valid vertex index less than n.
    pub open spec fn spec_bfs_distances_bounded(distances: Seq<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < distances.len() ==>
            (#[trigger] distances[j]) == UNREACHABLE || distances[j] < n
    }

    /// Every parent entry is either NO_PARENT or a valid vertex index less than n.
    pub open spec fn spec_bfs_parents_bounded(parents: Seq<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < parents.len() ==>
            (#[trigger] parents[j]) == NO_PARENT || parents[j] < n
    }

    //		Section 7. proof fns/broadcast groups


    /// A sequence where every entry equals UNREACHABLE satisfies spec_bfs_distances_bounded.
    pub proof fn lemma_bfs_all_unreachable(distances: Seq<usize>, n: int)
        requires
            distances.len() == n,
            forall|i: int| 0 <= i < n ==> #[trigger] distances[i] == UNREACHABLE,
        ensures
            spec_bfs_distances_bounded(distances, n),
    {}

    /// A sequence where every entry equals NO_PARENT satisfies spec_bfs_parents_bounded.
    pub proof fn lemma_bfs_all_no_parent(parents: Seq<usize>, n: int)
        requires
            parents.len() == n,
            forall|i: int| 0 <= i < n ==> #[trigger] parents[i] == NO_PARENT,
        ensures
            spec_bfs_parents_bounded(parents, n),
    {}

    /// A point update to a bounded-distances sequence preserves the bounded property.
    pub proof fn lemma_bfs_update_preserves_bounded(
        distances: Seq<usize>,
        old_distances: Seq<usize>,
        v: int,
        new_val: usize,
        n: int,
    )
        requires
            distances.len() == n,
            old_distances.len() == n,
            0 <= v < n,
            new_val < n,
            distances[v] == new_val,
            forall|j: int| 0 <= j < n && j != v ==>
                #[trigger] distances[j] == old_distances[j],
            spec_bfs_distances_bounded(old_distances, n),
        ensures
            spec_bfs_distances_bounded(distances, n),
    {
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < distances.len()
        implies
            distances[j] == UNREACHABLE || distances[j] < n
        by {
            if j == v { } else { }
        }
    }

    /// A copy of a bounded-distances sequence (same values, same length) is also bounded.
    pub proof fn lemma_bfs_copy_preserves_bounded(
        original: Seq<usize>,
        copy: Seq<usize>,
        n: int,
    )
        requires
            spec_bfs_distances_bounded(original, n),
            copy.len() == original.len(),
            forall|i: int| 0 <= i < original.len() ==>
                #[trigger] copy[i] == original[i],
        ensures
            spec_bfs_distances_bounded(copy, n),
    {
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < copy.len()
        implies
            copy[j] == UNREACHABLE || copy[j] < n
        by {}
    }

    /// A point update to a bounded-parents sequence preserves the bounded property.
    pub proof fn lemma_bfs_update_preserves_parents_bounded(
        parents: Seq<usize>,
        old_parents: Seq<usize>,
        v: int,
        new_val: usize,
        n: int,
    )
        requires
            parents.len() == n,
            old_parents.len() == n,
            0 <= v < n,
            new_val < n,
            parents[v] == new_val,
            forall|j: int| 0 <= j < n && j != v ==>
                #[trigger] parents[j] == old_parents[j],
            spec_bfs_parents_bounded(old_parents, n),
        ensures
            spec_bfs_parents_bounded(parents, n),
    {
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < parents.len()
        implies
            parents[j] == NO_PARENT || parents[j] < n
        by {
            if j == v { } else { }
        }
    }

    /// A copy of a bounded-parents sequence (same values, same length) is also bounded.
    pub proof fn lemma_bfs_copy_preserves_parents_bounded(
        original: Seq<usize>,
        copy: Seq<usize>,
        n: int,
    )
        requires
            spec_bfs_parents_bounded(original, n),
            copy.len() == original.len(),
            forall|i: int| 0 <= i < original.len() ==>
                #[trigger] copy[i] == original[i],
        ensures
            spec_bfs_parents_bounded(copy, n),
    {
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < copy.len()
        implies
            copy[j] == NO_PARENT || copy[j] < n
        by {}
    }

    } // verus!
}
