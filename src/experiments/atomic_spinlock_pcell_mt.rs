//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: AtomicBool spinlock + PCell for Mt module.
//!
//! Same 10-operation Vec-backed set as bst_plain_mt_tsm.rs, but using AtomicBool
//! spinlock + PCell instead of RwLock + TSM. The PointsTo proof token flows through
//! the atomic invariant — no TSM, no RwLock, no accepts.
//!
//! Architecture:
//! - PCell holds SetInner (the data).
//! - AtomicBool acts as a spinlock. Its ghost state holds Option<PointsTo<SetInner>>.
//! - When unlocked (false): PointsTo stored in atomic ghost state.
//! - When locked (true): PointsTo taken by the acquiring thread.
//! - struct_with_invariants! ties the atomic to the cell via PointsTo.id() == cell.id().
//!
//! What this proves vs bst_plain_mt_tsm.rs:
//! - Zero assumes, zero accepts — PointsTo IS the proof.
//! - No TSM — the atomic invariant handles everything.
//! - No RwLock — AtomicBool spinlock is simpler.
//! - Same operations, same inner type, same functional specs.
//!
//! View question: all operations take &self. With &self, there is no exclusive
//! ownership to keep a ghost field in sync. A ghost_count field would go stale
//! under concurrent access. Conclusion: View via ghost field requires &mut self
//! for writes (or accept). This experiment uses Approach B (specs on return
//! values only, no View on the outer struct).
//!
//! DO NOT register in lib.rs — experiments stay commented out.

#![cfg_attr(verus_keep_ghost, verifier::exec_allows_no_decreases_clause)]

pub mod atomic_spinlock_pcell_mt {

    use vstd::prelude::*;
    use vstd::atomic_ghost::*;
    use vstd::cell;
    use vstd::cell::*;
    use vstd::modes::*;

    verus! {

    // Inner sequential type — simple Vec-based set (no duplicates).
    // Copied from bst_plain_mt_tsm.rs for standalone compliance.
    pub struct SetInner {
        pub elements: Vec<u64>,
    }

    impl SetInner {
        pub fn empty() -> (s: Self)
            ensures s.elements@.len() == 0,
        {
            SetInner { elements: Vec::new() }
        }

        pub fn size(&self) -> (n: usize)
            ensures n == self.elements@.len(),
        {
            self.elements.len()
        }

        pub fn is_empty(&self) -> (b: bool)
            ensures b == (self.elements@.len() == 0),
        {
            self.elements.len() == 0
        }

        fn index_of(&self, val: &u64) -> (idx: Option<usize>)
            ensures
                idx.is_some() ==> idx.unwrap() < self.elements@.len()
                    && self.elements@[idx.unwrap() as int] == *val,
                idx.is_none() ==> !self.elements@.contains(*val),
        {
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    i <= self.elements@.len(),
                    forall|j: int| 0 <= j < i ==> self.elements@[j] != *val,
                decreases self.elements@.len() - i,
            {
                if self.elements[i] == *val {
                    return Some(i);
                }
                i += 1;
            }
            None
        }

        pub fn contains(&self, val: &u64) -> (found: bool)
            ensures found == self.elements@.contains(*val),
        {
            self.index_of(val).is_some()
        }

        pub fn insert(&mut self, val: u64) -> (inserted: bool)
            ensures
                inserted ==> self.elements@.len() == old(self).elements@.len() + 1,
                !inserted ==> self.elements@.len() == old(self).elements@.len(),
        {
            if self.contains(&val) {
                false
            } else {
                self.elements.push(val);
                true
            }
        }

        pub fn delete(&mut self, val: &u64) -> (deleted: bool)
            ensures
                deleted ==> self.elements@.len() == old(self).elements@.len() - 1,
                !deleted ==> self.elements@.len() == old(self).elements@.len(),
        {
            match self.index_of(val) {
                None => false,
                Some(idx) => {
                    self.elements.swap_remove(idx);
                    true
                }
            }
        }

        pub fn find(&self, val: &u64) -> (found: Option<u64>)
            ensures
                found.is_some() == self.elements@.contains(*val),
                found.is_some() ==> found.unwrap() == *val,
        {
            if self.contains(val) {
                Some(*val)
            } else {
                None
            }
        }

        pub fn minimum(&self) -> (min: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> min.is_none(),
                self.elements@.len() > 0 ==> min.is_some(),
        {
            if self.elements.len() == 0 {
                None
            } else {
                let mut min_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                {
                    if self.elements[i] < min_val {
                        min_val = self.elements[i];
                    }
                    i += 1;
                }
                Some(min_val)
            }
        }

        pub fn maximum(&self) -> (max: Option<u64>)
            ensures
                self.elements@.len() == 0 ==> max.is_none(),
                self.elements@.len() > 0 ==> max.is_some(),
        {
            if self.elements.len() == 0 {
                None
            } else {
                let mut max_val = self.elements[0];
                let mut i: usize = 1;
                while i < self.elements.len()
                    invariant 1 <= i <= self.elements@.len(),
                    decreases self.elements@.len() - i,
                {
                    if self.elements[i] > max_val {
                        max_val = self.elements[i];
                    }
                    i += 1;
                }
                Some(max_val)
            }
        }
    }

    // AtomicBool spinlock + PCell. PointsTo flows through the atomic invariant.
    struct_with_invariants!{
        pub struct SetMtAtomic {
            pub atomic: AtomicBool<_, Option<cell::PointsTo<SetInner>>, _>,
            pub cell: PCell<SetInner>,
        }

        spec fn wf(self) -> bool {
            invariant on atomic with (cell) is (v: bool, g: Option<cell::PointsTo<SetInner>>) {
                match g {
                    None => v == true,  // locked — PointsTo taken by a thread
                    Some(points_to) => {
                        points_to.id() == cell.id()
                        && points_to.is_init()
                        && v == false     // unlocked — PointsTo stored here
                    }
                }
            }
        }
    }

    impl SetMtAtomic {
        /// Construct with an empty set.
        fn new_empty() -> (s: Self)
            ensures s.wf(),
        {
            let inner = SetInner::empty();
            let (cell, Tracked(cell_perm)) = PCell::new(inner);
            let atomic = AtomicBool::new(Ghost(cell), false, Tracked(Some(cell_perm)));
            SetMtAtomic { atomic, cell }
        }

        /// Spinlock acquire: CAS false→true, take PointsTo from atomic ghost state.
        fn acquire(&self) -> (points_to: Tracked<cell::PointsTo<SetInner>>)
            requires self.wf(),
            ensures points_to@.id() == self.cell.id(), points_to@.is_init(),
        {
            loop
                invariant self.wf(),
            {
                let tracked mut points_to_opt = None;
                let res = atomic_with_ghost!(&self.atomic => compare_exchange(false, true);
                    ghost points_to_inv => {
                        tracked_swap(&mut points_to_opt, &mut points_to_inv);
                    }
                );
                if res.is_ok() {
                    return Tracked(points_to_opt.tracked_unwrap());
                }
            }
        }

        /// Spinlock release: store false, return PointsTo to atomic ghost state.
        fn release(&self, points_to: Tracked<cell::PointsTo<SetInner>>)
            requires
                self.wf(),
                points_to@.id() == self.cell.id(), points_to@.is_init(),
        {
            atomic_with_ghost!(&self.atomic => store(false);
                ghost points_to_inv => {
                    points_to_inv = Some(points_to.get());
                }
            );
        }

        // Write operations: acquire, take from PCell, mutate, put back, release.

        fn mt_insert(&self, val: u64)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let mut inner = self.cell.take(Tracked(&mut perm));
            inner.insert(val);
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
        }

        fn mt_delete(&self, val: &u64)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let mut inner = self.cell.take(Tracked(&mut perm));
            inner.delete(val);
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
        }

        // Read operations: acquire, take from PCell, read, put back, release.
        // Uses take/put rather than borrow to avoid tracked lifetime complexity.

        fn mt_contains(&self, val: &u64) -> (found: bool)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let found = inner.contains(val);
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            found
        }

