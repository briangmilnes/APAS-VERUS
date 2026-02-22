//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//!
//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

//  Table of Contents
//	1. module
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod MathSeq {
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::hash_map::HashMapWithView;

    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::vstdplus::seq_set::*;
    use vstd::slice::slice_subrange;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

        //		3. broadcast use

        // Table of Contents
        // 1. module
        // 2. imports
        // 3. broadcast use
        // 4. type definitions
        // 5. view impls
        // 6. spec fns
        // 8. traits
        // 9. impls
        // 10. iterators
        // 11. derive impls in verus!

        // 3. broadcast use

        broadcast use {
            // Vec
            vstd::std_specs::vec::group_vec_axioms,
            // Set groups
            vstd::set::group_set_axioms,
            vstd::set_lib::group_set_lib_default,
            vstd::set_lib::group_set_properties,
            // Seq groups
            vstd::seq::group_seq_axioms,
            vstd::prelude::Seq::group_seq_extra,
            vstd::seq_lib::group_seq_lib_default,
            vstd::seq_lib::group_seq_properties,
            // HashMap
            vstd::std_specs::hash::axiom_random_state_builds_valid_hashers,
            vstd::std_specs::hash::axiom_contains_deref_key,
            // Our groups
            crate::vstdplus::feq::feq::group_feq_axioms,
            crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        // Veracity: added broadcast groups
        vstd::seq_lib::group_to_multiset_ensures,
        };


        //		4. type definitions

        #[verifier::reject_recursive_types(T)]
        pub struct MathSeqS<T: StT> {
            pub data: Vec<T>,
        }


        //		5. view impls

        // 5. view impls
        
        impl<T: StT> View for MathSeqS<T> {
            type V = Seq<T::V>;
            
            open spec fn view(&self) -> Seq<T::V> {
                self.data@.map_values(|t: T| t@)
            }
        }


        //		6. spec fns

        // 4. type definitions

        pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
            &&& obeys_key_model::<T>()
                &&& obeys_feq_full::<T>()
        }

        // 6. spec fns

        pub open spec fn spec_clamp(val: int, max: int) -> int {
            if val < 0 { 0 } else if val > max { max } else { val }
        }


        //		8. traits

        // 8. traits

        pub trait MathSeqSTrait<T: StT>: Sized + View<V = Seq<T::V>> {
            spec fn spec_len(&self) -> nat;

            spec fn spec_nth(&self, i: int) -> T::V
                recommends 0 <= i < self.spec_len();

            spec fn spec_is_empty(&self) -> bool;

            spec fn spec_is_singleton(&self) -> bool;

            /// Raw data access for clone-level reasoning.
            spec fn spec_seq(&self) -> Seq<T>;

            /// - APAS: no cost spec (definitions chapter).
            /// - Claude-Opus-4.6: O(n) — Vec allocation + clone fill.
            fn new(length: N, init_value: T) -> (new_seq: Self)
                ensures
                    new_seq.spec_len() == length,
                    forall|i: int| #![trigger new_seq.spec_seq()[i]] 0 <= i < length ==> cloned(init_value, new_seq.spec_seq()[i]);

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — direct index write.
            fn set(&mut self, index: N, value: T) -> (success: bool)
                ensures
                    success ==> index < old(self).spec_len()
                        && self.spec_len() == old(self).spec_len()
                        && self@[index as int] == value@
                        && forall|i: int| 0 <= i < self.spec_len() && i != index as int ==> self@[i] == old(self)@[i],
                    !success ==> index >= old(self).spec_len() && self@ == old(self)@;

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1).
            fn length(&self) -> (len: N)
                ensures len == self.spec_len();

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — direct index read.
            fn nth(&self, index: N) -> (elem: &T)
                requires index < self.spec_len()
                ensures elem@ == self@[index as int];

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1).
            fn empty() -> (empty_seq: Self)
                ensures empty_seq.spec_len() == 0;

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1).
            fn singleton(item: T) -> (singleton: Self)
                ensures
                    singleton.spec_len() == 1,
                    singleton@[0] == item@;

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: amortized O(1) — Vec::push.
            fn add_last(&mut self, value: T)
                ensures
                    self.spec_len() == old(self).spec_len() + 1,
                    self@[self.spec_len() - 1] == value@,
                    forall|i: int| 0 <= i < old(self).spec_len() ==> self@[i] == old(self)@[i];

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — Vec::pop.
            fn delete_last(&mut self) -> (shortened: Option<T>)
                ensures
                    old(self).spec_len() == 0 ==> shortened is None && self@ == old(self)@,
                    old(self).spec_len() > 0  ==> shortened is Some
                        && shortened->Some_0@ == old(self)@[old(self).spec_len() - 1]
                        && self.spec_len() == old(self).spec_len() - 1
                        && forall|i: int| 0 <= i < self.spec_len() ==> self@[i] == old(self)@[i];

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1).
            fn is_empty(&self) -> (emptiness: bool)
                ensures emptiness == self.spec_is_empty();

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1).
            fn is_singleton(&self) -> (singularity: bool)
                ensures singularity == self.spec_is_singleton();

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — move, no copy.
            fn from_vec(data: Vec<T>) -> (seq: Self)
                ensures seq.spec_seq() == data@;

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(n) — delegates to new.
            fn with_len(length: N, init_value: T) -> (seq_of_len_value: Self)
                ensures
                    seq_of_len_value.spec_len() == length,
                    forall|i: int| #![trigger seq_of_len_value.spec_seq()[i]] 0 <= i < length ==> cloned(init_value, seq_of_len_value.spec_seq()[i]);

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — returns slice reference.
            fn subseq(&self, start: N, length: N) -> (subseq: &[T])
                ensures
                    subseq@.len() <= length,
                ({
                    let s = spec_clamp(start as int, self.spec_seq().len() as int);
                    let e = spec_clamp((start + length) as int, self.spec_seq().len() as int);
                    subseq@ == self.spec_seq().subrange(s, e)
                });

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(length) — copies subrange.
            fn subseq_copy(&self, start: N, length: N) -> (subseq: Self) where T: Copy
                requires
                    start as int + length as int <= self.spec_seq().len(),
                ensures
                    subseq.spec_len() == length,
                    subseq.spec_seq() == self.spec_seq().subrange(start as int, (start + length) as int);

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(n) — builds index vector.
            fn domain(&self) -> (domain: Vec<N>)
                ensures domain@.len() == self.spec_len();

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(n) expected — hash set dedup.
            fn range(&self) -> (range: Vec<T>)
                requires valid_key_type::<T>()
                ensures
                    range@.len() <= self.spec_seq().len(),
                    range@.no_duplicates();

            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(n) expected — hash map counting, two passes.
            fn multiset_range(&self) -> (range: Vec<(N, T)>)
                requires
                    valid_key_type::<T>(),
                    forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2,
                ensures
                    range@.len() <= self.spec_seq().len();

            /// Borrow iterator over the sequence elements.
            /// - APAS: no cost spec.
            /// - Claude-Opus-4.6: O(1) — returns iterator wrapper.
            fn iter(&self) -> (it: MathSeqIter<'_, T>)
                ensures
                    it@.0 == 0,
                    it@.1 == self.spec_seq(),
                    iter_invariant(&it);
        }


        //		9. impls

        // 9. impls

        impl<T: StT + Hash> MathSeqSTrait<T> for MathSeqS<T> {

            open spec fn spec_len(&self) -> nat {
                self@.len()
            }

            open spec fn spec_nth(&self, i: int) -> T::V {
                self@[i]
            }

            open spec fn spec_is_empty(&self) -> bool {
                self.spec_len() == 0
            }

            open spec fn spec_is_singleton(&self) -> bool {
                self.spec_len() == 1
            }

            open spec fn spec_seq(&self) -> Seq<T> {
                self.data@
            }

            fn new(length: N, init_value: T) -> (new_seq: Self)
            {
                let v = vec![init_value; length];
                MathSeqS { data: v }
            }

            fn set(&mut self, index: N, value: T) -> (success: bool)
            {
                if index < self.data.len() {
                    self.data.set(index, value);
                    true
                } else {
                    false
                }
            }

            fn length(&self) -> (len: N)
            {
                self.data.len()
            }

            fn nth(&self, index: N) -> (elem: &T)
            {
                &self.data[index]
            }

            fn empty() -> (empty_seq: Self)
            {
                MathSeqS { data: Vec::new() }
            }

            fn singleton(item: T) -> (singleton: Self)
            {
                MathSeqS { data: vec![item] }
            }

            fn add_last(&mut self, value: T)
            {
                self.data.push(value);
            }

            fn delete_last(&mut self) -> (shortened: Option<T>)
            {
                self.data.pop()
            }

            fn is_empty(&self) -> (emptiness: bool)
            {
                self.data.len() == 0
            }

            fn is_singleton(&self) -> (singularity: bool)
            {
                self.data.len() == 1
            }

            fn from_vec(data: Vec<T>) -> (seq: Self)
            {
                MathSeqS { data }
            }

            fn with_len(length: N, init_value: T) -> (seq_of_len_value: Self)
            {
                Self::new(length, init_value)
            }

            fn subseq(&self, start: N, length: N) -> (subseq: &[T])
            {
                let n = self.data.len();
                let s = start.min(n);
                let e = start.saturating_add(length).min(n);
                let slice: &[T] = self.data.as_slice();
                slice_subrange(slice, s, e)
            }

            fn subseq_copy(&self, start: N, length: N) -> (subseq: Self) where T: Copy
            {
                let _n = self.data.len();
                let end = start + length;
                let slice = vstd::slice::slice_subrange(self.data.as_slice(), start, end);
                let vec = vstd::slice::slice_to_vec(slice);
                MathSeqS { data: vec }
            }

            fn domain(&self) -> (domain: Vec<N>)
            {
                let mut v = Vec::new();
                let len = self.data.len();
                let mut i: usize = 0;
                while i < len
                    invariant
                    i <= len,
                    v@.len() == i as int,
                    decreases len - i,
                {
                    v.push(i);
                    i = i + 1;
                }
                v
            }

            fn range(&self) -> (range: Vec<T>)
            {
                let mut seen: HashSetWithViewPlus<T> = HashSetWithViewPlus::new();
                let mut out: Vec<T> = Vec::new();
                let mut i: usize = 0;
                while i < self.data.len()
                    invariant
                    i <= self.data@.len(),
                    out@.len() <= i,
                    out@.no_duplicates(),
                    valid_key_type::<T>(),
                    seen@.finite(),
                    forall|v: T::V| seen@.contains(v) <==> out@.map(|_j: int, t: T| t@).contains(v),
                    decreases self.data.len() - i,
                {
                    let x = self.data[i].clone();
                    let not_seen = !seen.contains(&x);
                    if not_seen {
                        proof {
                            assert(!seen@.contains(x@));
                            assert(!out@.map(|_j: int, t: T| t@).contains(x@));

                            lemma_map_not_contains_implies_all_ne(out@, x@);

                            assert forall|j: int| 0 <= j < out@.len() implies out@[j] != x by {
                                assert(out@[j]@ != x@);
                            }

                            assert(seq![x].no_duplicates());
                            assert(out@.disjoint(seq![x]));
                            vstd::seq_lib::lemma_no_dup_in_concat(out@, seq![x]);
                        }
                        let ghost old_seen = seen@;
                        let ghost old_out = out@;
                        let ghost old_out_mapped = old_out.map(|_j: int, t: T| t@);
                        let x_clone = x.clone();
                        proof {
                            lemma_cloned_view_eq(x, x_clone);
                            assert(x_clone@ == x@);
                        }
                        seen.insert(x_clone);
                        out.push(x);
                        proof {
                            assert(seen@ =~= old_seen.insert(x@));
                            assert(out@ =~= old_out.push(x));

                            let f = |t: T| t@;
                            old_out.lemma_push_map_commute(f, x);
                            let new_mapped = out@.map_values(f);
                            assert(new_mapped =~= old_out_mapped.push(x@));
                            assert(out@.map(|_j: int, t: T| t@) =~= new_mapped);

                            assert forall|v: T::V| seen@.contains(v) <==> out@.map(|_j: int, t: T| t@).contains(v) by {
                                if v == x@ {
                                    assert(new_mapped.last() == x@);
                                    assert(new_mapped.contains(x@));
                                    assert(seen@.contains(x@));
                                } else {
                                    assert(seen@.contains(v) <==> old_seen.contains(v));
                                    if old_out_mapped.contains(v) {
                                        let wit = choose|i: int| 0 <= i < old_out_mapped.len() && old_out_mapped[i] == v;
                                        assert(new_mapped[wit] == v);
                                        assert(new_mapped.contains(v));
                                    }
                                    if new_mapped.contains(v) {
                                        let wit = choose|i: int| 0 <= i < new_mapped.len() && new_mapped[i] == v;
                                        if wit < old_out_mapped.len() {
                                            assert(old_out_mapped[wit] == v);
                                            assert(old_out_mapped.contains(v));
                                        } else {
                                            assert(new_mapped[wit] == x@);
                                            assert(false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    i = i + 1;
                }
                out
            }

            fn multiset_range(&self) -> (range: Vec<(N, T)>)
            {
                let mut counts: HashMapWithView<T, N> = HashMapWithView::with_capacity(self.data.len());
                let mut order: Vec<T> = Vec::new();
                let mut i: usize = 0;
                let len = self.data.len();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < len
                    invariant
                        valid_key_type::<T>(),
                        forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2,
                        i <= len,
                        order@.len() <= i,
                        forall|idx: int| #![trigger order@[idx]@] 0 <= idx < order@.len() ==> counts@.contains_key(order@[idx]@),
                    decreases len - i,
                {
                    let x = self.data[i].clone();
                    let ghost old_counts = counts@;
                    let ghost old_order = order@;

                    if counts.contains_key(&x) {
                        let old_count = *counts.get(&x).unwrap();
                        if old_count < usize::MAX {
                            counts.insert(x, old_count + 1);
                            proof {
                                assert forall|idx: int| #![trigger order@[idx]@] 0 <= idx < order@.len()
                                    implies counts@.contains_key(order@[idx]@) by {
                                    assert(old_counts.contains_key(order@[idx]@));
                                }
                            }
                        }
                    } else {
                        let x2 = x.clone();
                        proof {
                            assert(cloned(x, x2));
                        }
                        counts.insert(x2, 1);
                        order.push(x);
                        proof {
                            assert(counts@.contains_key(x@));
                            assert(order@ =~= old_order.push(x));
                            assert forall|idx: int| #![trigger order@[idx]@] 0 <= idx < order@.len()
                                implies counts@.contains_key(order@[idx]@) by {
                                if idx < old_order.len() {
                                    assert(order@[idx] == old_order[idx]);
                                    assert(old_counts.contains_key(old_order[idx]@));
                                }
                            }
                        }
                    }
                    i = i + 1;
                }

                let ghost final_counts = counts@;

                let mut range: Vec<(N, T)> = Vec::new();
                let mut j: usize = 0;
                let order_len = order.len();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < order_len
                    invariant
                        valid_key_type::<T>(),
                        forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2,
                        j <= order_len,
                        range@.len() == j,
                        order_len <= len,
                        counts@ == final_counts,
                        forall|idx: int| #![trigger order@[idx]@] 0 <= idx < order@.len() ==> final_counts.contains_key(order@[idx]@),
                    decreases order_len - j,
                {
                    let x = order[j].clone();
                    proof {
                        assert(final_counts.contains_key(order@[j as int]@));
                        assert(cloned(order@[j as int], x));
                        assert(counts@.contains_key(x@));
                    }
                    let opt_count = counts.get(&x);
                    let count = *opt_count.unwrap();
                    range.push((count, x));
                    j = j + 1;
                }

                range
            }

            fn iter(&self) -> (it: MathSeqIter<'_, T>)
            {
                MathSeqIter { inner: self.data.iter() }
            }
        }

    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for MathSeqS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


        //		10. iterators

    // 10. iterators

    /// Borrow iterator wrapper with closed spec view.
    #[verifier::reject_recursive_types(T)]
    pub struct MathSeqIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for MathSeqIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct MathSeqGhostIter<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for MathSeqGhostIter<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    pub open spec fn iter_invariant<'a, T>(it: &MathSeqIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for MathSeqIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
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

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for MathSeqIter<'a, T> {
        type GhostIter = MathSeqGhostIter<'a, T>;
        open spec fn ghost_iter(&self) -> MathSeqGhostIter<'a, T> {
            MathSeqGhostIter { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for MathSeqGhostIter<'a, T> {
        type ExecIter = MathSeqIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &MathSeqIter<'a, T>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &MathSeqIter<'a, T>) -> MathSeqGhostIter<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT> std::iter::IntoIterator for &'a MathSeqS<T> {
        type Item = &'a T;
        type IntoIter = MathSeqIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.data@,
                iter_invariant(&it),
        {
            MathSeqIter { inner: self.data.iter() }
        }
    }

    impl<T: StT> std::iter::IntoIterator for MathSeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.data@,
        {
            self.data.into_iter()
        }
    }


        //		11. derive impls in verus!

    // 11. derive impls in verus!

    impl<T: StT> Clone for MathSeqS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = MathSeqS { data: self.data.clone() };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    impl<T: StT> Eq for MathSeqS<T> {}

    impl<T: StT> PartialEq for MathSeqS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.data == other.data;
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT + Hash> MathSeqS<T> {
        /// Mutable borrow iterator. Must stay outside verus! (returns &mut).
        /// - APAS: no cost spec.
        /// - Claude-Opus-4.6: O(1) — returns iterator wrapper.
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            self.data.iter_mut()
        }
    }

    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter_mut()
        }
    }
    

    //		13. derive impls outside verus!

    impl<T: StT> Debug for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }
    
    impl<T: StT> Display for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for x in &self.data {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "]")
        }
    }
    
    // 12. macros


    //		12. macros

    #[macro_export]
    macro_rules! MathSeqSLit {
        () => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::empty()
        };
        ($x:expr; $n:expr) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::with_len($n, $x)
        };
        ($($x:expr),* $(,)?) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::from_vec(vec![$($x),*])
        };
    }
}
