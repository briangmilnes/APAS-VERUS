//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false};
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 6. spec fns
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    // 6. spec fns

    /// Well-formed adjacency list for persistent graph representation.
    pub open spec fn spec_wf_adj_list_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        forall|v: int, i: int| #![auto]
            0 <= v < graph@.len() && 0 <= i < graph@[v]@.len()
            ==> graph@[v]@[i] < graph@.len()
    }

    // 8. traits

    pub trait CycleDetectStPerTrait {
        /// Detects if a directed graph contains a cycle
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> B;
    }

    // 9. impls

    /// Recursive DFS cycle detection using Vec<bool> ancestor tracking.
    /// Returns true if a cycle is found.
    fn dfs_check_cycle(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        ancestors: &mut Vec<bool>,
        vertex: N,
    ) -> (has_cycle: B)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(ancestors)@.len() == graph@.len(),
            spec_wf_adj_list_per(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            ancestors@.len() == old(ancestors)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if ancestors[vertex] {
            return true;
        }
        if visited[vertex] {
            return false;
        }

        assert(!old(visited)@[vertex as int]);
        visited.set(vertex, true);
        ancestors.set(vertex, true);
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
                spec_wf_adj_list_per(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            if dfs_check_cycle(graph, visited, ancestors, neighbor) {
                ancestors.set(vertex, false);
                return true;
            }
            i = i + 1;
        }

        ancestors.set(vertex, false);
        false
    }

    /// Detects if a directed graph contains a cycle.
    /// Returns true if a cycle exists, false otherwise.
    pub fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> B
        requires spec_wf_adj_list_per(graph),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut ancestors: Vec<bool> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant
                j <= n,
                visited@.len() == j as int,
                ancestors@.len() == j as int,
            decreases n - j,
        {
            visited.push(false);
            ancestors.push(false);
            j = j + 1;
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                ancestors@.len() == n,
                spec_wf_adj_list_per(graph),
            decreases n - start,
        {
            if !visited[start] {
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
