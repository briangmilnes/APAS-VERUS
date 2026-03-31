//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - ephemeral, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod OptBinSearchTreeMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 8. traits
// 9. impls
// 11. derive impls in verus!

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
};

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: MtVal> Clone for KeyProb<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned == *self
        {
            let cloned = KeyProb { key: self.key.clone(), prob: self.prob };
            proof { assume(cloned == *self); }
            cloned
        }
    }

        #[verifier::reject_recursive_types(T)]
        pub struct OptBSTMtEphKeysInv<T: MtVal> {
            pub ghost expected_keys: Seq<KeyProb<T>>,
        }
        impl<T: MtVal> RwLockPredicate<Vec<KeyProb<T>>> for OptBSTMtEphKeysInv<T> {
            open spec fn inv(self, v: Vec<KeyProb<T>>) -> bool {
                v@ =~= self.expected_keys
            }
        }

        pub struct OptBSTMtEphMemoInv;
        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, Probability>> for OptBSTMtEphMemoInv {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> bool {
                v@.dom().finite()
            }
        }

    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTMtEphS<T: MtVal> {
        pub keys: Arc<RwLock<Vec<KeyProb<T>>, OptBSTMtEphKeysInv<T>>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, OptBSTMtEphMemoInv>>,
        pub ghost_keys: Ghost<Seq<KeyProb<T>>>,
    }

    // 5. view impls
    #[verifier::reject_recursive_types(T)]
    pub ghost struct OBSTMtEphV<T: MtVal> {
        pub keys: Seq<KeyProb<T>>,
    }

    impl<T: MtVal> View for OBSTMtEphS<T> {
        type V = OBSTMtEphV<T>;
        open spec fn view(&self) -> Self::V {
            OBSTMtEphV { keys: self.ghost_keys@ }
        }
    }

    // 8. traits
    pub trait OBSTMtEphTrait<T: MtVal>: Sized + View<V = OBSTMtEphV<T>> {
        spec fn spec_optbinsearchtreemteph_wf(&self) -> bool;

        fn new() -> (empty: Self)
            ensures empty@.keys.len() == 0, empty.spec_optbinsearchtreemteph_wf();

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self)
            requires keys@.len() == probs@.len(),
            ensures constructed@.keys.len() == keys@.len(), constructed.spec_optbinsearchtreemteph_wf();

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self)
            ensures constructed@.keys =~= key_probs@, constructed.spec_optbinsearchtreemteph_wf();

        fn optimal_cost(&mut self) -> (cost: Probability) where T: Send + Sync + 'static
            requires old(self).spec_optbinsearchtreemteph_wf();

        fn keys(&self) -> (keys: Vec<KeyProb<T>>)
            requires self.spec_optbinsearchtreemteph_wf(),
            ensures keys@ =~= self@.keys;

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>)
            requires index < old(self)@.keys.len(), old(self).spec_optbinsearchtreemteph_wf(),
            ensures
                self@.keys =~= old(self)@.keys.update(index as int, key_prob),
                self.spec_optbinsearchtreemteph_wf();

        fn update_prob(&mut self, index: usize, prob: Probability)
            requires index < old(self)@.keys.len(), old(self).spec_optbinsearchtreemteph_wf(),
            ensures
                self@.keys.len() == old(self)@.keys.len(),
                self.spec_optbinsearchtreemteph_wf();

        fn num_keys(&self) -> (count: usize)
            requires self.spec_optbinsearchtreemteph_wf(),
            ensures count == self@.keys.len();

        fn clear_memo(&mut self)
            requires old(self).spec_optbinsearchtreemteph_wf(),
            ensures self@.keys =~= old(self)@.keys, self.spec_optbinsearchtreemteph_wf();

        fn memo_size(&self) -> (count: usize);
    }

    // 9. impls

    /// - Alg Analysis: APAS (Ch50 Alg 50.2): Work O(n^3), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — DIFFERS: sequential DP table fill, APAS Span O(n lg n) assumes parallel
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtEphS<T>, i: usize, l: usize) -> (cost: Probability)
        requires
            i + l <= table@.keys.len(),
            table.spec_optbinsearchtreemteph_wf(),
        ensures true,
        decreases l,
    {
        // Memo lookup.
        {
            let rwlock = arc_deref(&table.memo);
            let handle = rwlock.acquire_read();
            let cached = match handle.borrow().get(&Pair(i, l)) {
                Some(v) => Some(*v),
                None => None,
            };
            handle.release_read();
            if let Some(cost) = cached {
                return cost;
            }
        }

        let cost = if l == 0 {
            Probability::zero()
        } else {
            // Sum probabilities of keys[i..i+l].
            let keys_rwlock = arc_deref(&table.keys);
            let keys_handle = keys_rwlock.acquire_read();
            let keys_ref = keys_handle.borrow();
            let n = keys_ref.len();
            let mut prob_sum = Probability::zero();
            let mut k: usize = 0;
            while k < l
                invariant
                    k <= l,
                    i + l <= n,
                    n == keys_ref@.len(),
                decreases l - k,
            {
                prob_sum = prob_sum + keys_ref[i + k].prob;
                k = k + 1;
            }
            keys_handle.release_read();

            // Find minimum cost over all split points.
            let mut min_cost = Probability::infinity();
            let mut k: usize = 0;
            while k < l
                invariant
                    k <= l,
                    i + l <= n,
                    i + l <= table@.keys.len(),
                    table.spec_optbinsearchtreemteph_wf(),
                decreases l - k,
            {
                let left_cost = obst_rec(table, i, k);
                let right_cost = obst_rec(table, i + k + 1, l - k - 1);
                let split_cost = left_cost + right_cost;
                if split_cost <= min_cost {
                    min_cost = split_cost;
                }
                k = k + 1;
            }

            prob_sum + min_cost
        };

        // Memo store.
        {
            let rwlock = arc_deref(&table.memo);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.insert(Pair(i, l), cost);
            write_handle.release_write(memo);
        }

        cost
    }

    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        open spec fn spec_optbinsearchtreemteph_wf(&self) -> bool {
            self.keys.pred().expected_keys =~= self.ghost_keys@
        }

        fn new() -> (empty: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(Vec::new(), Ghost(OptBSTMtEphKeysInv { expected_keys: Seq::empty() })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(Seq::empty()),
            }
        }

        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (constructed: Self) {
            let mut key_probs: Vec<KeyProb<T>> = Vec::new();
            let mut idx: usize = 0;
            while idx < keys.len()
                invariant
                    idx <= keys@.len(),
                    keys@.len() == probs@.len(),
                    key_probs@.len() == idx as int,
                decreases keys@.len() - idx,
            {
                key_probs.push(KeyProb { key: keys[idx].clone(), prob: probs[idx] });
                idx += 1;
            }
            let ghost gk = key_probs@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(key_probs, Ghost(OptBSTMtEphKeysInv { expected_keys: gk })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(gk),
            }
        }

        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (constructed: Self) {
            let ghost gk = key_probs@;
            let _len = key_probs.len();
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                keys: new_arc_rwlock(key_probs, Ghost(OptBSTMtEphKeysInv { expected_keys: gk })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(gk),
            }
        }

        fn optimal_cost(&mut self) -> (cost: Probability) where T: Send + Sync + 'static {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let keys_len = handle.borrow().len();
            assert(keys_len == rwlock.pred().expected_keys.len());
            handle.release_read();
            if keys_len == 0 { return Probability::zero(); }
            {
                let memo_arc = self.memo.clone();
                let rwlock = arc_deref(&memo_arc);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }
            obst_rec(self, 0, keys_len)
        }

        fn keys(&self) -> (keys: Vec<KeyProb<T>>) {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            assert(borrowed@ =~= rwlock.pred().expected_keys);
            let keys = borrowed.clone();
            handle.release_read();
            keys
        }

        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            let ghost kp = key_prob;
            let ghost old_keys = self.ghost_keys@;
            let ghost new_keys_ghost = old_keys.update(index as int, kp);
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            assert(borrowed@ =~= rwlock.pred().expected_keys);
            let mut keys = borrowed.clone();
            handle.release_read();
            keys.set(index, key_prob);
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = OBSTMtEphS {
                keys: new_arc_rwlock(keys, Ghost(OptBSTMtEphKeysInv { expected_keys: new_keys_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(new_keys_ghost),
            };
        }

        fn update_prob(&mut self, index: usize, prob: Probability) {
            let ghost old_keys = self.ghost_keys@;
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            assert(borrowed@ =~= rwlock.pred().expected_keys);
            let mut keys = borrowed.clone();
            handle.release_read();
            let new_kp = KeyProb { key: keys[index].key.clone(), prob };
            let ghost new_keys_ghost = old_keys.update(index as int, new_kp);
            keys.set(index, new_kp);
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = OBSTMtEphS {
                keys: new_arc_rwlock(keys, Ghost(OptBSTMtEphKeysInv { expected_keys: new_keys_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(OptBSTMtEphMemoInv)),
                ghost_keys: Ghost(new_keys_ghost),
            };
        }

        fn num_keys(&self) -> (count: usize) {
            let rwlock = arc_deref(&self.keys);
            let handle = rwlock.acquire_read();
            let count = handle.borrow().len();
            assert(count == rwlock.pred().expected_keys.len());
            handle.release_read();
            count
        }

        fn clear_memo(&mut self) {
            let memo_arc = self.memo.clone();
            let rwlock = arc_deref(&memo_arc);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn memo_size(&self) -> (count: usize) {
            let rwlock = arc_deref(&self.memo);
            let handle = rwlock.acquire_read();
            let count = handle.borrow().len();
            handle.release_read();
            count
        }
    }

    // 11. derive impls in verus!
    impl<T: MtVal> Clone for OBSTMtEphS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = OBSTMtEphS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
                ghost_keys: Ghost(self.ghost_keys@),
            };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: MtVal> PartialEqSpecImpl for OBSTMtEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let self_rwlock = arc_deref(&self.keys);
            let other_rwlock = arc_deref(&other.keys);
            let self_handle = self_rwlock.acquire_read();
            let other_handle = other_rwlock.acquire_read();
            let r = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    impl<T: MtVal> Eq for OBSTMtEphS<T> {}

    impl<T: MtVal> Eq for KeyProb<T> {}

    } // verus!

    impl<T: MtVal + PartialEq> PartialEq for KeyProb<T> {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key && (self.prob.value() - other.prob.value()).abs() < f64::EPSILON
        }
    }

    // 13. derive impls outside verus!
    impl<T: MtVal> Debug for OBSTMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl<T: MtVal> Display for OBSTMtEphS<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under read locks
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            let keys_handle = self.keys.acquire_read();
            let keys_len = keys_handle.borrow().len();
            keys_handle.release_read();
            write!(f, "OBSTMtEph(keys: {keys_len}, memo_entries: {memo_size})")
        }
    }

    impl<T: MtVal> IntoIterator for OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec from Arc<RwLock>
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal> IntoIterator for &mut OBSTMtEphS<T> {
        type Item = KeyProb<T>;
        type IntoIter = IntoIter<KeyProb<T>>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys.into_iter()
        }
    }

    impl<T: MtVal + Display> Display for KeyProb<T> {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format key and probability
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "({}: {:.3})", self.key, self.prob) }
    }

    impl<T: MtVal> Debug for KeyProb<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "KeyProb({:?}, {:.3})", self.key, self.prob) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! OBSTMtEphLit {
        (keys: [$($k:expr),* $(,)?], probs: [$($p:expr),* $(,)?]) => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::from_keys_probs(
                vec![$($k),*],
                vec![$(<$crate::Chap30::Probability::Probability::Probability as $crate::Chap30::Probability::Probability::ProbabilityTrait>::new($p)),*]
            )
        };
        () => {
            $crate::Chap50::OptBinSearchTreeMtEph::OptBinSearchTreeMtEph::OBSTMtEphS::new()
        };
    }
}
