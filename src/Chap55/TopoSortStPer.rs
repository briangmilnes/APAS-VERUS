//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Persistent (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using decreasing finish times.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_all_false_num_false_eq_len, lemma_all_true_num_false_zero};
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;
    pub struct TopoSortStPer;

    // 6. spec fns

    /// Well-formed adjacency list for persistent graph representation.
    pub open spec fn spec_toposortstper_wf(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> bool {
        forall|v: int, i: int|
            0 <= v < graph@.len() && 0 <= i < graph@[v].len()
            ==> (#[trigger] graph@[v][i]) < graph@.len()
    }

    /// Whether there is a directed edge from u to v in the graph.
    pub open spec fn spec_has_edge_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, u: int, v: int) -> bool {
        0 <= u < graph@.len()
        && exists|i: int| 0 <= i < graph@[u].len() && (#[trigger] graph@[u][i]) == v
    }

    /// Whether a sequence of vertex indices forms a valid path in the graph.
    pub open spec fn spec_is_path_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, path: Seq<int>) -> bool {
        path.len() >= 1
        && (forall|k: int| 0 <= k < path.len() ==> 0 <= #[trigger] path[k] < graph@.len())
        && (forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1 ==> spec_has_edge_per(graph, path[k], path[k + 1]))
    }

    /// Whether vertex v is reachable from vertex u (Definition 55.3, reachability).
    pub open spec fn spec_reachable_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, u: int, v: int) -> bool {
        exists|path: Seq<int>| spec_is_path_per(graph, path) && path[0] == u && #[trigger] path.last() == v
    }

    /// Whether the graph is a directed acyclic graph (Definition 55.11).
    pub open spec fn spec_is_dag_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> bool {
        !exists|path: Seq<int>| spec_is_path_per(graph, path) && path.len() >= 2 && path[0] == #[trigger] path.last()
    }

    /// Whether a sequence is a valid topological ordering (Definition 55.12).
    pub open spec fn spec_is_topo_order_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, order: Seq<usize>) -> bool {
        order.len() == graph@.len()
        && order.no_duplicates()
        && (forall|k: int| 0 <= k < order.len() ==> (#[trigger] order[k] as int) < graph@.len())
        && (forall|i: int, j: int| #![trigger order[i], order[j]]
            0 <= i < order.len() && 0 <= j < order.len()
            && spec_has_edge_per(graph, order[i] as int, order[j] as int)
            ==> i < j)
    }

    /// Whether a set of vertices is strongly connected (Definition 55.14).
    pub open spec fn spec_strongly_connected_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, vertices: Set<int>) -> bool {
        forall|u: int, v: int| #![trigger vertices.contains(u), vertices.contains(v)]
            vertices.contains(u) && vertices.contains(v)
            ==> spec_reachable_per(graph, u, v)
    }

    /// Whether vertex v belongs to at least one component.
    pub open spec fn spec_vertex_covered_per(components: Seq<Set<int>>, v: int) -> bool {
        exists|c: int| 0 <= c < components.len() && (#[trigger] components[c]).contains(v)
    }

    /// Whether components form a valid SCC decomposition in topological order (Definition 55.17).
    pub open spec fn spec_is_scc_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, components: Seq<Set<int>>) -> bool {
        // Each component is strongly connected.
        (forall|c: int| 0 <= c < components.len()
            ==> #[trigger] spec_strongly_connected_per(graph, components[c]))
        // Components partition the vertex set.
        && (forall|v: int| 0 <= v < graph@.len() ==>
            #[trigger] spec_vertex_covered_per(components, v))
        // Components are disjoint.
        && (forall|c1: int, c2: int| #![trigger components[c1], components[c2]]
            0 <= c1 < components.len() && 0 <= c2 < components.len() && c1 != c2
            ==> components[c1].disjoint(components[c2]))
        // Inter-component edges go forward (topological order).
        && (forall|c1: int, c2: int, u: int, v: int| #![trigger components[c1].contains(u), components[c2].contains(v)]
            0 <= c1 < components.len() && 0 <= c2 < components.len()
            && components[c1].contains(u) && components[c2].contains(v)
            && spec_has_edge_per(graph, u, v) && c1 != c2
            ==> c1 < c2)
    }

    /// Whether all neighbors of vertex v are visited (persistent version).
    pub open spec fn spec_vertex_neighbors_visited_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: Seq<bool>,
        v: int,
    ) -> bool {
        forall|i: int| 0 <= i < graph@[v].len()
            ==> visited[#[trigger] graph@[v][i] as int]
    }

    /// Whether all finish_order elements have their neighbors visited (persistent version).
    pub open spec fn spec_neighbors_explored_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: Seq<bool>,
        finish_order: Seq<usize>,
    ) -> bool {
        forall|k: int| 0 <= k < finish_order.len()
            ==> #[trigger] spec_vertex_neighbors_visited_per(graph, visited, finish_order[k] as int)
    }

    /// Edge ordering for persistent version.
    pub open spec fn spec_edge_ordered_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        finish_order: Seq<usize>,
    ) -> bool {
        forall|a: int, b: int|
            #![trigger finish_order[a], finish_order[b]]
            0 <= a < finish_order.len() && 0 <= b < finish_order.len()
            && spec_has_edge_per(graph, finish_order[a] as int, finish_order[b] as int)
            ==> b < a
    }

    /// Bridge: for ArraySeqStPerS<usize>, view index equals spec_index.
    proof fn lemma_usize_per_view_eq_spec_index(a: &ArraySeqStPerS<usize>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: persistent graph adjacency list view at vertex.
    proof fn lemma_graph_per_view_bridge(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        neighbors: &ArraySeqStPerS<usize>,
        vertex: int,
    )
        requires
            0 <= vertex < graph@.len(),
            *neighbors == graph.spec_index(vertex),
        ensures
            neighbors@ =~= graph@[vertex],
    {
    }

    /// A single edge implies reachability (persistent version).
    proof fn lemma_edge_implies_reachable_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        u: int, v: int,
    )
        requires spec_has_edge_per(graph, u, v), spec_toposortstper_wf(graph),
        ensures spec_reachable_per(graph, u, v),
    {
        let path = seq![u, v];
        assert forall|k: int| 0 <= k < path.len()
            implies 0 <= #[trigger] path[k] < graph@.len() by {
            if k == 0 {} else {
                let i = choose|i: int| 0 <= i < graph@[u].len() && (#[trigger] graph@[u][i]) == v;
                assert(graph@[u][i] < graph@.len());
            }
        };
        assert forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1
            implies spec_has_edge_per(graph, path[k], path[k + 1]) by {};
        assert(spec_is_path_per(graph, path) && path[0] == u && path.last() == v);
    }

    /// A vertex is reachable from itself (persistent version).
    proof fn lemma_self_reachable_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        u: int,
    )
        requires 0 <= u < graph@.len(),
        ensures spec_reachable_per(graph, u, u),
    {
        let path = seq![u];
        assert forall|k: int| 0 <= k < path.len()
            implies 0 <= #[trigger] path[k] < graph@.len() by {};
        assert forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1
            implies spec_has_edge_per(graph, path[k], path[k + 1]) by {};
        assert(spec_is_path_per(graph, path) && path[0] == u && path.last() == u);
    }

    /// Reachable via edge (persistent version).
    proof fn lemma_reachable_via_edge_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        u: int, v: int, w: int,
    )
        requires
            spec_has_edge_per(graph, u, v),
            spec_reachable_per(graph, v, w),
            spec_toposortstper_wf(graph),
        ensures spec_reachable_per(graph, u, w),
    {
        let p = choose|path: Seq<int>|
            spec_is_path_per(graph, path) && path[0] == v && #[trigger] path.last() == w;
        let new_path = seq![u] + p;
        assert(new_path[0] == u);
        assert(new_path.last() == w) by {
            assert(new_path[new_path.len() - 1] == p[p.len() - 1]);
        };
        assert forall|k: int| 0 <= k < new_path.len()
            implies 0 <= #[trigger] new_path[k] < graph@.len() by {
            if k == 0 {} else { assert(new_path[k] == p[k - 1]); }
        };
        assert forall|k: int| #![trigger new_path[k]] 0 <= k < new_path.len() - 1
            implies spec_has_edge_per(graph, new_path[k], new_path[k + 1]) by {
            if k == 0 { assert(new_path[1] == p[0]); assert(p[0] == v); }
            else { assert(new_path[k] == p[k - 1]); assert(new_path[k + 1] == p[k]); }
        };
        assert(spec_is_path_per(graph, new_path));
    }

    /// Reachable + edge contradicts DAG (persistent version).
    proof fn lemma_reachable_edge_contradicts_dag_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        u: int, v: int,
    )
        requires
            spec_toposortstper_wf(graph),
            spec_reachable_per(graph, u, v),
            spec_has_edge_per(graph, v, u),
        ensures !spec_is_dag_per(graph),
    {
        let p = choose|path: Seq<int>|
            spec_is_path_per(graph, path) && path[0] == u && #[trigger] path.last() == v;
        let cycle = p.push(u);
        assert(cycle.len() >= 2);
        assert(cycle[0] == u);
        assert(cycle.last() == u) by { assert(cycle[cycle.len() - 1] == u); };
        assert forall|k: int| 0 <= k < cycle.len()
            implies 0 <= #[trigger] cycle[k] < graph@.len() by {
            if k < p.len() as int { assert(cycle[k] == p[k]); }
            else {
                let i = choose|i: int| 0 <= i < graph@[v].len() && (#[trigger] graph@[v][i]) == u;
                assert(graph@[v][i] < graph@.len());
            }
        };
        assert forall|k: int| #![trigger cycle[k]] 0 <= k < cycle.len() - 1
            implies spec_has_edge_per(graph, cycle[k], cycle[k + 1]) by {
            if k < p.len() as int - 1 { assert(cycle[k] == p[k]); assert(cycle[k + 1] == p[k + 1]); }
            else { assert(cycle[k] == p[p.len() - 1]); assert(p[p.len() - 1] == v); assert(cycle[k + 1] == u); }
        };
        assert(spec_is_path_per(graph, cycle) && cycle.len() >= 2 && cycle[0] == cycle.last());
    }

    // 8. traits

    pub trait TopoSortStPerTrait {
        /// Computes topological sort of a DAG (Algorithm 55.13)
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (order: AVLTreeSeqStPerS<usize>)
            requires
                spec_toposortstper_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                order@.len() == graph@.len(),
                spec_is_dag_per(graph) ==> spec_is_topo_order_per(graph, order@),
            ;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    fn dfs_finish_order(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: &mut Vec<bool>,
        finish_order: &mut Vec<usize>,
        vertex: usize,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
            forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> (#[trigger] old(finish_order)@[k] as int) < graph@.len(),
        ensures
            visited@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() + spec_num_false(visited@)
                == old(finish_order)@.len() + spec_num_false(old(visited)@),
            visited@[vertex as int],
            forall|k: int| 0 <= k < finish_order@.len()
                ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
            finish_order@.len() >= old(finish_order)@.len(),
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
                && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@))
                ==> spec_neighbors_explored_per(graph, visited@, finish_order@),
            // Conditional: edge ordered (needs DAG).
            (spec_is_dag_per(graph)
                && old(finish_order)@.no_duplicates()
                && (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@)
                && spec_edge_ordered_per(graph, old(finish_order)@))
                ==> spec_edge_ordered_per(graph, finish_order@),
            // New elements reachable from vertex.
            !old(visited)@[vertex as int]
                ==> (forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                    ==> spec_reachable_per(graph, vertex as int, #[trigger] finish_order@[k] as int)),
        decreases spec_num_false(old(visited)@),
    {
        assert(vertex < visited.len());
        if visited[vertex] {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        visited.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(spec_num_false(visited@) == spec_num_false(old(visited)@) - 1);
        assert(visited@.len() == graph@.len());

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

        // After set: visited@[vertex as int] is true.
        assert(visited@[vertex as int]);

        // Establish neighbors_explored at loop entry.
        proof {
            if (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@)
            {
                assert forall|k: int| 0 <= k < finish_order@.len()
                    implies #[trigger] spec_vertex_neighbors_visited_per(
                        graph, visited@, finish_order@[k] as int) by {
                    assert(spec_vertex_neighbors_visited_per(
                        graph, old(visited)@, old(finish_order)@[k] as int));
                    assert forall|ii: int| 0 <= ii < graph@[finish_order@[k] as int].len()
                        implies visited@[#[trigger] graph@[finish_order@[k] as int][ii] as int] by {
                        assert(old(visited)@[graph@[old(finish_order)@[k] as int][ii] as int]);
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
                spec_toposortstper_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() + spec_num_false(visited@) + 1
                    == old(finish_order)@.len() + spec_num_false(old(visited)@),
                visited@[vertex as int],
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                finish_order@.len() >= old(finish_order)@.len(),
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
                    && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@))
                    ==> spec_neighbors_explored_per(graph, visited@, finish_order@),
                // Conditional: edge ordered.
                (spec_is_dag_per(graph)
                    && old(finish_order)@.no_duplicates()
                    && (forall|k: int| 0 <= k < old(finish_order)@.len()
                        ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                    && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@)
                    && spec_edge_ordered_per(graph, old(finish_order)@))
                    ==> spec_edge_ordered_per(graph, finish_order@),
                // New elements reachable from vertex.
                !old(visited)@[vertex as int]
                    ==> (forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                        ==> spec_reachable_per(graph, vertex as int, #[trigger] finish_order@[k] as int)),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            let ghost fo_pre = finish_order@;
            let ghost vis_pre = visited@;
            assert(spec_has_edge_per(graph, vertex as int, neighbor as int)) by {
                assert(graph@[vertex as int][i as int] == neighbor);
            };
            dfs_finish_order(graph, visited, finish_order, neighbor);
            proof {
                // Chain prefix preservation.
                assert forall|k: int| 0 <= k < old(finish_order)@.len()
                    implies finish_order@[k] == old(finish_order)@[k] by {
                    assert(finish_order@[k] == fo_pre[k]);
                };
                // Chain new-elements-unvisited.
                assert forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                    implies !old(visited)@[#[trigger] finish_order@[k] as int] by {
                    if k < fo_pre.len() as int {
                        assert(finish_order@[k] == fo_pre[k]);
                    } else {
                        assert(!vis_pre[finish_order@[k] as int]);
                        if old(visited)@[finish_order@[k] as int] {
                            assert(vis_pre[finish_order@[k] as int]);
                        }
                    }
                };
                // Neighbor is visited.
                assert(visited@[neighbor as int]);
                // Prior neighbors still visited.
                assert forall|j: int| 0 <= j < i as int
                    implies visited@[#[trigger] graph@[vertex as int][j] as int] by {
                    assert(vis_pre[graph@[vertex as int][j] as int]);
                };
                // Reachability.
                if !old(visited)@[vertex as int] {
                    if vis_pre[neighbor as int] {
                        assert(finish_order@.len() == fo_pre.len());
                    }
                    assert forall|k: int| old(finish_order)@.len() <= k < finish_order@.len()
                        implies spec_reachable_per(graph, vertex as int, #[trigger] finish_order@[k] as int) by {
                        if k < fo_pre.len() as int {
                        } else {
                            assert(!vis_pre[neighbor as int]);
                            assert(spec_reachable_per(graph, neighbor as int, finish_order@[k] as int));
                            lemma_reachable_via_edge_per(graph, vertex as int, neighbor as int, finish_order@[k] as int);
                        }
                    };
                }
            }
            i = i + 1;
        }
        // All neighbors of vertex are visited.
        assert forall|j: int| 0 <= j < graph@[vertex as int].len()
            implies visited@[#[trigger] graph@[vertex as int][j] as int] by {};
        let ghost fo_pre_push = finish_order@;
        let ghost vis_at_push = visited@;
        finish_order.push(vertex);
        proof {
            assert(finish_order@ =~= fo_pre_push.push(vertex));
            // Neighbors explored for the extended sequence.
            if (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@)
            {
                assert forall|k: int| 0 <= k < finish_order@.len()
                    implies #[trigger] spec_vertex_neighbors_visited_per(graph, visited@, finish_order@[k] as int) by {
                    if k < fo_pre_push.len() as int {
                        assert(finish_order@[k] == fo_pre_push[k]);
                        assert(spec_vertex_neighbors_visited_per(graph, vis_at_push, fo_pre_push[k] as int));
                    } else {
                        assert(finish_order@[k] == vertex);
                    }
                };
            }
            // Edge ordered for the extended sequence.
            if spec_is_dag_per(graph)
                && old(finish_order)@.no_duplicates()
                && (forall|k: int| 0 <= k < old(finish_order)@.len()
                    ==> old(visited)@[#[trigger] old(finish_order)@[k] as int])
                && spec_neighbors_explored_per(graph, old(visited)@, old(finish_order)@)
                && spec_edge_ordered_per(graph, old(finish_order)@)
            {
                assert forall|a: int, b: int|
                    0 <= a < finish_order@.len() && 0 <= b < finish_order@.len()
                    && spec_has_edge_per(graph, finish_order@[a] as int, finish_order@[b] as int)
                    implies ({
                        #[trigger] finish_order@[a]; #[trigger] finish_order@[b];
                        b < a
                    }) by {
                    if a < fo_pre_push.len() as int && b < fo_pre_push.len() as int {
                        assert(finish_order@[a] == fo_pre_push[a]);
                        assert(finish_order@[b] == fo_pre_push[b]);
                    } else if a == fo_pre_push.len() as int && b < fo_pre_push.len() as int {
                    } else if a < fo_pre_push.len() as int && b == fo_pre_push.len() as int {
                        assert(finish_order@[b] == vertex);
                        if a < old(finish_order)@.len() as int {
                            assert(spec_vertex_neighbors_visited_per(graph, old(visited)@, old(finish_order)@[a] as int));
                            assert(finish_order@[a] == old(finish_order)@[a]);
                            assert(!old(visited)@[vertex as int]);
                        } else {
                            assert(spec_reachable_per(graph, vertex as int, finish_order@[a] as int));
                            lemma_reachable_edge_contradicts_dag_per(graph, vertex as int, finish_order@[a] as int);
                        }
                    } else {
                        assert(finish_order@[a] == vertex);
                        assert(finish_order@[b] == vertex);
                        lemma_self_reachable_per(graph, vertex as int);
                        lemma_reachable_edge_contradicts_dag_per(graph, vertex as int, vertex as int);
                    }
                };
            }
            // Reachability: vertex reaches itself.
            if !old(visited)@[vertex as int] {
                lemma_self_reachable_per(graph, vertex as int);
            }
        }
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: &mut Vec<bool>,
        rec_stack: &mut Vec<bool>,
        finish_order: &mut Vec<usize>,
        vertex: usize,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
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
        assert(vertex < rec_stack.len());
        assert(vertex < visited.len());
        if rec_stack[vertex] {
            return false;
        }
        if visited[vertex] {
            return true;
        }

        assert(!old(visited)@[vertex as int]);
        visited.set(vertex, true);
        rec_stack.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(visited@.len() == graph@.len());
        assert(rec_stack@.len() == graph@.len());

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
                rec_stack@.len() == graph@.len(),
                spec_toposortstper_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() + spec_num_false(visited@) <
                    old(finish_order)@.len() + spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, finish_order, neighbor) {
                return false;
            }
            i = i + 1;
        }

        rec_stack.set(vertex, false);
        finish_order.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    #[verifier::external_body]
    pub fn topological_sort_opt(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (topo_order: Option<AVLTreeSeqStPerS<usize>>)
        requires
            spec_toposortstper_wf(graph),
            graph@.len() < usize::MAX,
        ensures
            topo_order.is_some() <==> spec_is_dag_per(graph),
            topo_order.is_some() ==> spec_is_topo_order_per(graph, topo_order.unwrap()@),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut rec_stack: Vec<bool> = Vec::new();
        let mut finish_order: Vec<usize> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant
                j <= n,
                visited@.len() == j as int,
                rec_stack@.len() == j as int,
                forall|k: int| 0 <= k < visited@.len() ==> !#[trigger] visited@[k],
                forall|k: int| 0 <= k < rec_stack@.len() ==> !#[trigger] rec_stack@[k],
            decreases n - j,
        {
            visited.push(false);
            rec_stack.push(false);
            j = j + 1;
        }

        proof {
            lemma_all_false_num_false_eq_len(visited@);
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                n < usize::MAX,
                visited@.len() == n,
                rec_stack@.len() == n,
                spec_toposortstper_wf(graph),
                finish_order@.len() + spec_num_false(visited@) <= n,
            decreases n - start,
        {
            if !visited[start] {
                if !dfs_finish_order_cycle_detect(graph, &mut visited, &mut rec_stack, &mut finish_order, start) {
                    return None;
                }
            }
            start = start + 1;
        }
        assert(finish_order@.len() <= n);
        assert(finish_order@.len() < usize::MAX);
        let result_len = finish_order.len();
        let mut reversed: Vec<usize> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == finish_order@.len(),
                result_len < usize::MAX,
                reversed@.len() == (result_len - k) as nat,
                reversed@.len() < usize::MAX,
            decreases k,
        {
            k = k - 1;
            reversed.push(finish_order[k]);
        }
        assert(reversed@.len() < usize::MAX);
        Some(AVLTreeSeqStPerS::from_vec(reversed))
    }

    impl TopoSortStPerTrait for TopoSortStPer {
        /// Returns sequence of vertices in topological order.
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (order: AVLTreeSeqStPerS<usize>)
        {
            let n = graph.length();
            let mut visited: Vec<bool> = Vec::new();
            let mut finish_order: Vec<usize> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    visited@.len() == j as int,
                    forall|k: int| 0 <= k < visited@.len() ==> !#[trigger] visited@[k],
                decreases n - j,
            {
                visited.push(false);
                j = j + 1;
            }

            proof {
                lemma_all_false_num_false_eq_len(visited@);
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    n < usize::MAX,
                    visited@.len() == n,
                    spec_toposortstper_wf(graph),
                    finish_order@.len() + spec_num_false(visited@) == n,
                    forall|j: int| #![trigger visited@[j]] 0 <= j < start ==> visited@[j],
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                    // Topo sort invariants.
                    finish_order@.no_duplicates(),
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> visited@[#[trigger] finish_order@[k] as int],
                    spec_neighbors_explored_per(graph, visited@, finish_order@),
                    spec_is_dag_per(graph) ==> spec_edge_ordered_per(graph, finish_order@),
                decreases n - start,
            {
                if !visited[start] {
                    let ghost old_visited = visited@;
                    let ghost old_fo = finish_order@;
                    dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                    proof {
                        assert forall|j: int| #![trigger visited@[j]] 0 <= j < start + 1 implies visited@[j] by {
                            if j < start as int {
                                assert(old_visited[j]);
                            }
                        };
                        assert(old_fo.no_duplicates());
                        assert(forall|k: int| 0 <= k < old_fo.len()
                            ==> old_visited[#[trigger] old_fo[k] as int]);
                        assert(spec_neighbors_explored_per(graph, old_visited, old_fo));
                    }
                } else {
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            proof {
                lemma_all_true_num_false_zero(visited@);
            }
            assert(finish_order@.len() == n);
            assert(finish_order@.len() < usize::MAX);
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
                    forall|m: int| #![trigger reversed@[m]]
                        0 <= m < reversed@.len()
                        ==> reversed@[m] == fo_final[(result_len - 1 - m) as int],
                decreases k,
            {
                k = k - 1;
                reversed.push(finish_order[k]);
            }
            assert(reversed@.len() == n);
            assert(reversed@.len() < usize::MAX);
            proof {
                assert(reversed@.no_duplicates()) by {
                    assert forall|i: int, j: int|
                        0 <= i < reversed@.len() && 0 <= j < reversed@.len() && i != j
                        implies reversed@[i] != reversed@[j] by {
                        assert(reversed@[i] == fo_final[(n - 1 - i) as int]);
                        assert(reversed@[j] == fo_final[(n - 1 - j) as int]);
                    };
                };
                assert forall|m: int| 0 <= m < reversed@.len()
                    implies (reversed@[m] as int) < graph@.len() by {
                    assert(reversed@[m] == fo_final[(n - 1 - m) as int]);
                };
                assert(reversed@.map_values(|t: usize| t@) =~= reversed@) by {
                    assert(reversed@.map_values(|t: usize| t@).len() == reversed@.len());
                    assert forall|i: int| 0 <= i < reversed@.len()
                        implies reversed@.map_values(|t: usize| t@)[i] == reversed@[i] by {};
                };
            }
            let order = AVLTreeSeqStPerS::from_vec(reversed);
            proof {
                assert(order@ =~= reversed@);
                assert(order@.len() == n);
                if spec_is_dag_per(graph) {
                    assert(spec_edge_ordered_per(graph, fo_final));
                    assert forall|i: int, j: int|
                        #![trigger order@[i], order@[j]]
                        0 <= i < order@.len() && 0 <= j < order@.len()
                        && spec_has_edge_per(graph, order@[i] as int, order@[j] as int)
                        implies i < j by {
                        assert(order@[i] == fo_final[(n - 1 - i) as int]);
                        assert(order@[j] == fo_final[(n - 1 - j) as int]);
                        let a = (n as int) - 1 - i;
                        let b = (n as int) - 1 - j;
                        assert(fo_final[a] == order@[i]);
                        assert(fo_final[b] == order@[j]);
                    };
                    assert(spec_is_topo_order_per(graph, order@));
                }
            }
            order
        }
    } // impl TopoSortStPerTrait

    } // verus!
}
