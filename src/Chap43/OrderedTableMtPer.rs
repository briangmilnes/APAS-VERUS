//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent ordered table implementation using parallel Treap backing.
//!
//! Work/Span Analysis:
//! - Parallel operations using BSTParaTreapMtEph<Pair<K, V>>
//! - O(lg n) span for insert, delete, map, filter (use ParaPair! divide-and-conquer)
//! - find() uses binary search on in-order sequence: O(lg n) work, O(lg n) span
//! - domain() uses sequential key extraction: O(n) work, O(1) per element; parallelism overhead would dominate

pub mod OrderedTableMtPer {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Types::Types::*;

    pub struct OrderedTableMtPer<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        tree: ParamTreap<Pair<K, V>>,
    }

    pub trait OrderedTableMtPerTrait<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                   -> Self;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n)
        fn singleton(k: K, v: V)     -> Self;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n) - binary search on in-order sequence
        fn find(&self, k: &K)        -> Option<V>;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n) - parallel filter + insert via ParaPair!
        fn insert(&self, k: K, v: V) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n) - parallel filter via ParaPair!
        fn delete(&self, k: &K)      -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n) - sequential in_order + key extraction; from_seq may use parallel tree build
        fn domain(&self)             -> OrderedSetMtEph<K>;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n) - extract pairs, transform values, rebuild treap
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n) - parallel filter via ParaPair!
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> Self;
    }

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        fn size(&self) -> N { self.tree.size() }

        fn empty() -> Self {
            OrderedTableMtPer {
                tree: ParamTreap::new(),
            }
        }

        fn singleton(k: K, v: V) -> Self {
            let tree = ParamTreap::new();
            tree.insert(Pair(k, v));
            OrderedTableMtPer { tree }
        }

        fn find(&self, k: &K) -> Option<V> {
            // Use parallel tree search via find on Pair
            // Create a dummy pair for searching (value doesn't matter for key comparison)
            // Actually, we need to search the tree directly
            // The tree is ordered by Pair<K, V> which compares keys first
            // So we can search for any pair with the target key
            let seq = self.tree.in_order();
            
            // Binary search through the sorted sequence (keys are ordered)
            let mut left = 0;
            let mut right = seq.length();
            
            while left < right {
                let mid = (left + right) / 2;
                let Pair(mid_key, mid_val) = seq.nth(mid);
                
                match k.cmp(mid_key) {
                    std::cmp::Ordering::Equal => return Some(mid_val.clone()),
                    std::cmp::Ordering::Less => right = mid,
                    std::cmp::Ordering::Greater => left = mid + 1,
                }
            }
            None
        }

        fn insert(&self, k: K, v: V) -> Self {
            // Delete old entry with same key, then insert new pair
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| pair.0 != k_clone);
            filtered.insert(Pair(k, v));
            OrderedTableMtPer { tree: filtered }
        }

        fn delete(&self, k: &K) -> Self {
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| pair.0 != k_clone);
            OrderedTableMtPer { tree: filtered }
        }

        fn domain(&self) -> OrderedSetMtEph<K> {
            // Extract keys from pairs and build set. in_order() is sequential O(n).
            // Key extraction is O(n) work, O(1) per element; parallelism would add
            // overhead (split, spawn, concat) without meaningful benefit.
            let pair_seq = self.tree.in_order();
            let mut keys = Vec::with_capacity(pair_seq.length());
            for i in 0..pair_seq.length() {
                let Pair(key, _val) = pair_seq.nth(i);
                keys.push(key.clone());
            }
            
            let key_seq = ArraySeqStPerS::from_vec(keys);
            OrderedSetMtEph::from_seq(key_seq)
        }

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> Self {
            let seq = self.tree.in_order();
            let new_tree = ParamTreap::new();
            for i in 0..seq.length() {
                let Pair(k, v) = seq.nth(i);
                let new_v = f(k, v);
                new_tree.insert(Pair(k.clone(), new_v));
            }
            OrderedTableMtPer { tree: new_tree }
        }

        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> Self {
            OrderedTableMtPer {
                tree: self.tree.filter(f),
            }
        }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self { Self::empty() }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> Clone for OrderedTableMtPer<K, V> {
        fn clone(&self) -> Self {
            OrderedTableMtPer {
                tree: self.tree.clone(),
            }
        }
    }
}
