//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq};
    use crate::Chap55::TopoSortStPer::TopoSortStPer::spec_is_dag_per;
    use crate::Types::Types::*;

    verus! {

broadcast use vstd::seq::group_seq_axioms;

    // Table of Contents
    // 1. module
    // 2. imports
    // 6. spec fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;
    pub struct CycleDetectStPer;

    // 6. spec fns

    /// Well-formed adjacency list for persistent graph representation.
    pub open spec fn spec_cycledetectstper_wf(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        forall|v: int, i: int|
            0 <= v < graph@.len() && 0 <= i < graph@[v].len()
            ==> (#[trigger] graph@[v][i]) < graph@.len()
    }

    /// Bridge: for ArraySeqStPerS<usize>, view index equals spec_index.
    proof fn lemma_usize_per_view_eq_spec_index(a: &ArraySeqStPerS<N>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: persistent graph adjacency list view at vertex equals spec_index view.
    proof fn lemma_graph_per_view_bridge(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        neighbors: &ArraySeqStPerS<N>,
        vertex: int,
    )
        requires
            0 <= vertex < graph@.len(),
            *neighbors == graph.spec_index(vertex),
        ensures
            neighbors@ =~= graph@[vertex],
    {
    }

    // 8. traits

    pub trait CycleDetectStPerTrait {
        /// Detects if a directed graph contains a cycle (Algorithm 55.10)
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (has_cycle: B)
            requires
                spec_cycledetectstper_wf(graph),
            ensures
                has_cycle == !spec_is_dag_per(graph),
            ;
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
            spec_cycledetectstper_wf(graph),
        ensures
            visited@.len() == graph@.len(),
            ancestors@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
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
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
        // Vec::set gives us visited@ =~= old(visited)@.update(vertex, true).
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(visited@.len() == graph@.len());
        assert(ancestors@.len() == graph@.len());

        // Monotonicity.
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        // Bridge neighbors to graph view.
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_per_view_bridge(graph, neighbors, vertex as int); }
        assert(neighbors@ =~= graph@[vertex as int]);

        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len as int == neighbors.spec_len(),
                neighbors_len == graph@[vertex as int].len(),
                neighbors@ =~= graph@[vertex as int],
                *neighbors == graph.spec_index(vertex as int),
                (vertex as int) < graph@.len(),
                visited@.len() == graph@.len(),
                ancestors@.len() == graph@.len(),
                spec_cycledetectstper_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            if dfs_check_cycle(graph, visited, ancestors, neighbor) {
                ancestors.set(vertex, false);
                return true;
            }
            i = i + 1;
        }

        ancestors.set(vertex, false);
        false
    }

    impl CycleDetectStPerTrait for CycleDetectStPer {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        #[verifier::external_body]
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> B
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
                    spec_cycledetectstper_wf(graph),
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
    } // impl CycleDetectStPerTrait

    } // verus!
}
