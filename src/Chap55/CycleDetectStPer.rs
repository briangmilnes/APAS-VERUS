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

    /// An acyclic ordering of finished vertices (persistent variant).
    pub open spec fn spec_acyclic_ord_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
    ) -> bool {
        (forall|v: int| #[trigger] ord.contains_key(v)
            ==> ord[v] < next_time && 0 <= v < graph@.len())
        && (forall|u: int, v: int|
            ord.contains_key(u) && #[trigger] spec_has_edge_per(graph, u, v)
                && 0 <= v < graph@.len()
            ==> ord.contains_key(v) && ord[u] > ord[v])
    }

    /// Whether an ordering map is a valid DFS completion witness (persistent variant).
    pub open spec fn spec_is_valid_ord_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
        o: Map<int, nat>,
    ) -> bool {
        exists|n: nat| (#[trigger] spec_acyclic_ord_per(graph, o, n))
            && (forall|v: int| 0 <= v < visited.len()
                && #[trigger] visited[v] && !ancestors[v]
                ==> o.contains_key(v))
            && (forall|v: int| #[trigger] o.contains_key(v)
                ==> visited[v] && !ancestors[v])
            && (forall|v: int| #[trigger] prev_ord.contains_key(v)
                ==> o.contains_key(v) && o[v] == prev_ord[v])
            && n >= prev_next
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

    /// Extract a concrete ordering witness from an existential (persistent variant).
    proof fn lemma_extract_ord_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
    ) -> (result: (Map<int, nat>, nat))
        requires
            exists|o: Map<int, nat>|
                #[trigger] spec_is_valid_ord_per(graph, visited, ancestors, prev_ord, prev_next, o),
        ensures
            spec_acyclic_ord_per(graph, result.0, result.1)
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
            #[trigger] spec_is_valid_ord_per(graph, visited, ancestors, prev_ord, prev_next, o);
        let n: nat = choose|n: nat|
            (#[trigger] spec_acyclic_ord_per(graph, o, n))
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

    /// Along any path in an acyclic ordering, ordering strictly decreases (persistent variant).
    proof fn lemma_path_ord_decreases_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
        path: Seq<int>,
    )
        requires
            spec_acyclic_ord_per(graph, ord, next_time),
            spec_is_path_per(graph, path),
            path.len() >= 2,
            forall|k: int| 0 <= k < path.len() ==> #[trigger] ord.contains_key(path[k]),
        ensures
            ord[path[0]] > ord[path.last()],
        decreases path.len(),
    {
        assert(spec_has_edge_per(graph, path[0], path[1]));
        if path.len() == 2 {
            assert(path.last() == path[1]);
        } else {
            let sub = path.subrange(1, path.len() as int);
            assert(sub.len() >= 2);
            assert(sub[0] == path[1]);
            assert(sub.last() == path.last());
            assert(spec_is_path_per(graph, sub)) by {
                assert(sub.len() >= 1);
                assert forall|k: int| 0 <= k < sub.len()
                    implies 0 <= #[trigger] sub[k] < graph@.len() by {
                    assert(sub[k] == path[k + 1]);
                };
                assert forall|k: int| #![trigger sub[k]]
                    0 <= k < sub.len() - 1
                    implies spec_has_edge_per(graph, sub[k], sub[k + 1]) by {
                    assert(sub[k] == path[k + 1]);
                    assert(sub[k + 1] == path[k + 2]);
                };
            };
            assert forall|k: int| 0 <= k < sub.len()
                implies #[trigger] ord.contains_key(sub[k]) by {
                assert(sub[k] == path[k + 1]);
            };
            lemma_path_ord_decreases_per(graph, ord, next_time, sub);
        }
    }

    /// If an acyclic ordering covers all vertices, the graph is a DAG (persistent variant).
    proof fn lemma_acyclic_ord_implies_dag_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        ord: Map<int, nat>,
        next_time: nat,
    )
        requires
            spec_acyclic_ord_per(graph, ord, next_time),
            forall|v: int| 0 <= v < graph@.len() ==> #[trigger] ord.contains_key(v),
        ensures
            spec_is_dag_per(graph),
    {
        if !spec_is_dag_per(graph) {
            let path: Seq<int> = choose|p: Seq<int>|
                spec_is_path_per(graph, p) && p.len() >= 2 && p[0] == #[trigger] p.last();
            assert forall|k: int| 0 <= k < path.len()
                implies #[trigger] ord.contains_key(path[k]) by {
                assert(0 <= path[k] < graph@.len());
            };
            lemma_path_ord_decreases_per(graph, ord, next_time, path);
            assert(false);
        }
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
    /// Ghost parameters: dfs_path for cycle witness, ord/next_time for completeness ordering.
    fn dfs_check_cycle(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        ancestors: &mut Vec<bool>,
        vertex: N,
        Ghost(dfs_path): Ghost<Seq<int>>,
        Ghost(ord): Ghost<Map<int, nat>>,
        Ghost(next_time): Ghost<nat>,
    ) -> (has_cycle: B)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(ancestors)@.len() == graph@.len(),
            spec_cycledetectstper_wf(graph),
            forall|k: int| 0 <= k < dfs_path.len()
                ==> 0 <= #[trigger] dfs_path[k] < graph@.len(),
            forall|k: int| 0 <= k < dfs_path.len() - 1
                ==> #[trigger] spec_has_edge_per(graph, dfs_path[k], dfs_path[k + 1]),
            dfs_path.len() > 0 ==> spec_has_edge_per(graph, dfs_path.last(), vertex as int),
            forall|v: int| 0 <= v < old(ancestors)@.len() ==> (
                #[trigger] old(ancestors)@[v] == spec_in_path(dfs_path, v)
            ),
            spec_acyclic_ord_per(graph, ord, next_time),
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
            has_cycle ==> !spec_is_dag_per(graph),
            !has_cycle ==> ancestors@ =~= old(ancestors)@,
            !has_cycle ==> visited@[vertex as int],
            !has_cycle ==> !old(ancestors)@[vertex as int],
            !has_cycle ==> exists|ord_out: Map<int, nat>|
                #[trigger] spec_is_valid_ord_per(graph, visited@, ancestors@, ord, next_time, ord_out),
        decreases spec_num_false(old(visited)@),
    {
        if ancestors[vertex] {
            proof {
                assert(old(ancestors)@[vertex as int]);
                assert(spec_in_path(dfs_path, vertex as int));
                lemma_cycle_not_dag_per(graph, dfs_path, vertex as int);
            }
            return true;
        }
        if visited[vertex] {
            proof {
                assert(old(visited)@[vertex as int]);
                assert(!old(ancestors)@[vertex as int]);
                assert(ord.contains_key(vertex as int));
                assert(spec_is_valid_ord_per(graph, visited@, ancestors@, ord, next_time, ord));
            }
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
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(visited@.len() == graph@.len());
        assert(ancestors@.len() == graph@.len());

        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        let ghost ext_path = dfs_path.push(vertex as int);

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
            // vertex not in ord (was unvisited).
            assert(!ord.contains_key(vertex as int)) by {
                if ord.contains_key(vertex as int) {
                    assert(old(visited)@[vertex as int]);
                    assert(false);
                }
            };
        }

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_per_view_bridge(graph, neighbors, vertex as int); }
        assert(neighbors@ =~= graph@[vertex as int]);

        let ghost mut cur_ord: Map<int, nat> = ord;
        let ghost mut cur_next: nat = next_time;

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
                ext_path =~= dfs_path.push(vertex as int),
                forall|k: int| 0 <= k < ext_path.len()
                    ==> 0 <= #[trigger] ext_path[k] < graph@.len(),
                forall|k: int| 0 <= k < ext_path.len() - 1
                    ==> #[trigger] spec_has_edge_per(graph, ext_path[k], ext_path[k + 1]),
                forall|v: int| 0 <= v < ancestors@.len() ==> (
                    #[trigger] ancestors@[v] == spec_in_path(ext_path, v)
                ),
                ancestors@ =~= old(ancestors)@.update(vertex as int, true),
                old(ancestors)@.len() == graph@.len(),
                // Ordering invariants.
                spec_acyclic_ord_per(graph, cur_ord, cur_next),
                forall|v: int| 0 <= v < visited@.len()
                    && #[trigger] visited@[v] && !ancestors@[v]
                    ==> cur_ord.contains_key(v),
                forall|v: int| #[trigger] cur_ord.contains_key(v)
                    ==> visited@[v] && !ancestors@[v],
                forall|v: int| #[trigger] ord.contains_key(v)
                    ==> cur_ord.contains_key(v) && cur_ord[v] == ord[v],
                cur_next >= next_time,
                visited@[vertex as int],
                forall|j: int| 0 <= j < i as int
                    ==> #[trigger] visited@[graph@[vertex as int][j] as int],
                forall|j: int| 0 <= j < i as int
                    ==> !old(ancestors)@[#[trigger] graph@[vertex as int][j] as int],
                forall|j: int| 0 <= j < i as int
                    ==> graph@[vertex as int][j] != vertex as int,
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            proof {
                assert(spec_has_edge_per(graph, vertex as int, neighbor as int));
                assert(ext_path.len() == dfs_path.len() + 1);
                assert(ext_path[ext_path.len() - 1] == vertex as int);
            }
            let ghost visited_pre_call = visited@;
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path), Ghost(cur_ord), Ghost(cur_next)) {
                ancestors.set(vertex, false);
                return true;
            }
            assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
            assert(visited@[neighbor as int]);
            proof {
                // neighbor is not vertex (self-loop would cause back-edge detection).
                assert(neighbor != vertex) by {
                    if neighbor == vertex {
                        // DFS(vertex) with ancestors[vertex]=true returns true, contradiction.
                    }
                };
                // neighbor is not an old ancestor: use ancestors-ext_path biconditional.
                assert(!ancestors@[neighbor as int]);
                assert(!spec_in_path(ext_path, neighbor as int));
                assert(!spec_in_path(dfs_path, neighbor as int)) by {
                    if spec_in_path(dfs_path, neighbor as int) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == neighbor as int;
                        assert(ext_path[k] == neighbor as int);
                        assert(spec_in_path(ext_path, neighbor as int));
                    }
                };
                assert(old(ancestors)@[neighbor as int] == spec_in_path(dfs_path, neighbor as int));
                assert(!old(ancestors)@[neighbor as int]);
            }
            // Extract ordering witness.
            proof {
                let (new_ord, new_next) = lemma_extract_ord_per(
                    graph, visited@, ancestors@, cur_ord, cur_next,
                );
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
            assert(visited@[graph@[vertex as int][i as int] as int]);
            assert(visited@[vertex as int]);
            // Monotonicity for previously processed neighbors.
            proof {
                assert forall|k: int| 0 <= k < i as int
                    implies #[trigger] visited@[graph@[vertex as int][k] as int] by {
                    assert(visited_pre_call[graph@[vertex as int][k] as int]);
                };
            }
            i = i + 1;
        }

        // After loop: no cycle found.
        proof {
            assert(!cur_ord.contains_key(vertex as int)) by {
                if cur_ord.contains_key(vertex as int) {
                    assert(false);
                }
            };
            assert(!old(ancestors)@[vertex as int]);
            assert forall|v: int| #[trigger] spec_has_edge_per(graph, vertex as int, v)
                && 0 <= v < graph@.len()
                implies visited@[v] && !old(ancestors)@[v] by {
                let idx = choose|idx: int| 0 <= idx < graph@[vertex as int].len()
                    && graph@[vertex as int][idx] == v;
                assert(visited@[graph@[vertex as int][idx] as int]);
                assert(!old(ancestors)@[graph@[vertex as int][idx] as int]);
                assert(graph@[vertex as int][idx] != vertex as int);
            };
        }

        // Restore ancestors[vertex] = false.
        ancestors.set(vertex, false);
        assert(ancestors@ =~= old(ancestors)@);

        // Build the final ordering: add vertex with finish time cur_next.
        proof {
            let final_ord: Map<int, nat> = cur_ord.insert(vertex as int, cur_next);
            let final_next: nat = (cur_next + 1) as nat;

            assert forall|v: int| #[trigger] final_ord.contains_key(v)
                implies final_ord[v] < final_next && 0 <= v < graph@.len() by {
                if v == vertex as int {
                    assert(final_ord[v] == cur_next);
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
            assert forall|u: int, v: int|
                final_ord.contains_key(u) && #[trigger] spec_has_edge_per(graph, u, v)
                    && 0 <= v < graph@.len()
                implies final_ord.contains_key(v) && final_ord[u] > final_ord[v] by {
                if u == vertex as int {
                    assert(visited@[v]);
                    assert(!old(ancestors)@[v]);
                    assert(v != vertex as int);
                    assert(!ancestors@[v]);
                    assert(cur_ord.contains_key(v));
                    assert(final_ord.contains_key(v));
                    assert(final_ord[vertex as int] == cur_next);
                    assert(final_ord[v] == cur_ord[v]);
                } else {
                    assert(cur_ord.contains_key(u));
                    if v == vertex as int {
                        assert(false);
                    } else {
                        assert(final_ord[u] == cur_ord[u]);
                        assert(final_ord[v] == cur_ord[v]);
                    }
                }
            };
            assert(spec_acyclic_ord_per(graph, final_ord, final_next));
            assert forall|v: int| 0 <= v < visited@.len()
                && #[trigger] visited@[v] && !ancestors@[v]
                implies final_ord.contains_key(v) by {
                if v == vertex as int {
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
            assert forall|v: int| #[trigger] final_ord.contains_key(v)
                implies visited@[v] && !ancestors@[v] by {
                if v == vertex as int {
                    assert(visited@[vertex as int]);
                    assert(!ancestors@[vertex as int]);
                } else {
                    assert(cur_ord.contains_key(v));
                }
            };
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
            assert(spec_is_valid_ord_per(graph, visited@, ancestors@, ord, next_time, final_ord));
        }
        false
    }

    impl CycleDetectStPerTrait for CycleDetectStPer {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (has_cycle: B)
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
                    forall|k: int| 0 <= k < j as int ==> !#[trigger] visited@[k],
                    forall|k: int| 0 <= k < j as int ==> !#[trigger] ancestors@[k],
                decreases n - j,
            {
                visited.push(false);
                ancestors.push(false);
                j = j + 1;
            }

            // Prove initial state: all false after push loop.
            // Vec push semantics: each push appends false.

            let ghost mut cur_ord: Map<int, nat> = Map::empty();
            let ghost mut cur_next: nat = 0;

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    visited@.len() == n,
                    ancestors@.len() == n,
                    spec_cycledetectstper_wf(graph),
                    forall|j: int| 0 <= j < ancestors@.len() ==> !#[trigger] ancestors@[j],
                    forall|j: int| 0 <= j < start as int ==> #[trigger] visited@[j],
                    spec_acyclic_ord_per(graph, cur_ord, cur_next),
                    forall|v: int| 0 <= v < visited@.len()
                        && #[trigger] visited@[v] ==> cur_ord.contains_key(v),
                    forall|v: int| #[trigger] cur_ord.contains_key(v)
                        ==> visited@[v],
                decreases n - start,
            {
                if !visited[start] {
                    proof {
                        assert forall|v: int| 0 <= v < ancestors@.len()
                            implies #[trigger] ancestors@[v] == spec_in_path(Seq::<int>::empty(), v) by {
                            assert(!ancestors@[v]);
                            assert(!spec_in_path(Seq::<int>::empty(), v));
                        };
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
                    // Monotonicity.
                    proof {
                        assert forall|j: int| 0 <= j < start as int
                            implies #[trigger] visited@[j] by {
                            assert(visited_pre[j]);
                        };
                    }
                    // Extract ordering.
                    proof {
                        let (new_ord, new_next) = lemma_extract_ord_per(
                            graph, visited@, ancestors@, cur_ord, cur_next,
                        );
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
                proof {
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            // All vertices visited → DAG.
            proof {
                assert forall|v: int| 0 <= v < graph@.len()
                    implies #[trigger] cur_ord.contains_key(v) by {
                    assert(visited@[v]);
                };
                lemma_acyclic_ord_implies_dag_per(graph, cur_ord, cur_next);
            }
            false
        }
    } // impl CycleDetectStPerTrait

    } // verus!
}
