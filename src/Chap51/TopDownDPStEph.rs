//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Top-Down Dynamic Programming - Ephemeral Single-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMapWithViewPlus with in-place mutations for efficient subproblem caching.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 7. proof fns
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

pub mod TopDownDPStEph {

    //		Section 1. module
    //		Section 2. imports
    //		Section 3. broadcast use
    //		Section 4. type definitions
    //		Section 6. spec fns
    //		Section 7. proof fns
    //		Section 8. traits
    //		Section 9. impls
    //		Section 12. derive impls in verus!
    //		Section 14. derive impls outside verus!

    use std::fmt::{Formatter, Debug, Display};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {


    broadcast use {
        crate::Types::Types::group_Pair_axioms,
        vstd::map::group_map_axioms,
        vstd::seq::group_seq_axioms,
    };


    pub struct TopDownDPStEphS {
        pub seq_s: ArraySeqStEphS<char>,
        pub seq_t: ArraySeqStEphS<char>,
        pub memo_table: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    }


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


    pub trait TopDownDPStEphTrait: Sized {
        spec fn spec_s(&self) -> Seq<char>;
        spec fn spec_t(&self) -> Seq<char>;
        spec fn spec_s_len(&self) -> nat;
        spec fn spec_t_len(&self) -> nat;
        spec fn spec_memo(&self) -> Map<(usize, usize), usize>;
        spec fn spec_med(&self, i: nat, j: nat) -> nat;
        spec fn spec_memo_correct(&self) -> bool;
        spec fn spec_topdowndpsteph_wf(&self) -> bool;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- move sequences into struct.
        fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> (dp: Self)
            ensures
                dp.spec_topdowndpsteph_wf(),
                dp.spec_s() == s@,
                dp.spec_t() == t@,
                dp.spec_s_len() == s.spec_len(),
                dp.spec_t_len() == t.spec_len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return cached length.
        fn s_length(&self) -> (len: usize)
            requires self.spec_topdowndpsteph_wf(),
            ensures len as nat == self.spec_s_len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return cached length.
        fn t_length(&self) -> (len: usize)
            requires self.spec_topdowndpsteph_wf(),
            ensures len as nat == self.spec_t_len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- two length checks.
        fn is_empty(&self) -> (empty: bool)
            requires self.spec_topdowndpsteph_wf(),
            ensures empty == (self.spec_s_len() == 0 && self.spec_t_len() == 0);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- return hash map size.
        fn memo_size(&self) -> (size: usize)
            requires self.spec_topdowndpsteph_wf(),
            ensures size == self.spec_memo().len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- hash map contains_key.
        fn is_memoized(&self, i: usize, j: usize) -> (memoized: bool)
            requires self.spec_topdowndpsteph_wf(),
            ensures memoized == self.spec_memo().contains_key((i, j));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- hash map lookup.
        fn get_memoized(&self, i: usize, j: usize) -> (val: Option<usize>)
            requires self.spec_topdowndpsteph_wf(),
            ensures
                match val {
                    Some(v) => self.spec_memo().contains_key((i, j))
                        && v == self.spec_memo()[(i, j)],
                    None => !self.spec_memo().contains_key((i, j)),
                };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- hash map insert.
        fn insert_memo(&mut self, i: usize, j: usize, value: usize)
            requires old(self).spec_topdowndpsteph_wf(),
            ensures
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t(),
                self.spec_memo() == old(self).spec_memo().insert((i, j), value);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- clear hash map.
        fn clear_memo(&mut self)
            ensures
                self.spec_topdowndpsteph_wf(),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t(),
                self.spec_memo() == Map::<(usize, usize), usize>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- move sequence.
        fn set_s(&mut self, s: ArraySeqStEphS<char>)
            requires old(self).spec_topdowndpsteph_wf(),
            ensures
                self.spec_topdowndpsteph_wf(),
                self.spec_s() == s@,
                self.spec_t() == old(self).spec_t();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- move sequence.
        fn set_t(&mut self, t: ArraySeqStEphS<char>)
            requires old(self).spec_topdowndpsteph_wf(),
            ensures
                self.spec_topdowndpsteph_wf(),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == t@;

        /// - Alg Analysis: APAS (Ch51 ref): Work O(|S|*|T|), Span O(|S|+|T|) (Algorithm 51.4)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|*|T|) -- sequential recursion with memo.
        fn med_memoized(&mut self) -> (distance: usize)
            requires
                old(self).spec_topdowndpsteph_wf(),
                old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
            ensures
                self.spec_topdowndpsteph_wf(),
                distance as nat == old(self).spec_med(
                    old(self).spec_s_len(),
                    old(self).spec_t_len()
                ),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t();

        /// - Alg Analysis: APAS (Ch51 ref): Work O(|S|*|T|), Span O(|S|+|T|) (medOne from Algorithm 51.4)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S|*|T|), Span O(|S|*|T|) -- sequential recursion with memo.
        fn med_recursive(&mut self, i: usize, j: usize) -> (distance: usize)
            requires
                i <= old(self).spec_s_len(),
                j <= old(self).spec_t_len(),
                old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
                old(self).spec_memo_correct(),
            ensures
                distance as nat == old(self).spec_med(i as nat, j as nat),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t(),
                self.spec_s_len() == old(self).spec_s_len(),
                self.spec_t_len() == old(self).spec_t_len(),
                self.spec_memo_correct(),
            decreases i + j;
    }


    impl TopDownDPStEphTrait for TopDownDPStEphS {
        open spec fn spec_s(&self) -> Seq<char> { self.seq_s@ }
        open spec fn spec_t(&self) -> Seq<char> { self.seq_t@ }
        open spec fn spec_s_len(&self) -> nat { self.seq_s.spec_len() }
        open spec fn spec_t_len(&self) -> nat { self.seq_t.spec_len() }

        open spec fn spec_memo(&self) -> Map<(usize, usize), usize> { self.memo_table@ }

        open spec fn spec_memo_correct(&self) -> bool {
            forall|a: usize, b: usize| self.spec_memo().contains_key((a, b)) ==>
                self.spec_memo()[(a, b)] as nat == #[trigger] self.spec_med(a as nat, b as nat)
        }

        open spec fn spec_topdowndpsteph_wf(&self) -> bool {
            self.spec_memo_correct()
        }

        open spec fn spec_med(&self, i: nat, j: nat) -> nat {
            spec_med_fn(self.seq_s@, self.seq_t@, i, j)
        }

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j,
        {
            lemma_spec_med_fn_bounded(self.seq_s@, self.seq_t@, i, j);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — struct construction.
        fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> (dp: Self) {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            TopDownDPStEphS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMapWithViewPlus::new(),
            }
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.
        fn s_length(&self) -> (len: usize) { self.seq_s.length() }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — length access.
        fn t_length(&self) -> (len: usize) { self.seq_t.length() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — two length checks.
        fn is_empty(&self) -> (empty: bool) {
            let s_empty = self.seq_s.length() == 0;
            let t_empty = self.seq_t.length() == 0;
            s_empty && t_empty
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns cached size.
        fn memo_size(&self) -> (size: usize) { self.memo_table.len() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — checks memo entry.
        fn is_memoized(&self, i: usize, j: usize) -> (memoized: bool) {
            self.memo_table.contains_key(&Pair(i, j))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — memo table lookup.
        fn get_memoized(&self, i: usize, j: usize) -> (val: Option<usize>) {
            match self.memo_table.get(&Pair(i, j)) {
                Some(v) => Some(*v),
                None => None,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — memo table insert.
        fn insert_memo(&mut self, i: usize, j: usize, value: usize) {
            self.memo_table.insert(Pair(i, j), value);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clears memo table where n is entries.
        fn clear_memo(&mut self) {
            self.memo_table.clear();
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — field write + memo clear where n is memo entries.
        fn set_s(&mut self, s: ArraySeqStEphS<char>) {
            self.seq_s = s;
            self.memo_table.clear();
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — field write + memo clear where n is memo entries.
        fn set_t(&mut self, t: ArraySeqStEphS<char>) {
            self.seq_t = t;
            self.memo_table.clear();
        }

        /// Compute MED using top-down memoization (Algorithm 51.4).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n*m), Span O(n*m) — top-down DP with memoization; St sequential.
        fn med_memoized(&mut self) -> (distance: usize) {
            self.memo_table.clear();
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            self.med_recursive(s_len, t_len)
        }

        /// Recursive MED with memoization (medOne from Algorithm 51.4).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n*m), Span O(n*m) — memoized recursion filling n×m table; St sequential.
        fn med_recursive(&mut self, i: usize, j: usize) -> (distance: usize)
            decreases i + j,
        {
            // Check memo cache.
            match self.memo_table.get(&Pair(i, j)) {
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
                    self.med_recursive(i - 1, j - 1)
                } else {
                    let del_cost = self.med_recursive(i - 1, j);
                    let ins_cost = self.med_recursive(i, j - 1);
                    // Veracity: NEEDED proof block
                    proof {
                        lemma_spec_med_fn_bounded(self.seq_s@, self.seq_t@, (i - 1) as nat, j as nat);
                        lemma_spec_med_fn_bounded(self.seq_s@, self.seq_t@, i as nat, (j - 1) as nat);
                    }
                    if del_cost <= ins_cost {
                        1 + del_cost
                    } else {
                        1 + ins_cost
                    }
                }
            };

            let ghost s = self.seq_s@;
            let ghost t = self.seq_t@;
            let ghost pre_memo = self.memo_table@;
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|a: usize, b: usize| pre_memo.contains_key((a, b))
                implies
                    pre_memo[(a, b)] as nat == #[trigger] spec_med_fn(s, t, a as nat, b as nat)
                by {
                    // Veracity: NEEDED assert
                    assert(self.spec_med(a as nat, b as nat) == spec_med_fn(s, t, a as nat, b as nat));
                };
            }
            self.memo_table.insert(Pair(i, j), result);
            // Veracity: NEEDED assert
            assert forall|a: usize, b: usize| self.spec_memo().contains_key((a, b))
            implies
                self.spec_memo()[(a, b)] as nat == #[trigger] self.spec_med(a as nat, b as nat)
            by {
                if a == i && b == j {
                } else if pre_memo.contains_key((a, b)) {
                }
            };
            result
        }
    }


    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for TopDownDPStEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool {
            self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@
        }
    }

    impl Default for TopDownDPStEphS {
        fn default() -> (dp: Self)
            ensures
                dp.spec_topdowndpsteph_wf(),
                dp.spec_s_len() == 0,
                dp.spec_t_len() == 0,
        {
            let empty_s = ArraySeqStEphS::<char>::empty();
            let empty_t = ArraySeqStEphS::<char>::empty();
            Self::new(empty_s, empty_t)
        }
    }

    impl Clone for TopDownDPStEphS {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.seq_s@ == self.seq_s@,
                cloned.seq_t@ == self.seq_t@,
        {
            TopDownDPStEphS {
                seq_s: self.seq_s.clone(),
                seq_t: self.seq_t.clone(),
                memo_table: self.memo_table.clone(),
            }
        }
    }

    impl PartialEq for TopDownDPStEphS {
        fn eq(&self, other: &Self) -> (eq: bool)
            ensures eq == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)
        {
            let r = self.seq_s == other.seq_s && self.seq_t == other.seq_t;
            // Veracity: NEEDED proof block
            proof { assume(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for TopDownDPStEphS {}

    } // verus!


    impl Debug for TopDownDPStEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TopDownDPStEphS")
                .field("seq_s", &self.seq_s)
                .field("seq_t", &self.seq_t)
                .field("memo_table", &self.memo_table.inner)
                .finish()
        }
    }

    impl Display for TopDownDPStEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPStEph(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
