//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Multi-threaded persistent set implementation using AVLTreeSetStPer as backing store.
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
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 12. derive impls in verus!
    // 13. macros
    // 14. derive impls outside verus!

    use std::cmp::Ordering::{self, Equal, Greater, Less};
    use std::fmt;
    use std::sync::Arc;

    use vstd::prelude::*;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::ParamBSTTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use vstd::rwlock::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
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

    pub struct AVLTreeSetMtPerInv;

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
        pub locked_set: Arc<RwLock<AVLTreeSetStPer<T>, AVLTreeSetMtPerInv>>,
        pub ghost_set_view: Ghost<Set<<T as View>::V>>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.spec_set_view() }
    }

    // 6. spec fns

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPer<T> {
        pub open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.ghost_set_view@.finite()
        }

        pub open spec fn spec_set_view(&self) -> Set<<T as View>::V> {
            self.ghost_set_view@
        }
    }

    // 8. traits

    pub trait AVLTreeSetMtPerTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_avltreesetmtper_wf(&self) -> bool;

        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmtper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - APAS Cost Spec 41.4: Work |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
            requires self.spec_avltreesetmtper_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                seq.spec_avltreeseqmtper_wf(),
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_avltreesetmtper_wf();
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures constructed.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work Σ W(f(x)), Span lg |a| + max S(f(x))
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmtper_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            ensures common@ == self@.intersect(other@), common.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            ensures remaining@ == self@.difference(other@), remaining.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work m·lg(1+n/m), Span lg(n)
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            ensures combined@ == self@.union(other@), combined.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            requires
                self.spec_avltreesetmtper_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&self, x: &T) -> (updated: Self)
            ensures updated@ == self@.remove(x@), updated.spec_avltreesetmtper_wf();
        /// - APAS Cost Spec 41.4: Work lg |a|, Span lg |a|
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&self, x: T) -> (updated: Self)
            ensures updated@ == self@.insert(x@), updated.spec_avltreesetmtper_wf();
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> RwLockPredicate<AVLTreeSetStPer<T>> for AVLTreeSetMtPerInv {
        open spec fn inv(self, v: AVLTreeSetStPer<T>) -> bool {
            v.spec_avltreesetstper_wf()
        }
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtPerTrait<T> for AVLTreeSetMtPer<T> {
        open spec fn spec_avltreesetmtper_wf(&self) -> bool {
            self.ghost_set_view@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            let handle = self.locked_set.acquire_read();
            let count = handle.borrow().size();
            proof {
                // Reader accept: inner size matches ghost shadow size.
                assume(count == self@.len());
            }
            handle.release_read();
            count
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqMtPerS<T>)
        {
            let handle = self.locked_set.acquire_read();
            let inner_ref = handle.borrow();
            let mut vals = Vec::new();
            inner_ref.tree.collect_in_order(&mut vals);
            handle.release_read();
            let seq = AVLTreeSeqMtPerS::from_vec(vals);
            proof {
                // Reader accept: inner sequence view matches ghost shadow.
                assume(seq@.to_set() =~= self@);
                assert forall|i: int| 0 <= i < seq@.len()
                    implies #[trigger] self@.contains(seq@[i]) by {
                    assert(seq@.contains(seq@[i]));
                    vstd::seq_lib::seq_to_set_is_finite(seq@);
                };
            }
            seq
        }

        fn empty() -> (empty: Self)
        {
            let st = AVLTreeSetStPer::empty();
            assert(AVLTreeSetMtPerInv.inv(st));
            let empty = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(st@),
            };
            empty
        }

        fn singleton(x: T) -> (tree: Self)
        {
            let st = AVLTreeSetStPer::singleton(x);
            assert(AVLTreeSetMtPerInv.inv(st));
            let tree = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(st@),
            };
            tree
        }

        fn from_seq(seq: AVLTreeSeqMtPerS<T>) -> (constructed: Self)
        {
            let vals = seq.values_in_order();
            let n = vals.len();
            let mut st = AVLTreeSetStPer::empty();
            if n > usize::MAX - 2 {
                // Capacity guard: AVLTreeSetStPer::insert requires st@.len() + 1 < usize::MAX.
                assert(AVLTreeSetMtPerInv.inv(st));
                return AVLTreeSetMtPer {
                    locked_set: new_arc_rwlock(st, Ghost(AVLTreeSetMtPerInv)),
                    ghost_set_view: Ghost(st@),
                };
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    st.spec_avltreesetstper_wf(),
                    i <= n,
                    n == vals@.len(),
                    n <= usize::MAX - 2,
                    st@.len() <= i as nat,
                    vstd::laws_cmp::obeys_cmp_spec::<T>(),
                    view_ord_consistent::<T>(),
                decreases n - i,
            {
                st = st.insert(vals[i].clone());
                i += 1;
            }
            assert(AVLTreeSetMtPerInv.inv(st));
            let constructed = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(st, Ghost(AVLTreeSetMtPerInv)),
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
            let handle = self.locked_set.acquire_read();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
            }
            let inner_filtered = handle.borrow().filter(f, Ghost(spec_pred));
            handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(inner_filtered));
            let filtered = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(inner_filtered, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(inner_filtered@),
            };
            proof {
                // Reader accept: bridge inner StPer view to MtPer ghost shadow.
                assume(filtered@.subset_of(self@));
                assume(forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v));
                assume(forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v));
            }
            filtered
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let self_handle = self.locked_set.acquire_read();
            let other_handle = other.locked_set.acquire_read();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
            }
            let common_st = self_handle.borrow().intersection(other_handle.borrow());
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(common_st));
            let common = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(common_st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(common_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(common@ == self@.intersect(other@));
            }
            common
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let self_handle = self.locked_set.acquire_read();
            let other_handle = other.locked_set.acquire_read();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
            }
            let remaining_st = self_handle.borrow().difference(other_handle.borrow());
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(remaining_st));
            let remaining = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(remaining_st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(remaining_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(remaining@ == self@.difference(other@));
            }
            remaining
        }

        fn union(&self, other: &Self) -> (combined: Self)
        {
            let self_handle = self.locked_set.acquire_read();
            let other_handle = other.locked_set.acquire_read();
            let self_st: &AVLTreeSetStPer<T> = self_handle.borrow();
            let other_st: &AVLTreeSetStPer<T> = other_handle.borrow();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
                assume(self_st@.len() + other_st@.len() < usize::MAX as nat);
            }
            let combined_st = self_st.union(other_st);
            self_handle.release_read();
            other_handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(combined_st));
            let combined = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(combined_st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(combined_st@),
            };
            proof {
                // Reader accept: inner views match ghost shadows.
                assume(combined@ == self@.union(other@));
            }
            combined
        }

        fn find(&self, x: &T) -> (found: B)
        {
            let handle = self.locked_set.acquire_read();
            let found = handle.borrow().find(x);
            proof {
                // Reader accept: inner find result matches ghost shadow.
                assume(found == self@.contains(x@));
            }
            handle.release_read();
            found
        }

        fn delete(&self, x: &T) -> (updated: Self)
        {
            let handle = self.locked_set.acquire_read();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
            }
            let updated_st = handle.borrow().delete(x);
            handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(updated_st));
            let updated = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(updated_st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(updated_st@),
            };
            proof {
                // Reader accept: inner delete result matches ghost shadow.
                assume(updated@ == self@.remove(x@));
            }
            updated
        }

        fn insert(&self, x: T) -> (updated: Self)
        {
            let handle = self.locked_set.acquire_read();
            let st: &AVLTreeSetStPer<T> = handle.borrow();
            proof {
                assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
                assume(view_ord_consistent::<T>());
                assume(st@.len() + 1 < usize::MAX as nat);
            }
            let updated_st = st.insert(x);
            handle.release_read();
            assert(AVLTreeSetMtPerInv.inv(updated_st));
            let updated = AVLTreeSetMtPer {
                locked_set: new_arc_rwlock(updated_st, Ghost(AVLTreeSetMtPerInv)),
                ghost_set_view: Ghost(updated_st@),
            };
            proof {
                // Reader accept: inner insert result matches ghost shadow.
                assume(updated@ == self@.insert(x@));
            }
            updated
        }
    }

    // 12. derive impls in verus!

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
            let self_handle = self.locked_set.acquire_read();
            let other_handle = other.locked_set.acquire_read();
            let s: &AVLTreeSetStPer<T> = self_handle.borrow();
            let o: &AVLTreeSetStPer<T> = other_handle.borrow();
            let equal = s.eq(o);
            self_handle.release_read();
            other_handle.release_read();
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: StTInMtT + Ord + 'static> PartialOrd for AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        fn partial_cmp(&self, other: &Self) -> (ord: Option<Ordering>) {
            let ord = Some(self.cmp(other));
            ord
        }
    }

    impl<T: StTInMtT + Ord + 'static> Ord for AVLTreeSetMtPer<T> {
        #[verifier::external_body]
        fn cmp(&self, other: &Self) -> (ord: Ordering)
        {
            let self_handle = self.locked_set.acquire_read();
            let other_handle = other.locked_set.acquire_read();
            let mut self_seq: Vec<T> = Vec::new();
            self_handle.borrow().tree.collect_in_order(&mut self_seq);
            let mut other_seq = Vec::new();
            other_handle.borrow().tree.collect_in_order(&mut other_seq);
            self_handle.release_read();
            other_handle.release_read();
            let n_self = self_seq.len();
            let n_other = other_seq.len();
            let min_n = if n_self < n_other { n_self } else { n_other };
            let mut i: usize = 0;
            while i < min_n {
                let c = self_seq[i].cmp(&other_seq[i]);
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
            ensures cloned@ == self@, cloned.spec_avltreesetmtper_wf() == self.spec_avltreesetmtper_wf(),
        {
            let cloned = AVLTreeSetMtPer {
                locked_set: clone_arc_rwlock(&self.locked_set),
                ghost_set_view: Ghost(self.ghost_set_view@),
            };
            cloned
        }
    }

    } // verus!

    // Ghost<Set<V>> field is zero-sized; AVLTreeSetMtPer is Send/Sync via Arc<RwLock>.
    unsafe impl<T: StTInMtT + Ord + 'static> Send for AVLTreeSetMtPer<T> {}
    unsafe impl<T: StTInMtT + Ord + 'static> Sync for AVLTreeSetMtPer<T> {}

    // 13. macros

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

    // 14. derive impls outside verus!

    impl fmt::Debug for AVLTreeSetMtPerInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerInv")
        }
    }

    impl fmt::Display for AVLTreeSetMtPerInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtPerInv")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtPer<T> {
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

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtPer<T> {
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
