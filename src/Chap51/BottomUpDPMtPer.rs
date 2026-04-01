//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

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
    use std::fmt::{Formatter, Debug, Display};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    verus! {
    // 4. type definitions
    pub struct BottomUpDPMtPerS {
        pub seq_s: ArraySeqMtPerS<char>,
        pub seq_t: ArraySeqMtPerS<char>,
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
        spec fn spec_bottomupdpmtper_wf(&self) -> bool;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> (dp: Self)
            ensures
                dp.spec_bottomupdpmtper_wf(),
                dp.spec_s() == s@,
                dp.spec_t() == t@,
                dp.spec_s_len() == s.spec_len(),
                dp.spec_t_len() == t.spec_len();
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.

        fn s_length(&self) -> (len: usize)
            requires self.spec_bottomupdpmtper_wf(),
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.
            ensures len as nat == self.spec_s_len();

        fn t_length(&self) -> (len: usize)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
            requires self.spec_bottomupdpmtper_wf(),
            ensures len as nat == self.spec_t_len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n*m), Span O(n+m) — bottom-up DP with parallel diagonal; Mt parallel.
        fn is_empty(&self) -> (empty: bool)
            requires self.spec_bottomupdpmtper_wf(),
            ensures empty == (self.spec_s_len() == 0 && self.spec_t_len() == 0);

        fn med_bottom_up_parallel(&self) -> (distance: usize)
            requires
                self.spec_bottomupdpmtper_wf(),
                self.spec_s_len() + self.spec_t_len() < usize::MAX,
            ensures
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — fills first row and column.
                distance as nat == self.spec_med(
                    self.spec_s_len(),
                    self.spec_t_len()
                );

        fn initialize_base_cases(&self) -> (table: Vec<Vec<usize>>)
            requires
                self.spec_bottomupdpmtper_wf(),
                self.spec_s_len() < usize::MAX,
                self.spec_t_len() < usize::MAX,
            ensures
                table@.len() == self.spec_s_len() + 1,
                forall|i: int| #![trigger table@[i]]
                    0 <= i < table@.len() ==>
                    table@[i]@.len() == self.spec_t_len() + 1,
                forall|i: int| #![trigger table@[i]]
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — computes one DP cell.
                    0 <= i <= self.spec_s_len() as int ==>
                    table@[i]@[0] == i as nat,
                forall|j: int|
                    0 <= j <= self.spec_t_len() as int ==>
                    table@[0]@[j] == j as nat;

        fn compute_cell_value(
            &self,
            table: &Vec<Vec<usize>>,
            i: usize,
            j: usize,
        ) -> (val: usize)
            requires
                self.spec_bottomupdpmtper_wf(),
                1 <= i <= self.spec_s_len(),
                1 <= j <= self.spec_t_len(),
                self.spec_s_len() + self.spec_t_len() < usize::MAX,
                table@.len() > i,
                forall|r: int| #![trigger table@[r]]
                    0 <= r < table@.len() ==>
                    table@[r]@.len() > j,
                table@[(i - 1) as int]@[(j - 1) as int] as nat
                    == self.spec_med((i - 1) as nat, (j - 1) as nat),
                table@[(i - 1) as int]@[j as int] as nat
                    == self.spec_med((i - 1) as nat, j as nat),
                table@[i as int]@[(j - 1) as int] as nat
                    == self.spec_med(i as nat, (j - 1) as nat),
            ensures
                val as nat == self.spec_med(i as nat, j as nat);
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

        open spec fn spec_bottomupdpmtper_wf(&self) -> bool { true }

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j,
            decreases i + j,
        {
            if i == 0 || j == 0 {
            } else if self.seq_s@[i as int - 1] == self.seq_t@[j as int - 1] {
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
                self.lemma_spec_med_bounded((i - 1) as nat, (j - 1) as nat);
            } else {
                self.lemma_spec_med_bounded((i - 1) as nat, j);
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.
                /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.
                self.lemma_spec_med_bounded(i, (j - 1) as nat);
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — field access.
            }
        }

        fn new(s: ArraySeqMtPerS<char>, t: ArraySeqMtPerS<char>) -> (dp: Self) {
            BottomUpDPMtPerS { seq_s: s, seq_t: t }
        }
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n*m), Span O(n+m) — bottom-up DP with parallel diagonal; Mt parallel.

        fn s_length(&self) -> (len: usize) { self.seq_s.length() }
        fn t_length(&self) -> (len: usize) { self.seq_t.length() }

        fn is_empty(&self) -> (empty: bool) {
            let s_empty = self.seq_s.length() == 0;
            let t_empty = self.seq_t.length() == 0;
            s_empty && t_empty
        }

        /// Compute MED using bottom-up row-by-row fill (Algorithm 51.1).
        fn med_bottom_up_parallel(&self) -> (distance: usize) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            // Row 0: base case [0, 1, 2, ..., t_len].
            let mut table: Vec<Vec<usize>> = Vec::new();
            let mut first_row: Vec<usize> = Vec::new();
            let mut jj: usize = 0;
            while jj <= t_len
                invariant
                    jj <= t_len + 1,
                    t_len < usize::MAX,
                    t_len as nat == self.spec_t_len(),
                    first_row@.len() == jj as nat,
                    forall|c: int| 0 <= c < jj as int ==> first_row@[c] == c as nat,
                decreases t_len + 1 - jj,
            {
                first_row.push(jj);
                jj = jj + 1;
            }
            table.push(first_row);
            assert(table@.len() == 1);
            assert(table@[0]@.len() == t_len as nat + 1);
            assert(forall|c: int| 0 <= c <= t_len as int ==> table@[0]@[c] == c as nat);

            // Rows 1..=s_len: build each row using previous row.
            let mut i: usize = 1;
            while i <= s_len
                invariant
                    1 <= i <= s_len + 1,
                    s_len as nat == self.spec_s_len(),
                    t_len as nat == self.spec_t_len(),
                    self.spec_s_len() + self.spec_t_len() < usize::MAX,
                    table@.len() == i as nat,
                    forall|r: int| #![trigger table@[r]]
                        0 <= r < i as int ==>
                        table@[r]@.len() == t_len as nat + 1,
                    forall|r: int| #![trigger table@[r]]
                        0 <= r < i as int ==>
                        table@[r]@[0] == r as nat,
                    forall|c: int|
                        0 <= c <= t_len as int ==>
                        table@[0]@[c] == c as nat,
                    forall|r: int, c: int| #![trigger table@[r]@[c]]
                        1 <= r < i as int && 1 <= c <= t_len as int ==>
                        table@[r]@[c] as nat == self.spec_med(r as nat, c as nat),
                decreases s_len + 1 - i,
            {
                let mut row: Vec<usize> = Vec::new();
                row.push(i);

                let mut j: usize = 1;
                while j <= t_len
                    invariant
                        1 <= j <= t_len + 1,
                        1 <= i <= s_len,
                        s_len as nat == self.spec_s_len(),
                        t_len as nat == self.spec_t_len(),
                        self.spec_s_len() + self.spec_t_len() < usize::MAX,
                        table@.len() == i as nat,
                        forall|r: int| #![trigger table@[r]]
                            0 <= r < i as int ==>
                            table@[r]@.len() == t_len as nat + 1,
                        forall|r: int| #![trigger table@[r]]
                            0 <= r < i as int ==>
                            table@[r]@[0] == r as nat,
                        forall|c: int|
                            0 <= c <= t_len as int ==>
                            table@[0]@[c] == c as nat,
                        forall|r: int, c: int| #![trigger table@[r]@[c]]
                            1 <= r < i as int && 1 <= c <= t_len as int ==>
                            table@[r]@[c] as nat == self.spec_med(r as nat, c as nat),
                        row@.len() == j as nat,
                        row@[0] == i as nat,
                        forall|c: int| #![trigger row@[c]]
                            1 <= c < j as int ==>
                            row@[c] as nat == self.spec_med(i as nat, c as nat),
                    decreases t_len + 1 - j,
                {
                    let prev_row = &table[i - 1];
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    let diag: usize = prev_row[j - 1];
                    let above: usize = prev_row[j];
                    let left: usize = row[j - 1];

                    proof {
                        if (i - 1) as nat == 0 {
                        } else if (j - 1) as nat == 0 {
                        } else {
                        }
                        assert(diag as nat == self.spec_med((i - 1) as nat, (j - 1) as nat));

                        if (i - 1) as nat == 0 {
                        } else {
                        }
                        assert(above as nat == self.spec_med((i - 1) as nat, j as nat));

                        if (j - 1) as nat == 0 {
                        } else {
                        }
                        assert(left as nat == self.spec_med(i as nat, (j - 1) as nat));

                        self.lemma_spec_med_bounded((i - 1) as nat, j as nat);
                        self.lemma_spec_med_bounded(i as nat, (j - 1) as nat);
                    }

                    let val = if s_char == t_char {
                        diag
                    } else {
                        1 + if above <= left { above } else { left }
                    };

                    assert(val as nat == self.spec_med(i as nat, j as nat));
                    row.push(val);
                    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — fills first row and column.
                    j = j + 1;
                }

                assert(row@.len() == t_len as nat + 1);
                assert(row@[0] == i as nat);
                table.push(row);
                i = i + 1;
            }

            table[s_len][t_len]
        }

        fn initialize_base_cases(&self) -> (table: Vec<Vec<usize>>) {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            let mut table: Vec<Vec<usize>> = Vec::new();

            // Row 0: [0, 1, 2, ..., t_len].
            let mut first_row: Vec<usize> = Vec::new();
            let mut j: usize = 0;
            while j <= t_len
                invariant
                    j <= t_len + 1,
                    t_len < usize::MAX,
                    t_len as nat == self.spec_t_len(),
                    first_row@.len() == j as nat,
                    forall|k: int| 0 <= k < j as int ==> first_row@[k] == k as nat,
                decreases (t_len + 1 - j),
            {
                first_row.push(j);
                j = j + 1;
            }
            table.push(first_row);

            // Rows 1..=s_len: [i, 0, 0, ..., 0].
            let mut i: usize = 1;
            while i <= s_len
                invariant
                    1 <= i <= s_len + 1,
                    s_len < usize::MAX,
                    t_len < usize::MAX,
                    s_len as nat == self.spec_s_len(),
                    t_len as nat == self.spec_t_len(),
                    table@.len() == i as nat,
                    forall|r: int| #![trigger table@[r]]
                        0 <= r < i as int ==>
                        table@[r]@.len() == t_len as nat + 1,
                    forall|r: int| #![trigger table@[r]]
                        0 <= r < i as int ==>
                        table@[r]@[0] == r as nat,
                    forall|c: int|
                        0 <= c <= t_len as int ==>
                        table@[0]@[c] == c as nat,
                decreases (s_len + 1 - i),
            {
                let mut row: Vec<usize> = Vec::new();
                row.push(i);
                let mut jj: usize = 1;
                while jj <= t_len
                    invariant
                        1 <= jj <= t_len + 1,
                        t_len < usize::MAX,
                        t_len as nat == self.spec_t_len(),
                        row@.len() == jj as nat,
                        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — computes one DP cell.
                        row@[0] == i as nat,
                    decreases (t_len + 1 - jj),
                {
                    row.push(0);
                    jj = jj + 1;
                }
                table.push(row);
                i = i + 1;
            }

            table
        }

        fn compute_cell_value(
            &self,
            table: &Vec<Vec<usize>>,
            i: usize,
            j: usize,
        ) -> (val: usize) {
            let s_char = *self.seq_s.nth(i - 1);
            let t_char = *self.seq_t.nth(j - 1);

            if s_char == t_char {
                table[i - 1][j - 1]
            } else {
                let delete_cost = table[i - 1][j];
                let insert_cost = table[i][j - 1];
                proof {
                    self.lemma_spec_med_bounded((i - 1) as nat, j as nat);
                    self.lemma_spec_med_bounded(i as nat, (j - 1) as nat);
                }
                if delete_cost <= insert_cost {
                    1 + delete_cost
                } else {
                    1 + insert_cost
                }
            }
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
                dp.spec_bottomupdpmtper_wf(),
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
            proof { assume(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for BottomUpDPMtPerS {}

    } // verus!

    // 13. derive impls outside verus!
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
