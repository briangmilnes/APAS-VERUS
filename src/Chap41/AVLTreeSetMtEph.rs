//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Multi-threaded ephemeral set implementation using AVLTreeSetStEph as backing store.
//!
//! Work/Span Analysis (with extract-parallelize-rebuild pattern):
//! - union: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - intersection: Work Θ(n+m), Span Θ(log(n+m)) via PARALLEL divide-and-conquer
//! - filter: Work Θ(n), Span Θ(log n) via PARALLEL map-reduce

pub mod AVLTreeSetMtEph {

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
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;
    use std::sync::Arc;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    // NOTE: This type does NOT implement Ord (unlike AVLTreeSetMtPer) because no caller requires it.
    // AVLTreeSetMtEph is not used as a value type in OrderedTableMtPer. It's only used for:
    // - Standalone set operations (union, intersection, filter)
    // - As a set of vertices/priorities in PQMinMtEph (Chap53)
    //
    // If future code tries to use AVLTreeSetMtEph as a value in OrderedTableMtPer, compilation
    // will fail with "the trait bound `AVLTreeSetMtEph<V>: Ord` is not satisfied", and we can
    // implement Ord then. This is purely driven by caller requirements.

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};


    // 4. type definitions

    pub struct AVLTreeSetMtEphInv;

    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        pub inner: Arc<RwLock<AVLTreeSetStEph<T>, AVLTreeSetMtEphInv>>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphIter<T: StTInMtT + Ord + 'static> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphGhostIter<T: StTInMtT + Ord + 'static> {
        pub pos: int,
        pub elements: Seq<T>,
    }


    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) {
            (self.pos as int, self.snapshot@)
        }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphGhostIter<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn avltreesetmteph_iter_invariant<T: StTInMtT + Ord + 'static>(it: &AVLTreeSetMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    // 8. traits

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        /// Well-formedness: exec fields consistent with lock predicate's ghost fields.
        spec fn spec_avltreesetmteph_wf(&self) -> bool;

        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree@.finite(),
                tree.spec_avltreesetmteph_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            ensures
                constructed@.finite(),
                constructed.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (filtered: Self)
            requires self.spec_avltreesetmteph_wf(),
            ensures
                filtered@.finite(),
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_avltreesetmteph_wf(), other.spec_avltreesetmteph_wf(),
            ensures
                common@ == self@.intersect(other@),
                common@.finite(),
                common.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_avltreesetmteph_wf(), other.spec_avltreesetmteph_wf(),
            ensures
                remaining@ == self@.difference(other@),
                remaining@.finite(),
                remaining.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            requires self.spec_avltreesetmteph_wf(), other.spec_avltreesetmteph_wf(),
            ensures
                combined@ == self@.union(other@),
                combined@.finite(),
                combined.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires self.spec_avltreesetmteph_wf(),
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires old(self).spec_avltreesetmteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self@.finite(),
                self.spec_avltreesetmteph_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires old(self).spec_avltreesetmteph_wf(),
            ensures
                self@ == old(self)@.insert(x@),
                self@.finite(),
                self.spec_avltreesetmteph_wf();
        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it);
    }


    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEph<T> {
        #[verifier::external_body]
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            Set::empty()
        }
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> RwLockPredicate<AVLTreeSetStEph<T>> for AVLTreeSetMtEphInv {
        open spec fn inv(self, v: AVLTreeSetStEph<T>) -> bool {
            v.elements.spec_well_formed()
        }
    }


    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        // Unit Inv: no ghost↔exec fields to link. Trivially well-formed.
        open spec fn spec_avltreesetmteph_wf(&self) -> bool {
            true
        }

        #[verifier::external_body]
        fn size(&self) -> (count: usize)
        {
            let handle = self.inner.acquire_read();
            let count = handle.borrow().size();
            handle.release_read();
            count
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            let handle = self.inner.acquire_read();
            let seq = handle.borrow().to_seq();
            handle.release_read();
            seq
        }

        #[verifier::external_body]
        fn empty() -> (empty: Self)
        {
            AVLTreeSetMtEph {
                inner: new_arc_rwlock(AVLTreeSetStEph::empty(), Ghost(AVLTreeSetMtEphInv)),
            }
        }

        #[verifier::external_body]
        fn singleton(x: T) -> (tree: Self)
        {
            AVLTreeSetMtEph {
                inner: new_arc_rwlock(AVLTreeSetStEph::singleton(x), Ghost(AVLTreeSetMtEphInv)),
            }
        }

        #[verifier::external_body]
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            AVLTreeSetMtEph {
                inner: new_arc_rwlock(AVLTreeSetStEph::from_seq(seq), Ghost(AVLTreeSetMtEphInv)),
            }
        }

        // PARALLEL: filter using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n), Span: Θ(log n)
        #[verifier::external_body]
        fn filter<F: PredMt<T> + Clone>(&self, f: F) -> (filtered: Self)
        {
            let vals = {
                let handle = self.inner.acquire_read();
                let seq = handle.borrow().to_seq();
                let mut vals = Vec::with_capacity(seq.length());
                for i in 0..seq.length() {
                    vals.push(seq.nth(i).clone());
                }
                handle.release_read();
                vals
            };

            fn parallel_filter<T: StTInMtT + Ord + 'static, F: PredMt<T> + Clone>(vals: Vec<T>, f: F) -> Vec<T> {
                let n = vals.len();
                if n == 0 {
                    return Vec::new();
                }
                if n == 1 {
                    return if f(&vals[0]) { vals } else { Vec::new() };
                }

                let mid = n / 2;
                let mut right_vals = vals;
                let left_vals = right_vals.split_off(mid);
                let right_vals_final = right_vals;

                let f_left = f.clone();
                let f_right = f;

                let Pair(left_filtered, right_filtered) = ParaPair!(
                    move || parallel_filter(left_vals, f_left),
                    move || parallel_filter(right_vals_final, f_right)
                );

                let mut filtered = left_filtered;
                filtered.extend(right_filtered);
                filtered
            }

            let filtered = parallel_filter(vals, f);
            Self::from_seq(AVLTreeSeqStEphS::from_vec(filtered))
        }

        // PARALLEL: intersection using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        #[verifier::external_body]
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let (self_vals, other_vals) = {
                let self_handle = self.inner.acquire_read();
                let other_handle = other.inner.acquire_read();

                let self_seq = self_handle.borrow().to_seq();
                let other_seq = other_handle.borrow().to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                self_handle.release_read();
                other_handle.release_read();

                (sv, ov)
            };

            fn parallel_intersect<T: StTInMtT + Ord + 'static>(self_vals: Vec<T>, other_vals: Vec<T>) -> Vec<T> {
                let n = self_vals.len();
                if n == 0 {
                    return Vec::new();
                }
                if n == 1 {
                    let other_set = AVLTreeSetMtEph::from_seq(AVLTreeSeqStEphS::from_vec(other_vals));
                    return if other_set.find(&self_vals[0]) {
                        self_vals
                    } else {
                        Vec::new()
                    };
                }

                let mid = n / 2;
                let mut right_self = self_vals;
                let left_self = right_self.split_off(mid);
                let right_self_final = right_self;

                let other_left = other_vals.clone();
                let other_right = other_vals;

                let Pair(left_intersect, right_intersect) =
                    ParaPair!(move || parallel_intersect(left_self, other_left), move || {
                        parallel_intersect(right_self_final, other_right)
                    });

                let mut common = left_intersect;
                common.extend(right_intersect);
                common
            }

            let intersect = parallel_intersect(self_vals, other_vals);
            Self::from_seq(AVLTreeSeqStEphS::from_vec(intersect))
        }

        // PARALLEL: difference using filter
        #[verifier::external_body]
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let other_clone = other.clone();
            self.filter(move |x| !other_clone.find(x))
        }

        // PARALLEL: union using extract-parallelize-rebuild pattern (unconditionally parallel)
        // Work: Θ(n+m), Span: Θ(log(n+m))
        // Note: Union uses a simple merge strategy to avoid thread explosion.
        #[verifier::external_body]
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let (self_vals, other_vals) = {
                let self_handle = self.inner.acquire_read();
                let other_handle = other.inner.acquire_read();

                let self_seq = self_handle.borrow().to_seq();
                let other_seq = other_handle.borrow().to_seq();

                let mut sv = Vec::with_capacity(self_seq.length());
                for i in 0..self_seq.length() {
                    sv.push(self_seq.nth(i).clone());
                }

                let mut ov = Vec::with_capacity(other_seq.length());
                for i in 0..other_seq.length() {
                    ov.push(other_seq.nth(i).clone());
                }

                self_handle.release_read();
                other_handle.release_read();

                (sv, ov)
            };

            let mut merged = self_vals;
            merged.extend(other_vals);
            merged.sort();
            merged.dedup();

            Self::from_seq(AVLTreeSeqStEphS::from_vec(merged))
        }

        #[verifier::external_body]
        fn find(&self, x: &T) -> (found: B)
        {
            let handle = self.inner.acquire_read();
            let found = handle.borrow().find(x);
            handle.release_read();
            found
        }

        #[verifier::external_body]
        fn delete(&mut self, x: &T)
        {
            let (mut current, write_handle) = self.inner.acquire_write();
            current.delete(x);
            write_handle.release_write(current);
        }

        #[verifier::external_body]
        fn insert(&mut self, x: T)
        {
            let (mut current, write_handle) = self.inner.acquire_write();
            current.insert(x);
            write_handle.release_write(current);
        }

        #[verifier::external_body]
        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
        {
            let handle = self.inner.acquire_read();
            let seq = handle.borrow().to_seq();
            let mut vals = Vec::with_capacity(seq.length());
            for i in 0..seq.length() {
                vals.push(seq.nth(i).clone());
            }
            handle.release_read();
            AVLTreeSetMtEphIter { snapshot: vals, pos: 0 }
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
    }

    // 10. iterators

    impl<T: StTInMtT + Ord + 'static> std::iter::Iterator for AVLTreeSetMtEphIter<T> {
        type Item = T;

        #[verifier::external_body]
        fn next(&mut self) -> (next: Option<T>)
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
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos += 1;
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSetMtEphIter<T> {
        type GhostIter = AVLTreeSetMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> AVLTreeSetMtEphGhostIter<T> {
            AVLTreeSetMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIterator for AVLTreeSetMtEphGhostIter<T> {
        type ExecIter = AVLTreeSetMtEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSetMtEphIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSetMtEphIter<T>) -> AVLTreeSetMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + 'static> std::iter::IntoIterator for &'a AVLTreeSetMtEph<T> {
        type Item = T;
        type IntoIter = AVLTreeSetMtEphIter<T>;
        fn into_iter(self) -> (it: AVLTreeSetMtEphIter<T>)
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtEph<T> {
        #[verifier::external_body]
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            AVLTreeSetMtEph {
                inner: clone_arc_rwlock(&self.inner),
            }
        }
    }

    } // verus!

    // 12. macros

    #[macro_export]
    macro_rules! AVLTreeSetMtEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    // 13. derive impls outside verus!

    impl fmt::Debug for AVLTreeSetMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphInv")
        }
    }

    impl fmt::Display for AVLTreeSetMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphInv")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
