//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Topological Sort - Sequential Ephemeral (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using ephemeral structures.
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

pub mod TopoSortStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap55::CycleDetectStEph::CycleDetectStEph::{CycleDetectStEph, CycleDetectStEphTrait};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::DFSSpecsAndLemmas::DFSSpecsAndLemmas::*;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;


    pub struct TopoSortStEph;

    //		Section 6. spec fns


    /// Well-formed adjacency list: all neighbor indices are valid vertex indices.
    pub open spec fn spec_toposortsteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> bool {
        forall|v: int, i: int|
            0 <= v < graph@.len() && 0 <= i < graph@[v].len()
            ==> (#[trigger] graph@[v][i]) < graph@.len()
    }

    /// Whether there is a directed edge from u to v in the graph.
    pub open spec fn spec_has_edge(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, u: int, v: int) -> bool {
        0 <= u < graph@.len()
        && exists|i: int| 0 <= i < graph@[u].len() && (#[trigger] graph@[u][i]) == v
    }

    /// Whether a sequence of vertex indices forms a valid path in the graph.
    pub open spec fn spec_is_path(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, path: Seq<int>) -> bool {
        path.len() >= 1
        && (forall|k: int| 0 <= k < path.len() ==> 0 <= #[trigger] path[k] < graph@.len())
        && (forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1 ==> spec_has_edge(graph, path[k], path[k + 1]))
    }

    /// Whether vertex v is reachable from vertex u (Definition 55.3, reachability).
    pub open spec fn spec_reachable(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, u: int, v: int) -> bool {
        exists|path: Seq<int>| spec_is_path(graph, path) && path[0] == u && #[trigger] path.last() == v
    }

    /// Whether the graph is a directed acyclic graph (Definition 55.11).
    pub open spec fn spec_is_dag(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> bool {
        !exists|path: Seq<int>| spec_is_path(graph, path) && path.len() >= 2 && path[0] == #[trigger] path.last()
    }

    /// Whether a sequence is a valid topological ordering (Definition 55.12).
    pub open spec fn spec_is_topo_order(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, order: Seq<usize>) -> bool {
        order.len() == graph@.len()
        && order.no_duplicates()
        && (forall|k: int| 0 <= k < order.len() ==> (#[trigger] order[k] as int) < graph@.len())
        && (forall|i: int, j: int| #![trigger order[i], order[j]]
            0 <= i < order.len() && 0 <= j < order.len()
            && spec_has_edge(graph, order[i] as int, order[j] as int)
            ==> i < j)
    }

    /// Whether a set of vertices is strongly connected (Definition 55.14).
    pub open spec fn spec_strongly_connected(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, vertices: Set<int>) -> bool {
        forall|u: int, v: int| #![trigger vertices.contains(u), vertices.contains(v)]
            vertices.contains(u) && vertices.contains(v)
            ==> spec_reachable(graph, u, v)
    }

    /// Whether vertex v belongs to at least one component.
    pub open spec fn spec_vertex_covered(components: Seq<Set<int>>, v: int) -> bool {
        exists|c: int| 0 <= c < components.len() && (#[trigger] components[c]).contains(v)
    }

    /// Whether components form a valid SCC decomposition in topological order (Definition 55.17).
    pub open spec fn spec_is_scc(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, components: Seq<Set<int>>) -> bool {
        // Each component is strongly connected.
        (forall|c: int| 0 <= c < components.len()
            ==> #[trigger] spec_strongly_connected(graph, components[c]))
        // Components partition the vertex set.
        && (forall|v: int| 0 <= v < graph@.len() ==>
            #[trigger] spec_vertex_covered(components, v))
        // Components are disjoint.
        && (forall|c1: int, c2: int| #![trigger components[c1], components[c2]]
            0 <= c1 < components.len() && 0 <= c2 < components.len() && c1 != c2
            ==> components[c1].disjoint(components[c2]))
        // Inter-component edges go forward (topological order).
        && (forall|c1: int, c2: int, u: int, v: int| #![trigger components[c1].contains(u), components[c2].contains(v)]
            0 <= c1 < components.len() && 0 <= c2 < components.len()
            && components[c1].contains(u) && components[c2].contains(v)
            && spec_has_edge(graph, u, v) && c1 != c2
            ==> c1 < c2)
    }

    /// Whether all neighbors of vertex v are visited.
    pub open spec fn spec_vertex_neighbors_visited(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: Seq<bool>,
        v: int,
    ) -> bool {
        forall|i: int| 0 <= i < graph@[v].len()
            ==> visited[#[trigger] graph@[v][i] as int]
    }

    /// Whether all finish_order elements have their neighbors visited.
    pub open spec fn spec_neighbors_explored(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: Seq<bool>,
        finish_order: Seq<usize>,
    ) -> bool {
        forall|k: int| 0 <= k < finish_order.len()
            ==> #[trigger] spec_vertex_neighbors_visited(graph, visited, finish_order[k] as int)
    }

    /// Edge ordering: for any edge u->v in finish_order, v appears before u.
    pub open spec fn spec_edge_ordered(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        finish_order: Seq<usize>,
    ) -> bool {
        forall|a: int, b: int|
            #![trigger finish_order[a], finish_order[b]]
            0 <= a < finish_order.len() && 0 <= b < finish_order.len()
            && spec_has_edge(graph, finish_order[a] as int, finish_order[b] as int)
            ==> b < a
    }

    //		Section 7. proof fns/broadcast groups


    /// A single edge implies reachability.
    proof fn lemma_edge_implies_reachable(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        u: int,
        v: int,
    )
        requires
            spec_has_edge(graph, u, v),
            spec_toposortsteph_wf(graph),
        ensures
            spec_reachable(graph, u, v),
    {
        let path = seq![u, v];
        // Veracity: NEEDED assert (speed hint)
        assert forall|k: int| 0 <= k < path.len()
            implies 0 <= #[trigger] path[k] < graph@.len() by {
            if k == 0 {
            } else {
                // v is a neighbor, wf gives v < graph@.len().
                let i = choose|i: int| 0 <= i < graph@[u].len() && (#[trigger] graph@[u][i]) == v;
// Veracity: UNNEEDED assert                 assert(graph@[u][i] < graph@.len());
            }
        };
        // Veracity: NEEDED assert
        assert forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1
            implies spec_has_edge(graph, path[k], path[k + 1]) by {};
        // Veracity: NEEDED assert (speed hint)
        assert(spec_is_path(graph, path));
        // Veracity: NEEDED assert
        assert(path[0] == u && path.last() == v);
    }

    /// A vertex is reachable from itself.
    proof fn lemma_self_reachable(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        u: int,
    )
        requires 0 <= u < graph@.len(),
        ensures spec_reachable(graph, u, u),
    {
        let path = seq![u];
        // Veracity: NEEDED assert (speed hint)
        assert forall|k: int| 0 <= k < path.len()
            implies 0 <= #[trigger] path[k] < graph@.len() by {};
        // Veracity: NEEDED assert
        assert forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1
            implies spec_has_edge(graph, path[k], path[k + 1]) by {};
        // Veracity: NEEDED assert
        assert(spec_is_path(graph, path) && path[0] == u && path.last() == u);
    }

    /// Reachable via an edge: if u->v edge and v reaches w, then u reaches w.
    proof fn lemma_reachable_via_edge(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        u: int,
        v: int,
        w: int,
    )
        requires
            spec_has_edge(graph, u, v),
            spec_reachable(graph, v, w),
            spec_toposortsteph_wf(graph),
        ensures
            spec_reachable(graph, u, w),
    {
        let p = choose|path: Seq<int>|
            spec_is_path(graph, path) && path[0] == v && #[trigger] path.last() == w;
        let new_path = seq![u] + p;
// Veracity: UNNEEDED assert         assert(new_path.len() >= 2);
// Veracity: UNNEEDED assert         assert(new_path[0] == u);
        // Veracity: NEEDED assert
        assert(new_path.last() == w) by {
// Veracity: UNNEEDED assert             assert(new_path[new_path.len() - 1] == p[p.len() - 1]);
        };
// Veracity: UNNEEDED assert         assert forall|k: int| 0 <= k < new_path.len()
// Veracity: UNNEEDED assert             implies 0 <= #[trigger] new_path[k] < graph@.len() by {
// Veracity: UNNEEDED assert             if k == 0 {
// Veracity: UNNEEDED assert             } else {
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(new_path[k] == p[k - 1]);
// Veracity: UNNEEDED assert             }
// Veracity: UNNEEDED assert         };
        // Veracity: NEEDED assert
        assert forall|k: int| #![trigger new_path[k]] 0 <= k < new_path.len() - 1
            implies spec_has_edge(graph, new_path[k], new_path[k + 1]) by {
            if k == 0 {
                // Veracity: NEEDED assert (speed hint)
                assert(new_path[0] == u);
// Veracity: UNNEEDED assert                 assert(new_path[1] == p[0]);
                // Veracity: NEEDED assert (speed hint)
                assert(p[0] == v);
            } else {
                // Veracity: NEEDED assert (speed hint)
                assert(new_path[k] == p[k - 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(new_path[k + 1] == p[k]);
            }
        };
// Veracity: UNNEEDED assert         assert(spec_is_path(graph, new_path));
    }

    /// If u reaches v and there is an edge v->u, the graph has a cycle (contradicts DAG).
    proof fn lemma_reachable_edge_contradicts_dag(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        u: int,
        v: int,
    )
        requires
            spec_toposortsteph_wf(graph),
            spec_reachable(graph, u, v),
            spec_has_edge(graph, v, u),
        ensures
            !spec_is_dag(graph),
    {
        let p = choose|path: Seq<int>|
            spec_is_path(graph, path) && path[0] == u && #[trigger] path.last() == v;
        let cycle = p.push(u);
        // Veracity: NEEDED assert (speed hint)
        assert(cycle.len() >= 2);
        // Veracity: NEEDED assert (speed hint)
        assert(cycle[0] == u);
        // Veracity: NEEDED assert (speed hint)
        assert(cycle.last() == u) by {
// Veracity: UNNEEDED assert             assert(cycle[cycle.len() - 1] == u);
        };
        // Veracity: NEEDED assert (speed hint)
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < p.len() as int {
// Veracity: UNNEEDED assert                 assert(cycle[k] == p[k]);
            } else {
                // k == p.len(), cycle[k] == u. spec_has_edge(v, u) + wf → 0 <= u < graph@.len().
                let i = choose|i: int| 0 <= i < graph@[v].len() && (#[trigger] graph@[v][i]) == u;
                // Veracity: NEEDED assert (speed hint)
                assert(graph@[v][i] < graph@.len());
            }
        };
        // Veracity: NEEDED assert
        assert forall|k: int| #![trigger cycle[k]] 0 <= k < cycle.len() - 1
            implies spec_has_edge(graph, cycle[k], cycle[k + 1]) by {
            if k < p.len() as int - 1 {
// Veracity: UNNEEDED assert                 assert(cycle[k] == p[k]);
                // Veracity: NEEDED assert (speed hint)
                assert(cycle[k + 1] == p[k + 1]);
            } else {
                // k == p.len() - 1: cycle[k] == p.last() == v, cycle[k+1] == u.
// Veracity: UNNEEDED assert                 assert(cycle[k] == p[p.len() - 1]);
                // Veracity: NEEDED assert (speed hint)
                assert(p[p.len() - 1] == v);
// Veracity: UNNEEDED assert                 assert(cycle[k + 1] == u);
            }
        };
        // Veracity: NEEDED assert (speed hint)
        assert(spec_is_path(graph, cycle) && cycle.len() >= 2 && cycle[0] == cycle.last());
    }

    //		Section 8. traits


    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG (Algorithm 55.13).
        /// - Alg Analysis: APAS (Ch55 Ex 55.6): Work O(|V| + |E|), Span O(|V| + |E|) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (order: AVLTreeSeqStEphS<usize>)
            requires
                spec_toposortsteph_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                order@.len() == graph@.len(),
                spec_is_dag(graph) ==> spec_is_topo_order(graph, order@),
            ;
    }

    //		Section 9. impls


    /// Recursive DFS that appends vertices in finish order.
    /// Also used by SCCStEph::compute_finish_order.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS appending vertices at finish time; St sequential.
    pub fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: &mut ArraySeqStEphS<bool>,
        finish_order: &mut Vec<usize>,
        vertex: usize,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> (#[trigger] old(finish_order)@[k] as int) < graph@.len(),
        ensures
            visited@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() >= old(finish_order)@.len(),
            forall|k: int| 0 <= k < finish_order@.len()
                ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
            visited@[vertex as int],
            finish_order@.len() + spec_num_false(visited@)
                == old(finish_order)@.len() + spec_num_false(old(visited)@),
            // Prefix preservation.
            forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> finish_order@[k] == old(finish_order)@[k],
            // New elements were unvisited at call start.
            forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                ==> !old(visited)@[#[trigger] finish_order@[k] as int],
            // Conditional: all elements in finish_order are visited.
            (forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                ==> (forall|k: int| 0 <= k < finish_order@.len()
                    ==> visited@[#[trigger] finish_order@[k] as int]),
            // Conditional: no duplicates.
            (old(finish_order)@.no_duplicates()
                && (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int]))
                ==> finish_order@.no_duplicates(),
            // Vertex is last when unvisited.
            !old(visited)@[vertex as int]
                ==> finish_order@[finish_order@.len() - 1] == vertex,
            // If vertex was already visited, finish_order is unchanged.
            old(visited)@[vertex as int]
                ==> finish_order@.len() == old(finish_order)@.len(),
            // Conditional: neighbors explored.
            ((forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@))
                ==> spec_neighbors_explored(graph, visited@, finish_order@),
            // Conditional: edge ordered (needs DAG).
            (spec_is_dag(graph)
                && old(finish_order)@.no_duplicates()
                && (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@)
                && spec_edge_ordered(graph, old(finish_order)@))
                ==> spec_edge_ordered(graph, finish_order@),
            // New elements reachable from vertex.
            !old(visited)@[vertex as int]
                ==> (forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                    ==> spec_reachable(graph, vertex as int, #[trigger] finish_order@[k] as int)),
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        if *visited.nth(vertex) {
            return;
        }
// Veracity: UNNEEDED assert         assert(!old(visited)@[vertex as int]);
// Veracity: UNNEEDED assert         assert(vertex < visited.spec_len());
        let set_ok = visited.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ == old(visited)@.update(vertex, true).
        proof { lemma_bool_array_set_view(visited, old(visited)@, vertex as int, true); }
        // Veracity: NEEDED assert (speed hint)
        assert(visited@ =~= old(visited)@.update(vertex as int, true));

        // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();

        // Bridge neighbors to graph view.
        // Veracity: NEEDED assert (speed hint)
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
        // Veracity: NEEDED assert (speed hint)
        assert(neighbors@ =~= graph@[vertex as int]);

        // Establish neighbors_explored and edge_ordered at loop entry.
        proof {
            if (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@)
            {
                // Veracity: NEEDED assert
                assert forall|k: int| 0 <= k < finish_order@.len()
                    implies #[trigger] spec_vertex_neighbors_visited(
                        graph, visited@, finish_order@[k] as int) by {
                    // Veracity: NEEDED assert
                    assert(spec_vertex_neighbors_visited(
                        graph, old(visited)@, old(finish_order)@[k] as int));
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|ii: int| 0 <= ii < graph@[finish_order@[k] as int].len()
                        implies visited@[#[trigger] graph@[finish_order@[k] as int][ii] as int] by {
// Veracity: UNNEEDED assert                         assert(old(visited)@[graph@[old(finish_order)@[k] as int][ii] as int]);
                    };
                };
            }
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
                visited@.len() == graph@.len(),
                spec_toposortsteph_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() >= old(finish_order)@.len(),
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                visited@[vertex as int],
                finish_order@.len() + spec_num_false(visited@) + 1
                    == old(finish_order)@.len() + spec_num_false(old(visited)@),
                // Prefix preservation from outer old.
                forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> finish_order@[k] == old(finish_order)@[k],
                // New elements since outer old were unvisited then.
                forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                    ==> !old(visited)@[#[trigger] finish_order@[k] as int],
                // Conditional: all elements visited.
                (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                    ==> (forall|k: int| 0 <= k < finish_order@.len()
                        ==> visited@[#[trigger] finish_order@[k] as int]),
                // Conditional: no duplicates.
                (old(finish_order)@.no_duplicates()
                    && (forall|k: int| 0 <= k < old(finish_order)@.len()
                        ==> old(visited)@[#[trigger] old(finish_order)@[k] as int]))
                    ==> finish_order@.no_duplicates(),
                // Conditional: vertex not in finish_order.
                (old(finish_order)@.no_duplicates()
                    && (forall|k: int| 0 <= k < old(finish_order)@.len()
                        ==> old(visited)@[#[trigger] old(finish_order)@[k] as int]))
                    ==> (forall|k: int| 0 <= k < finish_order@.len()
                        ==> finish_order@[k] != vertex),
                // Neighbors of vertex processed so far are visited.
                forall|j: int| 0 <= j < i as int
                    ==> visited@[#[trigger] graph@[vertex as int][j] as int],
                // Conditional: neighbors explored.
                ((forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                    && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@))
                    ==> spec_neighbors_explored(graph, visited@, finish_order@),
                // Conditional: edge ordered.
                (spec_is_dag(graph)
                    && old(finish_order)@.no_duplicates()
                    && (forall|k: int| 0 <= k < old(finish_order)@.len()
                        ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                    && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@)
                    && spec_edge_ordered(graph, old(finish_order)@))
                    ==> spec_edge_ordered(graph, finish_order@),
                // New elements reachable from vertex.
                !old(visited)@[vertex as int]
                    ==> (forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                        ==> spec_reachable(graph, vertex as int, #[trigger] finish_order@[k] as int)),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert
            assert(neighbor == neighbors@[i as int]);
            let ghost fo_pre = finish_order@;
            let ghost vis_pre = visited@;
// Veracity: UNNEEDED assert             assert(spec_has_edge(graph, vertex as int, neighbor as int)) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                 assert(graph@[vertex as int][i as int] == neighbor);
// Veracity: UNNEEDED assert             };
            dfs_finish_order(graph, visited, finish_order, neighbor);
            proof {
                // Chain prefix preservation.
// Veracity: UNNEEDED assert                 assert forall|k: int| 0 <= k < old(finish_order)@.len()
// Veracity: UNNEEDED assert                     implies finish_order@[k] == old(finish_order)@[k] by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(finish_order@[k] == fo_pre[k]);
// Veracity: UNNEEDED assert                 };
                // Chain new-elements-unvisited.
                // Veracity: NEEDED assert (speed hint)
                assert forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                    implies !old(visited)@[#[trigger] finish_order@[k] as int] by {
                    if k < fo_pre.len() as int {
// Veracity: UNNEEDED assert                         assert(finish_order@[k] == fo_pre[k]);
                    } else {
// Veracity: UNNEEDED assert                         assert(!vis_pre[finish_order@[k] as int]);
                        if old(visited)@[finish_order@[k] as int] {
                            // Veracity: NEEDED assert (speed hint)
                            assert(vis_pre[finish_order@[k] as int]);
                        }
                    }
                };
                // Neighbor is visited (monotonicity from recursive call ensures).
// Veracity: UNNEEDED assert                 assert(visited@[neighbor as int]);
                // Prior neighbors still visited (visited monotonic).
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < i as int
                    implies visited@[#[trigger] graph@[vertex as int][j] as int] by {
                    // vis_pre had this, visited grew.
// Veracity: UNNEEDED assert                     assert(vis_pre[graph@[vertex as int][j] as int]);
                };
                // Reachability: new elements from recursive call reachable from vertex.
                if !old(visited)@[vertex as int] {
                    // If neighbor was already visited, the call added no elements.
                    if vis_pre[neighbor as int] {
                        // Veracity: NEEDED assert (speed hint)
                        assert(finish_order@.len() == fo_pre.len());
                    }
                    // Veracity: NEEDED assert
                    assert forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                        implies spec_reachable(graph, vertex as int, #[trigger] finish_order@[k] as int) by {
                        if k < fo_pre.len() as int {
                            // From loop invariant.
                        } else {
                            // New from recursive call. Neighbor must have been unvisited.
// Veracity: UNNEEDED assert                             assert(!vis_pre[neighbor as int]);
                            // Veracity: NEEDED assert (speed hint)
                            assert(spec_reachable(graph, neighbor as int, finish_order@[k] as int));
                            lemma_reachable_via_edge(graph, vertex as int, neighbor as int, finish_order@[k] as int);
                        }
                    };
                }
            }
            i = i + 1;
        }
        // All neighbors of vertex are visited.
// Veracity: UNNEEDED assert         assert forall|j: int| 0 <= j < graph@[vertex as int].len()
// Veracity: UNNEEDED assert             implies visited@[#[trigger] graph@[vertex as int][j] as int] by {};
        let ghost fo_pre_push = finish_order@;
        let ghost vis_at_push = visited@;
        finish_order.push(vertex);
        proof {
            // Veracity: NEEDED assert (speed hint)
            assert(finish_order@ =~= fo_pre_push.push(vertex));
            // Neighbors explored for the extended sequence.
            // For elements at positions < fo_pre_push.len(): same as before (loop invariant).
            // For vertex (at position fo_pre_push.len()): all neighbors visited (proved above).
            if (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@)
            {
// Veracity: UNNEEDED assert                 assert forall|k: int| 0 <= k < finish_order@.len()
// Veracity: UNNEEDED assert                     implies #[trigger] spec_vertex_neighbors_visited(graph, visited@, finish_order@[k] as int) by {
// Veracity: UNNEEDED assert                     if k < fo_pre_push.len() as int {
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                         assert(finish_order@[k] == fo_pre_push[k]);
// Veracity: UNNEEDED assert                         // Loop invariant gave neighbors explored for fo_pre_push.
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                         assert(spec_vertex_neighbors_visited(graph, vis_at_push, fo_pre_push[k] as int));
// Veracity: UNNEEDED assert                         // visited@ grew from vis_at_push by push (which doesn't change visited).
// Veracity: UNNEEDED assert                     } else {
// Veracity: UNNEEDED assert                         // k == fo_pre_push.len(), finish_order@[k] == vertex.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert(finish_order@[k] == vertex);
// Veracity: UNNEEDED assert                     }
// Veracity: UNNEEDED assert                 };
            }
            // Edge ordered for the extended sequence.
            if spec_is_dag(graph)
                && old(finish_order)@.no_duplicates()
                && (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored(graph, old(visited)@, old(finish_order)@)
                && spec_edge_ordered(graph, old(finish_order)@)
            {
                // Veracity: NEEDED assert
                assert forall|a: int, b: int|
                    0 <= a < finish_order@.len() && 0 <= b < finish_order@.len()
                    && spec_has_edge(graph, finish_order@[a] as int, finish_order@[b] as int)
                    implies ({
                        #[trigger] finish_order@[a]; #[trigger] finish_order@[b];
                        b < a
                    }) by {
                    if a < fo_pre_push.len() as int && b < fo_pre_push.len() as int {
                        // Both in pre-push range: loop invariant.
                        // Veracity: NEEDED assert (speed hint)
                        assert(finish_order@[a] == fo_pre_push[a]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(finish_order@[b] == fo_pre_push[b]);
                    } else if a == fo_pre_push.len() as int && b < fo_pre_push.len() as int {
                        // Edge from vertex to element at b < a. b < a = fo_pre_push.len(). ✓
                    } else if a < fo_pre_push.len() as int && b == fo_pre_push.len() as int {
                        // Edge from finish_order@[a] to vertex. Impossible:
                        let src = finish_order@[a];
// Veracity: UNNEEDED assert                         assert(finish_order@[b] == vertex);
                        if a < old(finish_order)@.len() as int {
                            // Old element: its neighbors are visited in old(visited).
                            // vertex was !old(visited). So vertex not a neighbor.
                            // Veracity: NEEDED assert
                            assert(spec_vertex_neighbors_visited(graph, old(visited)@, old(finish_order)@[a] as int));
// Veracity: UNNEEDED assert                             assert(finish_order@[a] == old(finish_order)@[a]);
// Veracity: UNNEEDED assert                             assert(!old(visited)@[vertex as int]);
                            // spec_has_edge(src, vertex) means vertex is a neighbor of src.
                            // But all neighbors of src are visited in old(visited). vertex is not. Contradiction.
                        } else {
                            // New element: reachable from vertex. Edge to vertex → cycle → ¬DAG.
// Veracity: UNNEEDED assert                             assert(spec_reachable(graph, vertex as int, finish_order@[a] as int));
                            lemma_reachable_edge_contradicts_dag(graph, vertex as int, finish_order@[a] as int);
                        }
                    } else {
                        // Both at fo_pre_push.len(): self-edge on vertex. DAG forbids self-loops.
                        // Veracity: NEEDED assert (speed hint)
                        assert(finish_order@[a] == vertex);
// Veracity: UNNEEDED assert                         assert(finish_order@[b] == vertex);
                        lemma_self_reachable(graph, vertex as int);
                        lemma_reachable_edge_contradicts_dag(graph, vertex as int, vertex as int);
                    }
                };
            }
            // Reachability: vertex reaches itself (for the push).
            if !old(visited)@[vertex as int] {
                lemma_self_reachable(graph, vertex as int);
            }
        }
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS with ancestor tracking; St sequential.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: &mut ArraySeqStEphS<bool>,
        rec_stack: &mut ArraySeqStEphS<bool>,
        finish_order: &mut Vec<usize>,
        vertex: usize,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
        ensures
            visited@.len() == graph@.len(),
            rec_stack@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() + spec_num_false(visited@) <=
                old(finish_order)@.len() + spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        if *rec_stack.nth(vertex) {
            return false;
        }
        if *visited.nth(vertex) {
            return true;
        }

// Veracity: UNNEEDED assert         assert(!old(visited)@[vertex as int]);
        // Veracity: NEEDED assert (speed hint)
        assert(vertex < visited.spec_len());
// Veracity: UNNEEDED assert         assert(vertex < rec_stack.spec_len());
        let ok1 = visited.set(vertex, true);
        let ok2 = rec_stack.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ after both sets.
        proof { lemma_bool_array_set_view(visited, old(visited)@, vertex as int, true); }
// Veracity: UNNEEDED assert         assert(visited@ =~= old(visited)@.update(vertex as int, true));

// Veracity: UNNEEDED assert         assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();

        // Bridge neighbors to graph view.
// Veracity: UNNEEDED assert         assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
// Veracity: UNNEEDED assert         assert(neighbors@ =~= graph@[vertex as int]);

        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len as int == neighbors.spec_len(),
                neighbors_len == graph@[vertex as int].len(),
                neighbors@ =~= graph@[vertex as int],
                *neighbors == graph.spec_index(vertex as int),
                (vertex as int) < graph@.len(),
                vertex < rec_stack.spec_len(),
                visited@.len() == graph@.len(),
                visited.spec_len() == visited@.len(),
                rec_stack@.len() == graph@.len(),
                rec_stack.spec_len() == rec_stack@.len(),
                spec_toposortsteph_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() + spec_num_false(visited@) <
                    old(finish_order)@.len() + spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert
            assert(neighbor == neighbors@[i as int]);
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, finish_order, neighbor) {
                return false;
            }
            i = i + 1;
        }

        let ok3 = rec_stack.set(vertex, false);
        finish_order.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS + cycle check + reverse; St sequential.
    pub fn topological_sort_opt(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (topo_order: Option<AVLTreeSeqStEphS<usize>>)
        requires
            spec_toposortsteph_wf(graph),
            graph@.len() < usize::MAX,
        ensures
            topo_order.is_some() <==> spec_is_dag(graph),
            topo_order.is_some() ==> spec_is_topo_order(graph, topo_order.unwrap()@),
    {
        // Delegate to proved has_cycle (CycleDetect) and topo_sort (TopoSort).
        if CycleDetectStEph::has_cycle(graph) {
            // has_cycle ensures: has_cycle == !spec_is_dag(graph).
            // So !spec_is_dag(graph) holds here. Return None.
            None
        } else {
            // !has_cycle means spec_is_dag(graph).
            let order = TopoSortStEph::topo_sort(graph);
            // topo_sort ensures: spec_is_dag(graph) ==> spec_is_topo_order(graph, order@).
            Some(order)
        }
    }

    impl TopoSortStEphTrait for TopoSortStEph {
        /// Returns sequence of vertices in topological order.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS finish order + reverse; St sequential.
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (order: AVLTreeSeqStEphS<usize>)
        {
            let n = graph.length();
            let f_false = |_x: usize| -> (r: bool) ensures !r { false };
            let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
            let mut finish_order: Vec<usize> = Vec::new();

            proof {
// Veracity: UNNEEDED assert                 assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                     assert(!visited.seq@[j]);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                     assert(visited@[j] == visited.seq@[j]@);
// Veracity: UNNEEDED assert                 }
                lemma_all_false_num_false_eq_len(visited@);
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    n < usize::MAX,
                    visited@.len() == n,
                    visited.spec_len() == visited@.len(),
                    spec_toposortsteph_wf(graph),
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                    finish_order@.len() + spec_num_false(visited@) == n,
                    forall|j: int| #![trigger visited@[j]] 0 <= j < start ==> visited@[j],
                    // Topo sort invariants.
                    finish_order@.no_duplicates(),
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> visited@[#[trigger] finish_order@[k] as int],
                    spec_neighbors_explored(graph, visited@, finish_order@),
                    spec_is_dag(graph) ==> spec_edge_ordered(graph, finish_order@),
                decreases n - start,
            {
// Veracity: UNNEEDED assert                 assert(start < visited.spec_len());
                proof { lemma_bool_view_eq_spec_index(&visited); }
                if !*visited.nth(start) {
                    let ghost old_visited = visited@;
                    let ghost old_fo = finish_order@;
                    dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                    proof {
                        // Monotonicity preserves prior visited entries.
                        // Veracity: NEEDED assert
                        assert forall|j: int| #![trigger visited@[j]] 0 <= j < start + 1 implies visited@[j] by {
                            if j < start as int {
                                // Veracity: NEEDED assert
                                assert(old_visited[j]);
                            }
                        };
                        // Conditional ensures fire because old state satisfied invariants.
                        // no_duplicates: old had no_dup and elements visited → new has no_dup.
// Veracity: UNNEEDED assert                         assert(old_fo.no_duplicates());
// Veracity: UNNEEDED assert                         assert(forall|k: int| 0 <= k < old_fo.len()
// Veracity: UNNEEDED assert                             ==> old_visited[#[trigger] old_fo[k] as int]);
                        // elements visited: same condition → new elements visited.
                        // neighbors_explored: condition + old neighbors explored → new.
// Veracity: UNNEEDED assert                         assert(spec_neighbors_explored(graph, old_visited, old_fo));
                        // edge_ordered: DAG + all conditions → new edge ordered.
                    }
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            proof {
                lemma_all_true_num_false_zero(visited@);
            }
// Veracity: UNNEEDED assert             assert(finish_order@.len() == n);
// Veracity: UNNEEDED assert             assert(finish_order@.len() < usize::MAX);
            let ghost fo_final = finish_order@;
            let result_len = finish_order.len();
            let mut reversed: Vec<usize> = Vec::new();
            let mut k: usize = result_len;
            while k > 0
                invariant
                    k <= result_len,
                    result_len == finish_order@.len(),
                    result_len == n,
                    result_len < usize::MAX,
                    reversed@.len() == (result_len - k) as nat,
                    reversed@.len() < usize::MAX,
                    finish_order@ == fo_final,
                    // Reversal relationship.
                    forall|m: int| #![trigger reversed@[m]]
                        0 <= m < reversed@.len()
                        ==> reversed@[m] == fo_final[(result_len - 1 - m) as int],
                decreases k,
            {
                k = k - 1;
                reversed.push(finish_order[k]);
            }
// Veracity: UNNEEDED assert             assert(reversed@.len() == n);
            // Veracity: NEEDED assert (speed hint)
            assert(reversed@.len() < usize::MAX);
            // Prove reversed properties from finish_order properties.
            proof {
                // reversed is the reverse of fo_final.
                // Veracity: NEEDED assert (speed hint)
                assert forall|m: int| 0 <= m < reversed@.len()
                    implies reversed@[m] == fo_final[(n - 1 - m) as int] by {};
                // reversed has no duplicates (from fo_final no_duplicates + reversal injection).
// Veracity: UNNEEDED assert                 assert(reversed@.no_duplicates()) by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert
// Veracity: UNNEEDED assert                     assert forall|i: int, j: int|
// Veracity: UNNEEDED assert                         0 <= i < reversed@.len() && 0 <= j < reversed@.len() && i != j
// Veracity: UNNEEDED assert                         implies reversed@[i] != reversed@[j] by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert(reversed@[i] == fo_final[(n - 1 - i) as int]);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert(reversed@[j] == fo_final[(n - 1 - j) as int]);
// Veracity: UNNEEDED assert                         // n-1-i != n-1-j since i != j.
// Veracity: UNNEEDED assert                         // fo_final.no_duplicates() → fo_final[n-1-i] != fo_final[n-1-j].
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                 };
                // reversed elements are valid indices.
                // Veracity: NEEDED assert (speed hint)
                assert forall|m: int| 0 <= m < reversed@.len()
                    implies (reversed@[m] as int) < graph@.len() by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(reversed@[m] == fo_final[(n - 1 - m) as int]);
                };
                // map_values identity for usize.
// Veracity: UNNEEDED assert                 assert(reversed@.map_values(|t: usize| t@) =~= reversed@) by {
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert(reversed@.map_values(|t: usize| t@).len() == reversed@.len());
// Veracity: UNNEEDED assert                     // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                     assert forall|i: int| 0 <= i < reversed@.len()
// Veracity: UNNEEDED assert                         implies reversed@.map_values(|t: usize| t@)[i] == reversed@[i] by {};
// Veracity: UNNEEDED assert                 };
            }
            // Veracity: NEEDED assert
            proof { assert(crate::vstdplus::feq::feq::obeys_feq_full_trigger::<usize>()); }
            let order = AVLTreeSeqStEphS::from_vec(reversed);
            proof {
                // order@ =~= reversed@ (from from_vec ensures + map_values identity).
                // Veracity: NEEDED assert (speed hint)
                assert(order@ =~= reversed@);
// Veracity: UNNEEDED assert                 assert(order@.len() == n);
                // Prove topo_order under DAG assumption.
                if spec_is_dag(graph) {
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_edge_ordered(graph, fo_final));
                    // Edge ordering for reversed: for edge order[i]→order[j], need i < j.
                    // Veracity: NEEDED assert
                    assert forall|i: int, j: int|
                        #![trigger order@[i], order@[j]]
                        0 <= i < order@.len() && 0 <= j < order@.len()
                        && spec_has_edge(graph, order@[i] as int, order@[j] as int)
                        implies i < j by {
                        // order@[i] = fo_final[n-1-i], order@[j] = fo_final[n-1-j].
                        // Veracity: NEEDED assert (speed hint)
                        assert(order@[i] == fo_final[(n - 1 - i) as int]);
                        // Veracity: NEEDED assert (speed hint)
                        assert(order@[j] == fo_final[(n - 1 - j) as int]);
                        // Edge fo_final[n-1-i] → fo_final[n-1-j].
                        // spec_edge_ordered: (n-1-j) < (n-1-i), so i < j.
                        let a = (n as int) - 1 - i;
                        let b = (n as int) - 1 - j;
// Veracity: UNNEEDED assert                         assert(fo_final[a] == order@[i]);
// Veracity: UNNEEDED assert                         assert(fo_final[b] == order@[j]);
                    };
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_is_topo_order(graph, order@));
                }
            }
            order
        }
    } // impl TopoSortStEphTrait

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for TopoSortStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TopoSortStEph")
        }
    }

    impl std::fmt::Display for TopoSortStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TopoSortStEph")
        }
    }
}
