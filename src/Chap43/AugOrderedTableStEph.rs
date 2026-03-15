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
    // 10. iterators
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
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

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
    ) -> (reduced: V)
    where
        F: Fn(&V, &V) -> V + Clone,
        ensures base@.dom().finite(),
    {
        if base.size() == 0 {
            return identity.clone();
        }

        let pairs = base.collect();
        let mut reduced = identity.clone();
        let mut first = true;

        for i in 0..pairs.length() {
            let pair = pairs.nth(i);
            if first {
                reduced = pair.1.clone();
                first = false;
            } else {
                reduced = reducer(&reduced, &pair.1);
            }
        }

        reduced
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
        spec fn spec_augorderedtablesteph_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            requires self.spec_augorderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@.dom().finite();
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_augorderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_augorderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite();
        fn insert<G: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: G)
            requires
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@.dom().finite();
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_augorderedtablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@.dom().finite();
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures self@.dom().finite();
        fn tabulate<G: Fn(&K) -> V>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>()
            ensures tabulated@.dom().finite();
        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (mapped: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures mapped@.dom().finite();
        fn filter<G: Fn(&K, &V) -> B>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            requires
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures filtered@.dom().finite();
        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            requires
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
            requires
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        fn difference(&mut self, other: &Self)
            requires old(self).spec_augorderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures self@.dom().finite();
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures self@.dom().finite();
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_augorderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures self@.dom().finite();
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// ADT 43.1 first_key = min key in dom. Work Θ(log n), Span Θ(log n).
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(v, t);
        /// ADT 43.1 last_key = max key in dom. Work Θ(log n), Span Θ(log n).
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(t, v);
        /// ADT 43.1 previous_key = max{k' in dom | k' < k}. Work Θ(log n), Span Θ(log n).
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// ADT 43.1 next_key = min{k' in dom | k' > k}. Work Θ(log n), Span Θ(log n).
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// ADT 43.1 split_key. Work Θ(log n), Span Θ(log n).
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized,
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@);
        fn join_key(&mut self, other: Self)
            requires obeys_feq_clone::<K>(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures self@.dom().finite();
        /// ADT 43.1 get_key_range. Work Θ(log n), Span Θ(log n).
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom());
        /// ADT 43.1 rank_key. Work Θ(log n), Span Θ(log n).
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// ADT 43.1 select_key. Work Θ(log n), Span Θ(log n).
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// ADT 43.1 split_rank_key. Work Θ(log n), Span Θ(log n).
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized,
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom());
        fn reduce_val(&self) -> (reduced: V)
            ensures self@.dom().finite();
        fn reduce_range(&self, k1: &K, k2: &K) -> (reduced: V)
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT, F> AugOrderedTableStEphTrait<K, V, F> for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        open spec fn spec_augorderedtablesteph_wf(&self) -> bool {
            self.base_table.spec_orderedtablesteph_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.size()
        }

        fn empty(reducer: F, identity: V) -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
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

        fn singleton(k: K, v: V, reducer: F, identity: V) -> (tree: Self)
            ensures tree@.dom().finite()
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

        fn find(&self, k: &K) -> (found: Option<V>)
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (value: Option<V>)
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                }
        {
            self.base_table.lookup(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty(), self@.dom().finite()
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

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@.dom().finite()
        {
            let updated = self.base_table.delete(k);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
            updated
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

        fn map<G: Fn(&K, &V) -> V>(&self, f: G) -> (mapped: Self)
            ensures mapped@.dom().finite()
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

        fn filter<G: Fn(&K, &V) -> B>(&self, f: G, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (filtered: Self)
            ensures filtered@.dom().finite()
        {
            let new_base = self.base_table.filter(f, Ghost(spec_pred));
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

        fn reduce<R, G: Fn(R, &K, &V) -> R>(&self, init: R, f: G) -> (reduced: R)
            ensures self@.dom().finite()
        {
            proof { lemma_aug_view(self); }
            self.base_table.reduce(init, f)
        }

        fn intersection<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
        {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = calculate_reduction(&self.base_table, &self.reducer, &self.identity);
            proof { lemma_aug_view(self); }
        }

        fn union<G: Fn(&V, &V) -> V>(&mut self, other: &Self, f: G)
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

        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len()
        {
            proof { lemma_aug_view(self); }
            self.base_table.collect()
        }

        #[verifier::external_body]
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(v, t),
        {
            self.base_table.first_key()
        }

        #[verifier::external_body]
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> TotalOrder::le(t, v),
        {
            self.base_table.last_key()
        }

        #[verifier::external_body]
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            self.base_table.previous_key(k)
        }

        #[verifier::external_body]
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            self.base_table.next_key(k)
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
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

            proof {
                lemma_aug_view(self);
                lemma_aug_view(&left);
                lemma_aug_view(&right);
            }
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

        #[verifier::external_body]
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, *k) && t@ != k@).len(),
        {
            self.base_table.rank_key(k)
        }

        #[verifier::external_body]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int,
        {
            self.base_table.select_key(i)
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
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

    impl<K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> AugOrderedTableStEph<K, V, F> {
        /// Returns an iterator over the table entries via the base ordered table.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.base_table.entries.seq@,
                iter_invariant(&it),
        {
            self.base_table.iter()
        }
    }

    impl<'a, K: StT + Ord, V: StT, F: Fn(&V, &V) -> V + Clone> std::iter::IntoIterator for &'a AugOrderedTableStEph<K, V, F> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<'a, K, V>;
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

    impl<K: StT + Ord, V: StT, F> Clone for AugOrderedTableStEph<K, V, F>
    where
        F: Fn(&V, &V) -> V + Clone,
    {
        #[verifier::external_body]
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
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
