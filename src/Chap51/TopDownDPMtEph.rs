//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Top-Down Dynamic Programming - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using concurrent HashMap with in-place mutations for thread-safe subproblem caching.

//  Table of Contents

pub mod TopDownDPMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use std::collections::HashMap;
    use std::fmt::{Formatter, Debug, Display};
    use std::sync::Arc;
    use vstd::rwlock::*;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {
    // 4. type definitions
    pub struct TopDownDPMtEphS {
        pub seq_s: ArraySeqMtEphS<char>,
        pub seq_t: ArraySeqMtEphS<char>,
    }

    /// Memo table context for thread-safe memoization.
    pub struct TopDownDPMtEphInv {
        pub ghost s_len: nat,
        pub ghost t_len: nat,
    }

    // 6. spec fns
    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    // 8. traits
    pub trait TopDownDPMtEphTrait: Sized {
        spec fn spec_s(&self) -> Seq<char>;
        spec fn spec_t(&self) -> Seq<char>;
        spec fn spec_s_len(&self) -> nat;
        spec fn spec_t_len(&self) -> nat;
        spec fn spec_med(&self, i: nat, j: nat) -> nat;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> (dp: Self)
            ensures
                dp.spec_s() == s@,
                dp.spec_t() == t@,
                dp.spec_s_len() == s.spec_len(),
                dp.spec_t_len() == t.spec_len();

        fn s_length(&self) -> (len: usize)
            ensures len as nat == self.spec_s_len();

        fn t_length(&self) -> (len: usize)
            ensures len as nat == self.spec_t_len();

        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self.spec_s_len() == 0 && self.spec_t_len() == 0);

        fn set_s(&mut self, s: ArraySeqMtEphS<char>)
            ensures
                self.spec_s() == s@,
                self.spec_t() == old(self).spec_t();

        fn set_t(&mut self, t: ArraySeqMtEphS<char>)
            ensures
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == t@;

        fn med_memoized_concurrent(&mut self) -> (distance: usize)
            requires old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
            ensures
                distance as nat == old(self).spec_med(
                    old(self).spec_s_len(),
                    old(self).spec_t_len()
                ),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t();

        fn med_memoized_parallel(&mut self) -> (distance: usize)
            requires old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
            ensures
                distance as nat == old(self).spec_med(
                    old(self).spec_s_len(),
                    old(self).spec_t_len()
                ),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t();
    }

    /// std::collections::HashMap has no View trait in vstd, so the invariant
    /// cannot constrain map contents. Ghost fields carry context for documentation.
    impl RwLockPredicate<HashMap<(usize, usize), usize>> for TopDownDPMtEphInv {
        open spec fn inv(self, v: HashMap<(usize, usize), usize>) -> bool { true }
    }


    // 9. impls
    impl TopDownDPMtEphTrait for TopDownDPMtEphS {
        open spec fn spec_s(&self) -> Seq<char> { self.seq_s@ }
        open spec fn spec_t(&self) -> Seq<char> { self.seq_t@ }
        open spec fn spec_s_len(&self) -> nat { self.seq_s.spec_len() }
        open spec fn spec_t_len(&self) -> nat { self.seq_t.spec_len() }

        open spec fn spec_med(&self, i: nat, j: nat) -> nat
            decreases i + j,
        {
            if i == 0 { j }
            else if j == 0 { i }
            else if self.seq_s@[i as int - 1] == self.seq_t@[j as int - 1] {
                self.spec_med((i - 1) as nat, (j - 1) as nat)
            } else {
                let del = self.spec_med((i - 1) as nat, j);
                let ins = self.spec_med(i, (j - 1) as nat);
                1 + spec_min(del, ins)
            }
        }

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j,
            decreases i + j,
        {
            if i == 0 || j == 0 {
            } else if self.seq_s@[i as int - 1] == self.seq_t@[j as int - 1] {
                self.lemma_spec_med_bounded((i - 1) as nat, (j - 1) as nat);
            } else {
                self.lemma_spec_med_bounded((i - 1) as nat, j);
                self.lemma_spec_med_bounded(i, (j - 1) as nat);
            }
        }

        fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> (dp: Self) {
            TopDownDPMtEphS { seq_s: s, seq_t: t }
        }

        fn s_length(&self) -> (len: usize) { self.seq_s.length() }
        fn t_length(&self) -> (len: usize) { self.seq_t.length() }

        fn is_empty(&self) -> (empty: bool) {
            let s_empty = self.seq_s.length() == 0;
            let t_empty = self.seq_t.length() == 0;
            s_empty && t_empty
        }

        fn set_s(&mut self, s: ArraySeqMtEphS<char>) { self.seq_s = s; }
        fn set_t(&mut self, t: ArraySeqMtEphS<char>) { self.seq_t = t; }

        /// Compute MED using concurrent top-down memoization (Algorithm 51.4).
        #[verifier::external_body]
        fn med_memoized_concurrent(&mut self) -> (distance: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let mut memo = HashMap::new();
            med_recursive_concurrent(&self.seq_s, &self.seq_t, s_len, t_len, &mut memo)
        }

        /// Compute MED with parallel subproblem exploration.
        #[verifier::external_body]
        fn med_memoized_parallel(&mut self) -> (distance: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let memo: Arc<RwLock<HashMap<(usize, usize), usize>, TopDownDPMtEphInv>> = new_arc_rwlock(HashMap::new(), Ghost(TopDownDPMtEphInv { s_len: s_len as nat, t_len: t_len as nat }));
            med_recursive_parallel(&self.seq_s, &self.seq_t, s_len, t_len, &memo)
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for TopDownDPMtEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@
        }
    }

    impl Default for TopDownDPMtEphS {
        fn default() -> (dp: Self)
            ensures
                dp.spec_s_len() == 0,
                dp.spec_t_len() == 0,
        {
            let empty_s = ArraySeqMtEphS::<char>::empty();
            let empty_t = ArraySeqMtEphS::<char>::empty();
            Self::new(empty_s, empty_t)
        }
    }

    // 11. derive impls in verus!
    impl Clone for TopDownDPMtEphS {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.seq_s@ == self.seq_s@,
                cloned.seq_t@ == self.seq_t@,
        {
            TopDownDPMtEphS {
                seq_s: self.seq_s.clone(),
                seq_t: self.seq_t.clone(),
            }
        }
    }

    impl PartialEq for TopDownDPMtEphS {
        fn eq(&self, other: &Self) -> (eq: bool)
            ensures eq == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)
        {
            let r = self.seq_s == other.seq_s && self.seq_t == other.seq_t;
            proof { accept(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for TopDownDPMtEphS {}

    } // verus!

    // 13. derive impls outside verus!

    /// Sequential recursive MED with memoization.
    fn med_recursive_concurrent(
        seq_s: &ArraySeqMtEphS<char>,
        seq_t: &ArraySeqMtEphS<char>,
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }

        let result = match (i, j) {
            | (0, j) => j,
            | (i, 0) => i,
            | (i, j) => {
                let s_char = seq_s.nth(i - 1).clone();
                let t_char = seq_t.nth(j - 1).clone();

                if s_char == t_char {
                    med_recursive_concurrent(seq_s, seq_t, i - 1, j - 1, memo)
                } else {
                    let insert_cost = 1 + med_recursive_concurrent(seq_s, seq_t, i, j - 1, memo);
                    let delete_cost = 1 + med_recursive_concurrent(seq_s, seq_t, i - 1, j, memo);
                    insert_cost.min(delete_cost)
                }
            }
        };

        memo.insert((i, j), result);
        result
    }

    /// Parallel recursive MED with thread-safe memoization.
    fn med_recursive_parallel(
        seq_s: &ArraySeqMtEphS<char>,
        seq_t: &ArraySeqMtEphS<char>,
        i: usize,
        j: usize,
        memo: &Arc<RwLock<HashMap<(usize, usize), usize>, TopDownDPMtEphInv>>,
    ) -> usize {
        {
            let read_handle = memo.acquire_read();
            let cached = read_handle.borrow().get(&(i, j)).copied();
            read_handle.release_read();
            if let Some(cached_result) = cached {
                return cached_result;
            }
        }

        let result = match (i, j) {
            | (0, j) => j,
            | (i, 0) => i,
            | (i, j) => {
                let s_char = seq_s.nth(i - 1).clone();
                let t_char = seq_t.nth(j - 1).clone();

                if s_char == t_char {
                    med_recursive_parallel(seq_s, seq_t, i - 1, j - 1, memo)
                } else {
                    let s_clone1 = seq_s.clone();
                    let t_clone1 = seq_t.clone();
                    let memo_clone1 = memo.clone();
                    let s_clone2 = seq_s.clone();
                    let t_clone2 = seq_t.clone();
                    let memo_clone2 = memo.clone();

                    let f1 = move || {
                        1 + med_recursive_parallel(&s_clone1, &t_clone1, i, j - 1, &memo_clone1)
                    };
                    let f2 = move || {
                        1 + med_recursive_parallel(&s_clone2, &t_clone2, i - 1, j, &memo_clone2)
                    };
                    let (insert_cost, delete_cost) = join(f1, f2);
                    insert_cost.min(delete_cost)
                }
            }
        };

        {
            let (mut current, write_handle) = memo.acquire_write();
            current.insert((i, j), result);
            write_handle.release_write(current);
        }
        result
    }
    // 13. derive impls outside verus!
    impl Debug for TopDownDPMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPMtEphInv").finish()
        }
    }

    impl Display for TopDownDPMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "TopDownDPMtEphInv")
        }
    }

    impl Debug for TopDownDPMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPMtEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for TopDownDPMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPMtEph(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
