//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Ephemeral (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ephemeral ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    pub trait CycleDetectStEphTrait {
        /// Detects if a directed graph contains a cycle
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> B;
    }

    /// Detects if a directed graph contains a cycle.
    /// Returns true if a cycle exists, false otherwise.
    pub fn has_cycle(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> B {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);

        for start in 0..n {
            if !*visited.nth(start) {
                let mut ancestors = AVLTreeSetStEph::empty();
                if dfs_check_cycle(graph, &mut visited, &mut ancestors, start) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs_check_cycle(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        ancestors: &mut AVLTreeSetStEph<N>,
        vertex: N,
    ) -> B {
        if ancestors.find(&vertex) {
            return true;
        }

        if *visited.nth(vertex) {
            return false;
        }

        let _ = visited.set(vertex, true);
        ancestors.insert(vertex);

        let neighbors = graph.nth(vertex);
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            if dfs_check_cycle(graph, visited, ancestors, neighbor) {
                ancestors.delete(&vertex);
                return true;
            }
        }

        ancestors.delete(&vertex);
        false
    }
}
