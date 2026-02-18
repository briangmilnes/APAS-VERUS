//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Sequential Persistent (Chapter 54, Algorithm 54.3).
//! Queue-based BFS for finding distances and reachability from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod BFSStPer {

    use std::collections::VecDeque;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub trait BFSStPerTrait {
        /// Performs BFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> ArraySeqStPerS<N>;
    }

    const UNREACHABLE: N = N::MAX;

    /// Performs BFS from source vertex s on adjacency list graph G.
    /// Graph is represented as sequence of sequences (adjacency list).
    /// Returns array where result[v] = distance if reachable, UNREACHABLE otherwise.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
    pub fn bfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> ArraySeqStPerS<N> {
        let n = graph.length();
        if source >= n {
            return ArraySeqStPerS::tabulate(&|_| UNREACHABLE, n);
        }

        let mut distances = ArraySeqStPerS::tabulate(&|_| UNREACHABLE, n);
        distances = ArraySeqStPerS::update(&distances, source, 0);

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(u) = queue.pop_front() {
            let dist = *distances.nth(u);
            if dist != UNREACHABLE {
                let neighbors = graph.nth(u);
                for i in 0..neighbors.length() {
                    let v = *neighbors.nth(i);
                    if *distances.nth(v) == UNREACHABLE {
                        distances = ArraySeqStPerS::update(&distances, v, dist + 1);
                        queue.push_back(v);
                    }
                }
            }
        }

        distances
    }
}
