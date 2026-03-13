//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Multi-threaded persistent set implementation using AVLTreeSeqMtPer (Arc-based).
//!
//! Work/Span Analysis:
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtPer {

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

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::fmt;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::ParaPair;
    use crate::Types::Types::*;

    // NOTE: This type implements Ord because it is used as a VALUE in OrderedTableMtPer.
    // OrderedTableMtPer<K, V> is backed by BSTParaTreapMtEph<Pair<K, V>>, which requires
    // BOTH K and V to be Ord (via MtKey trait). For example, AdjTableGraphMtPer uses
    // OrderedTableMtPer<V, AVLTreeSetMtPer<V>>, so AVLTreeSetMtPer<V> must implement Ord.
    //
    // This is purely a caller requirement - if no code used AVLTreeSetMtPer as a value in
    // an ordered table, we wouldn't need Ord. See AVLTreeSetMtEph for comparison (no Ord needed).

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        pub elements: AVLTreeSeqMtPerS<T>,
    }

    /// Sequential cutoff to prevent thread explosion from recursive ParaPair! calls.
    pub const SEQUENTIAL_CUTOFF: usize = 1;


    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }


    // 8. traits

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static> {
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
            ensures constructed@.finite();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (filtered: Self)
            ensures filtered@.finite(), filtered@.subset_of(self@);
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            ensures updated@ == self@.remove(x@), updated@.finite();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            ensures updated@ == self@.insert(x@), updated@.finite();
    }


    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPer<T> {
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            self.elements@.to_set()
        }
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        fn size(&self) -> (count: usize)
        {
            proof { assume(self.elements.spec_avltreeseqmtper_wf()); }
            let r = self.elements.length();
            proof {
                vstd::seq_lib::seq_to_set_is_finite(self.elements@);
                assume(r == self@.len());
            }
            r
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
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
            let empty = AVLTreeSetMtPer { elements: AVLTreeSeqMtPerS::empty() };
            assert(empty.elements@ =~= Seq::<<T as View>::V>::empty());
            assert(empty@ =~= Set::<<T as View>::V>::empty());
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let ghost x_view = x@;
            let tree = AVLTreeSetMtPer { elements: AVLTreeSeqMtPerS::singleton(x) };
            proof {
                assert(tree.elements@ =~= seq![x_view]);
                let s = tree.elements@;
                assert forall |v| #[trigger] s.to_set().contains(v) <==> Set::<<T as View>::V>::empty().insert(x_view).contains(v) by {
                    if Set::<<T as View>::V>::empty().insert(x_view).contains(v) {
                        assert(v == x_view);
                        assert(s[0] == x_view);
                        assert(s.contains(v));
                    }
                    if s.to_set().contains(v) {
                        assert(s.contains(v));
                        let i = choose |i: int| 0 <= i < s.len() && s[i] == v;
                        assert(i == 0);
                        assert(v == x_view);
                    }
                }
                assert(tree@ =~= Set::<<T as View>::V>::empty().insert(x_view));
                vstd::seq_lib::seq_to_set_is_finite(tree.elements@);
            }
            tree
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
            ensures constructed@.finite()
        {
            let mut vals = seq.values_in_order();

            fn parallel_sort<T: StTInMtT + Ord + 'static>(mut vals: Vec<T>) -> Vec<T> {
                let n = vals.len();
                if n <= 1 {
                    return vals;
                }
                if n <= SEQUENTIAL_CUTOFF {
                    vals.sort();
                    return vals;
                }

                let mid = n / 2;
                let right_vals = vals.split_off(mid);
                let left_vals = vals;

                let Pair(left_sorted, right_sorted) =
                    ParaPair!(move || parallel_sort(left_vals), move || parallel_sort(right_vals));

                // Merge sorted halves
                let mut constructed = Vec::with_capacity(n);
                let mut i = 0;
                let mut j = 0;
                while i < left_sorted.len() && j < right_sorted.len() {
                    if left_sorted[i] <= right_sorted[j] {
                        constructed.push(left_sorted[i].clone());
                        i += 1;
                    } else {
                        constructed.push(right_sorted[j].clone());
                        j += 1;
                    }
                }
                constructed.extend_from_slice(&left_sorted[i..]);
                constructed.extend_from_slice(&right_sorted[j..]);
                constructed
            }

            vals = parallel_sort(vals);
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        // PARALLEL: filter using divide-and-conquer with sequential cutoff
        // Work: Θ(n), Span: Θ(log n) when parallel
        #[verifier::external_body]
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (filtered: Self)
            ensures filtered@.finite(), filtered@.subset_of(self@)
        {
            let n = self.size();

            if n <= 1 {
                if n == 0 {
                    return Self::empty();
                }
                let elem = self.elements.nth(0);
                if f(elem) {
                    return Self::singleton(elem.clone());
                } else {
                    return Self::empty();
                }
            }

            if n <= SEQUENTIAL_CUTOFF {
                let mut vals: Vec<T> = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if f(elem) {
                        vals.push(elem.clone());
                    }
                }
                vals.sort();
                vals.dedup();
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));

            let f_left = f.clone();
            let f_right = f;

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.filter(f_left), move || right_set.filter(f_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        // PARALLEL: intersection using divide-and-conquer with sequential cutoff
        // Work: Θ(n+m), Span: Θ(log(n+m)) when parallel
        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common@.finite()
        {
            let n = self.size();
            let m = other.size();

            if n == 0 || m == 0 {
                return Self::empty();
            }

            if n == 1 {
                let elem = self.elements.nth(0);
                if other.find(elem) {
                    return Self::singleton(elem.clone());
                } else {
                    return Self::empty();
                }
            }

            if n + m <= SEQUENTIAL_CUTOFF {
                let mut vals: Vec<T> = Vec::new();
                for i in 0..n {
                    let elem = self.elements.nth(i);
                    if other.find(elem) {
                        vals.push(elem.clone());
                    }
                }
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) =
                ParaPair!(move || left_set.intersection(&other_left), move || right_set
                    .intersection(&other_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining@.finite()
        {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using divide-and-conquer with sequential cutoff
        // Work: Θ(n+m), Span: Θ(log(n+m)) when parallel
        #[verifier::external_body]
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined@.finite()
        {
            let n = self.size();
            let m = other.size();

            if n == 0 {
                return other.clone();
            }
            if m == 0 {
                return self.clone();
            }

            if n == 1 {
                return other.insert(self.elements.nth(0).clone());
            }

            if n + m <= SEQUENTIAL_CUTOFF {
                let mut vals = self.elements.values_in_order();
                let other_vals = other.elements.values_in_order();
                vals.extend(other_vals);
                vals.sort();
                vals.dedup();
                return AVLTreeSetMtPer {
                    elements: AVLTreeSeqMtPerS::from_vec(vals),
                };
            }

            // Parallel divide-and-conquer using ParaPair!
            let mid = n / 2;

            let left_vals = (0..mid).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();
            let right_vals = (mid..n).map(|i| self.elements.nth(i).clone()).collect::<Vec<T>>();

            let left_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(left_vals));
            let right_set = Self::from_seq(AVLTreeSeqMtPerS::from_vec(right_vals));
            let other_left = other.clone();
            let other_right = other.clone();

            let Pair(left_result, right_result) = ParaPair!(move || left_set.union(&other_left), move || right_set
                .union(&other_right));

            // Sequential merge of results to avoid nested parallel recursion
            let mut vals = left_result.elements.values_in_order();
            vals.extend(right_result.elements.values_in_order());
            vals.sort();
            vals.dedup();
            AVLTreeSetMtPer {
                elements: AVLTreeSeqMtPerS::from_vec(vals),
            }
        }

        fn find(&self, x: &T) -> (found: B)
        {
            proof {
                assume(self.elements.spec_avltreeseqmtper_wf());
                assume(obeys_feq_full::<T>());  // accept hole: feq bridge
            }
            let n = self.elements.length();
            let mut lo: usize = 0;
            let mut hi: usize = n;
            while lo < hi
                invariant
                    self.elements.spec_avltreeseqmtper_wf(),
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

        #[verifier::external_body]
        fn delete(&self, x: &T) -> (updated: Self)
            ensures updated@ == self@.remove(x@), updated@.finite()
        {
            
            let x_clone = x.clone();
            self.filter(move |v| v != &x_clone)
        }

        #[verifier::external_body]
        fn insert(&self, x: T) -> (updated: Self)
            ensures updated@ == self@.insert(x@), updated@.finite()
        {
            if self.find(&x) {
                return self.clone();
            }
            let mut vals = self.elements.values_in_order();
            vals.push(x);

            
            Self::from_seq(AVLTreeSeqMtPerS::from_vec(vals))
        }
    }


    

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtPer<T> {
        fn default() -> Self { Self::empty() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StTInMtT + Ord + 'static> PartialEqSpecImpl for AVLTreeSetMtPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StTInMtT + Ord + 'static> Eq for AVLTreeSetMtPer<T> {}

    impl<T: StTInMtT + Ord + 'static> PartialEq for AVLTreeSetMtPer<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.size() == other.size() && {
                let n = self.size();
                let mut i: usize = 0;
                let mut all_found = true;
                while i < n
                    invariant
                        self.elements.spec_avltreeseqmtper_wf(),
                        other.elements.spec_avltreeseqmtper_wf(),
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
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StTInMtT + Ord + 'static> PartialOrd for AVLTreeSetMtPer<T> {
        fn partial_cmp(&self, other: &Self) -> (ord: Option<Ordering>) {
            let ord = Some(self.cmp(other));
            ord
        }
    }

    impl<T: StTInMtT + Ord + 'static> Ord for AVLTreeSetMtPer<T> {
        fn cmp(&self, other: &Self) -> (ord: Ordering) {
            // Lexicographic ordering: compare element by element.
            let n_self = self.size();
            let n_other = other.size();
            let min_n = if n_self < n_other { n_self } else { n_other };
            let mut i: usize = 0;
            while i < min_n
                invariant
                    self.elements.spec_avltreeseqmtper_wf(),
                    other.elements.spec_avltreeseqmtper_wf(),
                    i <= min_n,
                    min_n <= n_self,
                    min_n <= n_other,
                    n_self == self.elements.spec_seq().len(),
                    n_other == other.elements.spec_seq().len(),
                decreases min_n - i,
            {
                let a = self.elements.nth(i);
                let b = other.elements.nth(i);
                let c = a.cmp(b);
                if c != Equal {
                    return c;
                }
                i += 1;
            }
            n_self.cmp(&n_other)
        }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = AVLTreeSetMtPer { elements: self.elements.clone() };
            proof { accept(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! AVLTreeSetMtPerLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPer<_> as $crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::AVLTreeSetMtPerTrait<_> >::empty();
            $( __set = __set.insert($x); )*
            __set
        }};
    }

    // 13. derive impls outside verus!

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.size() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
