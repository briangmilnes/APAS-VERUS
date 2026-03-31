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
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
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

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        pub inner: Arc<RwLock<AVLTreeSetStEph<T>, AVLTreeSetMtEphInv>>,
        pub ghost_set_view: Ghost<Set<<T as View>::V>>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphIter<T: StTInMtT + Ord + 'static> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphGhostIter<T: StTInMtT + Ord + 'static> {
        pub pos: int,
        pub elements: Seq<T::V>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphIter<T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) {
            (self.pos as int, self.snapshot@.map_values(|t: T| t@))
        }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphGhostIter<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn avltreesetmteph_iter_invariant<T: StTInMtT + Ord + 'static>(it: &AVLTreeSetMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 8. traits

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        /// Well-formedness: exec fields consistent with lock predicate's ghost fields.
        spec fn spec_avltreesetmteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                seq.spec_avltreeseqsteph_wf(),
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetmteph_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetmteph_wf();
        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it);
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEph<T> {
        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            self.ghost_set_view@
        }
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> RwLockPredicate<AVLTreeSetStEph<T>> for AVLTreeSetMtEphInv {
        open spec fn inv(self, v: AVLTreeSetStEph<T>) -> bool {
            v.spec_avltreesetsteph_wf()
        }
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        open spec fn spec_avltreesetmteph_wf(&self) -> bool {
            self.ghost_set_view@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            let handle = self.inner.acquire_read();
            let count = handle.borrow().size();
            proof { assume(count == self@.len()); }
            handle.release_read();
            count
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            let handle = self.inner.acquire_read();
            let inner_ref = handle.borrow();
            let seq = inner_ref.to_seq();
            proof {
                // Reader accept: inner's set view matches ghost shadow.
                assume(seq@.to_set() =~= self@);
                vstd::seq_lib::seq_to_set_is_finite(seq@);
            }
            handle.release_read();
            seq
        }

        fn empty() -> (empty: Self)
        {
            let st = AVLTreeSetStEph::empty();
            assert(AVLTreeSetMtEphInv.inv(st));
            let empty = AVLTreeSetMtEph {
                inner: new_arc_rwlock(st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(st@),
            };
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let st = AVLTreeSetStEph::singleton(x);
            assert(AVLTreeSetMtEphInv.inv(st));
            let tree = AVLTreeSetMtEph {
                inner: new_arc_rwlock(st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(st@),
            };
            tree
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            let st = AVLTreeSetStEph::from_seq(seq);
            assert(AVLTreeSetMtEphInv.inv(st));
            let constructed = AVLTreeSetMtEph {
                inner: new_arc_rwlock(st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(st@),
            };
            constructed
        }

        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let handle = self.inner.acquire_read();
            let inner_ref = handle.borrow();
            let inner_filtered = inner_ref.filter(f, Ghost(spec_pred));
            handle.release_read();
            assert(AVLTreeSetMtEphInv.inv(inner_filtered));
            let filtered = AVLTreeSetMtEph {
                inner: new_arc_rwlock(inner_filtered, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(inner_filtered@),
            };
            proof {
                // Reader accept: inner's set view matches ghost shadow.
                assume(filtered@.subset_of(self@));
                assume(filtered.spec_avltreesetmteph_wf());
                assume(forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v));
                assume(forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v));
            }
            filtered
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let self_handle = self.inner.acquire_read();
            let other_handle = other.inner.acquire_read();
            let common_st = self_handle.borrow().intersection(other_handle.borrow());
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtEphInv.inv(common_st));
            let common = AVLTreeSetMtEph {
                inner: new_arc_rwlock(common_st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(common_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(common@ == self@.intersect(other@));
                assert(common_st@.finite());
            }
            common
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let self_handle = self.inner.acquire_read();
            let other_handle = other.inner.acquire_read();
            let remaining_st = self_handle.borrow().difference(other_handle.borrow());
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtEphInv.inv(remaining_st));
            let remaining = AVLTreeSetMtEph {
                inner: new_arc_rwlock(remaining_st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(remaining_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(remaining@ == self@.difference(other@));
                assert(remaining_st@.finite());
            }
            remaining
        }

        fn union(&self, other: &Self) -> (combined: Self)
        {
            let self_handle = self.inner.acquire_read();
            let other_handle = other.inner.acquire_read();
            let self_st = self_handle.borrow();
            let other_st = other_handle.borrow();
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(self_st@.len() + other_st@.len() < usize::MAX as nat);
            }
            let combined_st = self_st.union(other_st);
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtEphInv.inv(combined_st));
            let combined = AVLTreeSetMtEph {
                inner: new_arc_rwlock(combined_st, Ghost(AVLTreeSetMtEphInv)),
                ghost_set_view: Ghost(combined_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(combined@ == self@.union(other@));
                assert(combined_st@.finite());
            }
            combined
        }

        fn find(&self, x: &T) -> (found: bool)
        {
            let handle = self.inner.acquire_read();
            let found = handle.borrow().find(x);
            proof { assume(found == self@.contains(x@)); }  // accept hole: reader predicate
            handle.release_read();
            found
        }

        fn delete(&mut self, x: &T)
        {
            let ghost old_view = self.ghost_set_view@;
            let ghost x_view = x@;
            let (mut current, write_handle) = self.inner.acquire_write();
            current.delete(x);
            assert(AVLTreeSetMtEphInv.inv(current));
            write_handle.release_write(current);
            self.ghost_set_view = Ghost(old_view.remove(x_view));
        }

        fn insert(&mut self, x: T)
        {
            let ghost old_view = self.ghost_set_view@;
            let ghost x_view = x@;
            let (mut current, write_handle) = self.inner.acquire_write();
            proof {
                assume(current@.len() + 1 < usize::MAX as nat); // RWLOCK_GHOST
            }
            current.insert(x);
            assert(AVLTreeSetMtEphInv.inv(current));
            write_handle.release_write(current);
            self.ghost_set_view = Ghost(old_view.insert(x_view));
        }

        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
        {
            let handle = self.inner.acquire_read();
            let inner_ref = handle.borrow();
            proof {
                assert(inner_ref@.finite());
            }
            let seq = inner_ref.to_seq();
            handle.release_read();
            let mut vals: Vec<T> = Vec::new();
            let len = seq.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    seq.spec_avltreeseqsteph_wf(),
                    len == seq@.len(),
                decreases len - i,
            {
                vals.push(seq.nth(i).clone());
                i = i + 1;
            }
            AVLTreeSetMtEphIter { snapshot: vals, pos: 0 }
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> std::iter::Iterator for AVLTreeSetMtEphIter<T> {
        type Item = T;

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
                        &&& element@ == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                proof { assume(item@ == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
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
        type Item = T::V;
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

        open spec fn ghost_peek_next(&self) -> Option<T::V> {
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
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = AVLTreeSetMtEph {
                inner: clone_arc_rwlock(&self.inner),
                ghost_set_view: Ghost(self.ghost_set_view@),
            };
            cloned
        }
    }

    } // verus!

    // Ghost<Set<V>> field is zero-sized; AVLTreeSetMtEph is Send/Sync via Arc<RwLock>.
    unsafe impl<T: StTInMtT + Ord + 'static> Send for AVLTreeSetMtEph<T> {}
    unsafe impl<T: StTInMtT + Ord + 'static> Sync for AVLTreeSetMtEph<T> {}

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
