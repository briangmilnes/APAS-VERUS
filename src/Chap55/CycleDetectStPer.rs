//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod CycleDetectStPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::DFSSpecsAndLemmas::DFSSpecsAndLemmas::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_usize_per_view_eq_spec_index, lemma_graph_per_view_bridge};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStPer::TopoSortStPer::{spec_is_dag_per, spec_has_edge_per, spec_is_path_per};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::CycleDetectStEph::CycleDetectStEph::spec_in_path;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use vstd::seq::group_seq_axioms;

    //		Section 4. type definitions


    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;


    pub struct CycleDetectStPer;

    //		Section 6. spec fns


    /// Well-formed adjacency list for persistent graph representation.
    pub open spec fn spec_cycledetectstper_wf(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> bool {
        forall|v: int, i: int|
            0 <= v < graph@.len() && 0 <= i < graph@[v].len()
            ==> (#[trigger] graph@[v][i]) < graph@.len()
    }

    /// An acyclic ordering of finished vertices (persistent variant).
    pub open spec fn spec_acyclic_ord_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
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
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
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

    //		Section 7. proof fns/broadcast groups


    /// If a path has a repeated vertex, the graph (persistent) is not a DAG.
    proof fn lemma_cycle_not_dag_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
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
// Veracity: UNNEEDED assert         assert(cycle[0] == vertex);
        // Veracity: NEEDED assert
        assert(cycle.last() == vertex);
        // Veracity: NEEDED assert (speed hint)
        assert(cycle.len() >= 2);
        // Veracity: NEEDED assert (speed hint)
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < cycle.len() - 1 {
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k] == dfs_path[i + k]);
            }
        };
        // Veracity: NEEDED assert
        assert forall|k: int| 0 <= k < cycle.len() - 1
            implies #[trigger] spec_has_edge_per(graph, cycle[k], cycle[k + 1]) by {
            if k < cycle.len() - 2 {
// Veracity: UNNEEDED assert                 assert(cycle[k] == dfs_path[i + k]);
                // Veracity: NEEDED assert
                assert(cycle[k + 1] == dfs_path[i + k + 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(i + k >= 0 && i + k < dfs_path.len() - 1);
            } else {
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k] == dfs_path[dfs_path.len() - 1]);
// Veracity: UNNEEDED assert                 assert(cycle[k + 1] == vertex);
            }
        };
        // Veracity: NEEDED assert (speed hint)
        assert(spec_is_path_per(graph, cycle));
    }

    /// Extract a concrete ordering witness from an existential (persistent variant).
    proof fn lemma_extract_ord_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
    ) -> (dfs_state: (Map<int, nat>, nat))
        requires
            exists|o: Map<int, nat>|
                #[trigger] spec_is_valid_ord_per(graph, visited, ancestors, prev_ord, prev_next, o),
        ensures
            spec_acyclic_ord_per(graph, dfs_state.0, dfs_state.1)
            && (forall|v: int| 0 <= v < visited.len()
                && #[trigger] visited[v] && !ancestors[v]
                ==> dfs_state.0.contains_key(v))
            && (forall|v: int| #[trigger] dfs_state.0.contains_key(v)
                ==> visited[v] && !ancestors[v])
            && (forall|v: int| #[trigger] prev_ord.contains_key(v)
                ==> dfs_state.0.contains_key(v) && dfs_state.0[v] == prev_ord[v])
            && dfs_state.1 >= prev_next,
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
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
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
        // Veracity: NEEDED assert (speed hint)
        assert(spec_has_edge_per(graph, path[0], path[1]));
        if path.len() == 2 {
            // Veracity: NEEDED assert (speed hint)
            assert(path.last() == path[1]);
        } else {
            let sub = path.subrange(1, path.len() as int);
            // Veracity: NEEDED assert (speed hint)
            assert(sub.len() >= 2);
// Veracity: UNNEEDED assert             assert(sub[0] == path[1]);
            // Veracity: NEEDED assert (speed hint)
            assert(sub.last() == path.last());
            // Veracity: NEEDED assert (speed hint)
            assert(spec_is_path_per(graph, sub)) by {
                // Veracity: NEEDED assert (speed hint)
                assert(sub.len() >= 1);
// Veracity: UNNEEDED assert                 assert forall|k: int| 0 <= k < sub.len()
// Veracity: UNNEEDED assert                     implies 0 <= #[trigger] sub[k] < graph@.len() by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                     assert(sub[k] == path[k + 1]);
// Veracity: UNNEEDED assert                 };
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger sub[k]]
                    0 <= k < sub.len() - 1
                    implies spec_has_edge_per(graph, sub[k], sub[k + 1]) by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(sub[k] == path[k + 1]);
// Veracity: UNNEEDED assert                     assert(sub[k + 1] == path[k + 2]);
                };
            };
