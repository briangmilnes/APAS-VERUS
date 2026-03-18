//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Top-Down Dynamic Programming - Ephemeral Multi-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using concurrent HashMapWithViewPlus with in-place mutations for thread-safe subproblem caching.

pub mod TopDownDPMtEph {

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
    use std::sync::Arc;
    use vstd::rwlock::*;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;

    verus! {
    // 3. broadcast use
    broadcast use {
        crate::Types::Types::group_Pair_axioms,
        vstd::map::group_map_axioms,
        vstd::seq::group_seq_axioms,
        vstd::std_specs::hash::group_hash_axioms,
    };

    // 4. type definitions
    pub struct TopDownDPMtEphS {
        pub seq_s: ArraySeqMtEphS<char>,
        pub seq_t: ArraySeqMtEphS<char>,
    }

    /// RwLock predicate for parallel memo table. Ghost sequences enable
    /// content correctness: every cached value equals spec_med_fn.
    pub struct TopDownDPMtEphInv {
        pub ghost seq_s: Seq<char>,
        pub ghost seq_t: Seq<char>,
    }

    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for TopDownDPMtEphInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            &&& v@.dom().finite()
            &&& spec_memo_correct(v@, self.seq_s, self.seq_t)
        }
    }

    // 6. spec fns
    pub open spec fn spec_min(a: nat, b: nat) -> nat {
        if a <= b { a } else { b }
    }

    /// Minimum edit distance spec — standalone for SMT congruence across state changes.
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

    /// Memo correctness: every cached value equals the spec.
    pub open spec fn spec_memo_correct(
        memo: Map<(usize, usize), usize>,
        s: Seq<char>,
        t: Seq<char>,
    ) -> bool {
        forall|a: usize, b: usize| #[trigger] memo.contains_key((a, b)) ==>
            memo[(a, b)] as nat == spec_med_fn(s, t, a as nat, b as nat)
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
    pub trait TopDownDPMtEphTrait: Sized {
        spec fn spec_s(&self) -> Seq<char>;
        spec fn spec_t(&self) -> Seq<char>;
        spec fn spec_s_len(&self) -> nat;
        spec fn spec_t_len(&self) -> nat;
        spec fn spec_med(&self, i: nat, j: nat) -> nat;

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat)
            ensures self.spec_med(i, j) <= i + j;

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- move sequences into struct.
        fn new(s: ArraySeqMtEphS<char>, t: ArraySeqMtEphS<char>) -> (dp: Self)
            ensures
                dp.spec_s() == s@,
                dp.spec_t() == t@,
                dp.spec_s_len() == s.spec_len(),
                dp.spec_t_len() == t.spec_len();

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- return cached length.
        fn s_length(&self) -> (len: usize)
            ensures len as nat == self.spec_s_len();

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- return cached length.
        fn t_length(&self) -> (len: usize)
            ensures len as nat == self.spec_t_len();

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- two length checks.
        fn is_empty(&self) -> (empty: bool)
            ensures empty == (self.spec_s_len() == 0 && self.spec_t_len() == 0);

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- move sequence.
        fn set_s(&mut self, s: ArraySeqMtEphS<char>)
            ensures
                self.spec_s() == s@,
                self.spec_t() == old(self).spec_t();

        /// - APAS: N/A -- Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work O(1), Span O(1) -- move sequence.
        fn set_t(&mut self, t: ArraySeqMtEphS<char>)
            ensures
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == t@;

        /// - APAS: Work O(|S|*|T|), Span O(|S|*|T|) (sequential memo threading)
        /// - Claude-Opus-4.6: Work O(|S|*|T|), Span O(|S|*|T|) -- sequential memoized recursion.
        fn med_memoized_concurrent(&mut self) -> (distance: usize)
            requires old(self).spec_s_len() + old(self).spec_t_len() < usize::MAX,
            ensures
                distance as nat == old(self).spec_med(
                    old(self).spec_s_len(),
                    old(self).spec_t_len()
                ),
                self.spec_s() == old(self).spec_s(),
                self.spec_t() == old(self).spec_t();

        /// - APAS: Work O(|S|*|T|), Span O(|S|+|T|) (parallel subproblem exploration)
        /// - Claude-Opus-4.6: Work O(|S|*|T|), Span O(|S|+|T|) -- fork-join on branches.
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

    // 9. impls

    /// Sequential recursive MED with verified memoization.
    /// - APAS: Work O(|S|*|T|), Span O(|S|*|T|) (Algorithm 51.4, sequential)
    /// - Claude-Opus-4.6: Work O(|S|*|T|), Span O(|S|*|T|) -- sequential recursion with memo.
    fn med_recursive_sequential(
        seq_s: &ArraySeqMtEphS<char>,
        seq_t: &ArraySeqMtEphS<char>,
        memo: &mut HashMapWithViewPlus<Pair<usize, usize>, usize>,
        i: usize,
        j: usize,
    ) -> (distance: usize)
        requires
            i <= seq_s.spec_len(),
            j <= seq_t.spec_len(),
            seq_s.spec_len() + seq_t.spec_len() < usize::MAX,
            old(memo)@.dom().finite(),
            spec_memo_correct(old(memo)@, seq_s@, seq_t@),
        ensures
            distance as nat == spec_med_fn(seq_s@, seq_t@, i as nat, j as nat),
            memo@.dom().finite(),
            spec_memo_correct(memo@, seq_s@, seq_t@),
        decreases i + j,
    {
        match memo.get(&Pair(i, j)) {
            Some(v) => { return *v; }
            None => {}
        }

        let result = if i == 0 {
            j
        } else if j == 0 {
            i
        } else {
            let s_char = *seq_s.nth(i - 1);
            let t_char = *seq_t.nth(j - 1);
            if s_char == t_char {
                med_recursive_sequential(seq_s, seq_t, memo, i - 1, j - 1)
            } else {
                let del_cost = med_recursive_sequential(seq_s, seq_t, memo, i - 1, j);
                let ins_cost = med_recursive_sequential(seq_s, seq_t, memo, i, j - 1);
                proof {
                    lemma_spec_med_fn_bounded(seq_s@, seq_t@, (i - 1) as nat, j as nat);
                    lemma_spec_med_fn_bounded(seq_s@, seq_t@, i as nat, (j - 1) as nat);
                }
                if del_cost <= ins_cost {
                    1 + del_cost
                } else {
                    1 + ins_cost
                }
            }
        };

        let ghost s = seq_s@;
        let ghost t = seq_t@;
        assert(result as nat == spec_med_fn(s, t, i as nat, j as nat));
        let ghost pre_memo = memo@;
        proof {
            assert forall|a: usize, b: usize| #[trigger] pre_memo.contains_key((a, b))
            implies
                pre_memo[(a, b)] as nat == spec_med_fn(s, t, a as nat, b as nat)
            by {
                assert(spec_memo_correct(pre_memo, s, t));
            };
        }
        memo.insert(Pair(i, j), result);
        assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
        implies
            memo@[(a, b)] as nat == spec_med_fn(s, t, a as nat, b as nat)
        by {
            if a == i && b == j {
            } else if pre_memo.contains_key((a, b)) {
                assert(pre_memo[(a, b)] as nat == spec_med_fn(s, t, a as nat, b as nat));
            }
        };
        result
    }

    /// Parallel recursive MED with thread-safe memoization.
    /// - APAS: Work O(|S|*|T|), Span O(|S|+|T|) (parallel subproblem exploration)
    /// - Claude-Opus-4.6: Work O(|S|*|T|), Span O(|S|+|T|) -- fork-join on delete/insert.
    fn med_recursive_parallel(
        seq_s: &ArraySeqMtEphS<char>,
        seq_t: &ArraySeqMtEphS<char>,
        memo: &Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, TopDownDPMtEphInv>>,
        i: usize,
        j: usize,
    ) -> (dist: usize)
        requires
            i <= seq_s.spec_len(),
            j <= seq_t.spec_len(),
            seq_s.spec_len() + seq_t.spec_len() < usize::MAX,
            memo.pred().seq_s == seq_s@,
            memo.pred().seq_t == seq_t@,
        ensures
            dist as nat == spec_med_fn(seq_s@, seq_t@, i as nat, j as nat),
        decreases i + j,
    {
        // Memo lookup.
        {
            let rwlock = arc_deref(memo);
            let handle = rwlock.acquire_read();
            let found = match handle.borrow().get(&Pair(i, j)) {
                Some(v) => Some(*v),
                None => None,
            };
            handle.release_read();
            if let Some(result) = found {
                proof { lemma_spec_med_fn_bounded(seq_s@, seq_t@, i as nat, j as nat); }
                if result <= i + j {
                    return result;
                }
            }
        }

        let dist = if i == 0 {
            j
        } else if j == 0 {
            i
        } else {
            let s_char = seq_s.nth(i - 1).clone();
            let t_char = seq_t.nth(j - 1).clone();

            if s_char == t_char {
                med_recursive_parallel(seq_s, seq_t, memo, i - 1, j - 1)
            } else {
                let s1 = seq_s.clone();
                let t1 = seq_t.clone();
                let memo1 = clone_arc_rwlock(memo);

                assert(i - 1 <= s1.spec_len());
                assert(j <= t1.spec_len());
                assert(s1.spec_len() + t1.spec_len() < usize::MAX);
                assert(memo1.pred().seq_s == s1@);
                assert(memo1.pred().seq_t == t1@);

                let s2 = seq_s.clone();
                let t2 = seq_t.clone();
                let memo2 = clone_arc_rwlock(memo);

                assert(i <= s2.spec_len());
                assert(j - 1 <= t2.spec_len());
                assert(s2.spec_len() + t2.spec_len() < usize::MAX);
                assert(memo2.pred().seq_s == s2@);
                assert(memo2.pred().seq_t == t2@);

                let ghost s_view = seq_s@;
                let ghost t_view = seq_t@;

                let f1 = move || -> (r: usize)
                    requires
                        i - 1 <= s1.spec_len(),
                        j <= t1.spec_len(),
                        s1.spec_len() + t1.spec_len() < usize::MAX,
                        memo1.pred().seq_s == s1@,
                        memo1.pred().seq_t == t1@,
                    ensures r as nat == spec_med_fn(s_view, t_view, (i - 1) as nat, j as nat),
                {
                    med_recursive_parallel(&s1, &t1, &memo1, i - 1, j)
                };
                let f2 = move || -> (r: usize)
                    requires
                        i <= s2.spec_len(),
                        j - 1 <= t2.spec_len(),
                        s2.spec_len() + t2.spec_len() < usize::MAX,
                        memo2.pred().seq_s == s2@,
                        memo2.pred().seq_t == t2@,
                    ensures r as nat == spec_med_fn(s_view, t_view, i as nat, (j - 1) as nat),
                {
                    med_recursive_parallel(&s2, &t2, &memo2, i, j - 1)
                };
                let (del_cost, ins_cost) = join(f1, f2);

                proof {
                    lemma_spec_med_fn_bounded(seq_s@, seq_t@, (i - 1) as nat, j as nat);
                    lemma_spec_med_fn_bounded(seq_s@, seq_t@, i as nat, (j - 1) as nat);
                }
                if del_cost <= ins_cost {
                    1 + del_cost
                } else {
                    1 + ins_cost
                }
            }
        };

        // Memo store.
        {
            let rwlock = arc_deref(memo);
            let (mut current, write_handle) = rwlock.acquire_write();
            let ghost pre_insert = current@;
            current.insert(Pair(i, j), dist);
            proof {
                assert forall|a: usize, b: usize| #[trigger] current@.contains_key((a, b))
                implies
                    current@[(a, b)] as nat == spec_med_fn(seq_s@, seq_t@, a as nat, b as nat)
                by {
                    if a == i && b == j {
                    } else if pre_insert.contains_key((a, b)) {
                        assert(spec_memo_correct(pre_insert, seq_s@, seq_t@));
                        assert(pre_insert[(a, b)] as nat == spec_med_fn(seq_s@, seq_t@, a as nat, b as nat));
                    }
                };
            }
            write_handle.release_write(current);
        }

        dist
    }

    impl TopDownDPMtEphTrait for TopDownDPMtEphS {
        open spec fn spec_s(&self) -> Seq<char> { self.seq_s@ }
        open spec fn spec_t(&self) -> Seq<char> { self.seq_t@ }
        open spec fn spec_s_len(&self) -> nat { self.seq_s.spec_len() }
        open spec fn spec_t_len(&self) -> nat { self.seq_t.spec_len() }

        open spec fn spec_med(&self, i: nat, j: nat) -> nat {
            spec_med_fn(self.seq_s@, self.seq_t@, i, j)
        }

        proof fn lemma_spec_med_bounded(&self, i: nat, j: nat) {
            lemma_spec_med_fn_bounded(self.seq_s@, self.seq_t@, i, j);
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

        /// Compute MED using sequential top-down memoization (Algorithm 51.4).
        fn med_memoized_concurrent(&mut self) -> (distance: usize) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let mut memo = HashMapWithViewPlus::new();
            med_recursive_sequential(&self.seq_s, &self.seq_t, &mut memo, s_len, t_len)
        }

        /// Compute MED with parallel subproblem exploration.
        fn med_memoized_parallel(&mut self) -> (distance: usize) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();
            let memo = new_arc_rwlock(
                HashMapWithViewPlus::new(),
                Ghost(TopDownDPMtEphInv { seq_s: self.seq_s@, seq_t: self.seq_t@ }),
            );
            med_recursive_parallel(&self.seq_s, &self.seq_t, &memo, s_len, t_len)
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
            proof { assume(r == (self.seq_s@ == other.seq_s@ && self.seq_t@ == other.seq_t@)); }
            r
        }
    }

    impl Eq for TopDownDPMtEphS {}

    } // verus!

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
