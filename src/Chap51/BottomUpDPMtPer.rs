//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Bottom-Up Dynamic Programming - Persistent Multi-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! parallel diagonal pebbling for multi-threaded computation.

//  Table of Contents

pub mod BottomUpDPMtPer {

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
    use std::cmp::{max, min};
    use std::fmt::{Formatter, Debug, Display};
    use std::sync::Arc;
    use vstd::rwlock::*;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::{spawn, wait};
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;

    verus! {
    // 4. type definitions
    pub struct BottomUpDPMtPerS {
        pub seq_s: ArraySeqMtPerS<char>,
        pub seq_t: ArraySeqMtPerS<char>,
    }

    pub struct BottomUpDPMtPerInv {
        pub ghost s_len: nat,
        pub ghost t_len: nat,
    }

    // 6. spec fns
    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    // 8. traits
    pub trait BottomUpDPMtPerTrait: Sized {
        spec fn spec_s(&self) -> Seq<char>;
        spec fn spec_t(&self) -> Seq<char>;
        spec fn spec_s_len(&self) -> nat;
        spec fn spec_t_len(&self) -> nat;
        spec fn spec_med(&self, i: nat, j: nat) -> nat;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> (dp: Self)
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

        fn med_bottom_up_parallel(&self) -> (distance: usize)
            requires self.spec_s_len() + self.spec_t_len() < usize::MAX,
            ensures
                distance as nat == self.spec_med(
                    self.spec_s_len(),
                    self.spec_t_len()
                );
    }

    impl RwLockPredicate<Vec<Vec<usize>>> for BottomUpDPMtPerInv {
        open spec fn inv(self, v: Vec<Vec<usize>>) -> bool {
            &&& v@.len() == self.s_len + 1
            &&& forall|i: int| #![trigger v@[i]]
                0 <= i < v@.len() ==> v@[i]@.len() == self.t_len + 1
        }
    }


    // 9. impls
    impl BottomUpDPMtPerTrait for BottomUpDPMtPerS {
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

        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> (dp: Self) {
            BottomUpDPMtPerS { seq_s: s, seq_t: t }
        }

        fn s_length(&self) -> (len: usize) { self.seq_s.length() }
        fn t_length(&self) -> (len: usize) { self.seq_t.length() }

        fn is_empty(&self) -> (empty: bool) {
            let s_empty = self.seq_s.length() == 0;
            let t_empty = self.seq_t.length() == 0;
            s_empty && t_empty
        }

        /// Compute MED using parallel bottom-up diagonal pebbling (Algorithm 51.1).
        #[verifier::external_body]
        fn med_bottom_up_parallel(&self) -> (distance: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let mut table = vec![vec![0usize; t_len + 1]; s_len + 1];

            for i in 0..=s_len {
                table[i][0] = i;
            }
            for j in 0..=t_len {
                table[0][j] = j;
            }

            let table: Arc<RwLock<Vec<Vec<usize>>, BottomUpDPMtPerInv>> = new_arc_rwlock(table, Ghost(BottomUpDPMtPerInv { s_len: 0, t_len: 0 }));

            for k in 1..=(s_len + t_len) {
                let start = max(1, k.saturating_sub(t_len));
                let end = min(k, s_len);

                let positions: Vec<(usize, usize)> = (start..=end)
                    .filter_map(|i| {
                        let j = k - i;
                        if j > 0 && j <= t_len { Some((i, j)) } else { None }
                    }).collect();

                let tasks: Vec<_> = positions
                    .into_iter()
                    .map(|(i, j)| {
                        let table_clone = Arc::clone(&table);
                        let seq_s_clone = self.seq_s.clone();
                        let seq_t_clone = self.seq_t.clone();

                        spawn(move || {
                            let s_char = *seq_s_clone.nth(i - 1);
                            let t_char = *seq_t_clone.nth(j - 1);

                            let read_handle = table_clone.acquire_read();
                            let val = if s_char == t_char {
                                read_handle.borrow()[i - 1][j - 1]
                            } else {
                                let del = read_handle.borrow()[i - 1][j];
                                let ins = read_handle.borrow()[i][j - 1];
                                1 + min(del, ins)
                            };
                            read_handle.release_read();
                            (i, j, val)
                        })
                    }).collect();

                let results: Vec<(usize, usize, usize)> = tasks
                    .into_iter()
                    .map(|t| wait(t))
                    .collect();

                let (mut current, write_handle) = table.acquire_write();
                for (i, j, val) in results {
                    current[i][j] = val;
                }
                write_handle.release_write(current);
            }

            let read_handle = table.acquire_read();
            let result = read_handle.borrow()[s_len][t_len];
            read_handle.release_read();
            result
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for BottomUpDPMtPerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@
        }
    }

    impl Default for BottomUpDPMtPerS {
        fn default() -> (dp: Self)
            ensures
                dp.spec_s_len() == 0,
                dp.spec_t_len() == 0,
        {
            let empty_s = ArraySeqMtPerS::<char>::empty();
            let empty_t = ArraySeqMtPerS::<char>::empty();
            Self::new(empty_s, empty_t)
        }
    }

    // 11. derive impls in verus!
    impl Clone for BottomUpDPMtPerS {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.seq_s@ == self.seq_s@,
                cloned.seq_t@ == self.seq_t@,
        {
            BottomUpDPMtPerS {
                seq_s: self.seq_s.clone(),
                seq_t: self.seq_t.clone(),
            }
        }
    }

    impl PartialEq for BottomUpDPMtPerS {
        fn eq(&self, other: &Self) -> (eq: bool)
            ensures eq == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)
        {
            let r = self.seq_s == other.seq_s && self.seq_t == other.seq_t;
            proof { accept(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for BottomUpDPMtPerS {}

    } // verus!
    // 13. derive impls outside verus!
    impl Debug for BottomUpDPMtPerInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPMtPerInv").finish()
        }
    }

    impl Display for BottomUpDPMtPerInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "BottomUpDPMtPerInv")
        }
    }

    impl Debug for BottomUpDPMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPMtPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPMtPer(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
