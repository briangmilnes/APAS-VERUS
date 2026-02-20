//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Strongly Connected Components - Sequential Ephemeral (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod SCCStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::{AVLTreeSeqStEphS, AVLTreeSeqStEphTrait};
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{
        spec_wf_adj_list, spec_num_false, lemma_set_true_decreases_num_false,
        dfs_finish_order,
    };
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 8. traits
    // 9. impls

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    // 8. traits

    pub trait SCCStEphTrait {
        /// Finds strongly connected components in a directed graph
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<N>>;
    }

    // 9. impls

    /// Computes the finish order for SCC (decreasing finish times).
    fn compute_finish_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>
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

    /// Transposes a directed graph (reverses all edges).
    fn transpose_graph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (result: ArraySeqStEphS<ArraySeqStEphS<N>>)
        requires spec_wf_adj_list(graph),
        ensures result@.len() == graph@.len(),
    {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<N>> = Vec::new();
        let mut k: usize = 0;
        while k < n
            invariant
                k <= n,
                adj_vecs@.len() == k as int,
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
                spec_wf_adj_list(graph),
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
                    spec_wf_adj_list(graph),
                decreases neighbors_len - i,
            {
                let v = *neighbors.nth(i);
                assert(graph@[u as int]@[i as int] < graph@.len());
                adj_vecs[v].push(u);
                i = i + 1;
            }
            u = u + 1;
        }

        let mut result_vecs: Vec<ArraySeqStEphS<N>> = Vec::new();
        let mut m: usize = 0;
        while m < n
            invariant
                m <= n,
                adj_vecs@.len() == n,
                result_vecs@.len() == m as int,
            decreases n - m,
        {
            result_vecs.push(ArraySeqStEphS::from_vec(adj_vecs[m].clone()));
            m = m + 1;
        }
        ArraySeqStEphS::from_vec(result_vecs)
    }

    /// Finds strongly connected components in a directed graph.
    /// Returns sequence of components, each component is a set of vertices.
    #[verifier::external_body]
    pub fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<N>> {
        let finish_order = compute_finish_order(graph);
        let transposed = transpose_graph(graph);

        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut components = AVLTreeSeqStEphS::<AVLTreeSetStEph<N>>::empty();

        for i in 0..finish_order.length() {
            let vertex = *finish_order.nth(i);
            if !*visited.nth(vertex) {
                let mut component = AVLTreeSetStEph::empty();
                dfs_reach(&transposed, &mut visited, &mut component, vertex);
                if component.size() > 0 {
                    let mut vec = Vec::new();
                    for i in 0..components.length() {
                        vec.push(components.nth(i).clone());
                    }
                    vec.push(component);
                    components = AVLTreeSeqStEphS::from_vec(vec);
                }
            }
        }
        components
    }

    fn dfs_reach(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        component: &mut AVLTreeSetStEph<N>,
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

        component.insert(vertex);

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
            dfs_reach(graph, visited, component, neighbor);
            i = i + 1;
        }
    }

    } // verus!
}
