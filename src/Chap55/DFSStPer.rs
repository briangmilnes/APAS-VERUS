//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Persistent (Chapter 55, Algorithm 55.2).
//! Recursive DFS for finding reachable vertices from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub trait DFSStPerTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> AVLTreeSetStPer<N>;
    }

    /// Performs DFS from source vertex s on adjacency list graph G.
    /// Returns the set of all vertices reachable from s.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O((|V| + |E|) log |V|), Span O((|V| + |E|) log |V|) — AVLTreeSetStPer find/insert are O(log n)
    #[verifier::external_body]
    pub fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> AVLTreeSetStPer<N> {
        let n = graph.length();
        if source >= n {
            return AVLTreeSetStPer::empty();
        }
        dfs_recursive(graph, AVLTreeSetStPer::empty(), source)
    }

    /// - APAS: (no cost stated — internal helper of DFS)
    /// - Claude-Opus-4.6: Work O(log |V|) per call for find/insert — O((|V| + |E|) log |V|) total
    #[verifier::external_body]
    fn dfs_recursive(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> AVLTreeSetStPer<N> {
        if visited.find(&vertex) {
            return visited;
        }

        let visited = visited.insert(vertex);
        let neighbors = graph.nth(vertex);

        let mut visited = visited;
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            visited = dfs_recursive(graph, visited, neighbor);
        }

        visited
    }

    } // verus!
}
