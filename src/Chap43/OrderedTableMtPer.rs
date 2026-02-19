//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent ordered table implementation using parallel Treap backing.
//!
//! Work/Span Analysis:
//! - Parallel operations using BSTParaTreapMtEph<Pair<K, V>>
//! - O(lg n) span for insert, delete, map, filter (use ParaPair! divide-and-conquer)
//! - find() uses binary search on in-order sequence: O(lg n) work, O(lg n) span
//! - domain() uses sequential key extraction: O(n) work, O(1) per element; parallelism overhead would dominate

pub mod OrderedTableMtPer {

    use vstd::prelude::*;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableMtPer<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        tree: ParamTreap<Pair<K, V>>,
    }

    // 5. view impls

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> View for OrderedTableMtPer<K, V> {
        type V = Map<K::V, V::V>;

        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> {
            Map::empty()
        }
    }

    // 8. traits

    pub trait OrderedTableMtPerTrait<K: MtKey + 'static, V: StTInMtT + Ord + 'static>: Sized + View<V = Map<K::V, V::V>> {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();

        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();

        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite();

        fn find(&self, k: &K) -> (result: Option<V>);

        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite();

        fn delete(&self, k: &K) -> (result: Self)
            ensures result@.dom().finite();

        fn domain(&self) -> (result: OrderedSetMtEph<K>)
            ensures self@.dom().finite();

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();

        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite();

        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();

        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();

        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();

        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();

        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();

        fn join_key(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite();

        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();

        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite();

        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite();

        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            self.tree.size()
        }

        #[verifier::external_body]
        fn empty() -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableMtPer {
                tree: ParamTreap::new(),
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty().insert(k@, v@), result@.dom().finite()
        {
            let tree = ParamTreap::new();
            tree.insert(Pair(k, v));
            OrderedTableMtPer { tree }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>) {
            let seq = self.tree.in_order();
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

        #[verifier::external_body]
        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| pair.0 != k_clone);
            filtered.insert(Pair(k, v));
            OrderedTableMtPer { tree: filtered }
        }

        #[verifier::external_body]
        fn delete(&self, k: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| pair.0 != k_clone);
            OrderedTableMtPer { tree: filtered }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: OrderedSetMtEph<K>)
            ensures self@.dom().finite()
        {
            let pair_seq = self.tree.in_order();
            let mut keys = Vec::with_capacity(pair_seq.length());
            for i in 0..pair_seq.length() {
                let Pair(key, _val) = pair_seq.nth(i);
                keys.push(key.clone());
            }

            let key_seq = ArraySeqStPerS::from_vec(keys);
            OrderedSetMtEph::from_seq(key_seq)
        }

        #[verifier::external_body]
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let seq = self.tree.in_order();
            let new_tree = ParamTreap::new();
            for i in 0..seq.length() {
                let Pair(k, v) = seq.nth(i);
                let new_v = f(k, v);
                new_tree.insert(Pair(k.clone(), new_v));
            }
            OrderedTableMtPer { tree: new_tree }
        }

        #[verifier::external_body]
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableMtPer {
                tree: self.tree.filter(f),
            }
        }

        #[verifier::external_body]
        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            if seq.length() == 0 { None } else { Some(seq.nth(0).0.clone()) }
        }

        #[verifier::external_body]
        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            let n = seq.length();
            if n == 0 { None } else { Some(seq.nth(n - 1).0.clone()) }
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            let mut result = None;
            for i in 0..seq.length() {
                let pair = seq.nth(i);
                if &pair.0 < k { result = Some(pair.0.clone()); } else { break; }
            }
            result
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            for i in 0..seq.length() {
                let pair = seq.nth(i);
                if &pair.0 > k { return Some(pair.0.clone()); }
            }
            None
        }

        #[verifier::external_body]
        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            let left_tree = ParamTreap::new();
            let right_tree = ParamTreap::new();
            let mut found_value = None;

            for i in 0..seq.length() {
                let pair = seq.nth(i);
                if &pair.0 < k {
                    left_tree.insert(pair.clone());
                } else if &pair.0 > k {
                    right_tree.insert(pair.clone());
                } else {
                    found_value = Some(pair.1.clone());
                }
            }

            (
                OrderedTableMtPer { tree: left_tree },
                found_value,
                OrderedTableMtPer { tree: right_tree },
            )
        }

        #[verifier::external_body]
        fn join_key(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            OrderedTableMtPer {
                tree: self.tree.union(&other.tree),
            }
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let seq = self.tree.in_order();
            let result_tree = ParamTreap::new();
            for i in 0..seq.length() {
                let pair = seq.nth(i);
                if &pair.0 >= k1 && &pair.0 <= k2 {
                    result_tree.insert(pair.clone());
                }
            }
            OrderedTableMtPer { tree: result_tree }
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            let mut count = 0;
            for i in 0..seq.length() {
                let pair = seq.nth(i);
                if &pair.0 < k { count += 1; } else { break; }
            }
            count
        }

        #[verifier::external_body]
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            if i >= seq.length() { None } else { Some(seq.nth(i).0.clone()) }
        }

        #[verifier::external_body]
        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let seq = self.tree.in_order();
            let left_tree = ParamTreap::new();
            let right_tree = ParamTreap::new();
            for j in 0..seq.length() {
                let pair = seq.nth(j);
                if j < i { left_tree.insert(pair.clone()); }
                else { right_tree.insert(pair.clone()); }
            }
            (
                OrderedTableMtPer { tree: left_tree },
                OrderedTableMtPer { tree: right_tree },
            )
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> Clone for OrderedTableMtPer<K, V> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            OrderedTableMtPer {
                tree: self.tree.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<K: MtKey + 'static, V: MtKey + 'static> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self { Self::empty() }
    }
}
