//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//!
//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod MathSeq {

    //		Section 2. imports

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
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::vstdplus::seq_set::*;
    use vstd::slice::slice_subrange;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{

    //		Section 3. broadcast use


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
        vstd::seq_lib::group_to_multiset_ensures,
        };

    //		Section 4. type definitions


        #[verifier::reject_recursive_types(T)]
        pub struct MathSeqS<T: StT> {
            pub data: Vec<T>,
        }

    //		Section 5. view impls


        impl<T: StT> View for MathSeqS<T> {
            type V = Seq<T::V>;

            open spec fn view(&self) -> Seq<T::V> {
                self.data@.map_values(|t: T| t@)
            }
        }

    //		Section 6. spec fns


        pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
            &&& obeys_key_model::<T>()
                &&& obeys_feq_full::<T>()
        }


        pub open spec fn spec_clamp(val: int, max: int) -> int {
            if val < 0 { 0 } else if val > max { max } else { val }
        }

    //		Section 8. traits


        pub trait MathSeqSTrait<T: StT>: Sized + View<V = Seq<T::V>> {
            spec fn spec_len(&self) -> nat;
            spec fn spec_nth(&self, i: int) -> T::V
                recommends 0 <= i < self.spec_len();
            spec fn spec_is_empty(&self) -> bool;
            spec fn spec_is_singleton(&self) -> bool;
            spec fn spec_seq(&self) -> Seq<T>;

            /// - Alg Analysis: APAS: no cost spec (definitions chapter).
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) — Vec allocation + clone fill.
            fn new(length: usize, init_value: T) -> (new_seq: Self)
                ensures
                    new_seq.spec_len() == length,
                    forall|i: int| #![trigger new_seq.spec_seq()[i]] 0 <= i < length ==> cloned(init_value, new_seq.spec_seq()[i]);

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — direct index write.
            fn set(&mut self, index: usize, value: T) -> (success: bool)
                ensures
                    success ==> index < old(self).spec_len()
                        && self.spec_len() == old(self).spec_len()
                        && self@[index as int] == value@
                        && forall|i: int| 0 <= i < self.spec_len() && i != index as int ==> self@[i] == old(self)@[i],
                    !success ==> index >= old(self).spec_len() && self@ == old(self)@;

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
            fn length(&self) -> (len: usize)
                ensures len == self.spec_len();

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — direct index read.
            fn nth(&self, index: usize) -> (elem: &T)
                requires index < self.spec_len()
                ensures elem@ == self@[index as int];

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
            fn empty() -> (empty_seq: Self)
                ensures empty_seq.spec_len() == 0;

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
            fn singleton(item: T) -> (singleton: Self)
                ensures
                    singleton.spec_len() == 1,
                    singleton@[0] == item@;

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): amortized O(1) — Vec::push.
            fn add_last(&mut self, value: T)
                ensures
                    self.spec_len() == old(self).spec_len() + 1,
                    self@[self.spec_len() - 1] == value@,
                    forall|i: int| 0 <= i < old(self).spec_len() ==> self@[i] == old(self)@[i];

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — Vec::pop.
            fn delete_last(&mut self) -> (shortened: Option<T>)
                ensures
                    old(self).spec_len() == 0 ==> shortened is None && self@ == old(self)@,
                    old(self).spec_len() > 0  ==> shortened is Some
                        && shortened->Some_0@ == old(self)@[old(self).spec_len() - 1]
                        && self.spec_len() == old(self).spec_len() - 1
                        && forall|i: int| 0 <= i < self.spec_len() ==> self@[i] == old(self)@[i];

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
            fn is_empty(&self) -> (emptiness: bool)
                ensures emptiness == self.spec_is_empty();

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
            fn is_singleton(&self) -> (singularity: bool)
                ensures singularity == self.spec_is_singleton();

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — move, no copy.
            fn from_vec(data: Vec<T>) -> (seq: Self)
                ensures seq.spec_seq() == data@;

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) — delegates to new.
            fn with_len(length: usize, init_value: T) -> (seq_of_len_value: Self)
                ensures
                    seq_of_len_value.spec_len() == length,
                    forall|i: int| #![trigger seq_of_len_value.spec_seq()[i]] 0 <= i < length ==> cloned(init_value, seq_of_len_value.spec_seq()[i]);

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — returns slice reference.
            fn subseq(&self, start: usize, length: usize) -> (subseq: &[T])
                ensures
                    subseq@.len() <= length,
                ({
                    let s = spec_clamp(start as int, self.spec_seq().len() as int);
                    let e = spec_clamp((start + length) as int, self.spec_seq().len() as int);
                    subseq@ == self.spec_seq().subrange(s, e)
                });

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(length) — copies subrange.
            fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) where T: Copy
                requires
                    start as int + length as int <= self.spec_seq().len(),
                ensures
                    subseq.spec_len() == length,
                    subseq.spec_seq() == self.spec_seq().subrange(start as int, (start + length) as int);

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) — builds index vector.
            fn domain(&self) -> (domain: Vec<usize>)
                ensures
                    domain@.len() == self.spec_len(),
                    forall|i: int| 0 <= i < domain@.len() ==> domain@[i] == i as usize;

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) expected — hash set dedup.
            fn range(&self) -> (range: Vec<T>)
                requires valid_key_type::<T>()
                ensures
                    range@.len() <= self.spec_seq().len(),
                    range@.no_duplicates();

            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) expected — hash map counting, two passes.
            fn multiset_range(&self) -> (range: Vec<(usize, T)>)
                requires
                    valid_key_type::<T>(),
                    obeys_feq_view_injective::<T>(),
                ensures
                    range@.len() <= self.spec_seq().len();

            /// Borrow iterator over the sequence elements.
            /// - Alg Analysis: APAS: no cost spec.
            /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — returns iterator wrapper.
            fn iter(&self) -> (it: MathSeqIter<'_, T>)
                ensures
                    it@.0 == 0,
                    it@.1 == self.spec_seq(),
                    iter_invariant(&it);
        }

    //		Section 9. impls


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

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — allocates and fills Vec of length n.
            fn new(length: usize, init_value: T) -> (new_seq: Self)
            {
                let v = vec![init_value; length];
                MathSeqS { data: v }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index write.
            fn set(&mut self, index: usize, value: T) -> (success: bool)
            {
                if index < self.data.len() {
                    self.data.set(index, value);
                    true
                } else {
                    false
                }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec len.
            fn length(&self) -> (len: usize)
            {
                self.data.len()
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index read.
            fn nth(&self, index: usize) -> (elem: &T)
            {
                &self.data[index]
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty Vec allocation.
            fn empty() -> (empty_seq: Self)
            {
                MathSeqS { data: Vec::new() }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single-element Vec.
            fn singleton(item: T) -> (singleton: Self)
            {
                MathSeqS { data: vec![item] }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — Vec push.
            fn add_last(&mut self, value: T)
            {
                self.data.push(value);
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec pop.
            fn delete_last(&mut self) -> (shortened: Option<T>)
            {
                self.data.pop()
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length check.
            fn is_empty(&self) -> (emptiness: bool)
            {
                self.data.len() == 0
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length check.
            fn is_singleton(&self) -> (singularity: bool)
            {
                self.data.len() == 1
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — moves ownership, no copy.
            fn from_vec(data: Vec<T>) -> (seq: Self)
            {
                MathSeqS { data }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — delegates to new().
            fn with_len(length: usize, init_value: T) -> (seq_of_len_value: Self)
            {
                Self::new(length, init_value)
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — slice_subrange returns a view, no copy.
            fn subseq(&self, start: usize, length: usize) -> (subseq: &[T])
            {
                let n = self.data.len();
                let s = start.min(n);
                let e = start.saturating_add(length).min(n);
                let slice: &[T] = self.data.as_slice();
                slice_subrange(slice, s, e)
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(k), Span O(k) — copies k = length elements from slice.
            fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) where T: Copy
            {
                let _n = self.data.len();
                let end = start + length;
                let slice = vstd::slice::slice_subrange(self.data.as_slice(), start, end);
                let vec = vstd::slice::slice_to_vec(slice);
                MathSeqS { data: vec }
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — builds Vec of indices 0..n.
            fn domain(&self) -> (domain: Vec<usize>)
            {
                let mut v = Vec::new();
                let len = self.data.len();
                let mut i: usize = 0;
                while i < len
                    invariant
                    i <= len,
                    v@.len() == i as int,
                    forall|k: int| 0 <= k < v@.len() ==> v@[k] == k as usize,
                    decreases len - i,
                {
                    v.push(i);
                    i = i + 1;
                }
                v
            }

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — single pass deduplicating with HashSet.
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
                        // Veracity: NEEDED proof block
                        proof {

                            lemma_map_not_contains_implies_all_ne(out@, x@);


                            vstd::seq_lib::lemma_no_dup_in_concat(out@, seq![x]);
                        }
                        let ghost old_seen = seen@;
                        let ghost old_out = out@;
                        let ghost old_out_mapped = old_out.map(|_j: int, t: T| t@);
                        let x_clone = x.clone();
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_cloned_view_eq(x, x_clone);
                        }
                        seen.insert(x_clone);
                        out.push(x);
                        // Veracity: NEEDED proof block
                        proof {

                            let f = |t: T| t@;
                            old_out.lemma_push_map_commute(f, x);
                            let new_mapped = out@.map_values(f);
                            // Veracity: NEEDED assert
                            assert(out@.map(|_j: int, t: T| t@) =~= new_mapped);

                            // Veracity: NEEDED assert
                            assert forall|v: T::V| seen@.contains(v) <==> out@.map(|_j: int, t: T| t@).contains(v) by {
                                if v == x@ {
                                } else {
                                    if old_out_mapped.contains(v) {
                                        let wit = choose|i: int| 0 <= i < old_out_mapped.len() && old_out_mapped[i] == v;
                                        // Veracity: NEEDED assert
                                        assert(new_mapped[wit] == v);
                                    }
                                    if new_mapped.contains(v) {
                                        let wit = choose|i: int| 0 <= i < new_mapped.len() && new_mapped[i] == v;
                                        if wit < old_out_mapped.len() {
                                            // Veracity: NEEDED assert
                                            assert(old_out_mapped[wit] == v);
                                        } else {
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

            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — single pass counting with HashMap + second pass emitting pairs.
            fn multiset_range(&self) -> (range: Vec<(usize, T)>)
            {
                // Veracity: NEEDED proof block
                proof { lemma_reveal_view_injective::<T>(); }
                let mut counts: HashMapWithView<T, usize> = HashMapWithView::with_capacity(self.data.len());
                let mut order: Vec<T> = Vec::new();
                let mut i: usize = 0;
                let len = self.data.len();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while i < len
                    invariant
                        valid_key_type::<T>(),
                        obeys_feq_view_injective::<T>(),
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
                            // Veracity: NEEDED proof block
                            proof {
                            }
                        }
                    } else {
                        let x2 = x.clone();
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert(cloned(x, x2));
                        }
                        counts.insert(x2, 1);
                        order.push(x);
                        // Veracity: NEEDED proof block
                        proof {
                        }
                    }
                    i = i + 1;
                }

                let ghost final_counts = counts@;

                let mut range: Vec<(usize, T)> = Vec::new();
                let mut j: usize = 0;
                let order_len = order.len();

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < order_len
                    invariant
                        valid_key_type::<T>(),
                        obeys_feq_view_injective::<T>(),
                        j <= order_len,
                        range@.len() == j,
                        order_len <= len,
                        counts@ == final_counts,
                        forall|idx: int| #![trigger order@[idx]@] 0 <= idx < order@.len() ==> final_counts.contains_key(order@[idx]@),
                    decreases order_len - j,
                {
                    let x = order[j].clone();
                    // Veracity: NEEDED proof block
                    proof {
                        // Veracity: NEEDED assert
                        assert(cloned(order@[j as int], x));
                    }
                    let opt_count = counts.get(&x);
                    let count = *opt_count.unwrap();
                    range.push((count, x));
                    j = j + 1;
                }

                range
            }

            fn iter(&self) -> (it: MathSeqIter<'_, T>)
                ensures
                    it@.0 == 0,
                    it@.1 == self.data@,
                    iter_invariant(&it),
            {
                MathSeqIter { inner: self.data.iter() }
            }
        }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct MathSeqIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for MathSeqIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct MathSeqGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for MathSeqGhostIterator<'a, T> {
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
        type GhostIter = MathSeqGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> MathSeqGhostIterator<'a, T> {
            MathSeqGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for MathSeqGhostIterator<'a, T> {
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

        open spec fn ghost_advance(&self, _exec_iter: &MathSeqIter<'a, T>) -> MathSeqGhostIterator<'a, T> {
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

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<T: StT> PartialEqSpecImpl for MathSeqS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<T: StT> Clone for MathSeqS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = MathSeqS { data: self.data.clone() };
            // Veracity: NEEDED proof block
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    impl<T: StT> Eq for MathSeqS<T> {}

    impl<T: StT> PartialEq for MathSeqS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.data == other.data;
            // Veracity: NEEDED proof block
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

    impl<T: StT + Hash> MathSeqS<T> {
        /// Mutable borrow iterator. Must stay outside verus! (returns &mut).
        /// - Alg Analysis: APAS: no cost spec.
        /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — returns iterator wrapper.
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

    impl<'a, T: Debug> Debug for MathSeqIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MathSeqIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for MathSeqIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MathSeqIter")
        }
    }

    impl<'a, T> Debug for MathSeqGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MathSeqGhostIterator")
        }
    }

    impl<'a, T> Display for MathSeqGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MathSeqGhostIterator")
        }
    }
}
