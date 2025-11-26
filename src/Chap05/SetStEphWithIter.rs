//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet`.
//! This version uses a set-based iterator view (visited, current, remaining).

pub mod SetStEphWithIter {

    use vstd::prelude::*;

verus! {

    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;
    use crate::vstdplus::seq_set::*;
    use crate::vstdplus::feq::feq::*;
    use vstd::hash_set::HashSetWithView;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::vec::vec::*;

    broadcast use {
        vstd::seq_lib::group_seq_properties, 
        vstd::seq::group_seq_axioms,
        vstd::set::group_set_axioms, 
        crate::vstdplus::feq::feq::group_feq_axioms
    };

    pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
        &&& obeys_key_model::<T>() 
        &&& obeys_feq_full::<T>()
    }

    #[verifier::reject_recursive_types(T)]
    pub struct SetStEphWithIter<T: StT + Hash> { pub elements: HashSetWithViewPlus<T> }

    // Ghost view: visited, current, remaining sets
    #[verifier::reject_recursive_types(T)]
    pub struct SetStEphIterView<T> {
        pub visited: Set<T>,
        pub current: Option<T>,
        pub remaining: Set<T>,
    }

    // Helper: convert Option<V> to Set<V>
    pub open spec fn option_to_set<V>(opt: Option<V>) -> Set<V> {
        match opt {
            None => Set::empty(),
            Some(v) => Set::empty().insert(v),
        }
    }

