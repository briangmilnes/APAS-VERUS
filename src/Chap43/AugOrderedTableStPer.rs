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
    use crate::Chap42::TableStEph::TableStEph::lemma_entries_to_map_finite;
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
        pub base_table: OrderedTableStPer<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTablePer<K, V, F> = AugOrderedTableStPer<K, V, F>;

    // 5. view impls

    impl<K: StT + Ord, V: StT, F> View for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
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

    // 7b. proof fns

    proof fn lemma_aug_view<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone>(
        t: &AugOrderedTableStPer<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

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
        fn size(&self) -> (result: N)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            let base = OrderedTableStPer::empty();
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
            let base = OrderedTableStPer::singleton(k, v.clone());
            let r = Self {
                base_table: base,
                cached_reduction: v,
                reducer,
                identity,
            };
            proof { lemma_aug_view(&r); }
            r
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

        fn insert(&self, k: K, v: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.insert(k, v);
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

        fn delete(&self, k: &K) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.delete(k);
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

        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite()
        {
            let base_table = OrderedTableStPer::tabulate(f, keys);
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

        fn map<G: Fn(&V) -> V>(&self, f: G) -> (result: Self)
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

        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (result: Self)
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

        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.intersection(&other.base_table, f);
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

        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.union(&other.base_table, f);
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

        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.difference(&other.base_table);
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

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.restrict(keys);
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

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (result: Self)
            ensures result@.dom().finite()
        {
            let new_base = self.base_table.subtract(keys);
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

            proof { lemma_aug_view(self); }
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

        fn rank_key(&self, k: &K) -> (result: N)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        fn select_key(&self, i: N) -> (result: Option<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

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
