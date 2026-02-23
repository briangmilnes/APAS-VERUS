//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral reducer-augmented ordered table implementation.

pub mod AugOrderedTableStEph {

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
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::OrderedTableStEphLit;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(F)]
    pub struct AugOrderedTableStEph<K: StT + Ord, V: StT, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        pub base_table: OrderedTableStEph<K, V>,
        pub reducer: F,
        pub identity: V,
        pub cached_reduction: V,
    }

    pub type AugOrderedTableEph<K, V, F> = AugOrderedTableStEph<K, V, F>;

    // 5. view impls

    impl<K: StT + Ord, V: StT, F> View for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        type V = Map<K::V, V::V>;
        open spec fn view(&self) -> Map<K::V, V::V> { self.base_table@ }
    }

    // 7. free functions (calculate_reduction)

    #[verifier::external_body]
    pub fn calculate_reduction<K: StT + Ord, V: StT, F>(
        base: &OrderedTableStEph<K, V>,
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
        t: &AugOrderedTableStEph<K, V, F>,
    )
        ensures t@ =~= t.base_table@
    {}

    // 8. traits

    /// Trait defining all augmented ordered table operations (ADT 43.3) with ephemeral semantics
    /// Extends ordered table operations with efficient reduction and in-place mutations
    pub trait AugOrderedTableStEphTrait<K: StT + Ord, V: StT, F>: Sized + View<V = Map<K::V, V::V>>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn size(&self) -> (result: usize)
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
        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn is_empty(&self) -> (result: B)
            ensures result == self@.dom().is_empty(), self@.dom().finite();
        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite();
        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@.dom().finite();
        fn domain(&self) -> (result: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (result: Self)
            ensures result@.dom().finite();
        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (result: Self)
            ensures result@.dom().finite();
        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (result: R)
            ensures self@.dom().finite();
        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite();
        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite();
        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite();
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
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
    }

    // 9. impls

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStEphTrait<K, V, F> for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn size(&self) -> (result: usize)
            ensures result == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (result: Self)
            ensures result@ == Map::<K::V, V::V>::empty()
        {
            let base = OrderedTableStEph::empty();
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
            let base = OrderedTableStEph::singleton(k, v.clone());
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

        #[verifier::external_body]
        fn lookup(&self, k: &K) -> (result: Option<V>)
            ensures
                match result {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
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

        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn delete(&mut self, k: &K) -> (result: Option<V>)
            ensures self@.dom().finite()
        {
            let result = self.base_table.delete(k);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
            result
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
            let base_table = OrderedTableStEph::tabulate(f, keys);
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

        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (result: Self)
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

        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (result: R)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.reduce(init, f)
        }

        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            ensures self@.dom().finite()
        {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
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
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT, F> Clone for AugOrderedTableStEph<K, V, F>
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

    // 13. derive impls outside verus! (Debug/Display must stay outside per Verus limitation)

    impl<K: StT + Ord, V: StT, F> PartialEq for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
                && self.cached_reduction == other.cached_reduction
        }
    }

    impl<K: StT + Ord, V: StT, F> Display for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "AugOrderedTableStEph(size: {}, reduction: {})",
                self.size(),
                self.cached_reduction
            )
        }
    }

    impl<K: StT + Ord, V: StT, F> Debug for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableStEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableStEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43::AugOrderedTableStEph::AugOrderedTableStEph::AugOrderedTableStEphTrait::empty($reducer, $identity)
        }};
    }
}
