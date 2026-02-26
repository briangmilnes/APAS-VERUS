//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Optimal Binary Search Tree - ephemeral, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod OptBinSearchTreeMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap30::Probability::Probability::{Probability, ProbabilityTrait};
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(T)]
    pub struct KeyProb<T: MtVal> {
        pub key: T,
        pub prob: Probability,
    }

    impl<T: MtVal> Clone for KeyProb<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            KeyProb { key: self.key.clone(), prob: self.prob }
        }
    }

        pub struct ObstEphKeysWf;
        impl<T: MtVal> RwLockPredicate<Vec<KeyProb<T>>> for ObstEphKeysWf {
            open spec fn inv(self, v: Vec<KeyProb<T>>) -> bool {
                v@.len() <= usize::MAX as nat
            }
        }
        #[verifier::external_body]
        fn new_obst_eph_keys_lock<T: MtVal>(val: Vec<KeyProb<T>>) -> (lock: RwLock<Vec<KeyProb<T>>, ObstEphKeysWf>)
            requires val@.len() <= usize::MAX as nat
        {
            RwLock::new(val, Ghost(ObstEphKeysWf))
        }

        pub struct spec_optbinsearchtreemteph_memo_wf;
        impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, Probability>> for spec_optbinsearchtreemteph_memo_wf {
            open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> bool {
                v@.dom().finite()
            }
        }
        #[verifier::external_body]
        fn new_obst_eph_memo_lock(val: HashMapWithViewPlus<Pair<usize, usize>, Probability>) -> (lock: RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, spec_optbinsearchtreemteph_memo_wf>)
            requires val@.dom().finite()
        {
            RwLock::new(val, Ghost(spec_optbinsearchtreemteph_memo_wf))
        }

    /// Ephemeral multi-threaded optimal binary search tree solver using parallel dynamic programming
    #[verifier::reject_recursive_types(T)]
    pub struct OBSTMtEphS<T: MtVal> {
        pub keys: Arc<RwLock<Vec<KeyProb<T>>, ObstEphKeysWf>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, Probability>, spec_optbinsearchtreemteph_memo_wf>>,
    }

    impl<T: MtVal> Clone for OBSTMtEphS<T> {
        #[verifier::external_body]
        fn clone(&self) -> Self {
            OBSTMtEphS {
                keys: self.keys.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    // 8. traits
    pub trait OBSTMtEphTrait<T: MtVal>: Sized {
        fn new() -> (result: Self);
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self);
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self);
        fn optimal_cost(&mut self) -> (result: Probability) where T: Send + Sync + 'static;
        fn keys(&self) -> (result: Vec<KeyProb<T>>);
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>);
        fn update_prob(&mut self, index: usize, prob: Probability);
        fn num_keys(&self) -> (result: usize);
        fn clear_memo(&mut self);
        fn memo_size(&self) -> (result: usize);
    }

    // 9. impls

    #[verifier::external_body]
    fn parallel_min_reduction<T: MtVal>(table: &OBSTMtEphS<T>, costs: Vec<Probability>) -> (result: Probability) {
        if costs.is_empty() {
            return Probability::infinity();
        }
        if costs.len() == 1 {
            return costs[0];
        }

        let mid = costs.len() / 2;
        let left_costs = costs[..mid].to_vec();
        let right_costs = costs[mid..].to_vec();

        let table_clone1 = table.clone();
        let table_clone2 = table.clone();

        let handle1 = thread::spawn(move || parallel_min_reduction(&table_clone1, left_costs));
        let handle2 = thread::spawn(move || parallel_min_reduction(&table_clone2, right_costs));

        let left_min = handle1.join().unwrap();
        let right_min = handle2.join().unwrap();

        std::cmp::min(left_min, right_min)
    }

    #[verifier::external_body]
    fn obst_rec<T: MtVal + Send + Sync + 'static>(table: &OBSTMtEphS<T>, i: usize, l: usize) -> (result: Probability) {
        {
            let handle = table.memo.acquire_read();
            let cached = handle.borrow().get(&Pair(i, l)).copied();
            handle.release_read();
            if let Some(result) = cached {
                return result;
            }
        }

        let result = if l == 0 {
            Probability::zero()
        } else {
            let prob_sum = {
                let handle = table.keys.acquire_read();
                let keys = handle.borrow();
                let sum = (0..l)
                    .map(|k| keys[i + k].prob)
                    .fold(Probability::zero(), |acc, p| acc + p);
                handle.release_read();
                sum
            };

            let costs = (0..l)
                .map(|k| {
                    let left_cost = obst_rec(table, i, k);
                    let right_cost = obst_rec(table, i + k + 1, l - k - 1);
                    left_cost + right_cost
                }).collect::<Vec<Probability>>();

            let min_cost = parallel_min_reduction(table, costs);

            prob_sum + min_cost
        };

        {
            let (mut memo, write_handle) = table.memo.acquire_write();
            memo.insert(Pair(i, l), result);
            write_handle.release_write(memo);
        }

        result
    }

    impl<T: MtVal> OBSTMtEphTrait<T> for OBSTMtEphS<T> {
        #[verifier::external_body]
        fn new() -> (result: Self) {
            Self {
                keys: Arc::new(new_obst_eph_keys_lock(Vec::new())),
                memo: Arc::new(new_obst_eph_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_keys_probs(keys: Vec<T>, probs: Vec<Probability>) -> (result: Self) {
            let key_probs = keys
                .into_iter()
                .zip(probs)
                .map(|(key, prob)| KeyProb { key, prob }).collect::<Vec<KeyProb<T>>>();
            Self {
                keys: Arc::new(new_obst_eph_keys_lock(key_probs)),
                memo: Arc::new(new_obst_eph_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn from_key_probs(key_probs: Vec<KeyProb<T>>) -> (result: Self) {
            Self {
                keys: Arc::new(new_obst_eph_keys_lock(key_probs)),
                memo: Arc::new(new_obst_eph_memo_lock(HashMapWithViewPlus::new())),
            }
        }

        #[verifier::external_body]
        fn optimal_cost(&mut self) -> (result: Probability) where T: Send + Sync + 'static {
            let keys_len = {
                let handle = self.keys.acquire_read();
                let len = handle.borrow().len();
                handle.release_read();
                len
            };
            if keys_len == 0 { return Probability::zero(); }
            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }
            obst_rec(self, 0, keys_len)
        }

        #[verifier::external_body]
        fn keys(&self) -> (result: Vec<KeyProb<T>>) {
            let handle = self.keys.acquire_read();
            let keys = handle.borrow().clone();
            handle.release_read();
            keys
        }

        #[verifier::external_body]
        fn set_key_prob(&mut self, index: usize, key_prob: KeyProb<T>) {
            {
                let (mut keys, write_handle) = self.keys.acquire_write();
                keys[index] = key_prob;
                write_handle.release_write(keys);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        #[verifier::external_body]
        fn update_prob(&mut self, index: usize, prob: Probability) {
            {
                let (mut keys, write_handle) = self.keys.acquire_write();
                keys[index].prob = prob;
                write_handle.release_write(keys);
            }
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        #[verifier::external_body]
        fn num_keys(&self) -> (result: usize) {
            let handle = self.keys.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }

        #[verifier::external_body]
        fn clear_memo(&mut self) {
            let (mut memo, write_handle) = self.memo.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        #[verifier::external_body]
        fn memo_size(&self) -> (result: usize) {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls in verus!
    impl<T: MtVal> PartialEq for OBSTMtEphS<T> {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> bool {
            let self_handle = self.keys.acquire_read();
            let other_handle = other.keys.acquire_read();
            let result = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            result
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
