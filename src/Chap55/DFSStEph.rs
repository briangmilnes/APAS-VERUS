//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Ephemeral (Chapter 55, Algorithm 55.7).
//! Recursive DFS using ephemeral arrays for efficient visited tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_wf_adj_list, lemma_set_true_decreases_num_false};
    use crate::Types::Types::*;

    verus! {

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    pub trait DFSStEphTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> AVLTreeSetStEph<N>;
    }

    /// Recursive DFS helper that marks visited vertices and inserts them into the result set.
    fn dfs_recursive(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        result: &mut AVLTreeSetStEph<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_wf_adj_list(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if *visited.nth(vertex) {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        let _ = visited.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
        }
        result.insert(vertex);

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                spec_wf_adj_list(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            dfs_recursive(graph, visited, result, neighbor);
            i = i + 1;
        }
    }

    /// Performs DFS from source vertex s on adjacency list graph G.
    /// Returns the set of all vertices reachable from s.
    pub fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> AVLTreeSetStEph<N>
        requires
            source < graph@.len(),
            spec_wf_adj_list(graph),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result = AVLTreeSetStEph::empty();
        dfs_recursive(graph, &mut visited, &mut result, source);
        result
    }

    } // verus!
}
