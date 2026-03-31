//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Top-Down Dynamic Programming - Persistent Single-Threaded Implementation.
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMapWithViewPlus for efficient subproblem caching.

//  Table of Contents

pub mod TopDownDPStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports
    use std::fmt::{Formatter, Debug, Display};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {
    // 3. broadcast use
    broadcast use {
        crate::Types::Types::group_Pair_axioms,
        vstd::map::group_map_axioms,
        vstd::seq::group_seq_axioms,
    };

    // 4. type definitions
    pub struct TopDownDPStPerS {
        pub seq_s: ArraySeqStPerS<char>,
        pub seq_t: ArraySeqStPerS<char>,
        pub memo_table: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    }

    // 6. spec fns
    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    /// Minimum edit distance spec — standalone so SMT congruence works across state changes.
    pub open spec fn spec_med_fn(s: Seq<char>, t: Seq<char>, i: nat, j: nat) -> nat
        decreases i + j,
    {
        if i == 0 { j }
        else if j == 0 { i }
        else if s[i as int - 1] == t[j as int - 1] {
            spec_med_fn(s, t, (i - 1) as nat, (j - 1) as nat)
        } else {
            let del = spec_med_fn(s, t, (i - 1) as nat, j);
            let ins = spec_med_fn(s, t, i, (j - 1) as nat);
            1 + spec_min(del, ins)
        }
    }

    // 7. proof fns
    pub proof fn lemma_spec_med_fn_bounded(s: Seq<char>, t: Seq<char>, i: nat, j: nat)
        ensures spec_med_fn(s, t, i, j) <= i + j,
        decreases i + j,
    {
        if i == 0 || j == 0 {
        } else if s[i as int - 1] == t[j as int - 1] {
            lemma_spec_med_fn_bounded(s, t, (i - 1) as nat, (j - 1) as nat);
        } else {
            lemma_spec_med_fn_bounded(s, t, (i - 1) as nat, j);
            lemma_spec_med_fn_bounded(s, t, i, (j - 1) as nat);
        }
    }

    // 8. traits
    pub trait TopDownDPStPerTrait: Sized {
        spec fn spec_s(&self) -> Seq<char>;
        spec fn spec_t(&self) -> Seq<char>;
        spec fn spec_s_len(&self) -> nat;
        spec fn spec_t_len(&self) -> nat;
        spec fn spec_memo(&self) -> Map<(usize, usize), usize>;
        spec fn spec_med(&self, i: nat, j: nat) -> nat;
        spec fn spec_memo_correct(&self, memo: Map<(usize, usize), usize>) -> bool;
        spec fn spec_topdowndpstper_wf(&self) -> bool;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> (dp: Self)
            ensures
                dp.spec_topdowndpstper_wf(),
                dp.spec_s() == s@,
                dp.spec_t() == t@,
                dp.spec_s_len() == s.spec_len(),
                dp.spec_t_len() == t.spec_len();

        fn s_length(&self) -> (len: usize)
            requires self.spec_topdowndpstper_wf(),
            ensures len as nat == self.spec_s_len();

        fn t_length(&self) -> (len: usize)
            requires self.spec_topdowndpstper_wf(),
            ensures len as nat == self.spec_t_len();

        fn is_empty(&self) -> (empty: bool)
            requires self.spec_topdowndpstper_wf(),
            ensures empty == (self.spec_s_len() == 0 && self.spec_t_len() == 0);

        fn memo_size(&self) -> (size: usize)
            requires self.spec_topdowndpstper_wf(),
            ensures size == self.spec_memo().len();

        fn is_memoized(&self, i: usize, j: usize) -> (memoized: bool)
            requires self.spec_topdowndpstper_wf(),
            ensures memoized == self.spec_memo().contains_key((i, j));

        fn get_memoized(&self, i: usize, j: usize) -> (val: Option<usize>)
            requires self.spec_topdowndpstper_wf(),
            ensures
                match val {
                    Some(v) => self.spec_memo().contains_key((i, j))
                        && v == self.spec_memo()[(i, j)],
                    None => !self.spec_memo().contains_key((i, j)),
                };

        fn with_memo_table(self, memo: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> (dp: Self)
            ensures
                dp.spec_s() == self.spec_s(),
                dp.spec_t() == self.spec_t();

        fn clear_memo(self) -> (dp: Self)
            ensures
                dp.spec_topdowndpstper_wf(),
                dp.spec_s() == self.spec_s(),
                dp.spec_t() == self.spec_t();

        fn med_memoized(&self) -> (distance: usize)
            requires
                self.spec_topdowndpstper_wf(),
                self.spec_s_len() + self.spec_t_len() < usize::MAX,
            ensures distance as nat == self.spec_med(self.spec_s_len(), self.spec_t_len());

        fn med_recursive(
            &self,
            i: usize,
            j: usize,
            memo: &mut HashMapWithViewPlus<Pair<usize, usize>, usize>,
        ) -> (distance: usize)
            requires
                i <= self.spec_s_len(),
                j <= self.spec_t_len(),
                self.spec_s_len() + self.spec_t_len() < usize::MAX,
                self.spec_memo_correct(old(memo)@),
            ensures
                distance as nat == self.spec_med(i as nat, j as nat),
                self.spec_memo_correct(memo@),
            decreases i + j;
    }

    // 9. impls
    impl TopDownDPStPerTrait for TopDownDPStPerS {
        open spec fn spec_s(&self) -> Seq<char> { self.seq_s@ }
        open spec fn spec_t(&self) -> Seq<char> { self.seq_t@ }
        open spec fn spec_s_len(&self) -> nat { self.seq_s.spec_len() }
        open spec fn spec_t_len(&self) -> nat { self.seq_t.spec_len() }

        open spec fn spec_memo(&self) -> Map<(usize, usize), usize> { self.memo_table@ }

        open spec fn spec_med(&self, i: nat, j: nat) -> nat {
            spec_med_fn(self.seq_s@, self.seq_t@, i, j)
        }

        open spec fn spec_memo_correct(&self, memo: Map<(usize, usize), usize>) -> bool {
            forall|a: usize, b: usize| #[trigger] memo.contains_key((a, b)) ==>
                memo[(a, b)] as nat == self.spec_med(a as nat, b as nat)
        }

        open spec fn spec_topdowndpstper_wf(&self) -> bool {
            self.spec_memo_correct(self.spec_memo())
        }

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j,
        {
            lemma_spec_med_fn_bounded(self.seq_s@, self.seq_t@, i, j);
        }

        fn new(s: ArraySeqStPerS<char>, t: ArraySeqStPerS<char>) -> (dp: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            TopDownDPStPerS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMapWithViewPlus::new(),
            }
        }

        fn s_length(&self) -> (len: usize) { self.seq_s.length() }
        fn t_length(&self) -> (len: usize) { self.seq_t.length() }

        fn is_empty(&self) -> (empty: bool) {
            let s_empty = self.seq_s.length() == 0;
            let t_empty = self.seq_t.length() == 0;
            s_empty && t_empty
        }

        fn memo_size(&self) -> (size: usize) { self.memo_table.len() }

        fn is_memoized(&self, i: usize, j: usize) -> (memoized: bool) {
            self.memo_table.contains_key(&Pair(i, j))
        }

        fn get_memoized(&self, i: usize, j: usize) -> (val: Option<usize>) {
            match self.memo_table.get(&Pair(i, j)) {
                Some(v) => Some(*v),
                None => None,
            }
        }

        fn with_memo_table(self, memo: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> (dp: Self) {
            TopDownDPStPerS { seq_s: self.seq_s, seq_t: self.seq_t, memo_table: memo }
        }

        fn clear_memo(self) -> (dp: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            TopDownDPStPerS {
                seq_s: self.seq_s,
                seq_t: self.seq_t,
                memo_table: HashMapWithViewPlus::new(),
            }
        }

        /// Compute MED using top-down memoization (Algorithm 51.4).
        fn med_memoized(&self) -> (distance: usize) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let mut memo: HashMapWithViewPlus<Pair<usize, usize>, usize> = HashMapWithViewPlus::new();
            self.med_recursive(s_len, t_len, &mut memo)
        }

        /// Recursive MED with memoization (medOne from Algorithm 51.4).
        fn med_recursive(
            &self,
            i: usize,
            j: usize,
            memo: &mut HashMapWithViewPlus<Pair<usize, usize>, usize>,
        ) -> (distance: usize)
            decreases i + j,
        {
            // Check memo cache.
            match memo.get(&Pair(i, j)) {
                Some(v) => { return *v; }
                None => {}
            }

            let result = if i == 0 {
                j
            } else if j == 0 {
                i
            } else {
                let s_char = *self.seq_s.nth(i - 1);
                let t_char = *self.seq_t.nth(j - 1);

                if s_char == t_char {
                    self.med_recursive(i - 1, j - 1, memo)
                } else {
                    let del_cost = self.med_recursive(i - 1, j, memo);
                    let ins_cost = self.med_recursive(i, j - 1, memo);
                    proof {
                        self.lemma_spec_med_bounded((i - 1) as nat, j as nat);
                        self.lemma_spec_med_bounded(i as nat, (j - 1) as nat);
                    }
                    if del_cost <= ins_cost {
                        1 + del_cost
                    } else {
                        1 + ins_cost
                    }
                }
            };

            let ghost pre_memo = memo@;
            memo.insert(Pair(i, j), result);
            proof {
                assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
                implies
                    memo@[(a, b)] as nat == self.spec_med(a as nat, b as nat)
                by {
                    if a == i && b == j {
                    } else if pre_memo.contains_key((a, b)) {
                        assert(pre_memo[(a, b)] as nat == self.spec_med(a as nat, b as nat));
                    }
                };
            }
            result
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for TopDownDPStPerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@
        }
    }

    impl Default for TopDownDPStPerS {
        fn default() -> (dp: Self)
            ensures
                dp.spec_topdowndpstper_wf(),
                dp.spec_s_len() == 0,
                dp.spec_t_len() == 0,
        {
            let empty_s = ArraySeqStPerS::<char>::empty();
            let empty_t = ArraySeqStPerS::<char>::empty();
            Self::new(empty_s, empty_t)
        }
    }

    // 11. derive impls in verus!
    impl Clone for TopDownDPStPerS {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.seq_s@ == self.seq_s@,
                cloned.seq_t@ == self.seq_t@,
        {
            TopDownDPStPerS {
                seq_s: self.seq_s.clone(),
                seq_t: self.seq_t.clone(),
                memo_table: self.memo_table.clone(),
            }
        }
    }

    impl PartialEq for TopDownDPStPerS {
        fn eq(&self, other: &Self) -> (eq: bool)
            ensures eq == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)
        {
            let r = self.seq_s == other.seq_s && self.seq_t == other.seq_t;
            proof { assume(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for TopDownDPStPerS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for TopDownDPStPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPStPerS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table.inner)
                .finish()
        }
    }

    impl Display for TopDownDPStPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPStPer(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