// Veracity: UNNEEDED assert             assert forall|k: int| 0 <= k < sub.len()
// Veracity: UNNEEDED assert                 implies #[trigger] ord.contains_key(sub[k]) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                 assert(sub[k] == path[k + 1]);
// Veracity: UNNEEDED assert             };
            lemma_path_ord_decreases_per(graph, ord, next_time, sub);
        }
    }

    /// If an acyclic ordering covers all vertices, the graph is a DAG (persistent variant).
    proof fn lemma_acyclic_ord_implies_dag_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
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
            // Veracity: NEEDED assert (speed hint)
            assert forall|k: int| 0 <= k < path.len()
                implies #[trigger] ord.contains_key(path[k]) by {
                // Veracity: NEEDED assert (speed hint)
                assert(0 <= path[k] < graph@.len());
            };
            lemma_path_ord_decreases_per(graph, ord, next_time, path);
// Veracity: UNNEEDED assert             assert(false);
        }
    }

    //		Section 8. traits


    pub trait CycleDetectStPerTrait {
        /// Detects if a directed graph contains a cycle (Algorithm 55.10)
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS-based cycle detection; St sequential.
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (has_cycle: bool)
            requires
                spec_cycledetectstper_wf(graph),
            ensures
                has_cycle == !spec_is_dag_per(graph),
            ;
    }

    //		Section 9. impls


    /// Recursive DFS cycle detection using Vec<bool> ancestor tracking.
    /// Ghost parameters: dfs_path for cycle witness, ord/next_time for completeness ordering.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — visits each vertex/edge once; St sequential.
    fn dfs_check_cycle(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: &mut Vec<bool>,
        ancestors: &mut Vec<bool>,
        vertex: usize,
        Ghost(dfs_path): Ghost<Seq<int>>,
        Ghost(ord): Ghost<Map<int, nat>>,
        Ghost(next_time): Ghost<nat>,
    ) -> (has_cycle: bool)
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
            // Veracity: NEEDED proof block
            proof { lemma_cycle_not_dag_per(graph, dfs_path, vertex as int); }
            return true;
        }
        // Veracity: NEEDED proof block
        if visited[vertex] {
            proof {
                // Veracity: NEEDED assert
                assert(spec_is_valid_ord_per(graph, visited@, ancestors@, ord, next_time, ord));
            }
            return false;
        }

        // Veracity: NEEDED assert (speed hint)
        assert(!old(visited)@[vertex as int]);
        // Veracity: NEEDED proof block
        visited.set(vertex, true);
        ancestors.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
        // Veracity: NEEDED assert (speed hint)
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
// Veracity: UNNEEDED assert         assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
// Veracity: NEEDED proof block

        let ghost ext_path = dfs_path.push(vertex as int);

        proof {
            // Veracity: NEEDED assert
            assert forall|v: int| 0 <= v < ancestors@.len()
                implies #[trigger] ancestors@[v] == spec_in_path(ext_path, v) by {
                if v == vertex as int {
                    // Veracity: NEEDED assert
                    assert(ext_path[dfs_path.len() as int] == vertex as int);
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_in_path(ext_path, vertex as int));
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(ancestors@[v] == old(ancestors)@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(old(ancestors)@[v] == spec_in_path(dfs_path, v));
                    if spec_in_path(dfs_path, v) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == v;
                        // Veracity: NEEDED assert
                        assert(ext_path[k] == v);
                        // Veracity: NEEDED assert (speed hint)
                        assert(spec_in_path(ext_path, v));
                    }
                    if spec_in_path(ext_path, v) {
                        let k = choose|k: int| 0 <= k < ext_path.len() && ext_path[k] == v;
                        if k < dfs_path.len() {
                            // Veracity: NEEDED assert (speed hint)
                            assert(dfs_path[k] == v);
// Veracity: UNNEEDED assert                             assert(spec_in_path(dfs_path, v));
                        } else {
// Veracity: UNNEEDED assert                             assert(ext_path[k] == vertex as int);
                            // Veracity: NEEDED assert (speed hint)
                            assert(false);
                        }
                    }
                }
            };
            // vertex not in ord (was unvisited).
