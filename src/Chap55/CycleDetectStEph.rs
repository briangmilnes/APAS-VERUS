// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ephemeral ancestor tracking.
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

pub mod CycleDetectStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::DFSSpecsAndLemmas::DFSSpecsAndLemmas::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_bool_view_eq_spec_index, lemma_bool_array_set_view, lemma_usize_view_eq_spec_index, lemma_graph_view_bridge};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_toposortsteph_wf, spec_is_dag, spec_has_edge, spec_is_path};
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use vstd::seq::group_seq_axioms;

    //		Section 4. type definitions


    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;


    pub struct CycleDetectStEph;

    //		Section 6. spec fns


    /// Whether a value appears in a ghost integer sequence.
    pub open spec fn spec_in_path(path: Seq<int>, v: int) -> bool {
        exists|k: int| 0 <= k < path.len() && path[k] == v
    }

    /// An acyclic ordering of finished vertices: for every edge u→v where u is
    /// finished, v is also finished and has a strictly smaller ordering value.
    /// This is the DFS finish-time ordering property that implies DAG-ness.
    pub open spec fn spec_acyclic_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
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

    /// Whether an ordering map is a valid DFS completion witness.
    /// Wrapping in a spec fn gives choose a usable trigger on the map argument.
    pub open spec fn spec_is_valid_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
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

    //		Section 7. proof fns/broadcast groups


    /// If a path has a repeated vertex, the graph is not a DAG.
    proof fn lemma_cycle_not_dag(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
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
        // Veracity: NEEDED assert (speed hint)
        assert(cycle[0] == vertex);
        // Veracity: NEEDED assert
        assert(cycle.last() == vertex);
// Veracity: UNNEEDED assert         assert(cycle.len() >= 2);
        // All vertices are valid graph vertices.
        // Veracity: NEEDED assert (speed hint)
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < cycle.len() - 1 {
// Veracity: UNNEEDED assert                 assert(cycle[k] == dfs_path[i + k]);
            }
        };
        // Consecutive edges exist.
        // Veracity: NEEDED assert
        assert forall|k: int| 0 <= k < cycle.len() - 1
            implies #[trigger] spec_has_edge(graph, cycle[k], cycle[k + 1]) by {
            if k < cycle.len() - 2 {
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k] == dfs_path[i + k]);
                // Veracity: NEEDED assert
                assert(cycle[k + 1] == dfs_path[i + k + 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(i + k >= 0 && i + k < dfs_path.len() - 1);
            } else {
                // Last edge: dfs_path.last() → vertex.
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k] == dfs_path[dfs_path.len() - 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k + 1] == vertex);
            }
        };
