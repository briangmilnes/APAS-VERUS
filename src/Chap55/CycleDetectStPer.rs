//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Cycle Detection - Sequential Persistent (Chapter 55, Algorithm 55.10).
//! Detects cycles in directed graphs using ancestor tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod CycleDetectStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub trait CycleDetectStPerTrait {
        /// Detects if a directed graph contains a cycle
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> B;
    }

    /// Detects if a directed graph contains a cycle.
    /// Returns true if a cycle exists, false otherwise.
    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Claude-Opus-4.6: Work O((|V| + |E|) log |V|), Span same — AVLTreeSetStPer find/insert/delete are O(log n)
    #[verifier::external_body]
    pub fn has_cycle(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> B {
        let n = graph.length();
        let mut visited = AVLTreeSetStPer::empty();

        for start in 0..n {
            if !visited.find(&start) {
                let (found_cycle, new_visited) = dfs_check_cycle(graph, visited, AVLTreeSetStPer::empty(), start);
                if found_cycle {
                    return true;
                }
                visited = new_visited;
            }
        }
        false
    }

    /// - APAS: (no cost stated — internal helper of cycle detection)
    /// - Claude-Opus-4.6: Work O(log |V|) per call — O((|V| + |E|) log |V|) total; ancestors clone is O(|V|) worst case
    #[verifier::external_body]
    fn dfs_check_cycle(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited: AVLTreeSetStPer<N>,
        ancestors: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> (B, AVLTreeSetStPer<N>) {
        if ancestors.find(&vertex) {
            return (true, visited);
        }

        if visited.find(&vertex) {
            return (false, visited);
        }

        let visited = visited.insert(vertex);
        let ancestors = ancestors.insert(vertex);
        let neighbors = graph.nth(vertex);

        let mut visited = visited;
        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            let (found_cycle, new_visited) = dfs_check_cycle(graph, visited, ancestors.clone(), neighbor);
            if found_cycle {
                return (true, new_visited);
            }
            visited = new_visited;
        }

        (false, visited)
    }

    } // verus!
}
