//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Persistent (Chapter 55, Algorithm 55.2).
//! Recursive DFS for finding reachable vertices from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false};
    use crate::Chap55::TopoSortStPer::TopoSortStPer::{spec_toposortstper_wf, spec_reachable_per};
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;
    pub struct DFSStPer;

    // 8. traits

    pub trait DFSStPerTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (reachable: AVLTreeSetStPer<N>)
            requires
                source < graph@.len(),
                spec_toposortstper_wf(graph),
            ensures
                reachable@.contains(source),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                forall|v: int| #![auto] 0 <= v < graph@.len()
                    ==> (reachable@.contains(v as usize) <==> spec_reachable_per(graph, source as int, v)),
            ;
    }

    // 9. impls

    /// Recursive DFS helper using a bool vector for termination tracking and
    /// an AVLTreeSetStPer for persistent result accumulation.
    fn dfs_recursive(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited_bool: &mut Vec<bool>,
        reachable: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> (out: AVLTreeSetStPer<N>)
        requires
            vertex < old(visited_bool)@.len(),
            old(visited_bool)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
        ensures
            visited_bool@.len() == old(visited_bool)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                ==> visited_bool@[j],
            spec_num_false(visited_bool@) <= spec_num_false(old(visited_bool)@),
        decreases spec_num_false(old(visited_bool)@),
    {
        if visited_bool[vertex] {
            return reachable;
        }
        assert(!old(visited_bool)@[vertex as int]);
        visited_bool.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited_bool)@, vertex as int);
        }
        let reachable = reachable.insert(vertex);

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        let mut reachable = reachable;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited_bool@.len() == graph@.len(),
                spec_toposortstper_wf(graph),
                forall|j: int| #![auto]
                    0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                    ==> visited_bool@[j],
                spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            reachable = dfs_recursive(graph, visited_bool, reachable, neighbor);
            i = i + 1;
        }
        reachable
    }

    impl DFSStPerTrait for DFSStPer {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// Returns the set of all vertices reachable from s.
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> AVLTreeSetStPer<N>
        {
            let n = graph.length();
            let mut visited_bool: Vec<bool> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant j <= n, visited_bool@.len() == j as int,
                decreases n - j,
            {
                visited_bool.push(false);
                j = j + 1;
            }
            let reachable = AVLTreeSetStPer::empty();
            dfs_recursive(graph, &mut visited_bool, reachable, source)
        }
    } // impl DFSStPerTrait

    } // verus!
}
