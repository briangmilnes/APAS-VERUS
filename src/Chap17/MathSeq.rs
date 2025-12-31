//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

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
    use crate::vstdplus::feq::feq::obeys_feq_full;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;
    #[cfg(verus_keep_ghost)]
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::seq_set::lemma_map_not_contains_implies_all_ne;
    use vstd::slice::slice_subrange;

    verus! {

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
            // Clone equality
            crate::vstdplus::feq::feq::axiom_cloned_implies_eq,
        };
        
        pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
            &&& obeys_key_model::<T>()
                &&& obeys_feq_full::<T>()
        }
        
        #[verifier::reject_recursive_types(T)]
        pub struct MathSeqS<T: StT> {
            pub data: Vec<T>,
        }
        
        impl<T: StT> View for MathSeqS<T> {
            type V = Seq<T::V>;
            
            open spec fn view(&self) -> Seq<T::V> {
                self.data@.map_values(|t: T| t@)
            }
        }
        
        impl<T: StT + Hash> MathSeqS<T> {
            
            pub open spec fn spec_len(&self) -> nat {
                self@.len()
            }
            
            pub open spec fn spec_nth(&self, i: int) -> T::V
                recommends 0 <= i < self.spec_len(),
            {
                self@[i]
            }
            
            pub open spec fn spec_is_empty(&self) -> bool {
                self.spec_len() == 0
            }
            
            pub open spec fn spec_is_singleton(&self) -> bool {
                self.spec_len() == 1
            }
            
            pub fn new(length: N, init_value: T) -> (result: Self)
                ensures
                result.spec_len() == length,
            forall|i: int| #![auto] 0 <= i < length ==> cloned(init_value, result.data@[i]),
            {
                let v = vec![init_value; length];
                MathSeqS { data: v }
            }
            
            pub fn set(&mut self, index: N, value: T) -> (success: bool)
                ensures
                success ==> index < old(self).spec_len()
                && self.spec_len() == old(self).spec_len()
                && self@[index as int] == value@
                && forall|i: int| 0 <= i < self.spec_len() && i != index as int ==> self@[i] == old(self)@[i],
            !success ==> index >= old(self).spec_len() && self@ == old(self)@,
            {
                if index < self.data.len() {
                    self.data.set(index, value);
                    true
                } else {
                    false
                }
            }
            
            pub fn length(&self) -> (len: N)
                ensures len == self.spec_len(),
            {
                self.data.len()
            }
            
            pub fn nth(&self, index: N) -> (elem: &T)
                requires index < self.spec_len(),
            ensures elem@ == self@[index as int],
            {
                &self.data[index]
            }
            
            pub fn empty() -> (result: Self)
                ensures result.spec_len() == 0,
            {
                MathSeqS { data: Vec::new() }
            }
            
            pub fn singleton(item: T) -> (result: Self)
                ensures
                result.spec_len() == 1,
            result@[0] == item@,
            {
                MathSeqS { data: vec![item] }
            }
            
            pub fn add_last(&mut self, value: T)
                ensures
                self.spec_len() == old(self).spec_len() + 1,
            self@[self.spec_len() - 1] == value@,
            forall|i: int| 0 <= i < old(self).spec_len() ==> self@[i] == old(self)@[i],
            {
                self.data.push(value);
            }
            
            pub fn delete_last(&mut self) -> (result: Option<T>)
                ensures
                old(self).spec_len() == 0 ==> result is None && self@ == old(self)@,
            old(self).spec_len() > 0 ==>
                result is Some
                && result->Some_0@ == old(self)@[old(self).spec_len() - 1]
                && self.spec_len() == old(self).spec_len() - 1
                && forall|i: int| 0 <= i < self.spec_len() ==> self@[i] == old(self)@[i],
            {
                self.data.pop()
            }
            
            pub fn is_empty(&self) -> (result: bool)
                ensures result == self.spec_is_empty(),
            {
                self.data.len() == 0
            }
            
            pub fn is_singleton(&self) -> (result: bool)
                ensures result == self.spec_is_singleton(),
            {
                self.data.len() == 1
            }
            
            pub fn from_vec(data: Vec<T>) -> (result: Self)
                ensures result.data@ == data@,
            {
                MathSeqS { data }
            }
            
            pub fn with_len(length: N, init_value: T) -> (result: Self)
                ensures
                result.spec_len() == length,
            forall|i: int| #![auto] 0 <= i < length ==> cloned(init_value, result.data@[i]),
            {
                Self::new(length, init_value)
            }
            
            pub open spec fn spec_clamp(val: int, max: int) -> int {
                if val < 0 { 0 } else if val > max { max } else { val }
            }
            
            pub fn subseq(&self, start: N, length: N) -> (result: &[T])
                ensures
                result@.len() <= length,
            ({
                let s = Self::spec_clamp(start as int, self.data@.len() as int);
                let e = Self::spec_clamp((start + length) as int, self.data@.len() as int);
                result@ == self.data@.subrange(s, e)
            }),
            {
                let n = self.data.len();
                let s = start.min(n);
                let e = start.saturating_add(length).min(n);
                let slice: &[T] = self.data.as_slice();
                slice_subrange(slice, s, e)
            }
            
            pub fn subseq_copy(&self, start: N, length: N) -> (result: Self) where T: Copy
                requires
                start as int + length as int <= self.data@.len(),
            ensures
                result.spec_len() == length,
            result.data@ == self.data@.subrange(start as int, (start + length) as int),
            {
                let _n = self.data.len(); // exec call bounds start + length <= usize::MAX
                let end = start + length;
                let slice = vstd::slice::slice_subrange(self.data.as_slice(), start, end);
                let vec = vstd::slice::slice_to_vec(slice);
                MathSeqS { data: vec }
            }
            
            pub fn domain(&self) -> (result: Vec<N>)
                ensures result@.len() == self.spec_len(),
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
            
            pub fn range(&self) -> (result: Vec<T>)
                requires valid_key_type::<T>(),
            ensures
                result@.len() <= self.data@.len(),
            result@.no_duplicates(),
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
                            // x@ not in seen@, so not in out@.map(...)
                            assert(!seen@.contains(x@));
                            assert(!out@.map(|_j: int, t: T| t@).contains(x@));
                            
                            // Use lemma to get forall|j| out@[j]@ != x@
                            lemma_map_not_contains_implies_all_ne(out@, x@);
                            
                            // By view injectivity, out@[j] != x
                            assert forall|j: int| 0 <= j < out@.len() implies out@[j] != x by {
                                assert(out@[j]@ != x@);
                            }
                            
                            // seq![x].no_duplicates() trivially (length 1)
                            assert(seq![x].no_duplicates());
                            
                            // out and seq![x] are disjoint
                            assert(out@.disjoint(seq![x]));
                            
                            // lemma gives (out@ + seq![x]).no_duplicates()
                            vstd::seq_lib::lemma_no_dup_in_concat(out@, seq![x]);
                        }
                        let ghost old_seen = seen@;
                        let ghost old_out = out@;
                        let ghost old_out_mapped = old_out.map(|_j: int, t: T| t@);
                        let x_clone = x.clone();
                        proof {
                            // Clone preserves view under obeys_feq_full
                            crate::vstdplus::feq::feq::lemma_cloned_view_eq(x, x_clone);
                            assert(x_clone@ == x@);
                        }
                        seen.insert(x_clone);
                        out.push(x);
                        proof {
                            // After insert: seen@ == old_seen.insert(x@)
                            assert(seen@ =~= old_seen.insert(x@));
                            
                            // After push: out@ == old_out.push(x)
                            assert(out@ =~= old_out.push(x));
                            
                            // Map commutes with push
                            let f = |t: T| t@;
                            old_out.lemma_push_map_commute(f, x);
                            let new_mapped = out@.map_values(f);
                            assert(new_mapped =~= old_out_mapped.push(x@));
                            
                            // map and map_values are the same for our function
                            assert(out@.map(|_j: int, t: T| t@) =~= new_mapped);
                            
                            // Now prove the invariant
                            assert forall|v: T::V| seen@.contains(v) <==> out@.map(|_j: int, t: T| t@).contains(v) by {
                                // new_mapped == old_out_mapped.push(x@)
                                // contains(v) on a pushed seq: exists in old OR equals pushed element
                                if v == x@ {
                                    // x@ is at the end of new_mapped
                                    assert(new_mapped.last() == x@);
                                    assert(new_mapped.contains(x@));
                                    assert(seen@.contains(x@));
                                } else {
                                    // v != x@, so seen@.contains(v) <==> old_seen.contains(v)
                                    assert(seen@.contains(v) <==> old_seen.contains(v));
                                    
                                    // new_mapped == old_out_mapped.push(x@)
                                    // For v != x@: new_mapped.contains(v) <==> old_out_mapped.contains(v)
                                    if old_out_mapped.contains(v) {
                                        // Get witness: some i with old_out_mapped[i] == v
                                        let wit = choose|i: int| 0 <= i < old_out_mapped.len() && old_out_mapped[i] == v;
                                        // new_mapped[wit] == old_out_mapped[wit] == v (since wit < old len)
                                        assert(new_mapped[wit] == v);
                                        assert(new_mapped.contains(v));
                                    }
                                    if new_mapped.contains(v) {
                                        // Get witness: some i with new_mapped[i] == v
                                        let wit = choose|i: int| 0 <= i < new_mapped.len() && new_mapped[i] == v;
                                        // If wit < old_out_mapped.len(), old has it
                                        // If wit == old_out_mapped.len(), new_mapped[wit] == x@ != v
                                        if wit < old_out_mapped.len() {
                                            assert(old_out_mapped[wit] == v);
                                            assert(old_out_mapped.contains(v));
                                        } else {
                                            // wit == old_out_mapped.len(), so new_mapped[wit] == x@
                                            assert(new_mapped[wit] == x@);
                                            assert(false); // contradiction: x@ == v but v != x@
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
            
            pub fn multiset_range(&self) -> (result: Vec<(N, T)>)
                requires
                    valid_key_type::<T>(),
                    forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2,
                ensures
                    result@.len() <= self.data@.len(),
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
                        forall|idx: int| #![auto] 0 <= idx < order@.len() ==> counts@.contains_key(order@[idx]@),
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
                                // insert preserves existing keys, order unchanged
                                assert forall|idx: int| #![auto] 0 <= idx < order@.len()
                                    implies counts@.contains_key(order@[idx]@) by {
                                    assert(old_counts.contains_key(order@[idx]@));
                                }
                            }
                        }
                    } else {
                        let x2 = x.clone();
                        proof {
                            // cloned(x, x2), so x == x2, so x@ == x2@
                            assert(cloned(x, x2));
                        }
                        counts.insert(x2, 1);
                        order.push(x);
                        proof {
                            // counts@ == old_counts.insert(x2@, 1) == old_counts.insert(x@, 1)
                            assert(counts@.contains_key(x@));
                            assert(order@ =~= old_order.push(x));
                            assert forall|idx: int| #![auto] 0 <= idx < order@.len()
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
                
                let mut result: Vec<(N, T)> = Vec::new();
                let mut j: usize = 0;
                let order_len = order.len();
                
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while j < order_len
                    invariant
                        valid_key_type::<T>(),
                        forall|k1: T, k2: T| k1@ == k2@ ==> k1 == k2,
                        j <= order_len,
                        result@.len() == j,
                        order_len <= len,
                        counts@ == final_counts,
                        forall|idx: int| #![auto] 0 <= idx < order@.len() ==> final_counts.contains_key(order@[idx]@),
                    decreases order_len - j,
                {
                    let x = order[j].clone();
                    proof {
                        // We have: final_counts.contains_key(order[j]@)
                        assert(final_counts.contains_key(order@[j as int]@));
                        // Clone gives cloned(order[j], x), by axiom: order[j] == x
                        assert(cloned(order@[j as int], x));
                        // Therefore x@ == order[j]@ and counts@.contains_key(x@)
                        assert(counts@.contains_key(x@));
                    }
                    // HashMapWithView::get: is_some <==> contains_key(k@)
                    let opt_count = counts.get(&x);
                    let count = *opt_count.unwrap();
                    result.push((count, x));
                    j = j + 1;
                }
                
                result
            }
        }

    impl<'a, T: StT> std::iter::IntoIterator for &'a MathSeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<T: StT> std::iter::IntoIterator for MathSeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }
        
    } // verus!
    
    // Clone implementation outside verus! block
    impl<T: StT> Clone for MathSeqS<T> {
        fn clone(&self) -> Self {
            MathSeqS { data: self.data.clone() }
        }
    }
    
    // Iterator methods outside verus! block
    impl<T: StT + Hash> MathSeqS<T> {
        pub fn iter(&self) -> Iter<'_, T> {
            self.data.iter()
        }
        
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            self.data.iter_mut()
        }
    }

    // &mut IntoIterator must stay outside verus! (Verus doesn't support &mut types)
    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter_mut()
        }
    }
    
    impl<T: StT> PartialEq for MathSeqS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }
    
    impl<T: StT> Eq for MathSeqS<T> {}
    
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