// Veracity: UNNEEDED assert             assert(!ord.contains_key(vertex as int)) by {
// Veracity: UNNEEDED assert                 if ord.contains_key(vertex as int) {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(old(visited)@[vertex as int]);
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(false);
// Veracity: UNNEEDED assert                 }
// Veracity: UNNEEDED assert             };
        }

// Veracity: UNNEEDED assert         assert((vertex as int) < graph@.len());
        // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        // Veracity: NEEDED proof block
        let neighbors_len = neighbors.length();
// Veracity: UNNEEDED assert         assert(neighbors_len as int == neighbors.spec_len());

// Veracity: UNNEEDED assert         assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_per_view_bridge(graph, neighbors, vertex as int); }
// Veracity: UNNEEDED assert         assert(neighbors@ =~= graph@[vertex as int]);

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
                // Veracity: NEEDED proof block
                forall|j: int| 0 <= j < i as int
                    ==> graph@[vertex as int][j] != vertex as int,
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert
            assert(neighbor == neighbors@[i as int]);
            let ghost visited_pre_call = visited@;
            // Veracity: NEEDED proof block
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path), Ghost(cur_ord), Ghost(cur_next)) {
                ancestors.set(vertex, false);
                return true;
            }
            // Veracity: NEEDED assert (speed hint)
            assert(visited@[neighbor as int]);
            proof {
                // neighbor is not vertex (self-loop would cause back-edge detection).
                // Veracity: NEEDED assert (speed hint)
                assert(neighbor != vertex) by {
                    if neighbor == vertex {
                        // DFS(vertex) with ancestors[vertex]=true returns true, contradiction.
                    }
                };
                // neighbor is not an old ancestor: use ancestors-ext_path biconditional.
                // Veracity: NEEDED assert (speed hint)
                assert(!ancestors@[neighbor as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(!spec_in_path(ext_path, neighbor as int));
                // Veracity: NEEDED assert
                assert(!spec_in_path(dfs_path, neighbor as int)) by {
                    if spec_in_path(dfs_path, neighbor as int) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == neighbor as int;
                        // Veracity: NEEDED assert
                        assert(ext_path[k] == neighbor as int);
                        // Veracity: NEEDED assert (speed hint)
                        assert(spec_in_path(ext_path, neighbor as int));
                    }
                // Veracity: NEEDED proof block
                };
                // Veracity: NEEDED assert (speed hint)
                assert(old(ancestors)@[neighbor as int] == spec_in_path(dfs_path, neighbor as int));
                // Veracity: NEEDED assert (speed hint)
                assert(!old(ancestors)@[neighbor as int]);
            }
            // Extract ordering witness.
            proof {
                let (new_ord, new_next) = lemma_extract_ord_per(
                    graph, visited@, ancestors@, cur_ord, cur_next,
                );
// Veracity: UNNEEDED assert                 assert forall|v: int| #[trigger] ord.contains_key(v)
// Veracity: UNNEEDED assert                     implies new_ord.contains_key(v) && new_ord[v] == ord[v] by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(v));
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(cur_ord[v] == ord[v]);
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                     assert(new_ord.contains_key(v));
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(new_ord[v] == cur_ord[v]);
// Veracity: UNNEEDED assert                 };
                cur_ord = new_ord;
                cur_next = new_next;
            }
            // Monotonicity for previously processed neighbors.
            proof {
                // Veracity: NEEDED assert
                assert forall|k: int| 0 <= k < i as int
                    // Veracity: NEEDED proof block
                    implies #[trigger] visited@[graph@[vertex as int][k] as int] by {
                    // Veracity: NEEDED assert
                    assert(visited_pre_call[graph@[vertex as int][k] as int]);
                };
            }
            i = i + 1;
        }

        // After loop: no cycle found.
        proof {
// Veracity: UNNEEDED assert             assert(!cur_ord.contains_key(vertex as int)) by {
// Veracity: UNNEEDED assert                 if cur_ord.contains_key(vertex as int) {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(false);
// Veracity: UNNEEDED assert                 }
// Veracity: UNNEEDED assert             };
// Veracity: UNNEEDED assert             assert(!old(ancestors)@[vertex as int]);
            // Veracity: NEEDED assert
            assert forall|v: int| #[trigger] spec_has_edge_per(graph, vertex as int, v)
                && 0 <= v < graph@.len()
                implies visited@[v] && !old(ancestors)@[v] by {
                let idx = choose|idx: int| 0 <= idx < graph@[vertex as int].len()
                    && graph@[vertex as int][idx] == v;
// Veracity: UNNEEDED assert                 assert(visited@[graph@[vertex as int][idx] as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(!old(ancestors)@[graph@[vertex as int][idx] as int]);
                // Veracity: NEEDED proof block
                // Veracity: NEEDED assert (speed hint)
                assert(graph@[vertex as int][idx] != vertex as int);
            };
        }

        // Restore ancestors[vertex] = false.
        ancestors.set(vertex, false);
