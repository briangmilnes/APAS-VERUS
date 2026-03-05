//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Persistent (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using decreasing finish times.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false};
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
    pub open spec fn spec_wf_adj_list_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        forall|v: int, i: int| #![auto]
            0 <= v < graph@.len() && 0 <= i < graph@[v]@.len()
            ==> graph@[v]@[i] < graph@.len()
    }

    /// Whether there is a directed edge from u to v in the graph.
    pub open spec fn spec_has_edge_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, u: int, v: int) -> bool {
        0 <= u < graph@.len()
        && exists|i: int| #![auto] 0 <= i < graph@[u]@.len() && graph@[u]@[i] == v
    }

    /// Whether a sequence of vertex indices forms a valid path in the graph.
    pub open spec fn spec_is_path_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, path: Seq<int>) -> bool {
        path.len() >= 1
        && (forall|k: int| #![auto] 0 <= k < path.len() ==> 0 <= path[k] < graph@.len())
        && (forall|k: int| #![auto] 0 <= k < path.len() - 1 ==> spec_has_edge_per(graph, path[k], path[k + 1]))
    }

    /// Whether vertex v is reachable from vertex u (Definition 55.3, reachability).
    pub open spec fn spec_reachable_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, u: int, v: int) -> bool {
        exists|path: Seq<int>| spec_is_path_per(graph, path) && path[0] == u && path.last() == v
    }

    /// Whether the graph is a directed acyclic graph (Definition 55.11).
    pub open spec fn spec_is_dag_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        !exists|path: Seq<int>| spec_is_path_per(graph, path) && path.len() >= 2 && path[0] == path.last()
    }

    /// Whether a sequence is a valid topological ordering (Definition 55.12).
    pub open spec fn spec_is_topo_order_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, order: Seq<usize>) -> bool {
        order.len() == graph@.len()
        && order.no_duplicates()
        && (forall|k: int| #![auto] 0 <= k < order.len() ==> (order[k] as int) < graph@.len())
        && (forall|i: int, j: int| #![auto]
            0 <= i < order.len() && 0 <= j < order.len()
            && spec_has_edge_per(graph, order[i] as int, order[j] as int)
            ==> i < j)
    }

    /// Whether a set of vertices is strongly connected (Definition 55.14).
    pub open spec fn spec_strongly_connected_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, vertices: Set<int>) -> bool {
        forall|u: int, v: int| #![auto] vertices.contains(u) && vertices.contains(v)
            ==> spec_reachable_per(graph, u, v)
    }

    /// Whether components form a valid SCC decomposition in topological order (Definition 55.17).
    pub open spec fn spec_is_scc_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, components: Seq<Set<int>>) -> bool {
        // Each component is strongly connected.
        (forall|c: int| #![auto] 0 <= c < components.len()
            ==> spec_strongly_connected_per(graph, components[c]))
        // Components partition the vertex set.
        && (forall|v: int| 0 <= v < graph@.len() ==>
            exists|c: int| #![auto] 0 <= c < components.len() && components[c].contains(v))
        // Components are disjoint.
        && (forall|c1: int, c2: int| #![auto]
            0 <= c1 < components.len() && 0 <= c2 < components.len() && c1 != c2
            ==> components[c1].disjoint(components[c2]))
        // Inter-component edges go forward (topological order).
        && (forall|c1: int, c2: int, u: int, v: int| #![auto]
            0 <= c1 < components.len() && 0 <= c2 < components.len()
            && components[c1].contains(u) && components[c2].contains(v)
            && spec_has_edge_per(graph, u, v) && c1 != c2
            ==> c1 < c2)
    }

    // 8. traits

    pub trait TopoSortStPerTrait {
        /// Computes topological sort of a DAG (Algorithm 55.13)
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (order: AVLTreeSeqStPerS<N>)
            requires
                spec_wf_adj_list_per(graph),
            ensures
                order@.len() == graph@.len(),
                spec_is_dag_per(graph) ==> spec_is_topo_order_per(graph, order@),
            ;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    fn dfs_finish_order(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        finish_order: &mut Vec<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_wf_adj_list_per(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if visited[vertex] {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        visited.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
        }

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                spec_wf_adj_list_per(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            dfs_finish_order(graph, visited, finish_order, neighbor);
            i = i + 1;
        }
        finish_order.push(vertex);
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        rec_stack: &mut Vec<bool>,
        finish_order: &mut Vec<N>,
        vertex: N,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_wf_adj_list_per(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            rec_stack@.len() == old(rec_stack)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
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
        }

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                rec_stack@.len() == graph@.len(),
                spec_wf_adj_list_per(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
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
    pub fn topological_sort_opt(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (topo_order: Option<AVLTreeSeqStPerS<N>>)
        requires spec_wf_adj_list_per(graph),
        ensures
            topo_order.is_some() <==> spec_is_dag_per(graph),
            topo_order.is_some() ==> spec_is_topo_order_per(graph, topo_order.unwrap()@),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut rec_stack: Vec<bool> = Vec::new();
        let mut finish_order: Vec<N> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant
                j <= n,
                visited@.len() == j as int,
                rec_stack@.len() == j as int,
            decreases n - j,
        {
            visited.push(false);
            rec_stack.push(false);
            j = j + 1;
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                rec_stack@.len() == n,
                spec_wf_adj_list_per(graph),
            decreases n - start,
        {
            if !visited[start] {
                if !dfs_finish_order_cycle_detect(graph, &mut visited, &mut rec_stack, &mut finish_order, start) {
                    return None;
                }
            }
            start = start + 1;
        }
        let result_len = finish_order.len();
        let mut reversed: Vec<N> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == finish_order@.len(),
            decreases k,
        {
            k = k - 1;
            reversed.push(finish_order[k]);
        }
        Some(AVLTreeSeqStPerS::from_vec(reversed))
    }

    impl TopoSortStPerTrait for TopoSortStPer {
        /// Returns sequence of vertices in topological order.
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N>
        {
            let n = graph.length();
            let mut visited: Vec<bool> = Vec::new();
            let mut finish_order: Vec<N> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    visited@.len() == j as int,
                decreases n - j,
            {
                visited.push(false);
                j = j + 1;
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    visited@.len() == n,
                    spec_wf_adj_list_per(graph),
                decreases n - start,
            {
                if !visited[start] {
                    dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                }
                start = start + 1;
            }
            let result_len = finish_order.len();
            let mut reversed: Vec<N> = Vec::new();
            let mut k: usize = result_len;
            while k > 0
                invariant
                    k <= result_len,
                    result_len == finish_order@.len(),
                decreases k,
            {
                k = k - 1;
                reversed.push(finish_order[k]);
            }
            AVLTreeSeqStPerS::from_vec(reversed)
        }
    } // impl TopoSortStPerTrait

    } // verus!
}
