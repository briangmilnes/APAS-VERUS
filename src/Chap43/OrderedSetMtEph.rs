//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Multi-threaded ephemeral ordered set using coarse RwLock over OrderedSetStEph.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 5b. view impls
//	Section 8b. traits
//	Section 9b. impls
//	Section 11a. top level coarse locking
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!


//		Section 1. module

pub mod OrderedSetMtEph {

    //		Section 2. imports

    use std::cmp::Ordering::{Less, Greater};

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap43::OrderedSetStEph::OrderedSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::laws_cmp::group_laws_cmp,
    };

    //		Section 4a. type definitions


    pub struct OrderedSetMtEphInv;

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    //		Section 9a. impls


    // Helper: construct Mt wrapper from St set (used by split/get_range/split_rank/from_seq).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- wraps inner in RwLock
    fn from_st<T: MtKey + TotalOrder + 'static>(inner: OrderedSetStEph<T>) -> (s: OrderedSetMtEph<T>)
        requires inner.spec_orderedsetsteph_wf(), inner@.finite()
        ensures s@ == inner@, s.spec_orderedsetmteph_wf()
    {
        let ghost view = inner@;
        OrderedSetMtEph {
            locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
            ghost_locked_set: Ghost(view),
        }
    }

    //		Section 4b. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetMtEph<T: MtKey + TotalOrder + 'static> {
        pub(crate) locked_set: RwLock<OrderedSetStEph<T>, OrderedSetMtEphInv>,
        pub(crate) ghost_locked_set: Ghost<Set<<T as View>::V>>,
    }

    //		Section 5b. view impls


    impl<T: MtKey + TotalOrder + 'static> View for OrderedSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> {
            self.spec_ghost_locked_set()
        }
    }

    //		Section 8b. traits


    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics.
    pub trait OrderedSetMtEphTrait<T: MtKey + TotalOrder + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetmteph_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- acquires read lock, delegates to StEph.size
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetmteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- constructs empty StEph + RwLock
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetmteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(1), Span Θ(1) -- wraps StEph.singleton + RwLock
        fn singleton(x: T) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_orderedsetmteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- acquires lock, delegates to StEph.find (BST search)
        fn find(&self, x: &T) -> (found: bool)
            requires self.spec_orderedsetmteph_wf(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- acquires lock, delegates to StEph.insert (BST insert)
        fn insert(&mut self, x: T)
            requires old(self)@.len() + 1 < usize::MAX as nat,
            ensures self@ == old(self)@.insert(x@), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(log n), Span Θ(log n) -- acquires lock, delegates to StEph.delete (BST delete)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires lock, delegates to StEph.filter
        fn filter<F: Pred<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
            requires
                old(self).spec_orderedsetmteph_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- acquires lock, delegates to StEph.intersection (sequential)
        fn intersection(&mut self, other: &Self)
            requires old(self).spec_orderedsetmteph_wf(), other.spec_orderedsetmteph_wf(),
            ensures self@ == old(self)@.intersect(other@), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- acquires lock, delegates to StEph.union (sequential)
        fn union(&mut self, other: &Self)
            requires
                old(self).spec_orderedsetmteph_wf(),
                other.spec_orderedsetmteph_wf(),
                old(self)@.len() + other@.len() < usize::MAX as nat,
            ensures self@ == old(self)@.union(other@), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(m log(n/m + 1)), Span Θ(m log(n/m + 1)) -- acquires lock, delegates to StEph.difference (sequential)
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedsetmteph_wf(), other.spec_orderedsetmteph_wf(),
            ensures self@ == old(self)@.difference(other@), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n), Span Θ(n) -- acquires lock, delegates to StEph.to_seq
        fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(n log n), Span Θ(n log n) -- delegates to StEph.from_seq (n inserts)
        fn from_seq(seq: ArraySeqStPerS<T>) -> (constructed: Self)
            requires
                seq.spec_len() < usize::MAX as int,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures constructed@.finite();

        // Ordering operations (ADT 43.1)
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn first(&self) -> (first: Option<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn last(&self) -> (last: Option<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> v.cmp_spec(k) == Less,
                predecessor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn next(&self, k: &T) -> (successor: Option<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&mut self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetmteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn join(&mut self, other: Self)
            requires
                old(self).spec_orderedsetmteph_wf(),
                other.spec_orderedsetmteph_wf(),
                old(self)@.len() + other@.len() < usize::MAX as nat,
            ensures self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn get_range(&self, k1: &T, k2: &T) -> (range: Result<Self, ()>)
            requires
                self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                range matches Ok(r) ==> r.spec_orderedsetmteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn rank(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn select(&self, i: usize) -> (selected: Option<T>)
            requires self.spec_orderedsetmteph_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetmteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures self@.finite();
    }

    //		Section 9b. impls


    impl<T: MtKey + TotalOrder + 'static> OrderedSetMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_set@.finite()
        }

        pub closed spec fn spec_ghost_locked_set(self) -> Set<<T as View>::V> {
            self.ghost_locked_set@
        }
    }


    impl<T: MtKey + TotalOrder + 'static> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        open spec fn spec_orderedsetmteph_wf(&self) -> bool {
            self@.finite()
            && obeys_feq_full::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn size(&self) -> (count: usize) {
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block (speed hint)
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let count = inner.size();
            // Veracity: NEEDED proof block (speed hint)
            // Veracity: NEEDED proof block
            proof { assume(count == self@.len()); }
            read_handle.release_read();
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self) {
            let inner = OrderedSetStEph::empty();
            let ghost view = inner@;
            OrderedSetMtEph {
                locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
                ghost_locked_set: Ghost(view),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self) {
            let inner = OrderedSetStEph::singleton(x);
            let ghost view = inner@;
            OrderedSetMtEph {
                locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
                ghost_locked_set: Ghost(view),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST search
        // Veracity: NEEDED proof block
        fn find(&self, x: &T) -> (found: bool) {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            // Veracity: NEEDED proof block
            let inner = read_handle.borrow();
            let found = inner.find(x);
            // Veracity: NEEDED proof block
            proof { assume(found == self@.contains(x@)); }
            read_handle.release_read();
            found
        }
// Veracity: NEEDED proof block (speed hint)

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, treap insert
        fn insert(&mut self, x: T) {
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost x_view = x@;
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            // Veracity: NEEDED proof block
            proof { assume(locked_val@.len() + 1 < usize::MAX as nat); } // RWLOCK_GHOST
            locked_val.insert(x);
            write_handle.release_write(locked_val);
            // Veracity: NEEDED proof block
            self.ghost_locked_set = Ghost(old_view.insert(x_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, treap delete
        fn delete(&mut self, x: &T) {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost x_view = x@;
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.delete(x);
            write_handle.release_write(locked_val);
            self.ghost_locked_set = Ghost(old_view.remove(x_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, BST filter
        fn filter<F: Pred<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.filter(f, Ghost(spec_pred));
            let ghost new_view = locked_val@;
            write_handle.release_write(locked_val);
            self.ghost_locked_set = Ghost(new_view);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, BST intersection
        fn intersection(&mut self, other: &Self) {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.intersection(other_ref);
            write_handle.release_write(locked_val);
// Veracity: UNNEEDED proof block             other_read.release_read();
            self.ghost_locked_set = Ghost(old_view.intersect(other_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, BST union
        fn union(&mut self, other: &Self) {
            // Veracity: NEEDED proof block
// Veracity: UNNEEDED proof block             proof { use_type_invariant(&*self); use_type_invariant(other); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            // Veracity: NEEDED proof block
            proof { assume(locked_val@.len() + other_ref@.len() < usize::MAX as nat); } // RWLOCK_GHOST
            locked_val.union(other_ref);
            write_handle.release_write(locked_val);
// Veracity: UNNEEDED proof block             other_read.release_read();
            self.ghost_locked_set = Ghost(old_view.union(other_view));
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, BST difference
        fn difference(&mut self, other: &Self) {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.difference(other_ref);
            write_handle.release_write(locked_val);
            other_read.release_read();
            // Veracity: NEEDED proof block
            self.ghost_locked_set = Ghost(old_view.difference(other_view));
        }

        #[verifier::loop_isolation(false)]
// Veracity: UNNEEDED proof block         /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + vec copy
        fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let avl_seq = inner.to_seq();
            // Veracity: NEEDED proof block
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
            let len = avl_seq.length();
            let mut elements: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    avl_seq.spec_avltreeseqstper_wf(),
// Veracity: UNNEEDED proof block                     len as nat == avl_seq@.len(),
                    0 <= i <= len,
                    elements@.len() == i as int,
                    forall|j: int| 0 <= j < i ==> (#[trigger] elements@[j])@ == avl_seq@[j],
                decreases len - i,
            // Veracity: NEEDED proof block
            {
                let elem_ref = avl_seq.nth(i);
                let cloned = elem_ref.clone_plus();
                proof { lemma_cloned_view_eq(*elem_ref, cloned); }
                elements.push(cloned);
                i = i + 1;
            }
            let result = ArraySeqStPerS { seq: elements };
            proof {
                // Veracity: NEEDED assert
                assert(result@ =~= avl_seq@);
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n treap inserts
        fn from_seq(seq: ArraySeqStPerS<T>) -> (constructed: Self) {
            let len = seq.length();
            let mut inner = OrderedSetStEph::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    inner.spec_orderedsetsteph_wf(),
                    inner@.finite(),
                    inner@.len() <= i as nat,
                    i <= len,
                    len as int == seq.spec_len(),
                    seq.spec_len() < usize::MAX as int,
                decreases len - i,
            {
                // Capacity: inner@.len() <= i < len < usize::MAX (from requires).
                inner.insert(seq.nth(i).clone());
                // Veracity: NEEDED proof block
                i += 1;
            }
            from_st(inner)
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST min_key
        fn first(&self) -> (first: Option<T>)

        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            // Veracity: NEEDED proof block
            let inner = read_handle.borrow();
            let first = inner.first();
            proof { assume(inner@ =~= self@); }
            // Veracity: NEEDED proof block (speed hint)
            read_handle.release_read();
            first
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST max_key
        fn last(&self) -> (last: Option<T>)

        {
            // Veracity: NEEDED proof block (speed hint)
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            // Veracity: NEEDED proof block
            let last = inner.last();
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            last
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST split + max_key
        // Veracity: NEEDED proof block
        fn previous(&self, k: &T) -> (predecessor: Option<T>)

        {
            // Veracity: NEEDED proof block (speed hint)
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let predecessor = inner.previous(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            predecessor
        // Veracity: NEEDED proof block
        }

        fn next(&self, k: &T) -> (successor: Option<T>)

        {
            // Veracity: NEEDED proof block
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let successor = inner.next(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            successor
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST split
        fn split(&mut self, k: &T) -> (split: (Self, bool, Self)) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            proof { assume(locked_val@.len() + 1 < usize::MAX as nat); } // RWLOCK_GHOST
            let (left, found, right) = locked_val.split(k);
// Veracity: UNNEEDED proof block             // Release with empty to satisfy inv (empty is wf by construction).
            let empty_val = OrderedSetStEph::empty();
            self.ghost_locked_set = Ghost(empty_val@);
            write_handle.release_write(empty_val);
            proof {
                assume(left.spec_orderedsetsteph_wf());
                assume(right.spec_orderedsetsteph_wf());
            }
            (from_st(left), found, from_st(right))
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- RwLock wrapper, BST union
        fn join(&mut self, other: Self) {
            // Use acquire_write on other to get inv-guaranteed wf (no clone needed).
            let (other_inner, other_write) = other.locked_set.acquire_write();
            // other_inner.spec_orderedsetsteph_wf() from RwLock inv.
            let empty_other = OrderedSetStEph::empty();
            // Veracity: NEEDED proof block
            other_write.release_write(empty_other);
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            proof { assume(locked_val@.len() + other_inner@.len() < usize::MAX as nat); } // RWLOCK_GHOST
            locked_val.join(other_inner);
            let ghost new_val = locked_val@;
            self.ghost_locked_set = Ghost(new_val);
            write_handle.release_write(locked_val);
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST splits
        // Veracity: NEEDED proof block
        fn get_range(&self, k1: &T, k2: &T) -> (range: Result<Self, ()>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            if inner.size() + 1 >= usize::MAX {
                read_handle.release_read();
                return Err(());
            }
            // Veracity: NEEDED proof block
            let range = inner.get_range(k1, k2);
            read_handle.release_read();
            proof { assume(range.spec_orderedsetsteph_wf()); }
            Ok(from_st(range))
        // Veracity: NEEDED proof block
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, BST split + size
        fn rank(&self, k: &T) -> (rank: usize)

        {
            proof { use_type_invariant(self); }
            // Veracity: NEEDED proof block
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let rank = inner.rank(k);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            // Veracity: NEEDED proof block
            rank
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, augmented BST
        fn select(&self, i: usize) -> (selected: Option<T>)

        {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            // RwLock inv provides inner.spec_orderedsetsteph_wf() which now includes sorted.
            let selected = inner.select(i);
            proof { assume(inner@ =~= self@); }
            read_handle.release_read();
            selected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- RwLock wrapper, select + BST split
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self)) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            proof { assume(locked_val@.len() + 1 < usize::MAX as nat); } // RWLOCK_GHOST
            let (left, right) = locked_val.split_rank(i);
            // Release with empty to satisfy inv (empty is wf by construction).
            let empty_val = OrderedSetStEph::empty();
            self.ghost_locked_set = Ghost(empty_val@);
            write_handle.release_write(empty_val);
            proof {
                assume(left.spec_orderedsetsteph_wf());
                assume(right.spec_orderedsetsteph_wf());
            }
            (from_st(left), from_st(right))
        }
    }

    //		Section 11a. top level coarse locking


    impl<T: MtKey + TotalOrder + 'static> RwLockPredicate<OrderedSetStEph<T>> for OrderedSetMtEphInv {
        open spec fn inv(self, v: OrderedSetStEph<T>) -> bool {
            v.spec_orderedsetsteph_wf()
        }
    }
    } // verus!

    //		Section 13. macros


    /// Macro for creating ordered sets from literals.
    #[macro_export]
    macro_rules! OrderedSetMtEphLit {
        ($($x:expr),* $(,)?) => {
            {
                let mut set = $crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEph::empty();
                $(
                    set.insert($x);
                )*
                set
            }
        };
    }

    //		Section 14. derive impls outside verus!

    use std::fmt;

    //		Section 14a. derive impls outside verus!

    impl fmt::Debug for OrderedSetMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEphInv")
        }
    }

    impl fmt::Display for OrderedSetMtEphInv {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEphInv")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: MtKey + TotalOrder + 'static> fmt::Debug for OrderedSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEph(size: {})", self.size())
        }
    }

    impl<T: MtKey + TotalOrder + 'static> fmt::Display for OrderedSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEph(size: {})", self.size())
        }
    }
}
