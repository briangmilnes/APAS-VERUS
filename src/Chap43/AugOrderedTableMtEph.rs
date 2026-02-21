//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral reducer-augmented ordered table implementation.
//!
//! Note: reduce_range_parallel() uses unconditional parallelism with ParaPair! for range reductions.

pub mod AugOrderedTableMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 7. free functions (calculate_reduction, recalculate_reduction)
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::thread;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableMtEph::TableMtEph::*;
    use crate::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::{OrderedTableMtEphLit, ParaPair};
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(F)]
    pub struct AugOrderedTableMtEph<K: MtKey, V: MtVal, F: MtReduceFn<V>> {
        pub base_table: OrderedTableMtEph<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTableMt<K, V, F> = AugOrderedTableMtEph<K, V, F>;

    // 5. view impls

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> View for AugOrderedTableMtEph<K, V, F> {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 7. free functions (calculate_reduction, recalculate_reduction)

    #[verifier::external_body]
    pub fn recalculate_reduction<K: MtKey, V: MtVal, F: MtReduceFn<V>>(
        table: &AugOrderedTableMtEph<K, V, F>,
    ) -> (result: V)
    ensures table@.dom().finite()
    {
        calculate_reduction(&table.base_table, &table.reducer, &table.identity)
    }

    #[verifier::external_body]
    pub fn calculate_reduction<K: MtKey, V: MtVal, F: MtReduceFn<V>>(
        base: &OrderedTableMtEph<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (result: V)
    ensures base@.dom().finite()
    {
        if base.size() == 0 {
            return identity.clone();
        }

        let pairs = base.collect();
        let mut result = identity.clone();
        let mut first = true;

        for i in 0..pairs.length() {
            let pair = pairs.nth(i);
            if first {
                result = pair.1.clone();
                first = false;
            } else {
                result = reducer(&result, &pair.1);
            }
        }

        result
    }

    // 7b. proof fns

    proof fn lemma_aug_view<K: MtKey, V: MtVal, F: MtReduceFn<V>>(
        t: &AugOrderedTableMtEph<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3) with multi-threaded ephemeral semantics
    /// Extends ordered table operations with efficient reduction and thread-safe operations
    pub trait AugOrderedTableMtEphTrait<K: MtKey, V: MtVal, F: MtReduceFn<V>>: Sized + View<V = Map<K::V, V::V>> {
        fn size(&self) -> (result: usize)
            ensures result == self@.dom().len(), self@.dom().finite();
        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite();
        fn find(&self, k: &K) -> (result: Option<V>);
        fn lookup(&self, k: &K) -> (result: Option<V>);
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty(), self@.dom().finite();
        fn insert<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite();
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@.dom().finite();
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<G: Fn(&K) -> V + Send + Sync + 'static>(
            f: G,
            keys: &ArraySetStEph<K>,
            reducer: F,
            identity: V,
        ) -> (result: Self)
            ensures result@.dom().finite();
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn filter<G: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn intersection<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite();
        fn union<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite();
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn reduce<R: StTInMtT + 'static, G: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: G) -> (result: R)
            ensures self@.dom().finite();
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite();
        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_key(&mut self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized,
            ensures self@.dom().finite();
        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite();
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn rank_key(&self, k: &K) -> (result: usize)
            ensures self@.dom().finite();
        fn select_key(&self, i: usize) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&mut self, i: usize) -> (result: (Self, Self))
            where Self: Sized,
            ensures self@.dom().finite();
        fn reduce_val(&self) -> (result: V)
            ensures self@.dom().finite();
        fn reduce_range(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite();
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> AugOrderedTableMtEphTrait<K, V, F> for AugOrderedTableMtEph<K, V, F> {
        fn size(&self) -> (result: usize)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            let base = OrderedTableMtEph::empty();
            let r = Self {
                base_table: base,
                cached_reduction: identity.clone(),
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn singleton(k: K, v: V, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let base = OrderedTableMtEph::singleton(k, v.clone());
            let r = Self {
                base_table: base,
                cached_reduction: v,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn find(&self, k: &K) -> (result: Option<V>) {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (result: Option<V>) {
            self.base_table.lookup(k)
        }

        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty(), self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            self.base_table.is_empty()
        }

        fn insert<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@.dom().finite()
        {
            let result = self.base_table.delete(k);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
            result
        }

        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V + Send + Sync + 'static>(
            f: G,
            keys: &ArraySetStEph<K>,
            reducer: F,
            identity: V,
        ) -> (result: Self)
            ensures result@.dom().finite()
        {
            let base_table = OrderedTableMtEph::tabulate(f, keys);
            let cached_reduction = calculate_reduction(&base_table, &reducer, &identity);

            let r = Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.map(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn filter<G: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.filter(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn intersection<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn union<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            self.cached_reduction = recalculate_reduction(self);
            proof { lemma_aug_view(self); }
        }

        fn reduce<R: StTInMtT + 'static, G: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: G) -> (result: R)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.reduce(init, f)
        }

        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.first_key()
        }

        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.last_key()
        }

        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.previous_key(k)
        }

        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.next_key(k)
        }

        fn split_key(&mut self, k: &K) -> (result: (Self, Option<V>, Self))
            ensures self@.dom().finite()
        {
            let (left_base, found_value, right_base) = self.base_table.split_key(k);

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            proof { lemma_aug_view(self); }
            (left, found_value, right)
        }

        #[verifier::external_body]
        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite()
        {
            let old_reduction = self.cached_reduction.clone();
            let other_reduction = other.cached_reduction.clone();
            let other_size = other.base_table.size();

            self.base_table.join_key(other.base_table);

            if self.base_table.size() == 0 {
                self.cached_reduction = other_reduction;
            } else if other_size == 0 {
                self.cached_reduction = old_reduction;
            } else {
                self.cached_reduction = (self.reducer)(&old_reduction, &other_reduction);
            }
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn rank_key(&self, k: &K) -> (result: usize)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        fn select_key(&self, i: usize) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

        fn split_rank_key(&mut self, i: usize) -> (result: (Self, Self))
            ensures self@.dom().finite()
        {
            let (left_base, right_base) = self.base_table.split_rank_key(i);

            let left_reduction = calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = calculate_reduction(&right_base, &self.reducer, &self.identity);

            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };

            proof { lemma_aug_view(self); }
            (left, right)
        }

        fn reduce_val(&self) -> (result: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            self.cached_reduction.clone()
        }

        fn reduce_range(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }

        #[verifier::external_body]
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite()
        {
            let range_table = self.get_key_range(k1, k2);

            if range_table.size() == 0 {
                return range_table.identity.clone();
            }
            if range_table.size() == 1 {
                return range_table.reduce_val();
            }

            let mid_rank = range_table.size() / 2;
            if let Some(mid_key) = range_table.select_key(mid_rank) {
                let left_table = range_table.get_key_range(k1, &mid_key);
                let right_start = range_table.next_key(&mid_key).unwrap_or_else(|| mid_key.clone());
                let right_table = range_table.get_key_range(&right_start, k2);
                
                let reducer = range_table.reducer.clone();
                let mid_val = range_table.find(&mid_key).unwrap_or_else(|| range_table.identity.clone());

                let Pair(left_val, right_val) =
                    ParaPair!(move || left_table.reduce_val(), move || right_table.reduce_val());

                let left_mid = reducer(&left_val, &mid_val);
                reducer(&left_mid, &right_val)
            } else {
                range_table.reduce_val()
            }
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> Clone for AugOrderedTableMtEph<K, V, F> {
        #[verifier::external_body]
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            Self {
                base_table: self.base_table.clone(),
                cached_reduction: self.cached_reduction.clone(),
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> PartialEq for AugOrderedTableMtEph<K, V, F> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
                && self.cached_reduction == other.cached_reduction
        }
    }

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> Display for AugOrderedTableMtEph<K, V, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableMtEph(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> Debug for AugOrderedTableMtEph<K, V, F> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableMtEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableMtEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity)
        }};
    }
}