// Veracity: UNNEEDED assert         assert(ancestors@ =~= old(ancestors)@);

        // Build the final ordering: add vertex with finish time cur_next.
        proof {
            let final_ord: Map<int, nat> = cur_ord.insert(vertex as int, cur_next);
            let final_next: nat = (cur_next + 1) as nat;

            // Veracity: NEEDED assert (speed hint)
            assert forall|v: int| #[trigger] final_ord.contains_key(v)
                implies final_ord[v] < final_next && 0 <= v < graph@.len() by {
                if v == vertex as int {
                    // Veracity: NEEDED assert (speed hint)
                    assert(final_ord[v] == cur_next);
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(cur_ord.contains_key(v));
                }
            };
            // Veracity: NEEDED assert
            assert forall|u: int, v: int|
                final_ord.contains_key(u) && #[trigger] spec_has_edge_per(graph, u, v)
                    && 0 <= v < graph@.len()
                implies final_ord.contains_key(v) && final_ord[u] > final_ord[v] by {
                if u == vertex as int {
// Veracity: UNNEEDED assert                     assert(visited@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(!old(ancestors)@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(v != vertex as int);
// Veracity: UNNEEDED assert                     assert(!ancestors@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(cur_ord.contains_key(v));
// Veracity: UNNEEDED assert                     assert(final_ord.contains_key(v));
// Veracity: UNNEEDED assert                     assert(final_ord[vertex as int] == cur_next);
                    // Veracity: NEEDED assert (speed hint)
                    assert(final_ord[v] == cur_ord[v]);
                } else {
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(u));
                    if v == vertex as int {
                        // Veracity: NEEDED assert (speed hint)
                        assert(false);
                    } else {
                        // Veracity: NEEDED assert (speed hint)
                        assert(final_ord[u] == cur_ord[u]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(final_ord[v] == cur_ord[v]);
                    }
                }
            };
            // Veracity: NEEDED assert
            assert(spec_acyclic_ord_per(graph, final_ord, final_next));
            // Veracity: NEEDED assert
            assert forall|v: int| 0 <= v < visited@.len()
                && #[trigger] visited@[v] && !ancestors@[v]
                implies final_ord.contains_key(v) by {
                if v == vertex as int {
                } else {
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(v));
                }
            };
