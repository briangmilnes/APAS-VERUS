//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table implementation extending TableStEph.

pub mod OrderedTableStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStEph::TableStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use vstd::prelude::*;
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

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        pub base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { self.base_table@ }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with ephemeral semantics.
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablesteph_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty();
        fn singleton(k: K, v: V) -> (tree: Self)
            requires obeys_feq_clone::<Pair<K, V>>()
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite();
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>(), obeys_feq_full::<V>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        fn is_empty(&self) -> (is_empty: B)
            requires self.spec_orderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty();
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@.dom().finite();
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<V>(),
                obeys_feq_full::<Pair<K, V>>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite();
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires obeys_feq_clone::<K>()
            ensures self@.dom().finite();
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires keys.spec_arraysetsteph_wf(), forall|k: &K| f.requires((k,)), obeys_feq_full::<K>()
            ensures tabulated@.dom().finite();
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures mapped@.dom().finite();
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (filtered: Self)
            requires forall|k: &K, v: &V| f.requires((k, v))
            ensures filtered@.dom().finite();
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            requires forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@.dom().finite();
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures self@.dom().finite();
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures self@.dom().finite();
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf(), obeys_feq_full::<Pair<K, V>>()
            ensures self@.dom().finite();
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf();
        /// ADT 43.1 first_key. Work Θ(log n), Span Θ(log n).
        fn first_key(&self) -> (first: Option<K>)
            ensures self@.dom().finite();
        /// ADT 43.1 last_key. Work Θ(log n), Span Θ(log n).
        fn last_key(&self) -> (last: Option<K>)
            ensures self@.dom().finite();
        /// ADT 43.1 previous_key. Work Θ(log n), Span Θ(log n).
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures self@.dom().finite();
        /// ADT 43.1 next_key. Work Θ(log n), Span Θ(log n).
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures self@.dom().finite();
        /// ADT 43.1 split_key. Work Θ(log n), Span Θ(log n).
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();
        /// ADT 43.1 join_key. Work Θ(log(|left|+|right|)), Span Θ(log(|left|+|right|)).
        fn join_key(&mut self, other: Self)
            requires obeys_feq_clone::<K>(), obeys_feq_full::<Pair<K, V>>(), obeys_view_eq::<K>()
            ensures self@.dom().finite();
        /// ADT 43.1 get_key_range. Work Θ(log n), Span Θ(log n).
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite();
        /// ADT 43.1 rank_key. Work Θ(log n), Span Θ(log n).
        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite();
        /// ADT 43.1 select_key. Work Θ(log n), Span Θ(log n).
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures self@.dom().finite();
        /// ADT 43.1 split_rank_key. Work Θ(log n), Span Θ(log n).
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.base_table.spec_tablesteph_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            let r = self.base_table.size();
            proof {
                assert(self@ =~= self.base_table@);
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            r
        }

        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite()
        {
            let base = TableStEph::singleton(k, v);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStEph { base_table: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            self.base_table.find(k)
        }

        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        fn is_empty(&self) -> (is_empty: B)
            ensures is_empty == self@.dom().is_empty()
        {
            proof { assert(self@ =~= self.base_table@); }
            self.size() == 0
        }

        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            ensures self@.dom().finite()
        {
            self.base_table.insert(k, v, combine);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            ensures self@ == old(self)@.remove(k@), self@.dom().finite()
        {
            let old_value = self.find(k);
            self.base_table.delete(k);
            proof {
                assert(self.base_table@ =~= old(self).base_table@.remove(k@));
                assert(self@ =~= old(self)@.remove(k@));
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            old_value
        }

        fn domain(&self) -> (domain: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            ensures tabulated@.dom().finite()
        {
            let base = TableStEph::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStEph { base_table: base }
        }

        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            ensures mapped@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                result_entries.push(Pair(pair.0.clone(), new_value));
                i += 1;
            }
            let result_seq = AVLTreeSeqStPerS::from_vec(result_entries);
            from_sorted_entries(result_seq)
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (filtered: Self)
            ensures filtered@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut result_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result_entries.push(Pair(pair.0.clone(), pair.1.clone()));
                }
                i += 1;
            }
            let result_seq = AVLTreeSeqStPerS::from_vec(result_entries);
            from_sorted_entries(result_seq)
        }

        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut reduced = init;
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    forall|r: R, k: &K, v: &V| f.requires((r, k, v)),
                decreases size - i,
            {
                let pair = entries.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
                i += 1;
            }
            reduced
        }

        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            self.base_table.union(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn difference(&mut self, other: &Self)
            ensures self@.dom().finite()
        {
            self.base_table.difference(&other.base_table);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.restrict(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            self.base_table.subtract(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
        }

        #[verifier::external_body]
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf()
        {
            let array_seq = self.base_table.entries();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            elements.sort_by(|a, b| a.0.cmp(&b.0));
            AVLTreeSeqStPerS::from_vec(elements)
        }

        fn first_key(&self) -> (first: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        fn last_key(&self) -> (last: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut i: usize = size;
            while i > 0
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    self@.dom().finite(),
                decreases i,
            {
                i -= 1;
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Less => return Some(pair.0.clone()),
                    _ => {},
                }
            }
            None
        }

        fn next_key(&self, k: &K) -> (successor: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                    self@.dom().finite(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Greater => return Some(pair.0.clone()),
                    _ => {},
                }
                i += 1;
            }
            None
        }

        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();
            let mut found_value: Option<V> = None;
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Less => left_entries.push(Pair(pair.0.clone(), pair.1.clone())),
                    std::cmp::Ordering::Greater => right_entries.push(Pair(pair.0.clone(), pair.1.clone())),
                    std::cmp::Ordering::Equal => found_value = Some(pair.1.clone()),
                }
                i += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            *self = Self::empty();
            (from_sorted_entries(left_seq), found_value, from_sorted_entries(right_seq))
        }

        fn join_key(&mut self, other: Self)
            ensures self@.dom().finite()
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            ensures range@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();
            let mut i: usize = 0;
            while i < size
                invariant
                    i <= size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                let ge_k1 = match pair.0.cmp(k1) {
                    std::cmp::Ordering::Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    std::cmp::Ordering::Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    range_entries.push(Pair(pair.0.clone(), pair.1.clone()));
                }
                i += 1;
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_entries);
            from_sorted_entries(range_seq)
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
        {
            let entries = self.collect();
            let size = entries.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < size
                invariant
                    count <= i,
                    i <= size,
                    entries.spec_avltreeseqstper_wf(),
                    size as nat == entries.spec_seq().len(),
                    self@.dom().finite(),
                decreases size - i,
            {
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Less => { count = count + 1; i = i + 1; },
                    _ => { i = size; },
                }
            }
            count
        }

        fn select_key(&self, i: usize) -> (selected: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            if i >= size {
                let current = self.clone();
                *self = Self::empty();
                return (current, Self::empty());
            }

            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();

            let mut j: usize = 0;
            while j < i
                invariant
                    j <= i,
                    i < size,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases i - j,
            {
                let elem = entries.nth(j);
                left_entries.push(Pair(elem.0.clone(), elem.1.clone()));
                j += 1;
            }
            let mut j: usize = i;
            while j < size
                invariant
                    j <= size,
                    i <= j,
                    size as nat == entries.spec_seq().len(),
                    entries.spec_avltreeseqstper_wf(),
                decreases size - j,
            {
                let elem = entries.nth(j);
                right_entries.push(Pair(elem.0.clone(), elem.1.clone()));
                j += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            *self = Self::empty();
            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT> OrderedTableStEph<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStEphIter { inner: self.base_table.entries.iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphIter<'a, K, V> {
        pub inner: ArraySeqStEphIter<'a, Pair<K, V>>,
    }

    impl<'a, K, V> View for OrderedTableStEphIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableStEphIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for OrderedTableStEphIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        fn next(&mut self) -> (next: Option<&'a Pair<K, V>>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableStEphGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;

        open spec fn view(&self) -> Seq<Pair<K, V>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStEphIter<'a, K, V> {
        type GhostIter = OrderedTableStEphGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStEphGhostIterator<'a, K, V> {
            OrderedTableStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIterator for OrderedTableStEphGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStEphIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStEphIter<'a, K, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<Pair<K, V>> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStEphIter<'a, K, V>) -> OrderedTableStEphGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::IntoIterator for &'a OrderedTableStEph<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStEphIter { inner: self.base_table.entries.iter() }
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            OrderedTableStEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (cloned: OrderedTableStEph<K, V>)
        requires entries.spec_avltreeseqstper_wf(),
        ensures cloned@.dom().finite(),
    {
        let len = entries.length();
        let mut elements: Vec<Pair<K, V>> = Vec::new();
        let mut i: usize = 0;
        while i < len
            invariant
                entries.spec_avltreeseqstper_wf(),
                len as nat == entries@.len(),
                i <= len,
            decreases len - i,
        {
            elements.push(entries.nth(i).clone_plus());
            i = i + 1;
        }
        OrderedTableStEph {
            base_table: crate::Chap42::TableStEph::TableStEph::from_sorted_entries(elements),
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.base_table == other.base_table
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Debug for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT> fmt::Display for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    // 12. macros

    /// Macro for creating ephemeral ordered tables from sorted key-value pairs
    #[macro_export]
    macro_rules! OrderedTableStEphLit {
        () => {
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::OrderedTableStEph::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::from_sorted_entries(seq)
        }};
    }
}
