//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ephemeral ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_wf_adj_list, lemma_set_true_decreases_num_false};
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    // 8. traits

    pub trait CycleDetectStEphTrait {
        /// Detects if a directed graph contains a cycle
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> B;
    }

    // 9. impls

    /// Recursive DFS cycle detection using an ancestor array.
    /// Returns true if a cycle is found.
    fn dfs_check_cycle(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        ancestors: &mut ArraySeqStEphS<B>,
        vertex: N,
    ) -> (has_cycle: B)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(ancestors)@.len() == graph@.len(),
            spec_wf_adj_list(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            ancestors@.len() == old(ancestors)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if *ancestors.nth(vertex) {
            return true;
        }
        if *visited.nth(vertex) {
            return false;
        }

        assert(!old(visited)@[vertex as int]);
        let ok1 = visited.set(vertex, true);
        assert(ok1.is_ok());
        let ok2 = ancestors.set(vertex, true);
        assert(ok2.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
        }

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                ancestors@.len() == graph@.len(),
                spec_wf_adj_list(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            if dfs_check_cycle(graph, visited, ancestors, neighbor) {
                let ok3 = ancestors.set(vertex, false);
                assert(ok3.is_ok());
                return true;
            }
            i = i + 1;
        }

        let ok3 = ancestors.set(vertex, false);
        assert(ok3.is_ok());
        false
    }

    /// Detects if a directed graph contains a cycle.
    /// Returns true if a cycle exists, false otherwise.
    pub fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> B
        requires spec_wf_adj_list(graph),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut ancestors = ArraySeqStEphS::tabulate(&|_| false, n);

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                ancestors@.len() == n,
                spec_wf_adj_list(graph),
            decreases n - start,
        {
            if !*visited.nth(start) {
                if dfs_check_cycle(graph, &mut visited, &mut ancestors, start) {
                    return true;
                }
            }
            start = start + 1;
        }
        false
    }

    } // verus!
}