// Veracity: UNNEEDED assert             assert forall|v: int| #[trigger] final_ord.contains_key(v)
// Veracity: UNNEEDED assert                 implies visited@[v] && !ancestors@[v] by {
// Veracity: UNNEEDED assert                 if v == vertex as int {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(visited@[vertex as int]);
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(!ancestors@[vertex as int]);
// Veracity: UNNEEDED assert                 } else {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(v));
// Veracity: UNNEEDED assert                 }
// Veracity: UNNEEDED assert             };
            // Veracity: NEEDED assert (speed hint)
            assert forall|v: int| #[trigger] ord.contains_key(v)
                implies final_ord.contains_key(v) && final_ord[v] == ord[v] by {
                // Veracity: NEEDED assert (speed hint)
                assert(cur_ord.contains_key(v));
                // Veracity: NEEDED assert (speed hint)
                assert(cur_ord[v] == ord[v]);
                // Veracity: NEEDED assert (speed hint)
                assert(v != vertex as int) by {
                    if v == vertex as int {
                        // Veracity: NEEDED assert (speed hint)
                        assert(old(visited)@[v]);
// Veracity: UNNEEDED assert                         assert(false);
                    }
                };
// Veracity: UNNEEDED assert                 assert(final_ord[v] == cur_ord[v]);
            };
            // Veracity: NEEDED assert
            assert(spec_is_valid_ord_per(graph, visited@, ancestors@, ord, next_time, final_ord));
        }
        false
    }

    impl CycleDetectStPerTrait for CycleDetectStPer {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS from each unvisited vertex; St sequential.
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (has_cycle: bool)
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
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|v: int| 0 <= v < ancestors@.len()
                            implies #[trigger] ancestors@[v] == spec_in_path(Seq::<int>::empty(), v) by {
                            // Veracity: NEEDED assert (speed hint)
                            assert(!ancestors@[v]);
                            // Veracity: NEEDED assert (speed hint)
                            assert(!spec_in_path(Seq::<int>::empty(), v));
                        };
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|v: int| #[trigger] cur_ord.contains_key(v)
                            // Veracity: NEEDED proof block
                            implies visited@[v] && !ancestors@[v] by {
                            // Veracity: NEEDED assert (speed hint)
                            assert(visited@[v]);
                            // Veracity: NEEDED assert (speed hint)
                            assert(!ancestors@[v]);
                        };
                    }
                    let ghost visited_pre = visited@;
                    // Veracity: NEEDED proof block
                    if dfs_check_cycle(graph, &mut visited, &mut ancestors, start, Ghost(Seq::empty()), Ghost(cur_ord), Ghost(cur_next)) {
                        return true;
                    }
                    // Monotonicity.
                    proof {
                        // Veracity: NEEDED assert
                        assert forall|j: int| 0 <= j < start as int
                            implies #[trigger] visited@[j] by {
                            // Veracity: NEEDED assert
                            assert(visited_pre[j]);
                        };
                    }
                    // Extract ordering.
                    proof {
                        let (new_ord, new_next) = lemma_extract_ord_per(
                            graph, visited@, ancestors@, cur_ord, cur_next,
                        // Veracity: NEEDED proof block
                        );
                        // Veracity: NEEDED assert
                        assert forall|v: int| 0 <= v < visited@.len()
                            && #[trigger] visited@[v]
                            implies new_ord.contains_key(v) by {
                            // Veracity: NEEDED assert (speed hint)
                            // Veracity: NEEDED proof block
                            assert(!ancestors@[v]);
                        };
// Veracity: UNNEEDED assert                         assert forall|v: int| #[trigger] new_ord.contains_key(v)
// Veracity: UNNEEDED assert                             implies visited@[v] by {};
                        cur_ord = new_ord;
                        cur_next = new_next;
                    }
                }
                proof {
                    // Veracity: NEEDED assert (speed hint)
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            // All vertices visited → DAG.
            proof {
                // Veracity: NEEDED assert
                assert forall|v: int| 0 <= v < graph@.len()
                    implies #[trigger] cur_ord.contains_key(v) by {
                    // Veracity: NEEDED assert
                    assert(visited@[v]);
                };
                lemma_acyclic_ord_implies_dag_per(graph, cur_ord, cur_next);
            }
            false
        }
    } // impl CycleDetectStPerTrait

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for CycleDetectStPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CycleDetectStPer")
        }
    }

    impl std::fmt::Display for CycleDetectStPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CycleDetectStPer")
        }
    }
}
