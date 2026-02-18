//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Data Structure (Sequential Ephemeral)
//!
//! Implements Union-Find (Disjoint Set Union) with path compression and union by rank.
//! Used in Kruskal's MST algorithm for efficient cycle detection.

pub mod UnionFindStEph {

    use vstd::prelude::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;

    verus! {
        pub trait UnionFindStEphTrait<V: StT + Hash> {
            /// Create a new empty Union-Find structure
            /// APAS: Work Θ(1), Span Θ(1)
            fn new() -> Self;

            /// Insert a new element into the Union-Find structure
            /// APAS: Work Θ(1), Span Θ(1)
            fn insert(&mut self, v: V);

            /// Find the representative (root) of the set containing v with path compression
            /// APAS: Work O(α(n)), Span O(α(n)) amortized (inverse Ackermann)
            fn find(&mut self, v: &V) -> V;

            /// Union two sets containing u and v using union by rank
            /// APAS: Work O(α(n)), Span O(α(n)) amortized
            fn union(&mut self, u: &V, v: &V);

            /// Check if two elements are in the same set
            /// APAS: Work O(α(n)), Span O(α(n)) amortized
            fn equals(&mut self, u: &V, v: &V) -> B;

            /// Get the number of distinct sets
            /// APAS: Work O(n α(n)), Span O(n α(n))
            fn num_sets(&mut self) -> usize;
        }
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct UnionFindStEph<V: StT + Hash> {
        parent: HashMap<V, V>,
        rank: HashMap<V, usize>,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: StT + Hash> UnionFindStEphTrait<V> for UnionFindStEph<V> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS
        fn new() -> Self {
            UnionFindStEph {
                parent: HashMap::new(),
                rank: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS
        fn insert(&mut self, v: V) {
            if !self.parent.contains_key(&v) {
                let _ = self.parent.insert(v.clone(), v.clone());
                let _ = self.rank.insert(v, 0);
            }
        }

        /// - APAS: Work O(α(n)), Span O(α(n)) amortized
        /// - Claude-Opus-4.6: Work O(α(n)), Span O(α(n)) amortized — agrees with APAS; path compression implemented
        fn find(&mut self, v: &V) -> V {
            let parent_v = self.parent.get(v).unwrap().clone();

            if parent_v == *v {
                v.clone()
            } else {
                let root = self.find(&parent_v);
                let _ = self.parent.insert(v.clone(), root.clone());
                root
            }
        }

        /// - APAS: Work O(α(n)), Span O(α(n)) amortized
        /// - Claude-Opus-4.6: Work O(α(n)), Span O(α(n)) amortized — agrees with APAS; union by rank implemented
        fn union(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            let root_v = self.find(v);

            if root_u == root_v {
                return;
            }

            let rank_u = *self.rank.get(&root_u).unwrap_or(&0);
            let rank_v = *self.rank.get(&root_v).unwrap_or(&0);

            if rank_u < rank_v {
                let _ = self.parent.insert(root_u, root_v);
            } else if rank_u > rank_v {
                let _ = self.parent.insert(root_v, root_u);
            } else {
                let _ = self.parent.insert(root_v, root_u.clone());
                let _ = self.rank.insert(root_u, rank_u + 1);
            }
        }

        /// - APAS: Work O(α(n)), Span O(α(n)) amortized
        /// - Claude-Opus-4.6: Work O(α(n)), Span O(α(n)) amortized — agrees with APAS; two find calls
        fn equals(&mut self, u: &V, v: &V) -> B { self.find(u) == self.find(v) }

        /// - APAS: Work O(n α(n)), Span O(n α(n))
        /// - Claude-Opus-4.6: Work O(n α(n)), Span O(n α(n)) — agrees with APAS; finds root for every element
        fn num_sets(&mut self) -> usize {
            let mut roots = std::collections::HashSet::new();
            let keys = self.parent.keys().cloned().collect::<Vec<V>>();
            for v in keys.iter() {
                let root = self.find(v);
                let _ = roots.insert(root);
            }
            roots.len()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<V: StT + Hash> Default for UnionFindStEph<V> {
        /// - APAS: N/A — Rust trait boilerplate.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to new()
        fn default() -> Self { Self::new() }
    }
}
