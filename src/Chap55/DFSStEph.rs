//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Ephemeral (Chapter 55, Algorithm 55.7).
//! Recursive DFS using ephemeral arrays for efficient visited tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    pub trait DFSStEphTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> AVLTreeSetStEph<N>;
    }

    /// Performs DFS from source vertex s on adjacency list graph G.
    /// Returns the set of all vertices reachable from s.
    pub fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> AVLTreeSetStEph<N> {
        let n = graph.length();
        if source >= n {
            return AVLTreeSetStEph::empty();
        }

        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result = AVLTreeSetStEph::empty();
        dfs_recursive(graph, &mut visited, &mut result, source);
        result
    }

    fn dfs_recursive(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        result: &mut AVLTreeSetStEph<N>,
        vertex: N,
    ) {
        if *visited.nth(vertex) {
            return;
        }

        let _ = visited.set(vertex, true);
        result.insert(vertex);

        let neighbors = graph.nth(vertex);
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            dfs_recursive(graph, visited, result, neighbor);
        }
    }
}
