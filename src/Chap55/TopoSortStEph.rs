//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Ephemeral (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

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
    pub struct TopoSortStEph;

    // 6. spec fns

    /// Counts false entries in a boolean sequence (termination measure for DFS).
    pub open spec fn spec_num_false(s: Seq<bool>) -> nat
        decreases s.len()
    {
        if s.len() == 0 { 0 }
        else if !s.last() { 1 + spec_num_false(s.drop_last()) }
        else { spec_num_false(s.drop_last()) }
    }

    /// Well-formed adjacency list: all neighbor indices are valid vertex indices.
    pub open spec fn spec_toposortsteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> bool {
        forall|v: int, i: int| #![auto]
            0 <= v < graph@.len() && 0 <= i < graph@[v]@.len()
            ==> graph@[v]@[i] < graph@.len()
    }

    /// Whether there is a directed edge from u to v in the graph.
    pub open spec fn spec_has_edge(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, u: int, v: int) -> bool {
        0 <= u < graph@.len()
        && exists|i: int| #![auto] 0 <= i < graph@[u]@.len() && graph@[u]@[i] == v
    }

    /// Whether a sequence of vertex indices forms a valid path in the graph.
    pub open spec fn spec_is_path(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, path: Seq<int>) -> bool {
        path.len() >= 1
        && (forall|k: int| #![auto] 0 <= k < path.len() ==> 0 <= path[k] < graph@.len())
        && (forall|k: int| #![auto] 0 <= k < path.len() - 1 ==> spec_has_edge(graph, path[k], path[k + 1]))
    }

    /// Whether vertex v is reachable from vertex u (Definition 55.3, reachability).
    pub open spec fn spec_reachable(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, u: int, v: int) -> bool {
        exists|path: Seq<int>| spec_is_path(graph, path) && path[0] == u && path.last() == v
    }

    /// Whether the graph is a directed acyclic graph (Definition 55.11).
    pub open spec fn spec_is_dag(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> bool {
        !exists|path: Seq<int>| spec_is_path(graph, path) && path.len() >= 2 && path[0] == path.last()
    }

    /// Whether a sequence is a valid topological ordering (Definition 55.12).
    pub open spec fn spec_is_topo_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, order: Seq<usize>) -> bool {
        order.len() == graph@.len()
        && order.no_duplicates()
        && (forall|k: int| #![auto] 0 <= k < order.len() ==> (order[k] as int) < graph@.len())
        && (forall|i: int, j: int| #![auto]
            0 <= i < order.len() && 0 <= j < order.len()
            && spec_has_edge(graph, order[i] as int, order[j] as int)
            ==> i < j)
    }

    /// Whether a set of vertices is strongly connected (Definition 55.14).
    pub open spec fn spec_strongly_connected(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, vertices: Set<int>) -> bool {
        forall|u: int, v: int| #![auto] vertices.contains(u) && vertices.contains(v)
            ==> spec_reachable(graph, u, v)
    }

    /// Whether components form a valid SCC decomposition in topological order (Definition 55.17).
    pub open spec fn spec_is_scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, components: Seq<Set<int>>) -> bool {
        // Each component is strongly connected.
        (forall|c: int| #![auto] 0 <= c < components.len()
            ==> spec_strongly_connected(graph, components[c]))
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
            && spec_has_edge(graph, u, v) && c1 != c2
            ==> c1 < c2)
    }

    // 7. proof fns

    /// Setting a false entry to true strictly decreases the count of false entries.
    proof fn lemma_set_true_decreases_num_false(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) < spec_num_false(s),
        decreases s.len(),
    {
        if s.len() == 1 {
        } else if idx == s.len() - 1 {
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_decreases_num_false(s.drop_last(), idx);
        }
    }

    /// Setting a false entry to true decreases the count by exactly one.
    pub proof fn lemma_set_true_num_false_eq(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) == spec_num_false(s) - 1,
        decreases s.len(),
    {
        if s.len() == 1 {
        } else if idx == s.len() - 1 {
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_num_false_eq(s.drop_last(), idx);
        }
    }

    /// An all-true sequence has zero false entries.
    pub proof fn lemma_all_true_num_false_zero(s: Seq<bool>)
        requires forall|j: int| #![auto] 0 <= j < s.len() ==> s[j],
        ensures spec_num_false(s) == 0,
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_true_num_false_zero(s.drop_last());
        }
    }

    /// An all-false sequence has num_false equal to its length.
    pub proof fn lemma_all_false_num_false_eq_len(s: Seq<bool>)
        requires forall|j: int| #![auto] 0 <= j < s.len() ==> !s[j],
        ensures spec_num_false(s) == s.len(),
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_false_num_false_eq_len(s.drop_last());
        }
    }

    // 8. traits

    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG (Algorithm 55.13).
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [Exercise 55.6]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (order: AVLTreeSeqStEphS<N>)
            requires
                spec_toposortsteph_wf(graph),
            ensures
                order@.len() == graph@.len(),
                spec_is_dag(graph) ==> spec_is_topo_order(graph, order@),
            ;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    /// Also used by SCCStEph::compute_finish_order.
    pub fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        finish_order: &mut Vec<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            forall|k: int| #![auto] 0 <= k < old(finish_order)@.len()
                ==> (old(finish_order)@[k] as int) < graph@.len(),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() >= old(finish_order)@.len(),
            forall|k: int| #![auto] 0 <= k < finish_order@.len()
                ==> (finish_order@[k] as int) < graph@.len(),
            visited@[vertex as int],
            finish_order@.len() + spec_num_false(visited@)
                == old(finish_order)@.len() + spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if *visited.nth(vertex) {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        let set_ok = visited.set(vertex, true);
        assert(set_ok.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                spec_toposortsteph_wf(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() >= old(finish_order)@.len(),
                forall|k: int| #![auto] 0 <= k < finish_order@.len()
                    ==> (finish_order@[k] as int) < graph@.len(),
                visited@[vertex as int],
                finish_order@.len() + spec_num_false(visited@) + 1
                    == old(finish_order)@.len() + spec_num_false(old(visited)@),
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
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        rec_stack: &mut ArraySeqStEphS<B>,
        finish_order: &mut Vec<N>,
        vertex: N,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            rec_stack@.len() == old(rec_stack)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if *rec_stack.nth(vertex) {
            return false;
        }
        if *visited.nth(vertex) {
            return true;
        }

        assert(!old(visited)@[vertex as int]);
        let ok1 = visited.set(vertex, true);
        assert(ok1.is_ok());
        let ok2 = rec_stack.set(vertex, true);
        assert(ok2.is_ok());
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
                spec_toposortsteph_wf(graph),
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

        let ok3 = rec_stack.set(vertex, false);
        assert(ok3.is_ok());
        finish_order.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    pub fn topological_sort_opt(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (topo_order: Option<AVLTreeSeqStEphS<N>>)
        requires spec_toposortsteph_wf(graph),
        ensures
            topo_order.is_some() <==> spec_is_dag(graph),
            topo_order.is_some() ==> spec_is_topo_order(graph, topo_order.unwrap()@),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut rec_stack = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut finish_order: Vec<N> = Vec::new();

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                rec_stack@.len() == n,
                spec_toposortsteph_wf(graph),
            decreases n - start,
        {
            if !*visited.nth(start) {
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
        Some(AVLTreeSeqStEphS::from_vec(reversed))
    }

    impl TopoSortStEphTrait for TopoSortStEph {
        /// Returns sequence of vertices in topological order.
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>
        {
            let n = graph.length();
            let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
            let mut finish_order: Vec<N> = Vec::new();

            proof {
                lemma_all_false_num_false_eq_len(visited@);
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    visited@.len() == n,
                    spec_toposortsteph_wf(graph),
                    forall|k: int| #![auto] 0 <= k < finish_order@.len()
                        ==> (finish_order@[k] as int) < graph@.len(),
                    forall|j: int| #![auto] 0 <= j < start as int ==> visited@[j],
                    finish_order@.len() + spec_num_false(visited@) == n,
                decreases n - start,
            {
                if !*visited.nth(start) {
                    dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                }
                start = start + 1;
            }
            proof {
                lemma_all_true_num_false_zero(visited@);
            }
            let result_len = finish_order.len();
            let mut reversed: Vec<N> = Vec::new();
            let mut k: usize = result_len;
            while k > 0
                invariant
                    k <= result_len,
                    result_len == finish_order@.len(),
                    reversed@.len() == (result_len - k) as nat,
                decreases k,
            {
                k = k - 1;
                reversed.push(finish_order[k]);
            }
            AVLTreeSeqStEphS::from_vec(reversed)
        }
    } // impl TopoSortStEphTrait

    } // verus!
}
