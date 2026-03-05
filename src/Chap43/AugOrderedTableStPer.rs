//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent reducer-augmented ordered table implementation.

pub mod AugOrderedTableStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 7. proof fns, helper fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt::{Debug, Display, Formatter, Result};

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::OrderedTableStPerLit;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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

    /// Fold all values in `base` through `reducer`, returning `identity` for empty tables.
    pub fn calculate_reduction<K: StT + Ord, V: StT, F>(
        base: &OrderedTableStPer<K, V>,
        reducer: &F,
        identity: &V,
    ) -> (reduced: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        ensures base@.dom().finite(),
    {
        let pairs = base.collect();
        // collect ensures: base@.dom().finite(), pairs.spec_well_formed()
        let sz = pairs.length();
        // length ensures: sz as nat == pairs@.len(), given spec_well_formed()
        if sz == 0 {
            return identity.clone();
        }
        // sz > 0 so pairs@.len() > 0, safe to call nth(0).
        let mut reduced = pairs.nth(0).1.clone();
        let mut i: usize = 1;
        while i < sz
            invariant
                1 <= i <= pairs@.len(),
                sz as nat == pairs@.len(),
                pairs.spec_well_formed(),
            decreases pairs@.len() - i,
        {
            let pair = pairs.nth(i);
            proof { assume(reducer.requires((&reduced, &pair.1))); }
            reduced = reducer(&reduced, &pair.1);
            i += 1;
        }
        reduced
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
        spec fn spec_augorderedtablestper_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablestper_wf();
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures tree@.dom().finite(), tree.spec_augorderedtablestper_wf();
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn insert(&self, k: K, v: V) -> (updated: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf();
        fn delete(&self, k: &K) -> (updated: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf();
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures self@.dom().finite();
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            requires keys.spec_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>(),
            ensures tabulated@.dom().finite();
        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
            requires self.spec_augorderedtablestper_wf(), forall|v: &V| f.requires((v,)), obeys_feq_full::<K>(),
            ensures mapped@.dom().finite(), mapped.spec_augorderedtablestper_wf();
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (filtered: Self)
            requires self.spec_augorderedtablestper_wf(), forall|k: &K, v: &V| f.requires((k, v)), obeys_feq_full::<Pair<K, V>>(),
            ensures filtered@.dom().finite(), filtered.spec_augorderedtablestper_wf();
        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<K>(),
            ensures common@.dom().finite(), common.spec_augorderedtablestper_wf();
        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
            requires
                self.spec_augorderedtablestper_wf(),
                other.spec_augorderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures combined@.dom().finite(), combined.spec_augorderedtablestper_wf();
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_view_eq::<K>(), obeys_feq_full::<Pair<K, V>>(),
            ensures remaining@.dom().finite(), remaining.spec_augorderedtablestper_wf();
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures restricted@.dom().finite(), restricted.spec_augorderedtablestper_wf();
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
            requires self.spec_augorderedtablestper_wf(), obeys_feq_full::<Pair<K, V>>(),
            ensures subtracted@.dom().finite(), subtracted.spec_augorderedtablestper_wf();
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_well_formed();
        fn first_key(&self) -> (first: Option<K>)
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@);
        fn last_key(&self) -> (last: Option<K>)
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@);
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@);
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@);
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@);
        fn join_key(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_augorderedtablestper_wf(),
                right.spec_augorderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures joined@.dom().finite(), joined.spec_augorderedtablestper_wf();
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom());
        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len();
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@);
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures
                self@.dom().finite(),
                split.0@.dom().finite(),
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().finite(),
                split.1@.dom().subset_of(self@.dom());
        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite();
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStPerTrait<K, V, F> for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        open spec fn spec_augorderedtablestper_wf(&self) -> bool {
            self.base_table.spec_orderedtablestper_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty(), empty.spec_augorderedtablestper_wf()
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

        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            ensures tree@.dom().finite(), tree.spec_augorderedtablestper_wf()
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

        fn find(&self, k: &K) -> (found: Option<V>)
        {
            proof { lemma_aug_view(self); }
            self.base_table.find(k)
        }

        fn insert(&self, k: K, v: V) -> (updated: Self)
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf()
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

        fn delete(&self, k: &K) -> (updated: Self)
            ensures updated@.dom().finite(), updated.spec_augorderedtablestper_wf()
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

        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.domain()
        }

        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            ensures tabulated@.dom().finite()
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

        fn map<G: Fn(&V) -> V>(&self, f: G) -> (mapped: Self)
            ensures mapped@.dom().finite(), mapped.spec_augorderedtablestper_wf()
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

        fn filter<G: Fn(&K, &V) -> B>(&self, f: G) -> (filtered: Self)
            ensures filtered@.dom().finite(), filtered.spec_augorderedtablestper_wf()
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

        fn intersection<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (common: Self)
            ensures common@.dom().finite(), common.spec_augorderedtablestper_wf()
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

        fn union<G: Fn(&V, &V) -> V>(&self, other: &Self, f: G) -> (combined: Self)
            ensures combined@.dom().finite(), combined.spec_augorderedtablestper_wf()
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

        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@.dom().finite(), remaining.spec_augorderedtablestper_wf()
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

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            ensures restricted@.dom().finite(), restricted.spec_augorderedtablestper_wf()
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

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (subtracted: Self)
            ensures subtracted@.dom().finite(), subtracted.spec_augorderedtablestper_wf()
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

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_well_formed()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        fn first_key(&self) -> (first: Option<K>)
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
        {
            proof { lemma_aug_view(self); }
            self.base_table.first_key()
        }

        fn last_key(&self) -> (last: Option<K>)
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
        {
            proof { lemma_aug_view(self); }
            self.base_table.last_key()
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
        {
            proof { lemma_aug_view(self); }
            self.base_table.previous_key(k)
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
        {
            proof { lemma_aug_view(self); }
            self.base_table.next_key(k)
        }

        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            ensures
                self@.dom().finite(),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, middle, right)
        }

        fn join_key(left: &Self, right: &Self) -> (joined: Self)
            ensures joined@.dom().finite(), joined.spec_augorderedtablestper_wf()
        {
            let new_base = OrderedTableStPer::join_key(&left.base_table, &right.base_table);
            let new_reduction = if left.base_table.size() == 0 {
                right.cached_reduction.clone()
            } else if right.base_table.size() == 0 {
                left.cached_reduction.clone()
            } else {
                proof { assume(left.reducer.requires(
                    (&left.cached_reduction, &right.cached_reduction))); }
                (left.reducer)(&left.cached_reduction, &right.cached_reduction)
            };

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: left.reducer.clone(),
                identity: left.identity.clone(),
            };
            proof { lemma_aug_view(&r); }
            r
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
        {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = calculate_reduction(&new_base, &self.reducer, &self.identity);

            let r = Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            proof {
                lemma_aug_view(self);
                lemma_aug_view(&r);
            }
            r
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
        {
            proof { lemma_aug_view(self); }
            self.base_table.rank_key(k)
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
        {
            proof { lemma_aug_view(self); }
            self.base_table.select_key(i)
        }

        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            ensures
                self@.dom().finite(),
                split.0@.dom().finite(),
                split.0@.dom().subset_of(self@.dom()),
                split.1@.dom().finite(),
                split.1@.dom().subset_of(self@.dom()),
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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
            (left, right)
        }

        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite()
        {
            proof {
                lemma_aug_view(self);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.base_table.entries@);
            }
            self.cached_reduction.clone()
        }

        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
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

    // 10. iterators

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> AugOrderedTableStPer<K, V, F> {
        /// Returns an iterator over the table entries via the base ordered table.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.base_table.entries.seq@,
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    impl<'a, K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> std::iter::IntoIterator for &'a AugOrderedTableStPer<K, V, F> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.base_table.entries.seq@,
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> PartialEqSpecImpl for AugOrderedTableStPer<K, V, F> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> Eq for AugOrderedTableStPer<K, V, F> {}

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> PartialEq for AugOrderedTableStPer<K, V, F> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_table == other.base_table;
            proof { lemma_aug_view(self); lemma_aug_view(other); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT, F> Clone for AugOrderedTableStPer<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let r = Self {
                base_table: self.base_table.clone(),
                cached_reduction: self.cached_reduction.clone(),
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            r
        }
    }

    } // verus!

    // 13. derive impls outside verus!

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
