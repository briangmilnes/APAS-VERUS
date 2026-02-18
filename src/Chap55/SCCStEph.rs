//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Strongly Connected Components - Sequential Ephemeral (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod SCCStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::{AVLTreeSeqStEphS, AVLTreeSeqStEphTrait};
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    verus! {

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    pub trait SCCStEphTrait {
        /// Finds strongly connected components in a directed graph
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<N>>;
    }

    /// Finds strongly connected components in a directed graph.
    /// Returns sequence of components, each component is a set of vertices.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V|^2 + (|V| + |E|) log |V|), Span same — Vec::insert(0, ..) O(|V|), AVL set ops O(log |V|), component rebuild O(|V|)
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

    /// - APAS: (no cost stated — internal helper, corresponds to decreasingFinish)
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span same
    #[verifier::external_body]
    fn compute_finish_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N> {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result = Vec::new();

        for start in 0..n {
            if !*visited.nth(start) {
                dfs_finish_order(graph, &mut visited, &mut result, start);
            }
        }
        result.reverse();
        AVLTreeSeqStEphS::from_vec(result)
    }

    /// - APAS: (no cost stated — internal helper)
    /// - Claude-Opus-4.6: Work O(degree(v)) per call
    #[verifier::external_body]
    fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        result: &mut Vec<N>,
        vertex: N,
    ) {
        if *visited.nth(vertex) {
            return;
        }

        let _ = visited.set(vertex, true);
        let neighbors = graph.nth(vertex);

        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            dfs_finish_order(graph, visited, result, neighbor);
        }

        result.push(vertex);
    }

    /// - APAS: (no cost stated — transpose is standard O(|V| + |E|))
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with expected cost
    #[verifier::external_body]
    fn transpose_graph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> ArraySeqStEphS<ArraySeqStEphS<N>> {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<N>> = vec![Vec::new(); n];

        for u in 0..n {
            let neighbors = graph.nth(u);
            for i in 0..neighbors.length() {
                let v = *neighbors.nth(i);
                adj_vecs[v].push(u);
            }
        }

        ArraySeqStEphS::tabulate(&|i| ArraySeqStEphS::from_vec(adj_vecs[i].clone()), n)
    }

    /// - APAS: (no cost stated — internal helper, corresponds to DFSReach)
    /// - Claude-Opus-4.6: Work O(deg(v) + log |V|) per call — AVL insert O(log |V|), visited array O(1)
    #[verifier::external_body]
    fn dfs_reach(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        component: &mut AVLTreeSetStEph<N>,
        vertex: N,
    ) {
        if *visited.nth(vertex) {
            return;
        }

        let _ = visited.set(vertex, true);
        component.insert(vertex);

        let neighbors = graph.nth(vertex);
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            dfs_reach(graph, visited, component, neighbor);
        }
    }

    } // verus!
}
