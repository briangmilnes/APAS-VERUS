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
        spec_toposortsteph_wf, spec_num_false, lemma_set_true_decreases_num_false,
        dfs_finish_order, lemma_all_true_num_false_zero, lemma_all_false_num_false_eq_len,
    };
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct SCCStEph;

    // 8. traits

    pub trait SCCStEphTrait {
        /// Finds strongly connected components in a directed graph (Algorithm 55.18).
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [inherits from DFS cost]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (components: AVLTreeSeqStEphS<AVLTreeSetStEph<N>>)
            requires
                spec_toposortsteph_wf(graph),
            ensures
                components@.len() >= 1 || graph@.len() == 0,
            ;
    }

    // 9. impls

    /// Computes the finish order for SCC (decreasing finish times).
    fn compute_finish_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (result: AVLTreeSeqStEphS<N>)
        requires spec_toposortsteph_wf(graph),
        ensures
            result.spec_avltreeseqsteph_wf(),
            result@.len() == graph@.len(),
            forall|i: int| #![auto] 0 <= i < result@.len()
                ==> (result@[i] as int) < graph@.len(),
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
                result_len == n,
                forall|j: int| #![auto] 0 <= j < finish_order@.len()
                    ==> (finish_order@[j] as int) < graph@.len(),
                forall|j: int| #![auto] 0 <= j < reversed@.len()
                    ==> (reversed@[j] as int) < graph@.len(),
                reversed@.len() == (result_len - k) as nat,
            decreases k,
        {
            k = k - 1;
            reversed.push(finish_order[k]);
        }
        AVLTreeSeqStEphS::from_vec(reversed)
    }

    /// Transposes a directed graph (reverses all edges).
    fn transpose_graph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (transposed: ArraySeqStEphS<ArraySeqStEphS<N>>)
        requires spec_toposortsteph_wf(graph),
        ensures transposed@.len() == graph@.len(),
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
                spec_toposortsteph_wf(graph),
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
                    spec_toposortsteph_wf(graph),
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

    /// Runtime check that all neighbor indices are valid vertex indices.
    fn check_wf_adj_list_eph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (valid: bool)
        requires true,
        ensures valid ==> spec_toposortsteph_wf(graph),
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

    impl SCCStEphTrait for SCCStEph {
        /// Finds strongly connected components in a directed graph.
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<N>>
        {
            let finish_order = compute_finish_order(graph);
            let transposed = transpose_graph(graph);

            if !check_wf_adj_list_eph(&transposed) {
                return AVLTreeSeqStEphS::empty();
            }

            let n = transposed.length();
            let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
            let mut components_vec: Vec<AVLTreeSetStEph<N>> = Vec::new();

            let finish_len = finish_order.length();
            let mut i: usize = 0;
            while i < finish_len
                invariant
                    i <= finish_len,
                    visited@.len() == n,
                    n == transposed@.len(),
                    spec_toposortsteph_wf(&transposed),
                decreases finish_len - i,
            {
                let vertex = *finish_order.nth(i);
                if vertex < n && !*visited.nth(vertex) {
                    let mut component = AVLTreeSetStEph::empty();
                    dfs_reach(&transposed, &mut visited, &mut component, vertex);
                    if component.size() > 0 {
                        components_vec.push(component);
                    }
                }
                i = i + 1;
            }
            AVLTreeSeqStEphS::from_vec(components_vec)
        }
    } // impl SCCStEphTrait

    fn dfs_reach(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        component: &mut AVLTreeSetStEph<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
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
                spec_toposortsteph_wf(graph),
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
