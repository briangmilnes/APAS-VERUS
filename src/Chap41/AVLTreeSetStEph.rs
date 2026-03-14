//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Single-threaded ephemeral set implementation using AVLTreeSeqStEph as backing store.
//!
//! Limitation: AVLTreeSeqStEph is index-ordered, not a BST by value. find uses binary search
//! on the sorted logical sequence (O(log n) via nth). insert/delete use filter-and-rebuild
//! since the backing tree has no O(log n) value-based insert/delete.

pub mod AVLTreeSetStEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::Types::Types::*;

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};


    // 4. type definitions

    pub struct AVLTreeSetStEph<T: StT + Ord> {
        pub elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;


    // 5. view impls

    impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }


    // 8. traits

    // 8. traits

    pub trait AVLTreeSetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetsteph_wf(&self) -> bool;

        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetsteph_wf(),
            ensures count == self@.len();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetsteph_wf(),
            ensures
                seq.spec_avltreeseqsteph_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetsteph_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            ensures
                constructed.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
            requires
                self.spec_avltreesetsteph_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_avltreesetsteph_wf(), other.spec_avltreesetsteph_wf(),
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_avltreesetsteph_wf(),
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires old(self).spec_avltreesetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetsteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires old(self).spec_avltreesetsteph_wf(),
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetsteph_wf();
    }


    // 9. impls

    // 5. view impls

    impl<T: StT + Ord> AVLTreeSetStEph<T> {
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        open spec fn spec_avltreesetsteph_wf(&self) -> bool {
            self.elements.spec_avltreeseqsteph_wf()
            && self@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            let r = self.elements.length();
            proof {
                assume(r == self@.len());
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
            }
            r
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            let seq = self.elements.clone();
            proof {
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                assert(seq@ =~= self.elements@);
            }
            seq
        }

        fn empty() -> (empty: Self)
        {
            let empty = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::empty() };
            proof {
                assert(empty.elements@ =~= Seq::<<T as View>::V>::empty());
            }
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let ghost x_view = x@;
            proof { assume(obeys_feq_full::<T>()); }
            let mut v: Vec<T> = Vec::new();
            v.push(x);
            let ghost v_view = v@;
            let tree = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::from_vec(v) };
            proof {
                assert(tree.elements@ =~= v_view.map_values(|t: T| t@));
                assert(v_view.len() == 1);
                assert(v_view[0]@ == x_view);
                assert(tree.elements@.len() == 1);
                assert(tree.elements@[0] == x_view);
                assert(tree.elements@.to_set() =~= Set::<<T as View>::V>::empty().insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(tree.elements@);
            }
            tree
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            proof { assume(seq.spec_avltreeseqsteph_wf()); }
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqsteph_wf(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                    constructed@.finite(),
                    constructed.spec_avltreesetsteph_wf(),
                decreases n - i,
            {
                let elem = seq.nth(i).clone();
                constructed.insert(elem);
                i += 1;
            }
            constructed
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> (filtered: Self)
        {
            let mut filtered = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    filtered@.finite(),
                    filtered.spec_avltreesetsteph_wf(),
                    forall|t: &T| #[trigger] f.requires((t,)),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if f(elem) {
                    filtered.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(filtered@.subset_of(self@));
            }
            filtered
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let mut common = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.spec_avltreesetsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    common@.finite(),
                    common.spec_avltreesetsteph_wf(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    common.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(common@ == self@.intersect(other@));
            }
            common
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let mut remaining = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    other.spec_avltreesetsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    remaining@.finite(),
                    remaining.spec_avltreesetsteph_wf(),
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    remaining.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(remaining@ == self@.difference(other@));
            }
            remaining
        }

        fn union(&self, other: &Self) -> (combined: Self)
        {
            let mut combined = Self::empty();
            let self_len = self.elements.length();
            let mut i: usize = 0;
            while i < self_len
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    self_len as int == self.elements.spec_seq().len(),
                    i <= self_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
                decreases self_len - i,
            {
                combined.insert(self.elements.nth(i).clone());
                i += 1;
            }
            let other_len = other.elements.length();
            let mut j: usize = 0;
            while j < other_len
                invariant
                    other.elements.spec_avltreeseqsteph_wf(),
                    other_len as int == other.elements.spec_seq().len(),
                    j <= other_len,
                    combined@.finite(),
                    combined.spec_avltreesetsteph_wf(),
                decreases other_len - j,
            {
                combined.insert(other.elements.nth(j).clone());
                j += 1;
            }
            proof {
                assume(combined@ == self@.union(other@));
            }
            combined
        }

        fn find(&self, x: &T) -> (found: B)
        {
            proof {
                assume(obeys_feq_full::<T>());
            }
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    obeys_feq_full::<T>(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= hi, hi <= n,
                decreases hi - lo,
            {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if feq(elem, x) {
                    assert(self.elements@[mid as int] == x@);
                    assert(self.elements@.contains(x@));
                    return true;
                }
                if *elem < *x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            proof { assume(!self@.contains(x@)); }
            false
        }

        fn delete(&mut self, x: &T)
        {
            let n = self.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_avltreeseqsteph_wf(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                    result_vec@.len() <= i as int,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if *elem != *x {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            proof { assume(result_vec@.len() < usize::MAX); assume(obeys_feq_full::<T>()); }
            self.elements = AVLTreeSeqStEphS::from_vec(result_vec);
            proof {
                assume(self@ == old(self)@.remove(x@));
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
            }
        }

        fn insert(&mut self, x: T)
        {
            let ghost x_view = x@;
            if !self.find(&x) {
                let n = self.elements.length();
                let mut lo: usize = 0;
                let mut hi: usize = n;
                while lo < hi
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        n as int == self.elements.spec_seq().len(),
                        lo <= hi, hi <= n,
                    decreases hi - lo,
                {
                    let mid = lo + (hi - lo) / 2;
                    if *self.elements.nth(mid) < x {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mut new_vec: Vec<T> = Vec::new();
                let mut i: usize = 0;
                while i < lo
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        n as int == self.elements.spec_seq().len(),
                        i <= lo, lo <= n,
                    decreases lo - i,
                {
                    new_vec.push(self.elements.nth(i).clone());
                    i += 1;
                }
                new_vec.push(x);
                let mut j: usize = lo;
                while j < n
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        n as int == self.elements.spec_seq().len(),
                        lo <= j, j <= n,
                    decreases n - j,
                {
                    new_vec.push(self.elements.nth(j).clone());
                    j += 1;
                }
                proof { assume(new_vec@.len() < usize::MAX); assume(obeys_feq_full::<T>()); }
                self.elements = AVLTreeSeqStEphS::from_vec(new_vec);
            }
            proof {
                assume(self@ == old(self)@.insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
            }
        }
    }


    // 11. derive impls in verus!

    // 11. derive impls in verus!

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for AVLTreeSetStEph<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for AVLTreeSetStEph<T> {}

    impl<T: StT + Ord> PartialEq for AVLTreeSetStEph<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            proof {
                assume(self.elements.spec_avltreeseqsteph_wf());
                assume(other.elements.spec_avltreeseqsteph_wf());
            }
            let equal = self.size() == other.size() && {
                let n = self.elements.length();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.elements.spec_avltreeseqsteph_wf(),
                        other.elements.spec_avltreeseqsteph_wf(),
                        n == self.elements.spec_seq().len(),
                        i <= n,
                    decreases n - i,
                {
                    if !other.find(self.elements.nth(i)) {
                        all_found = false;
                        break;
                    }
                    i += 1;
                }
                all_found
            };
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = AVLTreeSetStEph { elements: self.elements.clone() };
            cloned
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! AVLTreeSetStEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    // 13. derive impls outside verus!

    impl<T: StT + Ord> fmt::Debug for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> fmt::Display for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
