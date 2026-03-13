//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Bottom-Up Dynamic Programming - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the bottom-up approach to dynamic programming using
//! parallel diagonal pebbling with in-place mutations for multi-threaded computation.

//  Table of Contents

pub mod BottomUpDPMtEph {

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
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {
    // 4. type definitions
    pub struct BottomUpDPMtEphS {
        pub seq_s: ArraySeqMtEphS<char>,
        pub seq_t: ArraySeqMtEphS<char>,
    }

    // 6. spec fns
    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    // 8. traits
    pub trait BottomUpDPMtEphTrait: Sized {
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

        fn med_bottom_up_parallel(&mut self) -> (distance: usize)
            requires old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
            ensures
                distance as nat == old(self).spec_med(
                    old(self).spec_s_len(),
                    old(self).spec_t_len()
                ),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t();
    }

    // 9. impls
    impl BottomUpDPMtEphTrait for BottomUpDPMtEphS {
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
            BottomUpDPMtEphS { seq_s: s, seq_t: t }
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

        /// Compute MED using bottom-up row-by-row fill (Algorithm 51.1).
        fn med_bottom_up_parallel(&mut self) -> (distance: usize) {
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
                    self.spec_s() == old(self).spec_s(),
                    self.spec_t() == old(self).spec_t(),
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
                        self.spec_s() == old(self).spec_s(),
                        self.spec_t() == old(self).spec_t(),
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
                    j = j + 1;
                }

                assert(row@.len() == t_len as nat + 1);
                assert(row@[0] == i as nat);
                table.push(row);
                i = i + 1;
            }

            table[s_len][t_len]
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for BottomUpDPMtEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@
        }
    }

    impl Default for BottomUpDPMtEphS {
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
    impl Clone for BottomUpDPMtEphS {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.seq_s@ == self.seq_s@,
                cloned.seq_t@ == self.seq_t@,
        {
            BottomUpDPMtEphS {
                seq_s: self.seq_s.clone(),
                seq_t: self.seq_t.clone(),
            }
        }
    }

    impl PartialEq for BottomUpDPMtEphS {
        fn eq(&self, other: &Self) -> (eq: bool)
            ensures eq == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)
        {
            let r = self.seq_s == other.seq_s && self.seq_t == other.seq_t;
            proof { accept(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for BottomUpDPMtEphS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for BottomUpDPMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BottomUpDPMtEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .finish()
        }
    }

    impl Display for BottomUpDPMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "BottomUpDPMtEph(s_len={}, t_len={})",
                self.s_length(),
                self.t_length()
            )
        }
    }
}
