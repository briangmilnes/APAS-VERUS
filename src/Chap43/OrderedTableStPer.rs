//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered table implementation extending TableStPer.

pub mod OrderedTableStPer {

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT> {
        pub base_table: TableStPer<K, V>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT> View for OrderedTableStPer<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_entries_to_map(self.base_table.entries@) }
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_wf(&self) -> bool;

        fn size(&self) -> (count: usize)
            requires self.spec_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_wf();
        fn singleton(k: K, v: V) -> (table: Self)
            requires obeys_feq_clone::<Pair<K, V>>(),
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table@.dom().finite(), table.spec_wf();
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_wf(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        fn insert(&self, k: K, v: V) -> (table: Self)
            requires self.spec_wf(),
            ensures table@.dom().finite(), table.spec_wf();
        fn delete(&self, k: &K) -> (table: Self)
            requires self.spec_wf(), obeys_feq_clone::<Pair<K, V>>(),
            ensures table@ == self@.remove(k@), table@.dom().finite(), table.spec_wf();
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            ensures self@.dom().finite();
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            requires forall|k: &K| f.requires((k,)),
            ensures table@.dom().finite();
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            requires forall|v: &V| f.requires((v,)),
            ensures table@.dom().finite();
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (table: Self)
            requires forall|k: &K, v: &V| f.requires((k, v)),
            ensures table@.dom().finite();
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures table@.dom().finite();
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires self.spec_wf(), forall|v1: &V, v2: &V| f.requires((v1, v2)),
            ensures table@.dom().finite(), table.spec_wf();
        fn difference(&self, other: &Self) -> (table: Self)
            ensures table@.dom().finite();
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            ensures table@.dom().finite();
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            ensures table@.dom().finite();
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), sorted_entries.spec_well_formed();
        fn first_key(&self) -> (key: Option<K>)
            ensures self@.dom().finite();
        fn last_key(&self) -> (key: Option<K>)
            ensures self@.dom().finite();
        fn previous_key(&self, k: &K) -> (key: Option<K>)
            ensures self@.dom().finite();
        fn next_key(&self, k: &K) -> (key: Option<K>)
            ensures self@.dom().finite();
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite();
        fn join_key(left: &Self, right: &Self) -> (table: Self)
            requires left.spec_wf(),
            ensures table@.dom().finite(), table.spec_wf();
        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            ensures table@.dom().finite();
        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite();
        fn select_key(&self, i: usize) -> (key: Option<K>)
            ensures self@.dom().finite();
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_wf(&self) -> bool {
            self.base_table.spec_wf()
        }

        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@);
            }
            self.base_table.size()
        }

        fn empty() -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_wf()
        {
            OrderedTableStPer {
                base_table: TableStPer::empty(),
            }
        }

        fn singleton(k: K, v: V) -> (table: Self)
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table@.dom().finite(), table.spec_wf()
        {
            let base = TableStPer::singleton(k, v);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn find(&self, k: &K) -> (found: Option<V>) {
            self.base_table.find(k)
        }

        fn insert(&self, k: K, v: V) -> (table: Self)
            ensures table@.dom().finite(), table.spec_wf()
        {
            let base = self.base_table.insert(k, v, |_old: &V, new: &V| -> (r: V) { new.clone() });
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn delete(&self, k: &K) -> (table: Self)
            ensures table@ == self@.remove(k@), table@.dom().finite(), table.spec_wf()
        {
            let base = self.base_table.delete(k);
            proof {
                lemma_entries_to_map_finite::<K::V, V::V>(base.entries@);
                assume(spec_entries_to_map(base.entries@) == spec_entries_to_map(self.base_table.entries@).remove(k@));
            }
            OrderedTableStPer { base_table: base }
        }

        fn domain(&self) -> (keys: ArraySetStEph<K>)
            ensures self@.dom().finite()
        {
            proof { lemma_entries_to_map_finite::<K::V, V::V>(self.base_table.entries@); }
            self.base_table.domain()
        }

        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = TableStPer::tabulate(f, keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.map(f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.filter(f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.intersection(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            ensures table@.dom().finite(), table.spec_wf()
        {
            let base = self.base_table.union(&other.base_table, f);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn difference(&self, other: &Self) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.difference(&other.base_table);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.restrict(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            ensures table@.dom().finite()
        {
            let base = self.base_table.subtract(keys);
            proof { lemma_entries_to_map_finite::<K::V, V::V>(base.entries@); }
            OrderedTableStPer { base_table: base }
        }

        #[verifier::external_body]
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures self@.dom().finite(), sorted_entries.spec_well_formed()
        {
            let array_seq = self.base_table.collect();
            let len = array_seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            elements.sort_by(|a, b| a.0.cmp(&b.0));
            AVLTreeSeqStPerS::from_vec(elements)
        }

        fn first_key(&self) -> (key: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        fn last_key(&self) -> (key: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        fn previous_key(&self, k: &K) -> (key: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut i: usize = size;
            while i > 0
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
                    i <= size,
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

        fn next_key(&self, k: &K) -> (key: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut i: usize = 0;
            while i < size
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
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

        #[verifier::external_body]
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries = Vec::new();
            let mut right_entries = Vec::new();
            let mut found_value = None;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    left_entries.push(pair.clone());
                } else if &pair.0 > k {
                    right_entries.push(pair.clone());
                } else {
                    found_value = Some(pair.1.clone());
                }
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            (
                from_sorted_entries(left_seq),
                found_value,
                from_sorted_entries(right_seq),
            )
        }

        fn join_key(left: &Self, right: &Self) -> (table: Self)
            ensures table@.dom().finite(), table.spec_wf()
        {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
        }

        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            ensures table@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut range_entries: Vec<Pair<K, V>> = Vec::new();

            let mut i: usize = 0;
            while i < size
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
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
                    range_entries.push(pair.clone_plus());
                }
                i += 1;
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_entries);
            from_sorted_entries(range_seq)
        }

        fn rank_key(&self, k: &K) -> (rank: usize)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            let mut done = false;
            while i < size && !done
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
                    count <= i,
                    i <= size,
                decreases size - i,
            {
                let pair = entries.nth(i);
                match pair.0.cmp(k) {
                    std::cmp::Ordering::Less => { count += 1; },
                    _ => { done = true; },
                }
                i += 1;
            }
            count
        }

        fn select_key(&self, i: usize) -> (key: Option<K>)
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            ensures self@.dom().finite()
        {
            let entries = self.collect();
            let size = entries.length();

            if i >= size {
                return (self.clone(), Self::empty());
            }

            let mut left_entries: Vec<Pair<K, V>> = Vec::new();
            let mut right_entries: Vec<Pair<K, V>> = Vec::new();

            let mut j: usize = 0;
            while j < i
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
                    i < size,
                    j <= i,
                decreases i - j,
            {
                left_entries.push(entries.nth(j).clone_plus());
                j += 1;
            }
            let mut j: usize = i;
            while j < size
                invariant
                    entries.spec_well_formed(),
                    size as nat == entries.spec_seq().len(),
                    j <= size,
                decreases size - j,
            {
                right_entries.push(entries.nth(j).clone_plus());
                j += 1;
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            (from_sorted_entries(left_seq), from_sorted_entries(right_seq))
        }
    }

    pub fn from_sorted_entries<K: StT + Ord, V: StT>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (table: OrderedTableStPer<K, V>)
        ensures table@.dom().finite()
    {
        proof { assume(entries.spec_well_formed()); }
        let len = entries.length();
        let mut elements: Vec<Pair<K, V>> = Vec::new();
        let mut i: usize = 0;
        while i < len
            invariant
                entries.spec_well_formed(),
                len as nat == entries.spec_seq().len(),
                i <= len,
            decreases len - i,
        {
            elements.push(entries.nth(i).clone_plus());
            i += 1;
        }
        OrderedTableStPer {
            base_table: crate::Chap42::TableStPer::TableStPer::from_sorted_entries(elements),
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<'_, K, V>)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStPerIter { inner: self.base_table.entries.iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<'a, K, V> {
        inner: ArraySeqStPerIter<'a, Pair<K, V>>,
    }

    impl<'a, K, V> View for OrderedTableStPerIter<'a, K, V> {
        type V = (int, Seq<Pair<K, V>>);
        closed spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, K, V>(it: &OrderedTableStPerIter<'a, K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::Iterator for OrderedTableStPerIter<'a, K, V> {
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
    pub struct OrderedTableStPerGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a (K, V)>,
    }

    impl<'a, K, V> View for OrderedTableStPerGhostIterator<'a, K, V> {
        type V = Seq<Pair<K, V>>;

        open spec fn view(&self) -> Seq<Pair<K, V>> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<'a, K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<'a, K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<'a, K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: StT + Ord, V: StT> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<'a, K, V> {
        type ExecIter = OrderedTableStPerIter<'a, K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStPerIter<'a, K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<'a, K, V>) -> OrderedTableStPerGhostIterator<'a, K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = &'a Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<'a, K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.base_table.entries.seq@,
                iter_invariant(&it),
        {
            OrderedTableStPerIter { inner: self.base_table.entries.iter() }
        }
    }

    // 11. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord, V: StT> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.base_table.entries == other.base_table.entries;
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            OrderedTableStPer {
                base_table: self.base_table.clone(),
            }
        }
    }

    } // verus!

    /// Macro for creating ordered tables from sorted key-value pairs
    #[macro_export]
    macro_rules! OrderedTableStPerLit {
        () => {
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::OrderedTableStPer::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::from_sorted_entries(seq)
        }};
    }
}