        fn mt_size(&self) -> (n: usize)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let n = inner.size();
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            n
        }

        fn mt_is_empty(&self) -> (b: bool)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let b = inner.is_empty();
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            b
        }

        fn mt_find(&self, val: &u64) -> (found: Option<u64>)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let found = inner.find(val);
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            found
        }

        fn mt_minimum(&self) -> (min: Option<u64>)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let min = inner.minimum();
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            min
        }

        fn mt_maximum(&self) -> (max: Option<u64>)
            requires self.wf(),
        {
            let tracked_perm = self.acquire();
            let tracked mut perm = tracked_perm.get();
            let inner = self.cell.take(Tracked(&mut perm));
            let max = inner.maximum();
            self.cell.put(Tracked(&mut perm), inner);
            self.release(Tracked(perm));
            max
        }
    }

    } // verus!

    #[test]
    fn test_set_mt_atomic_spinlock() {
        let s = SetMtAtomic::new_empty();
        assert_eq!(s.mt_size(), 0);
        assert!(s.mt_is_empty());

        s.mt_insert(42);
        assert_eq!(s.mt_size(), 1);
        assert!(!s.mt_is_empty());
        assert!(s.mt_contains(&42));
        assert!(!s.mt_contains(&99));

        s.mt_insert(10);
        s.mt_insert(99);
        assert_eq!(s.mt_size(), 3);

        // Dup insert — no change.
        s.mt_insert(42);
        assert_eq!(s.mt_size(), 3);

        assert_eq!(s.mt_find(&42), Some(42));
        assert_eq!(s.mt_find(&7), None);
        assert_eq!(s.mt_minimum(), Some(10));
        assert_eq!(s.mt_maximum(), Some(99));

        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);
        assert!(!s.mt_contains(&42));

        // Missing delete — no change.
        s.mt_delete(&42);
        assert_eq!(s.mt_size(), 2);

        assert_eq!(s.mt_minimum(), Some(10));
        assert_eq!(s.mt_maximum(), Some(99));
    }
}
