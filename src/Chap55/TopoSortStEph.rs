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

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>;
    }

    /// Computes topological sort of a DAG.
    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span same
    #[verifier::external_body]
    pub fn topological_sort_opt(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> Option<AVLTreeSeqStEphS<N>> {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut rec_stack = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result = Vec::new();

        for start in 0..n {
            if !*visited.nth(start)
                && !dfs_finish_order_cycle_detect(graph, &mut visited, &mut rec_stack, &mut result, start)
            {
                return None;
            }
        }
        result.reverse();
        Some(AVLTreeSeqStEphS::from_vec(result))
    }

    /// Computes topological sort of a DAG.
    /// Returns sequence of vertices in topological order (respecting edge directions).
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span same
    #[verifier::external_body]
    pub fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N> {
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
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        rec_stack: &mut ArraySeqStEphS<B>,
        result: &mut Vec<N>,
        vertex: N,
    ) -> bool {
        if *rec_stack.nth(vertex) {
            return false;
        }
        if *visited.nth(vertex) {
            return true;
        }

        let _ = visited.set(vertex, true);
        let _ = rec_stack.set(vertex, true);
        let neighbors = graph.nth(vertex);

        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, result, neighbor) {
                return false;
            }
        }

        let _ = rec_stack.set(vertex, false);
        result.push(vertex);
        true
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

    } // verus!
}
