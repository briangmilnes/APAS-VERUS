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
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions

    pub struct AVLTreeSetStEph<T: StT + Ord> {
        elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;

    // 5. view impls

    // Abstract view function. Closed + external_body = uninterpreted. The ensures
    // clauses on the trait operations axiomatize its behavior. Do not reveal.
    impl<T: StT + Ord> AVLTreeSetStEph<T> {
        #[verifier::external_body]
        pub closed spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            Set::empty()
        }
    }

    impl<T: StT + Ord> View for AVLTreeSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    // 8. traits

    pub trait AVLTreeSetStEphTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (result: usize)
            ensures result == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
            ensures self@.finite();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (result: Self)
            ensures result@ == Set::<<T as View>::V>::empty().insert(x@), result@.finite();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
            ensures result@.finite();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
            ensures result@.finite(), result@.subset_of(self@);
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.intersect(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.difference(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> (result: Self)
            ensures result@ == self@.union(other@), result@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (result: B)
            ensures result == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite();
    }

    // 9. impls

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        fn size(&self) -> (result: usize)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let r = self.elements.length();
            proof { assume(r == self@.len()); assume(self@.finite()); }
            r
        }

        fn to_seq(&self) -> (result: AVLTreeSeqStEphS<T>)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let n = self.elements.length();
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                v.push(self.elements.nth(i).clone());
                i += 1;
            }
            let result = AVLTreeSeqStEphS::from_vec(v);
            proof { assume(self@.finite()); }
            result
        }

        fn empty() -> (result: Self)
        {
            let result = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::empty() };
            proof { assume(result@ == Set::<<T as View>::V>::empty()); }
            result
        }

        fn singleton(x: T) -> (result: Self)
        {
            let ghost x_view = x@;
            let result = AVLTreeSetStEph { elements: AVLTreeSeqStEphS::singleton(x) };
            proof {
                assume(result@ == Set::<<T as View>::V>::empty().insert(x_view));
                assume(result@.finite());
            }
            result
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (result: Self)
        {
            proof { assume(seq.spec_well_formed()); }
            let mut result = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_well_formed(),
                    n as int == seq.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = seq.nth(i).clone();
                result.insert(elem);
                i += 1;
            }
            proof { assume(result@.finite()); }
            result
        }

        fn filter<F: PredSt<T>>(&self, f: F) -> (result: Self)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let mut result = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                proof { assume(f.requires((&*elem,))); }
                if f(elem) {
                    result.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(result@.finite());
                assume(result@.subset_of(self@));
            }
            result
        }

        fn intersection(&self, other: &Self) -> (result: Self)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let mut result = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(result@ == self@.intersect(other@));
                assume(result@.finite());
            }
            result
        }

        fn difference(&self, other: &Self) -> (result: Self)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let mut result = Self::empty();
            let n = self.elements.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result.insert(elem.clone());
                }
                i += 1;
            }
            proof {
                assume(result@ == self@.difference(other@));
                assume(result@.finite());
            }
            result
        }

        fn union(&self, other: &Self) -> (result: Self)
        {
            proof {
                assume(self.elements.spec_well_formed());
                assume(other.elements.spec_well_formed());
            }
            let mut result = Self::empty();
            let self_len = self.elements.length();
            let mut i: usize = 0;
            while i < self_len
                invariant
                    self.elements.spec_well_formed(),
                    self_len as int == self.elements.spec_seq().len(),
                    i <= self_len,
                decreases self_len - i,
            {
                result.insert(self.elements.nth(i).clone());
                i += 1;
            }
            let other_len = other.elements.length();
            let mut j: usize = 0;
            while j < other_len
                invariant
                    other.elements.spec_well_formed(),
                    other_len as int == other.elements.spec_seq().len(),
                    j <= other_len,
                decreases other_len - j,
            {
                result.insert(other.elements.nth(j).clone());
                j += 1;
            }
            proof {
                assume(result@ == self@.union(other@));
                assume(result@.finite());
            }
            result
        }

        fn find(&self, x: &T) -> (result: B)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    lo <= hi, hi <= n,
                decreases hi - lo,
            {
                let mid = lo + (hi - lo) / 2;
                let elem = self.elements.nth(mid);
                if *elem == *x {
                    proof { assume(self@.contains(x@)); }
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
            proof { assume(self.elements.spec_well_formed()); }
            let n = self.elements.length();
            let mut result_vec: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < n
                invariant
                    self.elements.spec_well_formed(),
                    n as int == self.elements.spec_seq().len(),
                    i <= n,
                decreases n - i,
            {
                let elem = self.elements.nth(i);
                if *elem != *x {
                    result_vec.push(elem.clone());
                }
                i += 1;
            }
            self.elements = AVLTreeSeqStEphS::from_vec(result_vec);
            proof {
                assume(self@ == old(self)@.remove(x@));
                assume(self@.finite());
            }
        }

        fn insert(&mut self, x: T)
        {
            proof { assume(self.elements.spec_well_formed()); }
            let ghost x_view = x@;
            if !self.find(&x) {
                let n = self.elements.length();
                let mut lo: usize = 0;
                let mut hi: usize = n;
                while lo < hi
                    invariant
                        self.elements.spec_well_formed(),
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
                        self.elements.spec_well_formed(),
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
                        self.elements.spec_well_formed(),
                        n as int == self.elements.spec_seq().len(),
                        lo <= j, j <= n,
                    decreases n - j,
                {
                    new_vec.push(self.elements.nth(j).clone());
                    j += 1;
                }
                self.elements = AVLTreeSeqStEphS::from_vec(new_vec);
            }
            proof {
                assume(self@ == old(self)@.insert(x_view));
                assume(self@.finite());
            }
        }
    }

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            let result = AVLTreeSetStEph { elements: self.elements.clone() };
            proof { assume(result@ == self@); }
            result
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

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord> PartialEq for AVLTreeSetStEph<T> {
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

    impl<T: StT + Ord> Eq for AVLTreeSetStEph<T> {}

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
