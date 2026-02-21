//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Strongly Connected Components - Sequential Persistent (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using transpose and DFS.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod SCCStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false};
    use crate::Chap55::TopoSortStPer::TopoSortStPer::spec_wf_adj_list_per;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    // 8. traits

    pub trait SCCStPerTrait {
        /// Finds strongly connected components in a directed graph
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<AVLTreeSetStPer<N>>
            requires spec_wf_adj_list_per(graph);
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

    /// Computes the finish order for SCC (decreasing finish times).
    fn compute_finish_order(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N>
        requires spec_wf_adj_list_per(graph),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut result: Vec<N> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant j <= n, visited@.len() == j as int,
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

    /// Transposes a directed graph (reverses all edges).
    fn transpose_graph(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (result: ArraySeqStPerS<ArraySeqStPerS<N>>)
        requires spec_wf_adj_list_per(graph),
        ensures result@.len() == graph@.len(),
    {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<N>> = Vec::new();
        let mut k: usize = 0;
        while k < n
            invariant k <= n, adj_vecs@.len() == k as int,
            decreases n - k,
        {
            adj_vecs.push(Vec::new());
            k = k + 1;
        }

        let mut u: usize = 0;
        while u < n
            invariant
                u <= n,
                n == graph@.len(),
                adj_vecs@.len() == n,
                spec_wf_adj_list_per(graph),
            decreases n - u,
        {
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    neighbors_len == graph@[u as int]@.len(),
                    adj_vecs@.len() == n,
                    n == graph@.len(),
                    spec_wf_adj_list_per(graph),
                decreases neighbors_len - i,
            {
                let v = *neighbors.nth(i);
                assert(graph@[u as int]@[i as int] < graph@.len());
                adj_vecs[v].push(u);
                i = i + 1;
            }
            u = u + 1;
        }

        let mut result_vecs: Vec<ArraySeqStPerS<N>> = Vec::new();
        let mut m: usize = 0;
        while m < n
            invariant
                m <= n,
                adj_vecs@.len() == n,
                result_vecs@.len() == m as int,
            decreases n - m,
        {
            result_vecs.push(ArraySeqStPerS::from_vec(adj_vecs[m].clone()));
            m = m + 1;
        }
        ArraySeqStPerS::from_vec(result_vecs)
    }

    /// Runtime check that all neighbor indices are valid vertex indices.
    fn check_wf_adj_list_per(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> (result: bool)
        ensures result ==> spec_wf_adj_list_per(graph),
    {
        let n = graph.length();
        let mut u: usize = 0;
        while u < n
            invariant
                u <= n,
                n == graph@.len(),
                forall|v: int, i: int| #![auto]
                    0 <= v < u as int && 0 <= i < graph@[v]@.len()
                    ==> graph@[v]@[i] < graph@.len(),
            decreases n - u,
        {
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    u < n,
                    n == graph@.len(),
                    neighbors_len == graph@[u as int]@.len(),
                    forall|v: int, j: int| #![auto]
                        0 <= v < u as int && 0 <= j < graph@[v]@.len()
                        ==> graph@[v]@[j] < graph@.len(),
                    forall|j: int| #![auto]
                        0 <= j < i as int
                        ==> graph@[u as int]@[j] < graph@.len(),
                decreases neighbors_len - i,
            {
                let neighbor = *neighbors.nth(i);
                if neighbor >= n {
                    return false;
                }
                i = i + 1;
            }
            u = u + 1;
        }
        true
    }

    /// DFS reachability using Vec<bool> for termination and persistent set
    /// for component accumulation (same pattern as DFSStPer::dfs_recursive).
    fn dfs_reach(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited_bool: &mut Vec<bool>,
        component: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> (out: AVLTreeSetStPer<N>)
        requires
            vertex < old(visited_bool)@.len(),
            old(visited_bool)@.len() == graph@.len(),
            spec_wf_adj_list_per(graph),
        ensures
            visited_bool@.len() == old(visited_bool)@.len(),
            forall|j: int| #![auto]
                0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                ==> visited_bool@[j],
            spec_num_false(visited_bool@) <= spec_num_false(old(visited_bool)@),
        decreases spec_num_false(old(visited_bool)@),
    {
        if visited_bool[vertex] {
            return component;
        }
        assert(!old(visited_bool)@[vertex as int]);
        visited_bool.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited_bool)@, vertex as int);
        }
        let mut component = component.insert(vertex);

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len == graph@[vertex as int]@.len(),
                visited_bool@.len() == graph@.len(),
                spec_wf_adj_list_per(graph),
                forall|j: int| #![auto]
                    0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                    ==> visited_bool@[j],
                spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int]@[i as int] < graph@.len());
            component = dfs_reach(graph, visited_bool, component, neighbor);
            i = i + 1;
        }
        component
    }

    /// Finds strongly connected components in a directed graph.
    pub fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<AVLTreeSetStPer<N>>
        requires spec_wf_adj_list_per(graph),
    {
        let finish_order = compute_finish_order(graph);
        let transposed = transpose_graph(graph);

        if !check_wf_adj_list_per(&transposed) {
            return AVLTreeSeqStPerS::empty();
        }

        let n = transposed.length();
        let mut visited_bool: Vec<bool> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant j <= n, visited_bool@.len() == j as int,
            decreases n - j,
        {
            visited_bool.push(false);
            j = j + 1;
        }

        let finish_len = finish_order.length();
        let mut components_vec: Vec<AVLTreeSetStPer<N>> = Vec::new();
        let mut i: usize = 0;
        while i < finish_len
            invariant
                i <= finish_len,
                visited_bool@.len() == n,
                n == transposed@.len(),
                spec_wf_adj_list_per(&transposed),
            decreases finish_len - i,
        {
            let vertex = *finish_order.nth(i);
            if vertex < n && !visited_bool[vertex] {
                let component = AVLTreeSetStPer::empty();
                let component = dfs_reach(&transposed, &mut visited_bool, component, vertex);
                if component.size() > 0 {
                    components_vec.push(component);
                }
            }
            i = i + 1;
        }
        AVLTreeSeqStPerS::from_vec(components_vec)
    }

    } // verus!
}


