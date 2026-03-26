//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq};
    use crate::Chap55::TopoSortStPer::TopoSortStPer::{spec_is_dag_per, spec_has_edge_per, spec_is_path_per};
    use crate::Chap55::CycleDetectStEph::CycleDetectStEph::spec_in_path;
    use crate::Types::Types::*;

    verus! {

broadcast use vstd::seq::group_seq_axioms;

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
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

    // 7. proof fns

    /// If a path has a repeated vertex, the graph (persistent) is not a DAG.
    proof fn lemma_cycle_not_dag_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        dfs_path: Seq<int>,
        vertex: int,
    )
        requires
            forall|k: int| 0 <= k < dfs_path.len()
                ==> 0 <= #[trigger] dfs_path[k] < graph@.len(),
            forall|k: int| 0 <= k < dfs_path.len() - 1
                ==> #[trigger] spec_has_edge_per(graph, dfs_path[k], dfs_path[k + 1]),
            dfs_path.len() > 0 ==> spec_has_edge_per(graph, dfs_path.last(), vertex as int),
            spec_in_path(dfs_path, vertex),
            0 <= vertex < graph@.len(),
        ensures
            !spec_is_dag_per(graph),
    {
        let i = choose|i: int| 0 <= i < dfs_path.len() && dfs_path[i] == vertex;
        let cycle = dfs_path.subrange(i, dfs_path.len() as int).push(vertex);
        assert(cycle[0] == vertex);
        assert(cycle.last() == vertex);
        assert(cycle.len() >= 2);
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < cycle.len() - 1 {
                assert(cycle[k] == dfs_path[i + k]);
            }
        };
        assert forall|k: int| 0 <= k < cycle.len() - 1
            implies #[trigger] spec_has_edge_per(graph, cycle[k], cycle[k + 1]) by {
            if k < cycle.len() - 2 {
                assert(cycle[k] == dfs_path[i + k]);
                assert(cycle[k + 1] == dfs_path[i + k + 1]);
                assert(i + k >= 0 && i + k < dfs_path.len() - 1);
            } else {
                assert(cycle[k] == dfs_path[dfs_path.len() - 1]);
                assert(cycle[k + 1] == vertex);
            }
        };
        assert(spec_is_path_per(graph, cycle));
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
    /// Ghost parameter dfs_path tracks the DFS call stack for the cycle witness.
    fn dfs_check_cycle(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        ancestors: &mut Vec<bool>,
        vertex: N,
        Ghost(dfs_path): Ghost<Seq<int>>,
    ) -> (has_cycle: B)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(ancestors)@.len() == graph@.len(),
            spec_cycledetectstper_wf(graph),
            // Ghost DFS path structure:
            forall|k: int| 0 <= k < dfs_path.len()
                ==> 0 <= #[trigger] dfs_path[k] < graph@.len(),
            forall|k: int| 0 <= k < dfs_path.len() - 1
                ==> #[trigger] spec_has_edge_per(graph, dfs_path[k], dfs_path[k + 1]),
            dfs_path.len() > 0 ==> spec_has_edge_per(graph, dfs_path.last(), vertex as int),
            // Ancestors biconditional: true iff on the DFS path.
            forall|v: int| 0 <= v < old(ancestors)@.len() ==> (
                #[trigger] old(ancestors)@[v] == spec_in_path(dfs_path, v)
            ),
        ensures
            visited@.len() == graph@.len(),
            ancestors@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            has_cycle ==> !spec_is_dag_per(graph),
            !has_cycle ==> ancestors@ =~= old(ancestors)@,
        decreases spec_num_false(old(visited)@),
    {
        if ancestors[vertex] {
            // ancestors[vertex] is true → vertex is in dfs_path → cycle exists.
            proof {
                assert(old(ancestors)@[vertex as int]);
                assert(spec_in_path(dfs_path, vertex as int));
                lemma_cycle_not_dag_per(graph, dfs_path, vertex as int);
            }
            return true;
        }
        if visited[vertex] {
            return false;
        }

        assert(!old(visited)@[vertex as int]);
        assert(!old(ancestors)@[vertex as int]);
        assert(!spec_in_path(dfs_path, vertex as int));
        visited.set(vertex, true);
        ancestors.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
        // Vec::set gives direct update semantics.
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(visited@.len() == graph@.len());
        assert(ancestors@.len() == graph@.len());

        // Monotonicity.
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        // Ghost: extended path includes vertex.
        let ghost ext_path = dfs_path.push(vertex as int);

        // Prove ancestors <==> ext_path.
        proof {
            assert forall|v: int| 0 <= v < ancestors@.len()
                implies #[trigger] ancestors@[v] == spec_in_path(ext_path, v) by {
                if v == vertex as int {
                    assert(ext_path[dfs_path.len() as int] == vertex as int);
                    assert(spec_in_path(ext_path, vertex as int));
                } else {
                    assert(ancestors@[v] == old(ancestors)@[v]);
                    assert(old(ancestors)@[v] == spec_in_path(dfs_path, v));
                    if spec_in_path(dfs_path, v) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == v;
                        assert(ext_path[k] == v);
                        assert(spec_in_path(ext_path, v));
                    }
                    if spec_in_path(ext_path, v) {
                        let k = choose|k: int| 0 <= k < ext_path.len() && ext_path[k] == v;
                        if k < dfs_path.len() {
                            assert(dfs_path[k] == v);
                            assert(spec_in_path(dfs_path, v));
                        } else {
                            assert(ext_path[k] == vertex as int);
                            assert(false);
                        }
                    }
                }
            };
        }

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
                // Ghost path invariants.
                ext_path =~= dfs_path.push(vertex as int),
                forall|k: int| 0 <= k < ext_path.len()
                    ==> 0 <= #[trigger] ext_path[k] < graph@.len(),
                forall|k: int| 0 <= k < ext_path.len() - 1
                    ==> #[trigger] spec_has_edge_per(graph, ext_path[k], ext_path[k + 1]),
                forall|v: int| 0 <= v < ancestors@.len() ==> (
                    #[trigger] ancestors@[v] == spec_in_path(ext_path, v)
                ),
                // Ancestors match update state.
                ancestors@ =~= old(ancestors)@.update(vertex as int, true),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            // Edge vertex → neighbor for ghost path.
            proof {
                assert(spec_has_edge_per(graph, vertex as int, neighbor as int));
                assert(ext_path.len() == dfs_path.len() + 1);
                assert(ext_path[ext_path.len() - 1] == vertex as int);
            }
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path)) {
                // Cycle found.
                ancestors.set(vertex, false);
                return true;
            }
            // dfs_check_cycle returned false: ancestors restored.
            assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
            i = i + 1;
        }

        // Restore ancestors[vertex] = false.
        ancestors.set(vertex, false);
        // Vec::set: ancestors@ =~= old_pre_set@.update(vertex, false)
        //   == old(ancestors)@.update(vertex, true).update(vertex, false).
        // Since old(ancestors)@[vertex] was false:
        //   .update(vertex, true).update(vertex, false) =~= original.
        assert(ancestors@ =~= old(ancestors)@);
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
                    if dfs_check_cycle(graph, &mut visited, &mut ancestors, start, Ghost(Seq::empty())) {
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
