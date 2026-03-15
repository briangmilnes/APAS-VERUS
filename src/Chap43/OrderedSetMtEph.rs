//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered set using coarse RwLock over OrderedSetStEph.

pub mod OrderedSetMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 13. derive impls outside verus!
    // 12. macros

    // 2. imports

    use vstd::prelude::*;
    use vstd::rwlock::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap43::OrderedSetStEph::OrderedSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::set::group_set_axioms,
        vstd::set_lib::group_set_lib_default,
    };

    // 4. type definitions

    pub struct OrderedSetMtEphInv;

    impl<T: MtKey + 'static> RwLockPredicate<OrderedSetStEph<T>> for OrderedSetMtEphInv {
        open spec fn inv(self, v: OrderedSetStEph<T>) -> bool {
            v.spec_orderedsetsteph_wf()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetMtEph<T: MtKey + 'static> {
        pub(crate) locked_set: RwLock<OrderedSetStEph<T>, OrderedSetMtEphInv>,
        pub(crate) ghost_locked_set: Ghost<Set<<T as View>::V>>,
    }

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    impl<T: MtKey + 'static> OrderedSetMtEph<T> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.ghost_locked_set@.finite()
        }

        pub closed spec fn spec_ghost_locked_set(self) -> Set<<T as View>::V> {
            self.ghost_locked_set@
        }
    }

    // Helper: construct Mt wrapper from St set (used by split/get_range/split_rank/from_seq).
    fn from_st<T: MtKey + 'static>(inner: OrderedSetStEph<T>) -> (s: OrderedSetMtEph<T>)
        requires inner.spec_orderedsetsteph_wf(), inner@.finite()
        ensures s@ == inner@, s@.finite()
    {
        let ghost view = inner@;
        OrderedSetMtEph {
            locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
            ghost_locked_set: Ghost(view),
        }
    }

    // 5. view impls

    impl<T: MtKey + 'static> View for OrderedSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> {
            self.spec_ghost_locked_set()
        }
    }

    // 8. traits

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics.
    pub trait OrderedSetMtEphTrait<T: MtKey + 'static>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetmteph_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.len(), self@.finite();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetmteph_wf();
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree@.finite(), tree.spec_orderedsetmteph_wf();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: B)
            ensures found == self@.contains(x@);
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn insert(&mut self, x: T)
            ensures self@ == old(self)@.insert(x@), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn delete(&mut self, x: &T)
            ensures self@ == old(self)@.remove(x@), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
            requires
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&mut self, other: &Self)
            ensures self@ == old(self)@.intersect(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&mut self, other: &Self)
            ensures self@ == old(self)@.union(other@), self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&mut self, other: &Self)
            ensures self@ == old(self)@.difference(other@), self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn to_seq(&self) -> (seq: ArraySeqStPerS<T>)
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// claude-4-sonet: Work Θ(n lg n), Span Θ(lg n), Parallelism Θ(1)
        fn from_seq(seq: ArraySeqStPerS<T>) -> (constructed: Self)
            ensures constructed@.finite();

        // Ordering operations (ADT 43.1) - sequential (inherently sequential on trees)
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn first(&self) -> (first: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn last(&self) -> (last: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn next(&self, k: &T) -> (successor: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split(&mut self, k: &T) -> (split: (Self, B, Self))
            where Self: Sized
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn join(&mut self, other: Self)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn rank(&self, k: &T) -> (rank: usize)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn select(&self, i: usize) -> (selected: Option<T>)
            ensures self@.finite();
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            ensures self@.finite();
    }

    // 9. impls

    impl<T: MtKey + 'static> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        open spec fn spec_orderedsetmteph_wf(&self) -> bool {
            self@.finite()
        }

        fn size(&self) -> (count: usize) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let count = inner.size();
            proof { assume(count == self@.len()); }
            read_handle.release_read();
            count
        }

        fn empty() -> (empty: Self) {
            let inner = OrderedSetStEph::empty();
            let ghost view = inner@;
            OrderedSetMtEph {
                locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
                ghost_locked_set: Ghost(view),
            }
        }

        fn singleton(x: T) -> (tree: Self) {
            let inner = OrderedSetStEph::singleton(x);
            let ghost view = inner@;
            OrderedSetMtEph {
                locked_set: RwLock::new(inner, Ghost(OrderedSetMtEphInv)),
                ghost_locked_set: Ghost(view),
            }
        }

        fn find(&self, x: &T) -> (found: B) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let found = inner.find(x);
            proof { assume(found == self@.contains(x@)); }
            read_handle.release_read();
            found
        }

        fn insert(&mut self, x: T) {
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost x_view = x@;
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.insert(x);
            write_handle.release_write(locked_val);
            self.ghost_locked_set = Ghost(old_view.insert(x_view));
        }

        fn delete(&mut self, x: &T) {
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost x_view = x@;
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.delete(x);
            write_handle.release_write(locked_val);
            self.ghost_locked_set = Ghost(old_view.remove(x_view));
        }

        #[verifier::external_body]
        fn filter<F: PredMt<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.filter(f, Ghost(spec_pred));
            self.ghost_locked_set = Ghost(locked_val@);
            write_handle.release_write(locked_val);
        }

        fn intersection(&mut self, other: &Self) {
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.intersection(other_ref);
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_set = Ghost(old_view.intersect(other_view));
        }

        fn union(&mut self, other: &Self) {
            proof { use_type_invariant(&*self); use_type_invariant(other); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.union(other_ref);
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_set = Ghost(old_view.union(other_view));
        }

        fn difference(&mut self, other: &Self) {
            proof { use_type_invariant(&*self); }
            let ghost old_view = self.ghost_locked_set@;
            let ghost other_view = other.ghost_locked_set@;
            let other_read = other.locked_set.acquire_read();
            let other_ref = other_read.borrow();
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.difference(other_ref);
            write_handle.release_write(locked_val);
            other_read.release_read();
            self.ghost_locked_set = Ghost(old_view.difference(other_view));
        }

        #[verifier::external_body]
        fn to_seq(&self) -> (seq: ArraySeqStPerS<T>) {
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let st_seq = inner.to_seq();
            read_handle.release_read();
            use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
            let len = st_seq.length();
            let mut elements: Vec<T> = Vec::new();
            for i in 0..len {
                elements.push(st_seq.nth(i).clone());
            }
            ArraySeqStPerS::from_vec(elements)
        }

        fn from_seq(seq: ArraySeqStPerS<T>) -> (constructed: Self) {
            let len = seq.length();
            let mut inner = OrderedSetStEph::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    inner.spec_orderedsetsteph_wf(),
                    inner@.finite(),
                    i <= len,
                    len as int == seq.spec_len(),
                decreases len - i,
            {
                inner.insert(seq.nth(i).clone());
                i += 1;
            }
            from_st(inner)
        }

        fn first(&self) -> (first: Option<T>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let first = inner.first();
            read_handle.release_read();
            first
        }

        fn last(&self) -> (last: Option<T>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let last = inner.last();
            read_handle.release_read();
            last
        }

        fn previous(&self, k: &T) -> (predecessor: Option<T>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let predecessor = inner.previous(k);
            read_handle.release_read();
            predecessor
        }

        fn next(&self, k: &T) -> (successor: Option<T>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let successor = inner.next(k);
            read_handle.release_read();
            successor
        }

        fn split(&mut self, k: &T) -> (split: (Self, B, Self)) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            let (left, found, right) = locked_val.split(k);
            // Release with empty to satisfy inv (empty is wf by construction).
            let empty_val = OrderedSetStEph::empty();
            self.ghost_locked_set = Ghost(empty_val@);
            write_handle.release_write(empty_val);
            proof {
                assume(left.spec_orderedsetsteph_wf());
                assume(right.spec_orderedsetsteph_wf());
            }
            (from_st(left), found, from_st(right))
        }

        fn join(&mut self, other: Self) {
            // Use acquire_write on other to get inv-guaranteed wf (no clone needed).
            let (other_inner, other_write) = other.locked_set.acquire_write();
            // other_inner.spec_orderedsetsteph_wf() from RwLock inv.
            let empty_other = OrderedSetStEph::empty();
            other_write.release_write(empty_other);
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
            locked_val.join(other_inner);
            let ghost new_val = locked_val@;
            self.ghost_locked_set = Ghost(new_val);
            write_handle.release_write(locked_val);
        }

        fn get_range(&self, k1: &T, k2: &T) -> (range: Self) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let range = inner.get_range(k1, k2);
            read_handle.release_read();
            proof { assume(range.spec_orderedsetsteph_wf()); }
            from_st(range)
        }

        fn rank(&self, k: &T) -> (rank: usize) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let rank = inner.rank(k);
            read_handle.release_read();
            rank
        }

        fn select(&self, i: usize) -> (selected: Option<T>) {
            proof { use_type_invariant(self); }
            let read_handle = self.locked_set.acquire_read();
            let inner = read_handle.borrow();
            let selected = inner.select(i);
            read_handle.release_read();
            selected
        }

        fn split_rank(&mut self, i: usize) -> (split: (Self, Self)) {
            let (mut locked_val, write_handle) = self.locked_set.acquire_write();
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

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<T: MtKey + 'static> fmt::Debug for OrderedSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEph(size: {})", self.size())
        }
    }

    impl<T: MtKey + 'static> fmt::Display for OrderedSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetMtEph(size: {})", self.size())
        }
    }

    // 12. macros

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
}
