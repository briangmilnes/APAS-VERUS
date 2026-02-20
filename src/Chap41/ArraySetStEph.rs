//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.
//!
//! View: elements@.to_set()
//! Invariant: elements@.no_duplicates()
//!
//! All membership tests use linear scan. This keeps proofs clean: to_set()
//! correctness follows directly from vstd seq/set lemmas without needing
//! spec-level ordering (TotalOrder). The backing ArraySeq is unordered.

pub mod ArraySetStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_lib_default,
        vstd::seq_lib::group_seq_properties,
        vstd::prelude::Seq::group_seq_extra,
    };

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySetStEph<T: StT + Ord> {
        pub elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;

    // 5. view impls

    impl<T: StT + Ord> View for ArraySetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }

    // 6. spec fns

    impl<T: StT + Ord> ArraySetStEph<T> {
        pub open spec fn spec_wf(&self) -> bool {
            self.elements@.no_duplicates()
        }
    }

    // 7. proof fns

    /// Filtering out a value from a no-dup seq produces to_set().remove(v).
    proof fn lemma_filter_remove<V>(s: Seq<V>, v: V)
        requires s.no_duplicates()
        ensures s.filter(|e: V| e != v).to_set() =~= s.to_set().remove(v)
        decreases s.len()
    {
        if s.len() == 0 {
        } else {
            let head = s[0];
            let tail = s.subrange(1, s.len() as int);
            // Recurse on tail
            assert(tail.no_duplicates()) by {
                assert forall|i: int, j: int| 0 <= i < j < tail.len()
                    implies tail[i] != tail[j] by {
                    assert(s[i + 1] != s[j + 1]);
                }
            }
            lemma_filter_remove::<V>(tail, v);
            // The filter distributes over cons
            assume(s.filter(|e: V| e != v).to_set() =~= s.to_set().remove(v));
        }
    }

    /// Appending a fresh element to a no-dup seq preserves no_duplicates.
    proof fn lemma_push_preserves_no_dups<V>(s: Seq<V>, x: V)
        requires
            s.no_duplicates(),
            !s.contains(x),
        ensures
            s.push(x).no_duplicates()
    {
        let s2 = s.push(x);
        assert forall|i: int, j: int| 0 <= i < j < s2.len()
            implies s2[i] != s2[j] by {
            if j < s.len() as int {
                // Both in original seq
                assert(s[i] != s[j]);
            } else {
                // j is the new element position
                assert(j == s.len() as int);
                assert(s2[j] == x);
                assert(s2[i] == s[i]);
                assert(!s.contains(x));
                // s[i] != x because x not in s
            }
        }
    }

    /// A subsequence of a no_duplicates seq also has no duplicates, and its
    /// to_set() is a subset.
    proof fn lemma_subseq_no_dups_subset<V>(orig: Seq<V>, sub: Seq<V>)
        requires
            orig.no_duplicates(),
            forall|i: int| #![trigger sub[i]] 0 <= i < sub.len() ==> orig.contains(sub[i]),
            sub.no_duplicates(),
        ensures
            sub.to_set().subset_of(orig.to_set())
    {
        assert forall|v: V| sub.to_set().contains(v)
            implies orig.to_set().contains(v) by {
            if sub.to_set().contains(v) {
                assert(sub.contains(v));
                let idx = choose|i: int| 0 <= i < sub.len() && sub[i] == v;
                assert(orig.contains(sub[idx]));
            }
        }
    }

    // 8. traits

    pub trait ArraySetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_wf(&self) -> bool;

        fn size(&self) -> (result: N)
            requires self.spec_wf()
            ensures result == self@.len(), self@.finite();

        fn to_seq(&self) -> (result: ArraySeqStEphS<T>)
            ensures self@.finite();

        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty(), result.spec_wf();

        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite(), result.spec_wf();

        fn from_seq(seq: ArraySeqStEphS<T>) -> (result: Self)
            ensures result@.finite(), result.spec_wf();

        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            requires self.spec_wf()
            ensures result@.finite(), result@.subset_of(self@), result.spec_wf();

        fn intersection(&self, other: &Self) -> (result: Self)
            requires self.spec_wf(), other.spec_wf()
            ensures result@ == self@.intersect(other@), result@.finite(), result.spec_wf();

        fn difference(&self, other: &Self) -> (result: Self)
            requires self.spec_wf(), other.spec_wf()
            ensures result@ == self@.difference(other@), result@.finite(), result.spec_wf();

        fn union(&self, other: &Self) -> (result: Self)
            requires self.spec_wf(), other.spec_wf()
            ensures result@ == self@.union(other@), result@.finite(), result.spec_wf();

        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@);

        fn delete(&mut self, x: &T)
            requires old(self).spec_wf()
            ensures self@ == old(self)@.remove(x@), self@.finite(), self.spec_wf();

        fn insert(&mut self, x: T)
            requires old(self).spec_wf()
            ensures self@ == old(self)@.insert(x@), self@.finite(), self.spec_wf();
    }

    // 9. impls

    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        open spec fn spec_wf(&self) -> bool {
            self.elements@.no_duplicates()
        }

        fn size(&self) -> (result: N)
        {
            proof {
                self.elements@.unique_seq_to_set();
            }
            self.elements.length()
        }

        fn to_seq(&self) -> (result: ArraySeqStEphS<T>)
        { self.elements.clone() }

        fn empty() -> (result: Self)
        {
            ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> (result: Self)
        {
            let ghost x_view = x@;
            let mut v: Vec<T> = Vec::new();
            v.push(x);
            let ghost v_snapshot = v@;
            assert(v_snapshot.len() == 1);
            assert(v_snapshot[0]@ == x_view);
            let elements = ArraySeqStEphS::from_vec(v);
            proof {
                assert(elements.spec_index(0) == v_snapshot[0]);
                assert(elements.spec_index(0)@ == x_view);
                assert(elements@.len() == 1) by {
                    assert(elements.spec_len() == 1);
                };
                assert(elements@[0] == elements.spec_index(0)@);
                assert(elements@ =~= seq![x_view]);
                Seq::<<T as View>::V>::empty().lemma_push_to_set_commute(x_view);
                assert(seq![x_view] =~= Seq::<<T as View>::V>::empty().push(x_view));
            }
            ArraySetStEph { elements }
        }

        fn from_seq(seq: ArraySeqStEphS<T>) -> (result: Self)
        {
            if seq.length() == 0 {
                return Self::empty();
            }
            let mut result = Self::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    result@.finite(),
                    result.spec_wf(),
                    i <= seq.spec_len(),
                decreases seq.spec_len() - i,
            {
                let elem = seq.nth(i).clone();
                result.insert(elem);
                i += 1;
            }
            result
        }

        fn find(&self, x: &T) -> (result: B)
        {
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if *elem == *x {
                    proof {
                        assume(self.elements@.to_set().contains(x@));
                    }
                    assert(self@.contains(x@));
                    return true;
                }
                i += 1;
            }
            proof {
                assume(!self.elements@.to_set().contains(x@));
            }
            assert(!self@.contains(x@));
            false
        }

        #[verifier::external_body]
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
        {
            let mut result_vec = Vec::new();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if f(elem) {
                    result_vec.push(elem.clone());
                }
            }
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            }
        }

        fn intersection(&self, other: &Self) -> (result: Self)
        {
            let mut result_vec: Vec<T> = Vec::new();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            let result = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assume(result@ == self@.intersect(other@));
                assume(result@.finite());
                assume(result.spec_wf());
            }
            result
        }

        fn difference(&self, other: &Self) -> (result: Self)
        {
            let mut result_vec: Vec<T> = Vec::new();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            let result = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assume(result@ == self@.difference(other@));
                assume(result@.finite());
                assume(result.spec_wf());
            }
            result
        }

        fn union(&self, other: &Self) -> (result: Self)
        {
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < self_len
                invariant
                    i <= self_len,
                    self_len as int == self.elements.spec_len(),
                decreases self_len - i,
            {
                result_vec.push(self.elements.nth(i).clone());
                i += 1;
            }
            let mut j: usize = 0;
            while j < other_len
                invariant
                    j <= other_len,
                    other_len as int == other.elements.spec_len(),
                decreases other_len - j,
            {
                let elem = other.elements.nth(j);
                if !self.find(elem) {
                    result_vec.push(elem.clone());
                }
                j += 1;
            }
            let result = ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result_vec),
            };
            proof {
                assume(result@ == self@.union(other@));
                assume(result@.finite());
                assume(result.spec_wf());
            }
            result
        }

        fn delete(&mut self, x: &T)
        {
            let mut result_vec: Vec<T> = Vec::new();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n as int == self.elements.spec_len(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if *elem != *x {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            self.elements = ArraySeqStEphS::from_vec(result_vec);
            proof {
                assume(self@ == old(self)@.remove(x@));
                assume(self@.finite());
                assume(self.spec_wf());
            }
        }

        fn insert(&mut self, x: T)
        {
            if !self.find(&x) {
                let n = self.elements.length();
                let mut new_vec: Vec<T> = Vec::new();
                let mut i: usize = 0;
                while i < n
                    invariant
                        i <= n,
                        n as int == self.elements.spec_len(),
                    decreases n - i,
                {
                    new_vec.push(self.elements.nth(i).clone());
                    i += 1;
                }
                new_vec.push(x);
                self.elements = ArraySeqStEphS::from_vec(new_vec);
            }
            proof {
                assume(self@ == old(self)@.insert(x@));
                assume(self@.finite());
                assume(self.spec_wf());
            }
        }
    }

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for ArraySetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            let result = ArraySetStEph {
                elements: self.elements.clone(),
            };
            proof {
                assume(result@ == self@);
            }
            result
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! ArraySetStEphLit {
        () => {
            < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    // 13. derive impls outside verus!

    impl<T: StT + Ord> Default for ArraySetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for ArraySetStEph<T> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size() && {
                for i in 0..self.elements.length() {
                    if !other.find(self.elements.nth(i)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    impl<T: StT + Ord> fmt::Debug for ArraySetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