// Veracity: UNNEEDED assert         assert(spec_is_path(graph, cycle));
    }

    /// Along any valid path where all vertices are in an acyclic ordering,
    /// the ordering strictly decreases from first to last vertex.
    proof fn lemma_path_ord_decreases(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
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
        // Veracity: NEEDED assert (speed hint)
        assert(spec_has_edge(graph, path[0], path[1]));
        if path.len() == 2 {
// Veracity: UNNEEDED assert             assert(path.last() == path[1]);
        } else {
            let sub = path.subrange(1, path.len() as int);
// Veracity: UNNEEDED assert             assert(sub.len() >= 2);
// Veracity: UNNEEDED assert             assert(sub[0] == path[1]);
            // Veracity: NEEDED assert (speed hint)
            assert(sub.last() == path.last());
            // Veracity: NEEDED assert (speed hint)
            assert(spec_is_path(graph, sub)) by {
// Veracity: UNNEEDED assert                 assert(sub.len() >= 1);
// Veracity: UNNEEDED assert                 assert forall|k: int| 0 <= k < sub.len()
// Veracity: UNNEEDED assert                     implies 0 <= #[trigger] sub[k] < graph@.len() by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(sub[k] == path[k + 1]);
// Veracity: UNNEEDED assert                 };
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger sub[k]]
                    0 <= k < sub.len() - 1
                    implies spec_has_edge(graph, sub[k], sub[k + 1]) by {
// Veracity: UNNEEDED assert                     assert(sub[k] == path[k + 1]);
// Veracity: UNNEEDED assert                     assert(sub[k + 1] == path[k + 2]);
                };
            };
            // Veracity: NEEDED assert (speed hint)
            assert forall|k: int| 0 <= k < sub.len()
                implies #[trigger] ord.contains_key(sub[k]) by {
                // Veracity: NEEDED assert (speed hint)
                assert(sub[k] == path[k + 1]);
            };
            lemma_path_ord_decreases(graph, ord, next_time, sub);
        }
    }

    /// Extract a concrete ordering witness from an existential.
    proof fn lemma_extract_ord(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: Seq<bool>,
        ancestors: Seq<bool>,
        prev_ord: Map<int, nat>,
        prev_next: nat,
    ) -> (dfs_state: (Map<int, nat>, nat))
        requires
            exists|o: Map<int, nat>|
                #[trigger] spec_is_valid_ord(graph, visited, ancestors, prev_ord, prev_next, o),
        ensures
            spec_acyclic_ord(graph, dfs_state.0, dfs_state.1)
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
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
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
// Veracity: UNNEEDED assert             assert forall|k: int| 0 <= k < path.len()
// Veracity: UNNEEDED assert                 implies #[trigger] ord.contains_key(path[k]) by {
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(0 <= path[k] < graph@.len());
// Veracity: UNNEEDED assert             };
            lemma_path_ord_decreases(graph, ord, next_time, path);
            // Veracity: NEEDED assert (speed hint)
            assert(false);
        }
    }

    //		Section 8. traits


    pub trait CycleDetectStEphTrait {
        /// Detects if a directed graph contains a cycle (Algorithm 55.10).
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O(|V| + |E|), Span O(|V| + |E|) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (has_cycle: bool)
            requires
                spec_toposortsteph_wf(graph),
            ensures
                has_cycle == !spec_is_dag(graph),
            ;
    }

    //		Section 9. impls


    /// Recursive DFS cycle detection using an ancestor array.
    /// Returns true if a cycle is found.
    /// Ghost parameters: dfs_path for cycle witness, ord/next_time for completeness ordering.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — visits each vertex/edge once; St sequential.
    fn dfs_check_cycle(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: &mut ArraySeqStEphS<bool>,
        ancestors: &mut ArraySeqStEphS<bool>,
        vertex: usize,
        Ghost(dfs_path): Ghost<Seq<int>>,
        Ghost(ord): Ghost<Map<int, nat>>,
        Ghost(next_time): Ghost<nat>,
    ) -> (has_cycle: bool)
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
// Veracity: UNNEEDED proof block         proof { lemma_bool_view_eq_spec_index(visited); }

        if *ancestors.nth(vertex) {
            // ancestors[vertex] is true → vertex is in dfs_path → cycle exists.
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(old(ancestors)@[vertex as int]);
// Veracity: UNNEEDED assert                 assert(spec_in_path(dfs_path, vertex as int));
                lemma_cycle_not_dag(graph, dfs_path, vertex as int);
            }
            return true;
        }
        if *visited.nth(vertex) {
            // Veracity: NEEDED proof block
            // vertex visited, not ancestor → in ord. Witness: ord itself.
            proof {
                // Veracity: NEEDED assert
                assert(old(visited)@[vertex as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(!old(ancestors)@[vertex as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(ord.contains_key(vertex as int));
                // Prove spec_is_valid_ord with ord as the witness.
                // Veracity: NEEDED assert
                assert(spec_is_valid_ord(graph, visited@, ancestors@, ord, next_time, ord));
            }
            return false;
        }

        // vertex is not an ancestor and not visited.
        // Veracity: NEEDED assert (speed hint)
        assert(!old(visited)@[vertex as int]);
// Veracity: UNNEEDED assert         assert(!old(ancestors)@[vertex as int]);
// Veracity: UNNEEDED assert         assert(vertex < visited.spec_len());
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert         assert(vertex < ancestors.spec_len());
        // Pre-set bridge: lets Z3 connect old state spec_index to old state view.
        proof {
            lemma_bool_view_eq_spec_index(visited);
            lemma_bool_view_eq_spec_index(ancestors);
        // Veracity: NEEDED proof block
        }
        let ok1 = visited.set(vertex, true);
        let ok2 = ancestors.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
// Veracity: UNNEEDED proof block             lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ == old(visited)@.update(vertex, true) after BOTH sets.
        proof { lemma_bool_array_set_view(visited, old(visited)@, vertex as int, true); }
// Veracity: UNNEEDED assert         assert(visited@ =~= old(visited)@.update(vertex as int, true));
        // Veracity: NEEDED assert (speed hint)
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));

        // Monotonicity.
        // Veracity: NEEDED assert
        // Veracity: NEEDED proof block
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        // Bridge ancestors@ after set.
        proof { lemma_bool_array_set_view(ancestors, old(ancestors)@, vertex as int, true); }
        // Veracity: NEEDED assert (speed hint)
        assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
// Veracity: NEEDED proof block

        // Ghost: extended path includes vertex.
        let ghost ext_path = dfs_path.push(vertex as int);

        // Prove ancestors <==> ext_path.
        proof {
            // Veracity: NEEDED assert
            assert forall|v: int| 0 <= v < ancestors@.len()
                implies #[trigger] ancestors@[v] == spec_in_path(ext_path, v) by {
                if v == vertex as int {
                    // ancestors@[vertex] == true, and vertex is at ext_path[dfs_path.len()].
                    // Veracity: NEEDED assert
                    assert(ext_path[dfs_path.len() as int] == vertex as int);
// Veracity: UNNEEDED assert                     assert(spec_in_path(ext_path, vertex as int));
                } else {
                    // ancestors@[v] == old(ancestors)@[v] == spec_in_path(dfs_path, v).
                    // spec_in_path(ext_path, v) == spec_in_path(dfs_path, v) since the push only adds vertex != v.
// Veracity: UNNEEDED assert                     assert(ancestors@[v] == old(ancestors)@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(old(ancestors)@[v] == spec_in_path(dfs_path, v));
                    // ext_path = dfs_path.push(vertex). For v != vertex:
                    // spec_in_path(ext_path, v) <==> spec_in_path(dfs_path, v).
                    if spec_in_path(dfs_path, v) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == v;
                        // Veracity: NEEDED assert
                        assert(ext_path[k] == v);
// Veracity: UNNEEDED assert                         assert(spec_in_path(ext_path, v));
                    }
                    if spec_in_path(ext_path, v) {
                        let k = choose|k: int| 0 <= k < ext_path.len() && ext_path[k] == v;
                        if k < dfs_path.len() {
                            // Veracity: NEEDED assert (speed hint)
                            assert(dfs_path[k] == v);
// Veracity: UNNEEDED assert                             assert(spec_in_path(dfs_path, v));
                        } else {
                            // k == dfs_path.len(), ext_path[k] == vertex != v, contradiction.
                            // Veracity: NEEDED assert (speed hint)
                            assert(ext_path[k] == vertex as int);
                            // Veracity: NEEDED assert (speed hint)
                            assert(false);
                        }
                    }
                }
            };
        }

        // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        // Veracity: NEEDED proof block
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();

        // Bridge neighbors to graph view.
        // Veracity: NEEDED assert (speed hint)
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
        // Veracity: NEEDED assert (speed hint)
        // Veracity: NEEDED proof block
        assert(neighbors@ =~= graph@[vertex as int]);

        // Ghost: track the acyclic ordering through the neighbor loop.
        let ghost mut cur_ord: Map<int, nat> = ord;
        let ghost mut cur_next: nat = next_time;

        // Prove initial ordering invariants hold after setting visited/ancestors.
        proof {
            // vertex is now visited AND ancestor → not in ord (by converse requires).
            // Veracity: NEEDED assert (speed hint)
            assert(!ord.contains_key(vertex as int)) by {
                if ord.contains_key(vertex as int) {
                    // requires: ord.contains_key(v) ==> old(visited)[v] && !old(ancestors)[v]
                    // Veracity: NEEDED assert (speed hint)
                    assert(old(visited)@[vertex as int]); // but vertex was not visited
                    // Veracity: NEEDED assert (speed hint)
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
                    // Veracity: NEEDED proof block
                    ==> #[trigger] visited@[graph@[vertex as int][j] as int],
                forall|j: int| 0 <= j < i as int
                    ==> !old(ancestors)@[#[trigger] graph@[vertex as int][j] as int],
                // Veracity: NEEDED proof block
                forall|j: int| 0 <= j < i as int
                    ==> graph@[vertex as int][j] != vertex as int,
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert
            assert(neighbor == neighbors@[i as int]);
            // Edge vertex → neighbor for ghost path last-to-vertex requires.
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(spec_has_edge(graph, vertex as int, neighbor as int));
                // ext_path = dfs_path.push(vertex). Its last element is vertex.
                // Veracity: NEEDED assert (speed hint)
                assert(ext_path.len() == dfs_path.len() + 1);
                // Veracity: NEEDED assert (speed hint)
                assert(ext_path[ext_path.len() - 1] == vertex as int);
            }
            // Snapshot visited for monotonicity proof after the call.
            let ghost visited_pre_call = visited@;
            if dfs_check_cycle(graph, visited, ancestors, neighbor, Ghost(ext_path), Ghost(cur_ord), Ghost(cur_next)) {
                // Veracity: NEEDED proof block
                // Cycle found. dfs_check_cycle ensures !spec_is_dag(graph).
                let ok3 = ancestors.set(vertex, false);
                return true;
            }
            // dfs_check_cycle returned false: ancestors restored, vertex visited.
            // Veracity: NEEDED assert (speed hint)
            assert(ancestors@ =~= old(ancestors)@.update(vertex as int, true));
            // Veracity: NEEDED assert (speed hint)
            assert(visited@[neighbor as int]);
            // neighbor is not vertex (self-loop would cause back-edge detection).
            proof {
// Veracity: UNNEEDED assert                 assert(neighbor != vertex) by {
// Veracity: UNNEEDED assert                     if neighbor == vertex {
// Veracity: UNNEEDED assert                         // DFS(vertex) was called with ancestors[vertex] = true at callsite.
// Veracity: UNNEEDED assert                         // ensures: !has_cycle ==> !old(ancestors at callsite)@[vertex].
// Veracity: UNNEEDED assert                         // old(ancestors at callsite)@[vertex] = ancestors@[vertex] before call
// Veracity: UNNEEDED assert                         //   = old(ancestors)@.update(vertex, true)[vertex] = true.
// Veracity: UNNEEDED assert                         // So !has_cycle ==> false. But DFS returned false. Contradiction.
// Veracity: UNNEEDED assert                     }
// Veracity: UNNEEDED assert                 };
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
                // Veracity: NEEDED assert (speed hint)
                assert(!ancestors@[neighbor as int]);
                // Use the ancestors-ext_path biconditional to avoid seq update axiom.
                // !ancestors@[neighbor] (from DFS ensures + restoration).
                // Loop invariant: ancestors@[v] == spec_in_path(ext_path, v).
                // Veracity: NEEDED assert (speed hint)
                assert(!spec_in_path(ext_path, neighbor as int));
                // ext_path = dfs_path.push(vertex), neighbor != vertex.
                // Veracity: NEEDED assert
                assert(!spec_in_path(dfs_path, neighbor as int)) by {
                    if spec_in_path(dfs_path, neighbor as int) {
                        let k = choose|k: int| 0 <= k < dfs_path.len() && dfs_path[k] == neighbor as int;
                        // Veracity: NEEDED assert
                        assert(ext_path[k] == neighbor as int);
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED assert (speed hint)
                        assert(spec_in_path(ext_path, neighbor as int));
                    }
                };
                // Function requires biconditional: old(ancestors)@[v] == spec_in_path(dfs_path, v).
                // Veracity: NEEDED assert (speed hint)
                assert(old(ancestors)@[neighbor as int] == spec_in_path(dfs_path, neighbor as int));
                // Veracity: NEEDED assert (speed hint)
                assert(!old(ancestors)@[neighbor as int]);
            }
            // Extract the ordering witness via proof function (avoids choose trigger issues).
            proof {
                let (new_ord, new_next) = lemma_extract_ord(
                    graph, visited@, ancestors@, cur_ord, cur_next,
                );
                // Prove extends from ord: by transitivity.
                // Veracity: NEEDED assert (speed hint)
                assert forall|v: int| #[trigger] ord.contains_key(v)
                    implies new_ord.contains_key(v) && new_ord[v] == ord[v] by {
                    // Veracity: NEEDED assert (speed hint)
                    // Veracity: NEEDED proof block
                    assert(cur_ord.contains_key(v));
                    // Veracity: NEEDED assert (speed hint)
                    assert(cur_ord[v] == ord[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_ord.contains_key(v));
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_ord[v] == cur_ord[v]);
                };
                cur_ord = new_ord;
                cur_next = new_next;
            }
            // Monotonicity for previously processed neighbors.
            proof {
                // Veracity: NEEDED assert
                assert forall|k: int| 0 <= k < i as int
                    // Veracity: NEEDED proof block
                    implies #[trigger] visited@[graph@[vertex as int][k] as int] by {
                    // Before the call, this was true (loop invariant).
                    // Veracity: NEEDED assert
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
            // Veracity: NEEDED assert (speed hint)
            assert(!cur_ord.contains_key(vertex as int)) by {
                if cur_ord.contains_key(vertex as int) {
                    // Veracity: NEEDED assert (speed hint)
                    assert(false);
                }
            };
            // !old(ancestors)@[vertex as int] from function entry check.
            // Veracity: NEEDED assert (speed hint)
            assert(!old(ancestors)@[vertex as int]);
            // For every neighbor v of vertex, v is visited and was not an old ancestor.
            // Uses loop invariants at exit (i == neighbors_len).
            // Veracity: NEEDED assert
            assert forall|v: int| #[trigger] spec_has_edge(graph, vertex as int, v)
                && 0 <= v < graph@.len()
                implies visited@[v] && !old(ancestors)@[v] by {
                // Veracity: NEEDED proof block
                let idx = choose|idx: int| 0 <= idx < graph@[vertex as int].len()
                    && graph@[vertex as int][idx] == v;
                // Veracity: NEEDED proof block
                // From loop invariant: all neighbors visited.
// Veracity: UNNEEDED assert                 assert(visited@[graph@[vertex as int][idx] as int]);
                // From loop invariant: all neighbors not old ancestors.
// Veracity: UNNEEDED assert                 assert(!old(ancestors)@[graph@[vertex as int][idx] as int]);
                // From loop invariant: no neighbor equals vertex (no self-loop).
                // Veracity: NEEDED assert (speed hint)
                assert(graph@[vertex as int][idx] != vertex as int);
            };
        }

        // Restore ancestors[vertex] = false.
        // Pre-set bridge: lets Z3 connect spec_index to view for the pre-restore state.
        proof { lemma_bool_view_eq_spec_index(ancestors); }
        let ghost pre_set_view = ancestors@;
        let ok3 = ancestors.set(vertex, false);
        proof {
            lemma_bool_array_set_view(ancestors, pre_set_view, vertex as int, false);
            // Veracity: NEEDED assert (speed hint)
            // Veracity: NEEDED proof block
            assert(ancestors@ =~= pre_set_view.update(vertex as int, false));
            // Veracity: NEEDED assert (speed hint)
            assert forall|j: int| 0 <= j < ancestors@.len()
                implies #[trigger] ancestors@[j] == old(ancestors)@[j] by {
                if j == vertex as int {
                    // Veracity: NEEDED assert (speed hint)
                    assert(!old(ancestors)@[vertex as int]);
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(pre_set_view[j] == old(ancestors)@[j]);
                }
            };
        }
// Veracity: UNNEEDED assert         assert(ancestors@ =~= old(ancestors)@);

        // Build the final ordering: add vertex with finish time cur_next.
        proof {
            let final_ord: Map<int, nat> = cur_ord.insert(vertex as int, cur_next);
            let final_next: nat = (cur_next + 1) as nat;

            // Prove spec_acyclic_ord(graph, final_ord, final_next).
// Veracity: UNNEEDED assert             assert forall|v: int| #[trigger] final_ord.contains_key(v)
// Veracity: UNNEEDED assert                 implies final_ord[v] < final_next && 0 <= v < graph@.len() by {
// Veracity: UNNEEDED assert                 if v == vertex as int {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(final_ord[v] == cur_next);
// Veracity: UNNEEDED assert                 } else {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(v));
// Veracity: UNNEEDED assert                 }
// Veracity: UNNEEDED assert             };
            // Veracity: NEEDED assert
            assert forall|u: int, v: int|
                final_ord.contains_key(u) && #[trigger] spec_has_edge(graph, u, v)
                    && 0 <= v < graph@.len()
                implies final_ord.contains_key(v) && final_ord[u] > final_ord[v] by {
                if u == vertex as int {
                    // Vertex→v: v is visited, !old(ancestors)[v] (proved above).
                    // No self-loop: v != vertex (proved above).
                    // Veracity: NEEDED assert (speed hint)
                    assert(visited@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(!old(ancestors)@[v]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(v != vertex as int) by {
                        // No self-loop from the bridging proof above.
                    };
                    // Veracity: NEEDED assert (speed hint)
                    assert(!ancestors@[v]);
                    // v is visited, not ancestor (during loop: ancestors = old.update(vertex,true),
                    // for v != vertex: ancestors[v] = old(ancestors)[v] = false).
                    // After restoration: same. So v was in cur_ord.
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(v));
// Veracity: UNNEEDED assert                     assert(final_ord.contains_key(v));
                    // Veracity: NEEDED assert (speed hint)
                    assert(final_ord[vertex as int] == cur_next);
                    // Veracity: NEEDED assert (speed hint)
                    assert(final_ord[v] == cur_ord[v]);
                } else {
// Veracity: UNNEEDED assert                     assert(cur_ord.contains_key(u));
                    if v == vertex as int {
                        // Edge closure on cur_ord: u in cur_ord → v in cur_ord.
                        // But vertex NOT in cur_ord. Contradiction.
                        // Veracity: NEEDED assert (speed hint)
                        assert(false);
                    } else {
                        // Veracity: NEEDED assert (speed hint)
                        assert(final_ord[u] == cur_ord[u]);
// Veracity: UNNEEDED assert                         assert(final_ord[v] == cur_ord[v]);
                    }
                }
            };
            // Veracity: NEEDED assert
            assert(spec_acyclic_ord(graph, final_ord, final_next));

            // All visited non-ancestor vertices are in final_ord.
            // Veracity: NEEDED assert
            assert forall|v: int| 0 <= v < visited@.len()
                && #[trigger] visited@[v] && !ancestors@[v]
                implies final_ord.contains_key(v) by {
                if v == vertex as int {
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(cur_ord.contains_key(v));
                }
            };
            // final_ord keys are visited non-ancestor.
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
            // Extends from ord.
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
// Veracity: UNNEEDED assert                         assert(old(visited)@[v]);
// Veracity: UNNEEDED assert                         assert(false);
                    }
                };
// Veracity: UNNEEDED assert                 assert(final_ord[v] == cur_ord[v]);
            };
            // Prove spec_is_valid_ord to satisfy the ensures existential.
            // Veracity: NEEDED assert
            // Veracity: NEEDED proof block
            assert(spec_is_valid_ord(graph, visited@, ancestors@, ord, next_time, final_ord));
        }
        false
    }

    impl CycleDetectStEphTrait for CycleDetectStEph {
        /// Detects if a directed graph contains a cycle.
        /// Returns true if a cycle exists, false otherwise.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS from each unvisited vertex; St sequential.
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (has_cycle: bool)
        {
            let n = graph.length();
            let f_false = |_x: usize| -> (r: bool) ensures !r { false };
            let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
            let mut ancestors = ArraySeqStEphS::tabulate(&f_false, n);

            // Prove ancestors and visited are all false initially.
            proof {
// Veracity: UNNEEDED assert                 assert forall|j: int| 0 <= j < ancestors@.len() implies !#[trigger] ancestors@[j] by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(!ancestors.seq@[j]);
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(ancestors@[j] == ancestors.seq@[j]@);
// Veracity: UNNEEDED assert                 };
                // Veracity: NEEDED assert (speed hint)
                assert forall|j: int| 0 <= j < visited@.len() implies !#[trigger] visited@[j] by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(!visited.seq@[j]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(visited@[j] == visited.seq@[j]@);
                };
            }

            // Ghost: acyclic ordering accumulator.
            let ghost mut cur_ord: Map<int, nat> = Map::empty();
            let ghost mut cur_next: nat = 0;

            let mut start: usize = 0;
            while start < n
                // Veracity: NEEDED proof block
                invariant
                    start <= n,
                    // Veracity: NEEDED proof block
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
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|v: int| 0 <= v < ancestors@.len()
                            implies #[trigger] ancestors@[v] == spec_in_path(Seq::<int>::empty(), v) by {
// Veracity: UNNEEDED assert                             assert(!ancestors@[v]);
                            // Veracity: NEEDED assert (speed hint)
                            // Veracity: NEEDED proof block
                            assert(!spec_in_path(Seq::<int>::empty(), v));
                        };
                        // Ordering requires: visited && !ancestors ==> in ord.
                        // Since ancestors all false: visited ==> in ord. ✓ (loop invariant)
                        // Converse: in ord ==> visited && !ancestors. ✓
                        // Veracity: NEEDED assert (speed hint)
                        assert forall|v: int| #[trigger] cur_ord.contains_key(v)
                            implies visited@[v] && !ancestors@[v] by {
                            // Veracity: NEEDED assert (speed hint)
                            // Veracity: NEEDED proof block
                            assert(visited@[v]);
                            // Veracity: NEEDED assert (speed hint)
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
                        // Veracity: NEEDED assert
                        assert forall|j: int| 0 <= j < start as int
                            implies #[trigger] visited@[j] by {
                            // Veracity: NEEDED assert
                            assert(visited_pre[j]); // was true (loop invariant)
                            // DFS monotonicity: pre_call[j] ==> post_call[j].
                        // Veracity: NEEDED proof block
                        };
                    }
                    // Extract the ordering via proof function.
                    proof {
                        let (new_ord, new_next) = lemma_extract_ord(
                            graph, visited@, ancestors@, cur_ord, cur_next,
                        // Veracity: NEEDED proof block
                        );
                        // Since ancestors are all false: visited && !ancestors <==> visited.
                        // Veracity: NEEDED assert
                        assert forall|v: int| 0 <= v < visited@.len()
                            && #[trigger] visited@[v]
                            implies new_ord.contains_key(v) by {
                            // Veracity: NEEDED assert (speed hint)
                            assert(!ancestors@[v]);
                        };
// Veracity: UNNEEDED assert                         assert forall|v: int| #[trigger] new_ord.contains_key(v)
// Veracity: UNNEEDED assert                             implies visited@[v] by {};
                        cur_ord = new_ord;
                        cur_next = new_next;
                    }
                }
                // After if: start is visited (either was already, or DFS visited it).
                proof {
                    lemma_bool_view_eq_spec_index(&visited);
// Veracity: UNNEEDED assert                     assert(visited@[start as int]);
                }
                start = start + 1;
            }
            // After loop: all vertices visited, ordering covers all vertices → DAG.
            proof {
                // Veracity: NEEDED assert
                assert forall|v: int| 0 <= v < graph@.len()
                    implies #[trigger] cur_ord.contains_key(v) by {
                    // Veracity: NEEDED assert
                    assert(visited@[v]);
                };
                lemma_acyclic_ord_implies_dag(graph, cur_ord, cur_next);
            }
            false
        }
    } // impl CycleDetectStEphTrait

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for CycleDetectStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CycleDetectStEph")
        }
    }

    impl std::fmt::Display for CycleDetectStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CycleDetectStEph")
        }
    }
}
