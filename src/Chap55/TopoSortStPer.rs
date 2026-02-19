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
    // 6. spec fns
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    // 6. spec fns

    /// Well-formed adjacency list for persistent graph representation.
    pub open spec fn spec_wf_adj_list_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        forall|v: int, i: int| #![auto]
            0 <= v < graph@.len() && 0 <= i < graph@[v]@.len()
            ==> graph@[v]@[i] < graph@.len()
    }

    // 8. traits

    pub trait TopoSortStPerTrait {
        /// Computes topological sort of a DAG
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N>;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    fn dfs_finish_order(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        result: &mut Vec<N>,
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
            dfs_finish_order(graph, visited, result, neighbor);
            i = i + 1;
        }
        result.push(vertex);
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: &mut Vec<bool>,
        rec_stack: &mut Vec<bool>,
        result: &mut Vec<N>,
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
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, result, neighbor) {
                return false;
            }
            i = i + 1;
        }

        rec_stack.set(vertex, false);
        result.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    pub fn topological_sort_opt(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> Option<AVLTreeSeqStPerS<N>>
        requires spec_wf_adj_list_per(graph),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut rec_stack: Vec<bool> = Vec::new();
        let mut result: Vec<N> = Vec::new();
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
        Some(AVLTreeSeqStPerS::from_vec(reversed))
    }

    /// Returns sequence of vertices in topological order.
    pub fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N>
        requires spec_wf_adj_list_per(graph),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut result: Vec<N> = Vec::new();
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
        AVLTreeSeqStPerS::from_vec(reversed)
    }

    } // verus!
}
