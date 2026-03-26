//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ephemeral ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_toposortsteph_wf, spec_is_dag, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq};
    use crate::Types::Types::*;

    verus! {

broadcast use vstd::seq::group_seq_axioms;

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct CycleDetectStEph;

    // 6. spec fns

    /// Bridge: for ArraySeqStEphS<bool>, view index equals spec_index.
    proof fn lemma_bool_view_eq_spec_index(a: &ArraySeqStEphS<B>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for ArraySeqStEphS<usize>, view index equals spec_index.
    proof fn lemma_usize_view_eq_spec_index(a: &ArraySeqStEphS<N>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: graph adjacency list view at vertex equals spec_index view.
    proof fn lemma_graph_view_bridge(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        neighbors: &ArraySeqStEphS<N>,
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

    pub trait CycleDetectStEphTrait {
        /// Detects if a directed graph contains a cycle (Algorithm 55.10).
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [Cost Spec 55.8, array sequences]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (has_cycle: B)
            requires
                spec_toposortsteph_wf(graph),
            ensures
                has_cycle == !spec_is_dag(graph),
            ;
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
            spec_toposortsteph_wf(graph),
        ensures
            visited@.len() == graph@.len(),
            ancestors@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());
        assert(ancestors.spec_len() == ancestors@.len());

        if *ancestors.nth(vertex) {
            return true;
        }
        if *visited.nth(vertex) {
            return false;
        }

        assert(!old(visited)@[vertex as int]);
        assert(vertex < visited.spec_len());
        assert(vertex < ancestors.spec_len());
        let ok1 = visited.set(vertex, true);
        assert(ok1.is_ok());
        let ok2 = ancestors.set(vertex, true);
        assert(ok2.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ == old(visited)@.update(vertex, true) after BOTH sets.
        // visited was set first, then ancestors was set (on a different array).
        // The visited set changed visited, not ancestors. The ancestors set changed ancestors, not visited.
        // So visited@ is still as it was after visited.set(vertex, true).
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited@.len() == old(visited)@.len());
        assert(ancestors@.len() == old(ancestors)@.len());

        // After both sets: visited still has the same length and spec_index as after visited.set.
        // ancestors.set does NOT affect visited (non-aliasing &mut).
        assert(visited.spec_len() == old(visited).spec_len());
        assert(ancestors.spec_len() == old(ancestors).spec_len());

        // Bridge visited@ to old(visited)@.update.
        assert forall|j: int| 0 <= j < visited@.len()
            implies #[trigger] visited@[j] == old(visited)@.update(vertex as int, true)[j] by {
            assert(visited@[j] == visited.spec_index(j));
            if j == vertex as int {
                assert(visited.spec_index(j) == true);
            } else {
                assert(visited.spec_index(j) == old(visited).spec_index(j));
            }
        };
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));

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
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
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
                vertex < visited.spec_len(),
                visited@.len() == graph@.len(),
                visited.spec_len() == visited@.len(),
                ancestors@.len() == graph@.len(),
                ancestors.spec_len() == ancestors@.len(),
                spec_toposortsteph_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            if dfs_check_cycle(graph, visited, ancestors, neighbor) {
                // After recursive call: length preserved via loop invariant chain.
                assert(visited@.len() == graph@.len());
                assert(ancestors@.len() == graph@.len());
                assert(vertex < ancestors.spec_len());
                let ok3 = ancestors.set(vertex, false);
                assert(ok3.is_ok());
                // After set: spec_len preserved, bridge to @.len().
                assert(ancestors@.len() == ancestors.spec_len());
                assert(ancestors@.len() == graph@.len());
                assert(visited@.len() == graph@.len());
                return true;
            }
            i = i + 1;
        }

        assert(vertex < ancestors.spec_len());
        let ok3 = ancestors.set(vertex, false);
        assert(ok3.is_ok());
        false
    }

    impl CycleDetectStEphTrait for CycleDetectStEph {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        #[verifier::external_body]
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> B
        {
            let n = graph.length();
            let mut visited = ArraySeqStEphS::tabulate(&|_x| false, n);
            let mut ancestors = ArraySeqStEphS::tabulate(&|_x| false, n);

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    visited@.len() == n,
                    ancestors@.len() == n,
                    spec_toposortsteph_wf(graph),
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
    } // impl CycleDetectStEphTrait

    } // verus!
}
