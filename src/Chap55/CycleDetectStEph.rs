//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ephemeral ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_toposortsteph_wf, spec_is_dag, spec_has_edge, spec_is_path, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq};
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

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct CycleDetectStEph;

    // 6. spec fns

    /// Whether a value appears in a ghost integer sequence.
    pub open spec fn spec_in_path(path: Seq<int>, v: int) -> bool {
        exists|k: int| 0 <= k < path.len() && path[k] == v
    }

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

    // 7. proof fns

    /// If a path has a repeated vertex, the graph is not a DAG.
    proof fn lemma_cycle_not_dag(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        dfs_path: Seq<int>,
        vertex: int,
    )
        requires
            // dfs_path is a valid sequence of vertices with edges.
            forall|k: int| 0 <= k < dfs_path.len()
                ==> 0 <= #[trigger] dfs_path[k] < graph@.len(),
            forall|k: int| 0 <= k < dfs_path.len() - 1
                ==> #[trigger] spec_has_edge(graph, dfs_path[k], dfs_path[k + 1]),
            dfs_path.len() > 0 ==> spec_has_edge(graph, dfs_path.last(), vertex as int),
            // vertex appears in dfs_path (back edge).
            spec_in_path(dfs_path, vertex),
            0 <= vertex < graph@.len(),
        ensures
            !spec_is_dag(graph),
    {
        let i = choose|i: int| 0 <= i < dfs_path.len() && dfs_path[i] == vertex;
        let cycle = dfs_path.subrange(i, dfs_path.len() as int).push(vertex);
        // cycle[0] == dfs_path[i] == vertex, cycle.last() == vertex.
        assert(cycle[0] == vertex);
        assert(cycle.last() == vertex);
        assert(cycle.len() >= 2);
        // All vertices are valid graph vertices.
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < cycle.len() - 1 {
                assert(cycle[k] == dfs_path[i + k]);
            }
        };
        // Consecutive edges exist.
        assert forall|k: int| 0 <= k < cycle.len() - 1
            implies #[trigger] spec_has_edge(graph, cycle[k], cycle[k + 1]) by {
            if k < cycle.len() - 2 {
                assert(cycle[k] == dfs_path[i + k]);
                assert(cycle[k + 1] == dfs_path[i + k + 1]);
                assert(i + k >= 0 && i + k < dfs_path.len() - 1);
            } else {
                // Last edge: dfs_path.last() → vertex.
                assert(cycle[k] == dfs_path[dfs_path.len() - 1]);
                assert(cycle[k + 1] == vertex);
            }
        };
        assert(spec_is_path(graph, cycle));
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
    /// Ghost parameter dfs_path tracks the DFS call stack for the cycle witness.
    fn dfs_check_cycle(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        ancestors: &mut ArraySeqStEphS<B>,
        vertex: N,
        Ghost(dfs_path): Ghost<Seq<int>>,
    ) -> (has_cycle: B)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(ancestors)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            // Ghost DFS path structure:
            forall|k: int| 0 <= k < dfs_path.len()
                ==> 0 <= #[trigger] dfs_path[k] < graph@.len(),
            forall|k: int| 0 <= k < dfs_path.len() - 1
                ==> #[trigger] spec_has_edge(graph, dfs_path[k], dfs_path[k + 1]),
            dfs_path.len() > 0 ==> spec_has_edge(graph, dfs_path.last(), vertex as int),
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
            has_cycle ==> !spec_is_dag(graph),
            !has_cycle ==> ancestors@ =~= old(ancestors)@,
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());
        assert(ancestors.spec_len() == ancestors@.len());

        if *ancestors.nth(vertex) {
            // ancestors[vertex] is true → vertex is in dfs_path → cycle exists.
            proof {
                assert(old(ancestors)@[vertex as int]);
                assert(spec_in_path(dfs_path, vertex as int));
                lemma_cycle_not_dag(graph, dfs_path, vertex as int);
            }
            return true;
        }
        if *visited.nth(vertex) {
            return false;
        }

        // vertex is not an ancestor and not visited.
        assert(!old(visited)@[vertex as int]);
        assert(!old(ancestors)@[vertex as int]);
        assert(!spec_in_path(dfs_path, vertex as int));
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
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited@.len() == old(visited)@.len());
        assert(ancestors@.len() == old(ancestors)@.len());
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

        // Bridge ancestors@ after set.
        proof { lemma_bool_view_eq_spec_index(ancestors); }
        assert forall|j: int| 0 <= j < ancestors@.len()
            implies #[trigger] ancestors@[j] == old(ancestors)@.update(vertex as int, true)[j] by {
            assert(ancestors@[j] == ancestors.spec_index(j));
            if j == vertex as int {
                assert(ancestors.spec_index(j) == true);
            } else {
                assert(ancestors.spec_index(j) == old(ancestors).spec_index(j));
            }
        };
        assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));

        // Ghost: extended path includes vertex.
        let ghost ext_path = dfs_path.push(vertex as int);

        // Prove ancestors <==> ext_path.
        proof {
            assert forall|v: int| 0 <= v < ancestors@.len()
                implies #[trigger] ancestors@[v] == spec_in_path(ext_path, v) by {
                if v == vertex as int {
                    // ancestors@[vertex] == true, and vertex is at ext_path[dfs_path.len()].
                    assert(ext_path[dfs_path.len() as int] == vertex as int);
                    assert(spec_in_path(ext_path, vertex as int));
                } else {
                    // ancestors@[v] == old(ancestors)@[v] == spec_in_path(dfs_path, v).
                    // spec_in_path(ext_path, v) == spec_in_path(dfs_path, v) since the push only adds vertex != v.
                    assert(ancestors@[v] == old(ancestors)@[v]);
                    assert(old(ancestors)@[v] == spec_in_path(dfs_path, v));
                    // ext_path = dfs_path.push(vertex). For v != vertex:
                    // spec_in_path(ext_path, v) <==> spec_in_path(dfs_path, v).
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
                            // k == dfs_path.len(), ext_path[k] == vertex != v, contradiction.
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
                // Ghost path definition and invariants for recursive calls.
                ext_path =~= dfs_path.push(vertex as int),
                forall|k: int| 0 <= k < ext_path.len()
                    ==> 0 <= #[trigger] ext_path[k] < graph@.len(),
                forall|k: int| 0 <= k < ext_path.len() - 1
                    ==> #[trigger] spec_has_edge(graph, ext_path[k], ext_path[k + 1]),
                forall|v: int| 0 <= v < ancestors@.len() ==> (
                    #[trigger] ancestors@[v] == spec_in_path(ext_path, v)
                ),
                // ancestors matches old_ancestors.update(vertex, true).
                ancestors@ =~= old(ancestors)@.update(vertex as int, true),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            // Edge vertex → neighbor for ghost path last-to-vertex requires.
            proof {
                assert(spec_has_edge(graph, vertex as int, neighbor as int));
                // ext_path = dfs_path.push(vertex). Its last element is vertex.
                assert(ext_path.len() == dfs_path.len() + 1);
                assert(ext_path[ext_path.len() - 1] == vertex as int);
            }
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path)) {
                // Cycle found. dfs_check_cycle ensures !spec_is_dag(graph).
                assert(visited@.len() == graph@.len());
                assert(ancestors@.len() == graph@.len());
                assert(vertex < ancestors.spec_len());
                let ok3 = ancestors.set(vertex, false);
                assert(ok3.is_ok());
                assert(ancestors@.len() == ancestors.spec_len());
                assert(ancestors@.len() == graph@.len());
                assert(visited@.len() == graph@.len());
                return true;
            }
            // dfs_check_cycle returned false: ancestors restored.
            assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
            i = i + 1;
        }

        // After loop: no cycle found. Restore ancestors[vertex] = false.
        assert(vertex < ancestors.spec_len());
        // Loop invariant: ancestors@ =~= old(ancestors)@.update(vertex, true).
        // Establish per-element spec_index values BEFORE the set.
        proof {
            lemma_bool_view_eq_spec_index(ancestors);
            // For j != vertex: ancestors@[j] == old(ancestors)@[j] since update only changes vertex.
            assert forall|j: int| 0 <= j < ancestors@.len() && j != vertex as int
                implies #[trigger] ancestors.spec_index(j) == #[trigger] old(ancestors)@[j] by {
                assert(ancestors.spec_index(j) == ancestors@[j]);
            };
        }
        let ghost pre_set_view = ancestors@;
        let ok3 = ancestors.set(vertex, false);
        assert(ok3.is_ok());
        // After set: prove per-element equality through pre_set_view.
        proof {
            lemma_bool_view_eq_spec_index(ancestors);
            assert forall|j: int| 0 <= j < ancestors@.len()
                implies #[trigger] ancestors@[j] == old(ancestors)@[j] by {
                assert(ancestors@[j] == ancestors.spec_index(j));
                if j == vertex as int {
                    assert(ancestors.spec_index(j as int) == false);
                    assert(!old(ancestors)@[vertex as int]);
                } else {
                    // pre_set_view == old(ancestors)@.update(vertex, true).
                    // For j != vertex: pre_set_view[j] == old(ancestors)@[j].
                    assert(pre_set_view[j] == old(ancestors)@[j]);
                    // set preserves spec_index(j): post-set == pre-set.
                    // Bridge before set connected pre_set_view[j] to spec_index(j).
                    // So post-set spec_index(j) == pre_set_view[j] == old(ancestors)@[j].
                }
            };
        }
        assert(ancestors@ =~= old(ancestors)@);
        false
    }

    impl CycleDetectStEphTrait for CycleDetectStEph {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        #[verifier::external_body]
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (has_cycle: B)
        {
            let n = graph.length();
            let f_false = |_x: usize| -> (r: bool) ensures !r { false };
            let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
            let mut ancestors = ArraySeqStEphS::tabulate(&f_false, n);

            // Prove ancestors are all false initially.
            proof {
                assert forall|j: int| 0 <= j < ancestors@.len() implies !#[trigger] ancestors@[j] by {
                    assert(!ancestors.seq@[j]);
                    assert(ancestors@[j] == ancestors.seq@[j]@);
                };
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    visited@.len() == n,
                    ancestors@.len() == n,
                    spec_toposortsteph_wf(graph),
                    // Ancestors are all false at each iteration start.
                    forall|j: int| 0 <= j < ancestors@.len() ==> !#[trigger] ancestors@[j],
                decreases n - start,
            {
                proof { lemma_bool_view_eq_spec_index(&visited); }
                if !*visited.nth(start) {
                    // Prove ghost path requires for empty path.
                    proof {
                        assert forall|v: int| 0 <= v < ancestors@.len()
                            implies #[trigger] ancestors@[v] == spec_in_path(Seq::<int>::empty(), v) by {
                            assert(!ancestors@[v]);
                            assert(!spec_in_path(Seq::<int>::empty(), v));
                        };
                    }
                    if dfs_check_cycle(graph, &mut visited, &mut ancestors, start, Ghost(Seq::empty())) {
                        return true;
                    }
                    // dfs_check_cycle returned false: ancestors restored to pre-call state (all false).
                }
                start = start + 1;
            }
            false
        }
    } // impl CycleDetectStEphTrait

    } // verus!
}
