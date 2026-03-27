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

    /// An acyclic ordering of finished vertices: for every edge u→v where u is
    /// finished, v is also finished and has a strictly smaller ordering value.
    /// This is the DFS finish-time ordering property that implies DAG-ness.
    pub open spec fn spec_acyclic_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
    ) -> bool {
        (forall|v: int| #[trigger] ord.contains_key(v)
            ==> ord[v] < next_time && 0 <= v < graph@.len())
        && (forall|u: int, v: int|
            ord.contains_key(u) && #[trigger] spec_has_edge(graph, u, v)
                && 0 <= v < graph@.len()
            ==> ord.contains_key(v) && ord[u] > ord[v])
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

    /// Along any valid path where all vertices are in an acyclic ordering,
    /// the ordering strictly decreases from first to last vertex.
    proof fn lemma_path_ord_decreases(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
        path: Seq<int>,
    )
        requires
            spec_acyclic_ord(graph, ord, next_time),
            spec_is_path(graph, path),
            path.len() >= 2,
            forall|k: int| 0 <= k < path.len() ==> #[trigger] ord.contains_key(path[k]),
        ensures
            ord[path[0]] > ord[path.last()],
        decreases path.len(),
    {
        assert(spec_has_edge(graph, path[0], path[1]));
        if path.len() == 2 {
            assert(path.last() == path[1]);
        } else {
            let sub = path.subrange(1, path.len() as int);
            assert(sub.len() >= 2);
            assert(sub[0] == path[1]);
            assert(sub.last() == path.last());
            assert(spec_is_path(graph, sub)) by {
                assert(sub.len() >= 1);
                assert forall|k: int| 0 <= k < sub.len()
                    implies 0 <= #[trigger] sub[k] < graph@.len() by {
                    assert(sub[k] == path[k + 1]);
                };
                assert forall|k: int| #![trigger sub[k]]
                    0 <= k < sub.len() - 1
                    implies spec_has_edge(graph, sub[k], sub[k + 1]) by {
                    assert(sub[k] == path[k + 1]);
                    assert(sub[k + 1] == path[k + 2]);
                };
            };
            assert forall|k: int| 0 <= k < sub.len()
                implies #[trigger] ord.contains_key(sub[k]) by {
                assert(sub[k] == path[k + 1]);
            };
            lemma_path_ord_decreases(graph, ord, next_time, sub);
        }
    }

    /// Whether an ordering map is a valid DFS completion witness.
    /// Wrapping in a spec fn gives choose a usable trigger on the map argument.
    pub open spec fn spec_is_valid_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
        o: Map<int, nat>,
    ) -> bool {
        exists|n: nat| (#[trigger] spec_acyclic_ord(graph, o, n))
            && (forall|v: int| 0 <= v < visited.len()
                && #[trigger] visited[v] && !ancestors[v]
                ==> o.contains_key(v))
            && (forall|v: int| #[trigger] o.contains_key(v)
                ==> visited[v] && !ancestors[v])
            && (forall|v: int| #[trigger] prev_ord.contains_key(v)
                ==> o.contains_key(v) && o[v] == prev_ord[v])
            && n >= prev_next
    }

    /// Extract a concrete ordering witness from an existential.
    proof fn lemma_extract_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
    ) -> (result: (Map<int, nat>, nat))
        requires
            exists|o: Map<int, nat>|
                #[trigger] spec_is_valid_ord(graph, visited, ancestors, prev_ord, prev_next, o),
        ensures
            spec_acyclic_ord(graph, result.0, result.1)
            && (forall|v: int| 0 <= v < visited.len()
                && #[trigger] visited[v] && !ancestors[v]
                ==> result.0.contains_key(v))
            && (forall|v: int| #[trigger] result.0.contains_key(v)
                ==> visited[v] && !ancestors[v])
            && (forall|v: int| #[trigger] prev_ord.contains_key(v)
                ==> result.0.contains_key(v) && result.0[v] == prev_ord[v])
            && result.1 >= prev_next,
    {
        let o: Map<int, nat> = choose|o: Map<int, nat>|
            #[trigger] spec_is_valid_ord(graph, visited, ancestors, prev_ord, prev_next, o);
        let n: nat = choose|n: nat|
            (#[trigger] spec_acyclic_ord(graph, o, n))
            && (forall|v: int| 0 <= v < visited.len()
                && #[trigger] visited[v] && !ancestors[v]
                ==> o.contains_key(v))
            && (forall|v: int| #[trigger] o.contains_key(v)
                ==> visited[v] && !ancestors[v])
            && (forall|v: int| #[trigger] prev_ord.contains_key(v)
                ==> o.contains_key(v) && o[v] == prev_ord[v])
            && n >= prev_next;
        (o, n)
    }

    /// If an acyclic ordering covers all vertices, the graph is a DAG.
    proof fn lemma_acyclic_ord_implies_dag(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
    )
        requires
            spec_acyclic_ord(graph, ord, next_time),
            forall|v: int| 0 <= v < graph@.len() ==> #[trigger] ord.contains_key(v),
        ensures
            spec_is_dag(graph),
    {
        if !spec_is_dag(graph) {
            let path: Seq<int> = choose|p: Seq<int>|
                spec_is_path(graph, p) && p.len() >= 2 && p[0] == #[trigger] p.last();
            assert forall|k: int| 0 <= k < path.len()
                implies #[trigger] ord.contains_key(path[k]) by {
                assert(0 <= path[k] < graph@.len());
            };
            lemma_path_ord_decreases(graph, ord, next_time, path);
            assert(false);
        }
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
    /// Ghost parameters: dfs_path for cycle witness, ord/next_time for completeness ordering.
    fn dfs_check_cycle(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        ancestors: &mut ArraySeqStEphS<B>,
        vertex: N,
        Ghost(dfs_path): Ghost<Seq<int>>,
        Ghost(ord): Ghost<Map<int, nat>>,
        Ghost(next_time): Ghost<nat>,
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
            // Completeness ordering: acyclic ordering of finished (visited non-ancestor) vertices.
            spec_acyclic_ord(graph, ord, next_time),
            forall|v: int| 0 <= v < old(visited)@.len()
                && #[trigger] old(visited)@[v] && !old(ancestors)@[v]
                ==> ord.contains_key(v),
            forall|v: int| #[trigger] ord.contains_key(v)
                ==> old(visited)@[v] && !old(ancestors)@[v],
        ensures
            visited@.len() == graph@.len(),
            ancestors@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            has_cycle ==> !spec_is_dag(graph),
            !has_cycle ==> ancestors@ =~= old(ancestors)@,
            !has_cycle ==> visited@[vertex as int],
            !has_cycle ==> !old(ancestors)@[vertex as int],
            !has_cycle ==> exists|ord_out: Map<int, nat>|
                #[trigger] spec_is_valid_ord(graph, visited@, ancestors@, ord, next_time, ord_out),
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
            // vertex visited, not ancestor → in ord. Witness: ord itself.
            proof {
                assert(old(visited)@[vertex as int]);
                assert(!old(ancestors)@[vertex as int]);
                assert(ord.contains_key(vertex as int));
                // Prove spec_is_valid_ord with ord as the witness.
                assert(spec_is_valid_ord(graph, visited@, ancestors@, ord, next_time, ord));
            }
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

        // Ghost: track the acyclic ordering through the neighbor loop.
        let ghost mut cur_ord: Map<int, nat> = ord;
        let ghost mut cur_next: nat = next_time;

        // Prove initial ordering invariants hold after setting visited/ancestors.
        proof {
            // vertex is now visited AND ancestor → not in ord (by converse requires).
            assert(!ord.contains_key(vertex as int)) by {
                if ord.contains_key(vertex as int) {
                    // requires: ord.contains_key(v) ==> old(visited)[v] && !old(ancestors)[v]
                    assert(old(visited)@[vertex as int]); // but vertex was not visited
                    assert(false);
                }
            };
            // After update: visited non-ancestor vertices ↔ ord keys still holds.
            // For v != vertex: visited[v] && !ancestors[v] <==> old(visited)[v] && !old(ancestors)[v] <==> ord.contains_key(v).
            // For v == vertex: visited[vertex] && !ancestors[vertex] = true && false = false. ord doesn't have vertex. ✓
        }

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
                old(ancestors)@.len() == graph@.len(),
                // Ordering invariants.
                spec_acyclic_ord(graph, cur_ord, cur_next),
                forall|v: int| 0 <= v < visited@.len()
                    && #[trigger] visited@[v] && !ancestors@[v]
                    ==> cur_ord.contains_key(v),
                forall|v: int| #[trigger] cur_ord.contains_key(v)
                    ==> visited@[v] && !ancestors@[v],
                forall|v: int| #[trigger] ord.contains_key(v)
                    ==> cur_ord.contains_key(v) && cur_ord[v] == ord[v],
                cur_next >= next_time,
                // vertex itself stays visited throughout the loop.
                visited@[vertex as int],
                // All processed neighbors are visited, not old ancestors, and not vertex.
                forall|j: int| 0 <= j < i as int
                    ==> #[trigger] visited@[graph@[vertex as int][j] as int],
                forall|j: int| 0 <= j < i as int
                    ==> !old(ancestors)@[#[trigger] graph@[vertex as int][j] as int],
                forall|j: int| 0 <= j < i as int
                    ==> graph@[vertex as int][j] != vertex as int,
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
            // Prove ordering requires for recursive call.
            proof {
                // cur_ord keys are visited non-ancestor vertices.
                // ancestors biconditional holds for current ancestors state.
                // Establish: visited[v] && !ancestors[v] ==> cur_ord.contains_key(v).
                // Already a loop invariant. ✓
            }
            // Snapshot visited for monotonicity proof after the call.
            let ghost visited_pre_call = visited@;
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path), Ghost(cur_ord), Ghost(cur_next)) {
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
            // dfs_check_cycle returned false: ancestors restored, vertex visited.
            assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
            assert(visited@[neighbor as int]);
            // neighbor is not vertex (self-loop would cause back-edge detection).
            proof {
                assert(neighbor != vertex) by {
                    if neighbor == vertex {
                        // DFS(vertex) was called with ancestors[vertex] = true at callsite.
                        // ensures: !has_cycle ==> !old(ancestors at callsite)@[vertex].
                        // old(ancestors at callsite)@[vertex] = ancestors@[vertex] before call
                        //   = old(ancestors)@.update(vertex, true)[vertex] = true.
                        // So !has_cycle ==> false. But DFS returned false. Contradiction.
                    }
                };
                // neighbor is not an old ancestor.
                // DFS callee's old(ancestors) = ancestors at callsite.
                // Loop invariant: ancestors@ =~= old(ancestors)@.update(vertex, true).
                // DFS ensures: !has_cycle ==> !old_callee(ancestors)@[callee_vertex] where callee_vertex=neighbor.
                // old_callee(ancestors)@[neighbor] = callsite_ancestors@[neighbor]
                //   = old(ancestors)@.update(vertex, true)[neighbor] = old(ancestors)@[neighbor] (since neighbor != vertex).
                // DFS restored ancestors. Callee ensures: !old_callee(ancestors)@[neighbor].
                // old_callee(ancestors) = pre-call ancestors = caller's ancestors at call site.
                // Restoration: post-call ancestors@ =~= pre-call ancestors@.
                // So: !ancestors@[neighbor] (post-call).
                assert(!ancestors@[neighbor as int]);
                // Use the ancestors-ext_path biconditional to avoid seq update axiom.
                // !ancestors@[neighbor] (from DFS ensures + restoration).
                // Loop invariant: ancestors@[v] == spec_in_path(ext_path, v).
                assert(!spec_in_path(ext_path, neighbor as int));
                // ext_path = dfs_path.push(vertex), neighbor != vertex.
                assert(!spec_in_path(dfs_path, neighbor as int)) by {
                    if spec_in_path(dfs_path, neighbor as int) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == neighbor as int;
                        assert(ext_path[k] == neighbor as int);
                        assert(spec_in_path(ext_path, neighbor as int));
                    }
                };
                // Function requires biconditional: old(ancestors)@[v] == spec_in_path(dfs_path, v).
                assert(old(ancestors)@[neighbor as int] == spec_in_path(dfs_path, neighbor as int));
                assert(!old(ancestors)@[neighbor as int]);
            }
            // Extract the ordering witness via proof function (avoids choose trigger issues).
            proof {
                let (new_ord, new_next) = lemma_extract_ord(
                    graph, visited@, ancestors@, cur_ord, cur_next,
                );
                // Prove extends from ord: by transitivity.
                assert forall|v: int| #[trigger] ord.contains_key(v)
                    implies new_ord.contains_key(v) && new_ord[v] == ord[v] by {
                    assert(cur_ord.contains_key(v));
                    assert(cur_ord[v] == ord[v]);
                    assert(new_ord.contains_key(v));
                    assert(new_ord[v] == cur_ord[v]);
                };
                cur_ord = new_ord;
                cur_next = new_next;
            }
            // Help Z3 maintain loop invariants.
            assert(visited@[graph@[vertex as int][i as int] as int]);
            assert(visited@[vertex as int]);
            // Monotonicity for previously processed neighbors.
            proof {
                assert forall|k: int| 0 <= k < i as int
                    implies #[trigger] visited@[graph@[vertex as int][k] as int] by {
                    // Before the call, this was true (loop invariant).
                    assert(visited_pre_call[graph@[vertex as int][k] as int]);
                    // DFS monotonicity: pre_call[j] ==> post_call[j].
                };
            }
            i = i + 1;
        }

        // After loop: no cycle found.
        // Ghost: bridge spec_has_edge to the visited loop invariant.
        // vertex is NOT in cur_ord (it was an ancestor during the loop).
        proof {
            assert(!cur_ord.contains_key(vertex as int)) by {
                if cur_ord.contains_key(vertex as int) {
                    assert(false);
                }
            };
            // !old(ancestors)@[vertex as int] from function entry check.
            assert(!old(ancestors)@[vertex as int]);
            // For every neighbor v of vertex, v is visited and was not an old ancestor.
            // Uses loop invariants at exit (i == neighbors_len).
            assert forall|v: int| #[trigger] spec_has_edge(graph, vertex as int, v)
                && 0 <= v < graph@.len()
                implies visited@[v] && !old(ancestors)@[v] by {
                let idx = choose|idx: int| 0 <= idx < graph@[vertex as int].len()
                    && graph@[vertex as int][idx] == v;
                // From loop invariant: all neighbors visited.
                assert(visited@[graph@[vertex as int][idx] as int]);
                // From loop invariant: all neighbors not old ancestors.
                assert(!old(ancestors)@[graph@[vertex as int][idx] as int]);
                // From loop invariant: no neighbor equals vertex (no self-loop).
                assert(graph@[vertex as int][idx] != vertex as int);
            };
        }

        // Restore ancestors[vertex] = false.
        assert(vertex < ancestors.spec_len());
        proof {
            lemma_bool_view_eq_spec_index(ancestors);
            assert forall|j: int| 0 <= j < ancestors@.len() && j != vertex as int
                implies #[trigger] ancestors.spec_index(j) == #[trigger] old(ancestors)@[j] by {
                assert(ancestors.spec_index(j) == ancestors@[j]);
            };
        }
        let ghost pre_set_view = ancestors@;
        let ok3 = ancestors.set(vertex, false);
        assert(ok3.is_ok());
        proof {
            lemma_bool_view_eq_spec_index(ancestors);
            assert forall|j: int| 0 <= j < ancestors@.len()
                implies #[trigger] ancestors@[j] == old(ancestors)@[j] by {
                assert(ancestors@[j] == ancestors.spec_index(j));
                if j == vertex as int {
                    assert(ancestors.spec_index(j as int) == false);
                    assert(!old(ancestors)@[vertex as int]);
                } else {
                    assert(pre_set_view[j] == old(ancestors)@[j]);
                }
            };
        }
        assert(ancestors@ =~= old(ancestors)@);

        // Build the final ordering: add vertex with finish time cur_next.
        proof {
            let final_ord: Map<int, nat> = cur_ord.insert(vertex as int, cur_next);
            let final_next: nat = (cur_next + 1) as nat;

            // Prove spec_acyclic_ord(graph, final_ord, final_next).
            assert forall|v: int| #[trigger] final_ord.contains_key(v)
                implies final_ord[v] < final_next && 0 <= v < graph@.len() by {
                if v == vertex as int {
                    assert(final_ord[v] == cur_next);
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
            assert forall|u: int, v: int|
                final_ord.contains_key(u) && #[trigger] spec_has_edge(graph, u, v)
                    && 0 <= v < graph@.len()
                implies final_ord.contains_key(v) && final_ord[u] > final_ord[v] by {
                if u == vertex as int {
                    // Vertex→v: v is visited, !old(ancestors)[v] (proved above).
                    // No self-loop: v != vertex (proved above).
                    assert(visited@[v]);
                    assert(!old(ancestors)@[v]);
                    assert(v != vertex as int) by {
                        // No self-loop from the bridging proof above.
                    };
                    assert(!ancestors@[v]);
                    // v is visited, not ancestor (during loop: ancestors = old.update(vertex,true),
                    // for v != vertex: ancestors[v] = old(ancestors)[v] = false).
                    // After restoration: same. So v was in cur_ord.
                    assert(cur_ord.contains_key(v));
                    assert(final_ord.contains_key(v));
                    assert(final_ord[vertex as int] == cur_next);
                    assert(final_ord[v] == cur_ord[v]);
                } else {
                    assert(cur_ord.contains_key(u));
                    if v == vertex as int {
                        // Edge closure on cur_ord: u in cur_ord → v in cur_ord.
                        // But vertex NOT in cur_ord. Contradiction.
                        assert(false);
                    } else {
                        assert(final_ord[u] == cur_ord[u]);
                        assert(final_ord[v] == cur_ord[v]);
                    }
                }
            };
            assert(spec_acyclic_ord(graph, final_ord, final_next));

            // All visited non-ancestor vertices are in final_ord.
            assert forall|v: int| 0 <= v < visited@.len()
                && #[trigger] visited@[v] && !ancestors@[v]
                implies final_ord.contains_key(v) by {
                if v == vertex as int {
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
            // final_ord keys are visited non-ancestor.
            assert forall|v: int| #[trigger] final_ord.contains_key(v)
                implies visited@[v] && !ancestors@[v] by {
                if v == vertex as int {
                    assert(visited@[vertex as int]);
                    assert(!ancestors@[vertex as int]);
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
            // Extends from ord.
            assert forall|v: int| #[trigger] ord.contains_key(v)
                implies final_ord.contains_key(v) && final_ord[v] == ord[v] by {
                assert(cur_ord.contains_key(v));
                assert(cur_ord[v] == ord[v]);
                assert(v != vertex as int) by {
                    if v == vertex as int {
                        assert(old(visited)@[v]);
                        assert(false);
                    }
                };
                assert(final_ord[v] == cur_ord[v]);
            };
            // Prove spec_is_valid_ord to satisfy the ensures existential.
            assert(spec_is_valid_ord(graph, visited@, ancestors@, ord, next_time, final_ord));
        }
        false
    }

    impl CycleDetectStEphTrait for CycleDetectStEph {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (has_cycle: B)
        {
            let n = graph.length();
            let f_false = |_x: usize| -> (r: bool) ensures !r { false };
            let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
            let mut ancestors = ArraySeqStEphS::tabulate(&f_false, n);

            // Prove ancestors and visited are all false initially.
            proof {
                assert forall|j: int| 0 <= j < ancestors@.len() implies !#[trigger] ancestors@[j] by {
                    assert(!ancestors.seq@[j]);
                    assert(ancestors@[j] == ancestors.seq@[j]@);
                };
                assert forall|j: int| 0 <= j < visited@.len() implies !#[trigger] visited@[j] by {
                    assert(!visited.seq@[j]);
                    assert(visited@[j] == visited.seq@[j]@);
                };
            }

            // Ghost: acyclic ordering accumulator.
            let ghost mut cur_ord: Map<int, nat> = Map::empty();
            let ghost mut cur_next: nat = 0;

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
                    // All vertices before start are visited.
                    forall|j: int| 0 <= j < start as int ==> #[trigger] visited@[j],
                    // Ordering invariants.
                    spec_acyclic_ord(graph, cur_ord, cur_next),
                    forall|v: int| 0 <= v < visited@.len()
                        && #[trigger] visited@[v] ==> cur_ord.contains_key(v),
                    forall|v: int| #[trigger] cur_ord.contains_key(v)
                        ==> visited@[v],
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
                        // Ordering requires: visited && !ancestors ==> in ord.
                        // Since ancestors all false: visited ==> in ord. ✓ (loop invariant)
                        // Converse: in ord ==> visited && !ancestors. ✓
                        assert forall|v: int| #[trigger] cur_ord.contains_key(v)
                            implies visited@[v] && !ancestors@[v] by {
                            assert(visited@[v]);
                            assert(!ancestors@[v]);
                        };
                    }
                    let ghost visited_pre = visited@;
                    if dfs_check_cycle(graph, &mut visited, &mut ancestors, start, Ghost(Seq::empty()), Ghost(cur_ord), Ghost(cur_next)) {
                        return true;
                    }
                    // dfs_check_cycle returned false: ancestors restored, start visited.
                    // Monotonicity: vertices visited before are still visited.
                    proof {
                        assert forall|j: int| 0 <= j < start as int
                            implies #[trigger] visited@[j] by {
                            assert(visited_pre[j]); // was true (loop invariant)
                            // DFS monotonicity: pre_call[j] ==> post_call[j].
                        };
                    }
                    // Extract the ordering via proof function.
                    proof {
                        let (new_ord, new_next) = lemma_extract_ord(
                            graph, visited@, ancestors@, cur_ord, cur_next,
                        );
                        // Since ancestors are all false: visited && !ancestors <==> visited.
                        assert forall|v: int| 0 <= v < visited@.len()
                            && #[trigger] visited@[v]
                            implies new_ord.contains_key(v) by {
                            assert(!ancestors@[v]);
                        };
                        assert forall|v: int| #[trigger] new_ord.contains_key(v)
                            implies visited@[v] by {};
                        cur_ord = new_ord;
                        cur_next = new_next;
                    }
                }
                // After if: start is visited (either was already, or DFS visited it).
                proof {
                    lemma_bool_view_eq_spec_index(&visited);
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            // After loop: all vertices visited, ordering covers all vertices → DAG.
            proof {
                assert forall|v: int| 0 <= v < graph@.len()
                    implies #[trigger] cur_ord.contains_key(v) by {
                    assert(visited@[v]);
                };
                lemma_acyclic_ord_implies_dag(graph, cur_ord, cur_next);
            }
            false
        }
    } // impl CycleDetectStEphTrait

    } // verus!
}
