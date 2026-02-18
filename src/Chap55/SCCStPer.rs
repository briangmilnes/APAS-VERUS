//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Strongly Connected Components - Sequential Persistent (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using transpose and DFS.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod SCCStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub trait SCCStPerTrait {
        /// Finds strongly connected components in a directed graph
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<AVLTreeSetStPer<N>>;
    }

    /// Finds strongly connected components in a directed graph.
    /// Returns sequence of components, each component is a set of vertices.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V|^2 + (|V| + |E|) log |V|), Span same — Vec::insert(0, ..) O(|V|), AVL ops O(log |V|), union O(|comp|)
    #[verifier::external_body]
    pub fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<AVLTreeSetStPer<N>> {
        let finish_order = compute_finish_order(graph);
        let transposed = transpose_graph(graph);

        let mut visited = AVLTreeSetStPer::empty();
        let mut components = AVLTreeSeqStPerS::empty();

        for i in 0..finish_order.length() {
            let vertex = *finish_order.nth(i);
            if !visited.find(&vertex) {
                let (new_visited, component) = dfs_reach(&transposed, visited, vertex);
                visited = new_visited;
                if component.size() > 0 {
                    let mut vec = components.values_in_order();
                    vec.push(component);
                    components = AVLTreeSeqStPerS::from_vec(vec);
                }
            }
        }
        components
    }

    /// - APAS: (no cost stated — internal helper, corresponds to decreasingFinish)
    /// - Claude-Opus-4.6: Work O((|V| + |E|) log |V|), Span same
    #[verifier::external_body]
    fn compute_finish_order(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N> {
        let n = graph.length();
        let mut visited = AVLTreeSetStPer::empty();
        let mut result = Vec::new();

        for start in 0..n {
            if !visited.find(&start) {
                let (new_visited, new_result) = dfs_finish_order(graph, visited, result, start);
                visited = new_visited;
                result = new_result;
            }
        }
        result.reverse();
        AVLTreeSeqStPerS::from_vec(result)
    }

    /// - APAS: (no cost stated — internal helper)
    /// - Claude-Opus-4.6: Work O(degree(v) + log |V|) per call
    #[verifier::external_body]
    fn dfs_finish_order(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: AVLTreeSetStPer<N>,
        mut result: Vec<N>,
        vertex: N,
    ) -> (AVLTreeSetStPer<N>, Vec<N>) {
        if visited.find(&vertex) {
            return (visited, result);
        }

        let visited = visited.insert(vertex);
        let neighbors = graph.nth(vertex);

        let mut visited = visited;
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            let (new_visited, new_result) = dfs_finish_order(graph, visited, result, neighbor);
            visited = new_visited;
            result = new_result;
        }

        result.push(vertex);
        (visited, result)
    }

    /// - APAS: (no cost stated — transpose is standard O(|V| + |E|))
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with expected cost
    #[verifier::external_body]
    fn transpose_graph(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> ArraySeqStPerS<ArraySeqStPerS<N>> {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<N>> = vec![Vec::new(); n];

        for u in 0..n {
            let neighbors = graph.nth(u);
            for i in 0..neighbors.length() {
                let v = *neighbors.nth(i);
                adj_vecs[v].push(u);
            }
        }

        ArraySeqStPerS::tabulate(&|i| ArraySeqStPerS::from_vec(adj_vecs[i].clone()), n)
    }

    /// - APAS: (no cost stated — internal helper, corresponds to DFSReach)
    /// - Claude-Opus-4.6: Work O(deg(v) log |V| + |comp| log |comp|) per call — AVL insert + union
    #[verifier::external_body]
    fn dfs_reach(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> (AVLTreeSetStPer<N>, AVLTreeSetStPer<N>) {
        if visited.find(&vertex) {
            return (visited, AVLTreeSetStPer::empty());
        }

        let visited = visited.insert(vertex);
        let mut component = AVLTreeSetStPer::singleton(vertex);
        let neighbors = graph.nth(vertex);

        let mut visited = visited;
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            let (new_visited, sub_component) = dfs_reach(graph, visited, neighbor);
            visited = new_visited;
            component = component.union(&sub_component);
        }

        (visited, component)
    }

    } // verus!
}
