//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent reducer-augmented ordered table implementation.

pub mod AugOrderedTableStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 7. free functions (calculate_reduction)
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::OrderedTableStPerLit;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(F)]
    pub struct AugOrderedTableStPer<K: StT + Ord, V: StT, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        base_table: OrderedTableStPer<K, V>,
        reducer: F,
        identity: V,
        cached_reduction: V,
    }

    pub type AugOrderedTablePer<K, V, F> = AugOrderedTableStPer<K, V, F>;

    // 5. view impls

    impl<K: StT + Ord, V: StT, F> View for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        #[verifier::external_body]
        open spec fn view(&self) -> Map<K::V, V::V> {
            Map::empty()
        }
    }

    // 7. free functions (calculate_reduction)

    #[verifier::external_body]
    pub fn calculate_reduction<K: StT + Ord, V: StT, F>(
        base: &OrderedTableStPer<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (result: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        ensures base@.dom().finite(),
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

    /// Trait defining all augmented ordered table operations (ADT 43.3)
    /// Extends ordered table operations with efficient reduction
    pub trait AugOrderedTableStPerTrait<K: StT + Ord, V: StT, F>: Sized + View<V = Map<K::V, V::V>>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite();
        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite();
        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite();
        fn delete(&self, k: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite();
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite();
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite();
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
        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
            where Self: Sized,
            ensures self@.dom().finite();
        fn join_key(left: &Self, right: &Self) -> (result: Self)
            ensures result@.dom().finite();
        fn get_key_range(&self, k1: &K, k2: &K) -> (result: Self)
            ensures result@.dom().finite();
        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite();
        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
            where Self: Sized,
            ensures self@.dom().finite();
        fn reduce_val(&self) -> (result: V)
            ensures self@.dom().finite();
        fn reduce_range(&self, k1: &K, k2: &K) -> (result: V)
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStPerTrait<K, V, F> for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
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
                base_table: OrderedTableStPer::empty(),
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
                base_table: OrderedTableStPer::singleton(k, v.clone()),
                cached_reduction: v,
                reducer,
                identity,
            }
        }

        #[verifier::external_body]
        fn find(&self, k: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
            self.base_table.find(k)
        }

        #[verifier::external_body]
        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.insert(k, v);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn delete(&self, k: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.delete(k);
            // For simplicity, recalculate reduction from scratch
            // In practice, would maintain augmented tree structure
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.domain()
        }

        #[verifier::external_body]
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let base_table = OrderedTableStPer::tabulate(f, keys);
            let cached_reduction = calculate_reduction(&base_table, &reducer, &identity);

            Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            }
        }

        #[verifier::external_body]
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (result: Self)
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
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (result: Self)
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
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.intersection(&other.base_table, f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.union(&other.base_table, f);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.difference(&other.base_table);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.restrict(keys);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        #[verifier::external_body]
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.subtract(keys);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
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
        fn split_key(&self, k: &K) -> (result: (Self, Option<V>, Self))
            ensures self@.dom().finite()
        {
            let (left_base, middle, right_base) = self.base_table.split_key(k);

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

            (left, middle, right)
        }

        #[verifier::external_body]
        fn join_key(left: &Self, right: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = OrderedTableStPer::join_key(&left.base_table, &right.base_table);
            let new_reduction = if left.base_table.size() == 0 {
                right.cached_reduction.clone()
            } else if right.base_table.size() == 0 {
                left.cached_reduction.clone()
            } else {
                (left.reducer)(&left.cached_reduction, &right.cached_reduction)
            };

            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: left.reducer.clone(),
                identity: left.identity.clone(),
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
        fn split_rank_key(&self, i: N) -> (result: (Self, Self))
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
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT, F> Clone for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
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

    impl<K: StT + Ord, V: StT, F> PartialEq for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
                && self.cached_reduction == other.cached_reduction
        }
    }

    impl<K: StT + Ord, V: StT, F> Display for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableStPer(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: StT + Ord, V: StT, F> Debug for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableStPer")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableStPerLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::empty($reducer, $identity);
            $(
                table = $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::insert(&table, $k, $v);
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::AugOrderedTableStPerTrait::empty($reducer, $identity)
        }};
    }
}
