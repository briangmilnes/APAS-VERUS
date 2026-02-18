//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Parallel Ephemeral (Chapter 54).
//! Layer-by-layer parallel BFS for finding distances and reachability.
//! Work: O(|V| + |E|), Span: O(d·lg n) where d is diameter.

pub mod BFSMtEph {

    use std::collections::VecDeque;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqMtEphS<ArraySeqMtEphS<N>>;

    pub trait BFSMtEphTrait {
        /// Performs parallel BFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(d·lg n) where d is diameter
        fn bfs(graph: &ArraySeqMtEphS<ArraySeqMtEphS<N>>, source: N) -> ArraySeqMtEphS<N>;
    }

    const UNREACHABLE: N = N::MAX;

    /// Performs parallel BFS from source vertex s on adjacency list graph G.
    /// Graph is represented as sequence of sequences (adjacency list).
    /// Returns array where result[v] = distance if reachable, UNREACHABLE otherwise.
    /// - APAS: Work O(|V| + |E|), Span O(d·lg n) where d is diameter
    /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — sequential within layers; no thread::spawn, so Span == Work despite Mt module name.
    pub fn bfs(graph: &ArraySeqMtEphS<ArraySeqMtEphS<N>>, source: N) -> ArraySeqMtEphS<N> {
        let n = graph.length();
        if source >= n {
            return ArraySeqMtEphS::tabulate(&|_| UNREACHABLE, n);
        }

        let mut distances = ArraySeqMtEphS::tabulate(&|_| UNREACHABLE, n);
        let _ = distances.set(source, 0);

        let mut current_layer = VecDeque::new();
        current_layer.push_back(source);
        let mut current_dist = 0;

        while !current_layer.is_empty() {
            let layer_size = current_layer.len();
            let mut next_layer = VecDeque::new();

            for _ in 0..layer_size {
                if let Some(u) = current_layer.pop_front() {
                    let neighbors = graph.nth(u).clone();
                    for i in 0..neighbors.length() {
                        let v = neighbors.nth(i).clone();
                        if distances.nth(v).clone() == UNREACHABLE {
                            let _ = distances.set(v, current_dist + 1);
                            next_layer.push_back(v);
                        }
                    }
                }
            }

            current_layer = next_layer;
            current_dist += 1;
        }

        distances
    }
}
