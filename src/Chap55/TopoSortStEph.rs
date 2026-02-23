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
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

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
    pub open spec fn spec_wf_adj_list(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> bool {
        forall|v: int, i: int| #![auto]
            0 <= v < graph@.len() && 0 <= i < graph@[v]@.len()
            ==> graph@[v]@[i] < graph@.len()
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

    // 8. traits

    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    /// Also used by SCCStEph::compute_finish_order.
    pub fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        result: &mut Vec<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_wf_adj_list(graph),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited@.len() && old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
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
        }

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited@.len() == graph@.len(),
                spec_wf_adj_list(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            dfs_finish_order(graph, visited, result, neighbor);
            i = i + 1;
        }
        result.push(vertex);
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        rec_stack: &mut ArraySeqStEphS<B>,
        result: &mut Vec<N>,
        vertex: N,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_wf_adj_list(graph),
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
                spec_wf_adj_list(graph),
                forall|j: int| #![auto]
                    0 <= j < visited@.len() && old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, result, neighbor) {
                return false;
            }
            i = i + 1;
        }

        let ok3 = rec_stack.set(vertex, false);
        assert(ok3.is_ok());
        result.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    pub fn topological_sort_opt(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> Option<AVLTreeSeqStEphS<N>>
        requires spec_wf_adj_list(graph),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut rec_stack = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result: Vec<N> = Vec::new();

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                rec_stack@.len() == n,
                spec_wf_adj_list(graph),
            decreases n - start,
        {
            if !*visited.nth(start) {
                if !dfs_finish_order_cycle_detect(graph, &mut visited, &mut rec_stack, &mut result, start) {
                    return None;
                }
            }
            start = start + 1;
        }
        let result_len = result.len();
        let mut reversed: Vec<N> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == result@.len(),
            decreases k,
        {
            k = k - 1;
            reversed.push(result[k]);
        }
        Some(AVLTreeSeqStEphS::from_vec(reversed))
    }

    /// Returns sequence of vertices in topological order.
    pub fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>
        requires spec_wf_adj_list(graph),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result: Vec<N> = Vec::new();

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                spec_wf_adj_list(graph),
            decreases n - start,
        {
            if !*visited.nth(start) {
                dfs_finish_order(graph, &mut visited, &mut result, start);
            }
            start = start + 1;
        }
        let result_len = result.len();
        let mut reversed: Vec<N> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == result@.len(),
            decreases k,
        {
            k = k - 1;
            reversed.push(result[k]);
        }
        AVLTreeSeqStEphS::from_vec(reversed)
    }

    } // verus!
}
