//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Persistent (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using decreasing finish times.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub trait TopoSortStPerTrait {
        /// Computes topological sort of a DAG
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N>;
    }

    /// Computes topological sort of a DAG.
    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V|^2 + (|V| + |E|) log |V|), Span same — Vec::insert(0, ..) O(|V|) + AVL ops O(log |V|)
    pub fn topological_sort_opt(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> Option<AVLTreeSeqStPerS<N>> {
        let n = graph.length();
        let mut visited = AVLTreeSetStPer::empty();
        let mut rec_stack = AVLTreeSetStPer::empty();
        let mut result = Vec::new();

        for start in 0..n {
            if !visited.find(&start) {
                match dfs_finish_order_cycle_detect(graph, visited, rec_stack, result, start) {
                    | Some((new_visited, new_rec_stack, new_result)) => {
                        visited = new_visited;
                        rec_stack = new_rec_stack;
                        result = new_result;
                    }
                    | None => return None, // Cycle detected
                }
            }
        }
        Some(AVLTreeSeqStPerS::from_vec(result))
    }

    /// Computes topological sort of a DAG.
    /// Returns sequence of vertices in topological order (respecting edge directions).
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V|^2 + (|V| + |E|) log |V|), Span same — Vec::insert(0, ..) O(|V|) + AVL ops O(log |V|)
    pub fn topo_sort(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> AVLTreeSeqStPerS<N> {
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
        AVLTreeSeqStPerS::from_vec(result)
    }

    /// - APAS: (no cost stated — internal helper)
    /// - Claude-Opus-4.6: Work O(|V| + log |V|) per call — Vec::insert(0, ..) + AVL find/insert
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: AVLTreeSetStPer<N>,
        rec_stack: AVLTreeSetStPer<N>,
        mut result: Vec<N>,
        vertex: N,
    ) -> Option<(AVLTreeSetStPer<N>, AVLTreeSetStPer<N>, Vec<N>)> {
        if rec_stack.find(&vertex) {
            return None; // Cycle detected
        }
        if visited.find(&vertex) {
            return Some((visited, rec_stack, result));
        }

        let visited = visited.insert(vertex);
        let rec_stack = rec_stack.insert(vertex);
        let neighbors = graph.nth(vertex);

        let mut visited = visited;
        let mut rec_stack = rec_stack;
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            match dfs_finish_order_cycle_detect(graph, visited, rec_stack, result, neighbor) {
                | Some((new_visited, new_rec_stack, new_result)) => {
                    visited = new_visited;
                    rec_stack = new_rec_stack;
                    result = new_result;
                }
                | None => return None, // Cycle detected
            }
        }

        let rec_stack = rec_stack.delete(&vertex);
        result.insert(0, vertex);
        Some((visited, rec_stack, result))
    }

    /// - APAS: (no cost stated — internal helper)
    /// - Claude-Opus-4.6: Work O(|V| + log |V|) per call — Vec::insert(0, ..) + AVL find/insert
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

        result.insert(0, vertex);
        (visited, result)
    }
}