    // Iterator wrapper with set-based view
    #[verifier::reject_recursive_types(T)]
    pub struct SetStEphIter<'a, T: StT + Hash> {
        pub inner: std::collections::hash_set::Iter<'a, T>,
    }

    // Iterator view is (visited, current, remaining) sets based on position
    impl<'a, T: StT + Hash> View for SetStEphIter<'a, T> {
        type V = SetStEphIterView<T::V>;
        
        open spec fn view(&self) -> SetStEphIterView<T::V> {
            let pos = self.inner@.0;
            let seq = self.inner@.1.map(|_i: int, x: T| x@);
            if pos == 0 {
                SetStEphIterView {
                    visited: Set::empty(),
                    current: None,
                    remaining: seq.to_set(),
                }
            } else if pos <= seq.len() {
                SetStEphIterView {
                    visited: seq.take(pos - 1).to_set(),
                    current: Some(seq[pos - 1]),
                    remaining: seq.skip(pos).to_set(),
                }
            } else {
                // pos > len: exhausted
                SetStEphIterView {
                    visited: seq.to_set(),
                    current: None,
                    remaining: Set::empty(),
                }
            }
        }
    }

    // Iterator invariant
    pub open spec fn iter_invariant<'a, T: StT + Hash>(it: &SetStEphIter<'a, T>, original: Set<T::V>) -> bool {
        let pos = it.inner@.0;
        let seq = it.inner@.1.map(|_i: int, x: T| x@);
        &&& pos <= seq.len()
        &&& seq.to_set() == original
        &&& seq.no_duplicates()
        &&& it@.visited.union(option_to_set(it@.current)).union(it@.remaining) == original
    }

    impl<'a, T: StT + Hash> SetStEphIter<'a, T> {
        pub fn next(&mut self) -> (result: Option<&'a T>)
            requires
                old(self).inner@.1.no_duplicates(),
                obeys_feq_full::<T>(),
            ensures ({
                let (old_index, old_seq) = old(self).inner@;
                let (new_index, new_seq) = self.inner@;
                let old_mapped = old_seq.map(|_i: int, x: T| x@);
                match result {
                    None => {
                        &&& self.inner@ == old(self).inner@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element@ == old_mapped[old_index]
                    },
                }
            }),
            // Set-based postconditions
            ({
                let old_view = old(self)@;
                let new_view = self@;
                match result {
                    None => {
                        &&& old_view.remaining.is_empty()
                        &&& new_view == old_view  // No change when exhausted
                    },
                    Some(element) => {
                        &&& old_view.remaining.contains(element@)
                        &&& new_view.visited == old_view.visited.union(option_to_set(old_view.current))
                        &&& new_view.current == Some(element@)
                        &&& new_view.remaining == old_view.remaining.remove(element@)
                    },
                }
            })
        {
            let result = self.inner.next();
            proof {
                if result.is_some() {
                    let (old_index, old_seq) = old(self).inner@;
                    let (new_index, new_seq) = self.inner@;
                    let seq = old_seq.map(|_i: int, x: T| x@);
                    
                    // seq.skip(old_index + 1) == seq.skip(old_index).drop_first()
                    assert(seq.skip(old_index + 1) =~= seq.skip(old_index).drop_first());
                    
                    let skipped = seq.skip(old_index);
                    assert(skipped.len() > 0);
                    assert(skipped[0] == seq[old_index]);
                    
                    // From vstd: drop_first().to_set().insert(first()) == to_set()
                    // when first is not in drop_first (which holds for no_duplicates)
                    let df = skipped.drop_first();
                    let df_set = df.to_set();
                    let full_set = skipped.to_set();
                    let first = skipped[0];
                    
                    assert(df_set.insert(first) =~= full_set);
                    
                    // Since first is not in df_set (no_duplicates), we have:
                    // df_set.insert(first).remove(first) == df_set
                    // And df_set.insert(first) == full_set
                    // So full_set.remove(first) == df_set
                    assert(!df_set.contains(first)) by {
                        // df = skipped.drop_first()
                        // skipped = seq.skip(old_index)
                        // seq = old_seq.map(|_, x| x@)
                        // old_seq.no_duplicates() from precondition
                        
                        // Prove by showing: forall j: 0 <= j < df.len() ==> df[j] != first
                        assert forall|j: int| 0 <= j < df.len() implies df[j] != first by {
                            assert(skipped.len() == seq.len() - old_index);
                            assert(df.len() == skipped.len() - 1);
                            assert(0 <= j + 1 < skipped.len());
                            assert(df[j] == skipped[j + 1]);
                            assert(0 <= old_index + j + 1 < seq.len());
                            assert(skipped[j + 1] == seq[old_index + j + 1]);
                            assert(first == seq[old_index]);
                            
                            let idx1 = old_index + j + 1;
                            let idx2 = old_index;
                            assert(idx1 != idx2);
                            
                            // old_seq.no_duplicates() ==> old_seq[idx1] != old_seq[idx2]
                            assert(old_seq.no_duplicates());
                            assert(old_seq[idx1] != old_seq[idx2]);
                            // seq[k] = old_seq[k]@
                            assert(seq[idx1] == old_seq[idx1]@);
                            assert(seq[idx2] == old_seq[idx2]@);
                            // From obeys_feq_view_injective: x@ == y@ ==> x == y
                            // Contrapositive: x != y ==> x@ != y@
                            assert(obeys_feq_view_injective::<T>());
                            // If old_seq[idx1]@ == old_seq[idx2]@, then old_seq[idx1] == old_seq[idx2]
                            // But old_seq[idx1] != old_seq[idx2], so old_seq[idx1]@ != old_seq[idx2]@
                            assert(seq[idx1] != seq[idx2]);
                        }
                    }
                    assert(df_set =~= df_set.insert(first).remove(first));
                    assert(df_set =~= full_set.remove(first));
                }
            }
            result
        }
    }

    pub trait SetStEphWithIterTrait<T: StT + Hash> : View<V = Set<<T as View>::V>> + Sized {
        fn FromVec(v: Vec<T>) -> (s: SetStEphWithIter<T>)
            requires valid_key_type::<T>()
            ensures s@ == v@.map(|i: int, x: T| x@).to_set();

        fn iter<'a>(&'a self) -> (it: SetStEphIter<'a, T>)
            requires valid_key_type::<T>()
            ensures
                it@.visited == Set::<<T as View>::V>::empty(),
                it@.current.is_none(),
                it@.remaining == self@,
                it.inner@.0 == 0int,
                it.inner@.1.map(|i: int, k: T| k@).to_set() == self@,
                it.inner@.1.no_duplicates();

        fn empty() -> (empty: Self)
            requires valid_key_type::<T>()
            ensures empty@ == Set::<<T as View>::V>::empty();

        fn singleton(x: T) -> Self
            requires valid_key_type::<T>();

        fn size(&self) -> N;

        fn mem(&self, x: &T) -> (contains: B)
            requires valid_key_type::<T>()
            ensures contains == self@.contains(x@);

        fn insert(&mut self, x: T) -> (inserted: bool)
            requires valid_key_type::<T>()
            ensures
                self@ == old(self)@.insert(x@),
                inserted == !old(self)@.contains(x@);

        fn union(&self, s2: &SetStEphWithIter<T>) -> (union: Self)
            requires valid_key_type::<T>()
            ensures union@ == self@.union(s2@);

        fn intersection(&self, s2: &SetStEphWithIter<T>) -> (intersection: Self)
            requires valid_key_type::<T>()
            ensures intersection@ == self@.intersect(s2@);
    }

    impl<T: StT + Hash> View for SetStEphWithIter<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Self::V { self.elements@ }
    }

    impl<T: StT + Hash> Clone for SetStEphWithIter<T> {
        fn clone(&self) -> (clone: Self)
            ensures clone@ == self@
        { SetStEphWithIter { elements: self.elements.clone() } }
    }

    impl<T: StT + Hash> SetStEphWithIterTrait<T> for SetStEphWithIter<T> {
        fn FromVec(v: Vec<T>) -> SetStEphWithIter<T> {
            let mut s = SetStEphWithIter::empty();
            let mut i: usize = 0;
            let ghost v_seq = v@;
            
            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<T>(),
                    i <= v.len(),
                    v@ == v_seq,
                    s@ == v_seq.take(i as int).map(|idx: int, x: T| x@).to_set(),
                decreases v.len() - i,
            {
                if i >= v.len() {
                    proof { lemma_take_full_to_set_with_view(v_seq); }
                    break;
                }
                
                let x = &v[i];
                let x_clone = x.clone_plus();
                let _ = s.insert(x_clone);
                proof { lemma_take_one_more_extends_the_seq_set_with_view(v_seq, i as int); }
                i = i + 1;
            }
            
            s
        }

        fn iter<'a>(&'a self) -> (it: SetStEphIter<'a, T>) {
            let inner = self.elements.iter();
            proof { 
                lemma_seq_map_to_set_equality(inner@.1, self@); 
            }
            SetStEphIter { inner }
        }

        fn empty() -> SetStEphWithIter<T> { SetStEphWithIter { elements: HashSetWithViewPlus::new() } }

        fn singleton(x: T) -> SetStEphWithIter<T> {
            let mut s = HashSetWithViewPlus::new();
            let _ = s.insert(x);
            SetStEphWithIter { elements: s }
        }

        fn size(&self) -> (size: N)
            ensures size == self@.len()
        { self.elements.len() }

        fn mem(&self, x: &T) -> (contains: B) { self.elements.contains(x) }

        fn insert(&mut self, x: T) -> (inserted: bool)
        { self.elements.insert(x) }

        fn union(&self, s2: &SetStEphWithIter<T>) -> (union: SetStEphWithIter<T>)
        {
            let mut union = self.clone_plus();
            let mut it = s2.iter();
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            let ghost s2_seq = it.inner@.1;

            #[verifier::loop_isolation(false)]
            loop 
                invariant
                    valid_key_type::<T>(),
                    it.inner@.0 <= s2_seq.len(),
                    it.inner@.1 == s2_seq,
                    s2_seq.map(|i: int, k: T| k@).to_set() == s2_view,
                    s2_seq.no_duplicates(),
                    union@ == s1_view.union(it@.visited.union(option_to_set(it@.current))),
               decreases s2_seq.len() - it.inner@.0,
            {
                let ghost old_index = it.inner@.0;
                let ghost old_union = union@;
                
                match it.next() {
                    Some(x) => {
                        let x_clone = x.clone_plus();
                        let _ = union.insert(x_clone);
                    },
                    None => {
                        proof {
                            assert(it@.remaining.is_empty());
                        }
                        break;
                    }
                }
            }
            union
        }

        fn intersection(&self, s2: &SetStEphWithIter<T>) -> (intersection: SetStEphWithIter<T>)
        {
            let mut intersection = SetStEphWithIter::empty();
            let mut it = self.iter();
            let ghost s1_view = self@;
            let ghost s2_view = s2@;
            let ghost s1_seq = it.inner@.1;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    valid_key_type::<T>(),
                    it.inner@.0 <= s1_seq.len(),
                    it.inner@.1 == s1_seq,
                    s1_seq.map(|i: int, k: T| k@).to_set() == s1_view,
                    s1_seq.no_duplicates(),
                    intersection@ == it@.visited.union(option_to_set(it@.current)).intersect(s2_view),
                decreases s1_seq.len() - it.inner@.0,
            {
                let ghost old_index = it.inner@.0;
                let ghost old_intersection = intersection@;
                
                match it.next() {
                    Some(s1mem) => {
                        if s2.mem(s1mem) {
                            let s1mem_clone = s1mem.clone_plus();
                            let _ = intersection.insert(s1mem_clone);
                        } 
                    },
                    None => {
                        proof {
                            assert(it@.remaining.is_empty());
                        }
                        break;
                    }
                }
            }
            
            intersection
        }
    }

    impl<T: StT + Hash> std::hash::Hash for SetStEphWithIter<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.elements.hash(state); }
    }

    impl<T: StT + Hash> Eq for SetStEphWithIter<T> {}

  } // verus!

    impl<T: StT + Hash> PartialEq for SetStEphWithIter<T> {
        fn eq(&self, other: &Self) -> bool { self.elements == other.elements }
    }

    impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Display for SetStEphWithIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Set({})", self.elements.len())
        }
    }
    
    impl<T: crate::Types::Types::StT + std::hash::Hash> std::fmt::Debug for SetStEphWithIter<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "SetStEphWithIter({})", self.elements.len())
        }
    }
}

