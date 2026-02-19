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
    use crate::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::{OrderedTableMtEphLit, ParaPair};
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub struct AugOrderedTableMtEph<K: MtKey, V: MtVal, F: MtReduceFn<V>> {
        base_table: OrderedTableMtEph<K, V>,
        reducer: F,
        identity: V,
        cached_reduction: V,
    }

    pub type AugOrderedTableMt<K, V, F> = AugOrderedTableMtEph<K, V, F>;

    // 5. view impls

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> View for AugOrderedTableMtEph<K, V, F> {
        type V = Map<K::V, V::V>;
        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> {
            Map::empty()
        }
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

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3) with multi-threaded ephemeral semantics
    /// Extends ordered table operations with efficient reduction and thread-safe operations
    pub trait AugOrderedTableMtEphTrait<K: MtKey, V: MtVal, F: MtReduceFn<V>> {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();
        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite();
        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result == None;
        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result == None;
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
        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self)
            where Self: Sized,
            ensures self@.dom().finite();
        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite();
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite();
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&mut self, i: N) -> (Self, Self)
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
        #[verifier::external_body]
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            self.base_table.size()
        }

        #[verifier::external_body]
        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            Self {
                base_table: OrderedTableMtEph::empty(),
                cached_reduction: identity.clone(),
                reducer,
                identity,
            }
        }

        #[verifier::external_body]
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            Self {
                base_table: OrderedTableMtEph::singleton(k, v.clone()),
                cached_reduction: v,
                reducer,
                identity,
            }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result == None
        {
            self.base_table.find(k)
        }

        #[verifier::external_body]
        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                self@.contains_key(k@) ==> result == Some(self@[k@]),
                !self@.contains_key(k@) ==> result == None
        {
            self.base_table.lookup(k)
        }

        #[verifier::external_body]
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty(), self@.dom().finite()
        {
            self.base_table.is_empty()
        }

        #[verifier::external_body]
        fn insert<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@.dom().finite()
        {
            let result = self.base_table.delete(k);
            // Recalculate reduction after deletion
            self.cached_reduction = recalculate_reduction(self);
            result
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.domain()
        }

        #[verifier::external_body]
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

            Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            }
        }

        #[verifier::external_body]
        fn map<G: Fn(&K, &V) -> V + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.map(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn filter<G: Fn(&K, &V) -> B + Send + Sync + 'static>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.filter(f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn intersection<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn union<G: Fn(&V, &V) -> V + Send + Sync + 'static>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            self.cached_reduction = recalculate_reduction(self);
        }

        #[verifier::external_body]
        fn reduce<R: StTInMtT + 'static, G: Fn(R, &K, &V) -> R + Send + Sync + 'static>(&self, init: R, f: G) -> (result: R)
            ensures self@.dom().finite()
        {
            self.base_table.reduce(init, f)
        }

        #[verifier::external_body]
        fn collect(&self) -> (result: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite()
        {
            self.base_table.collect()
        }

        #[verifier::external_body]
        fn first_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            self.base_table.first_key()
        }

        #[verifier::external_body]
        fn last_key(&self) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            self.base_table.last_key()
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            self.base_table.previous_key(k)
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            self.base_table.next_key(k)
        }

        #[verifier::external_body]
        fn split_key(&mut self, k: &K) -> (Self, Option<V>, Self)
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

            // Combine reductions
            if self.base_table.size() == 0 {
                self.cached_reduction = other_reduction;
            } else if other_size == 0 {
                self.cached_reduction = old_reduction;
            } else {
                self.cached_reduction = (self.reducer)(&old_reduction, &other_reduction);
            }
        }

        #[verifier::external_body]
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite()
        {
            self.base_table.rank_key(k)
        }

        #[verifier::external_body]
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            self.base_table.select_key(i)
        }

        #[verifier::external_body]
        fn split_rank_key(&mut self, i: N) -> (Self, Self)
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

            (left, right)
        }

        #[verifier::external_body]
        fn reduce_val(&self) -> (result: V)
            ensures self@.dom().finite()
        {
            self.cached_reduction.clone()
        }

        #[verifier::external_body]
        fn reduce_range(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite()
        {
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }

        #[verifier::external_body]
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite()
        {
            let range_table = self.get_key_range(k1, k2);

            // Base cases
            if range_table.size() == 0 {
                return range_table.identity.clone();
            }
            if range_table.size() == 1 {
                return range_table.reduce_val();
            }

            // Unconditionally parallel split using ParaPair!
            let mid_rank = range_table.size() / 2;
            if let Some(mid_key) = range_table.select_key(mid_rank) {
                // Split [k1, mid_key) and [mid_key, k2] to avoid overlap and infinite recursion
                let left_table = range_table.get_key_range(k1, &mid_key);
                // Get next key after mid_key for exclusive split
                let right_start = range_table.next_key(&mid_key).unwrap_or_else(|| mid_key.clone());
                let right_table = range_table.get_key_range(&right_start, k2);
                
                let reducer = range_table.reducer.clone();
                let mid_val = range_table.find(&mid_key).unwrap_or_else(|| range_table.identity.clone());

                let Pair(left_val, right_val) =
                    ParaPair!(move || left_table.reduce_val(), move || right_table.reduce_val());

                // Combine left + mid + right
                let left_mid = reducer(&left_val, &mid_val);
                reducer(&left_mid, &right_val)
            } else {
                range_table.reduce_val()
            }
        }
    }

    // 11. derive impls in verus!

    impl<K: MtKey, V: MtVal, F: MtReduceFn<V>> Clone for AugOrderedTableMtEph<K, V, F> {
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
