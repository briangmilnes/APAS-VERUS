//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 52: Adjacency Sequence Graph (ephemeral, single-threaded).

pub mod AdjSeqGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub struct AdjSeqGraphStEph {
        pub adj: ArraySeqStEphS<ArraySeqStEphS<usize>>,
        pub num_edges: usize,
    }

    // 5. view impls

    impl View for AdjSeqGraphStEph {
        type V = Seq<Seq<int>>;
        open spec fn view(&self) -> Self::V {
            Seq::new(self.adj.spec_len(), |i: int|
                Seq::new(self.adj.spec_index(i).spec_len(), |j: int|
                    self.adj.spec_index(i).spec_index(j) as int
                )
            )
        }
    }

    // 6. spec fns

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    // 7. proof fns

    /// Partial sums are bounded by the total sum.
    proof fn lemma_sum_of_monotone(i: int, n: int, f: spec_fn(int) -> nat)
        requires 0 <= i <= n
        ensures spec_sum_of(i, f) <= spec_sum_of(n, f)
        decreases n - i
    {
        if i < n {
            lemma_sum_of_monotone(i, n - 1, f);
        }
    }

    /// Unfolding one step: spec_sum_of(i+1, f) == f(i) + spec_sum_of(i, f).
    proof fn lemma_sum_of_unfold(i: int, f: spec_fn(int) -> nat)
        requires i >= 0
        ensures spec_sum_of(i + 1, f) == f(i) + spec_sum_of(i, f)
    {
    }

    /// Sum of all-zero function is zero.
    proof fn lemma_sum_of_all_zero(f: spec_fn(int) -> nat, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == 0nat
        ensures spec_sum_of(n, f) == 0
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == 0nat);
            lemma_sum_of_all_zero(f, n - 1);
        }
    }

    /// Extensionality for spec_sum_of: identical functions yield identical sums.
    proof fn lemma_sum_of_ext(f: spec_fn(int) -> nat, g: spec_fn(int) -> nat, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == g(i)
        ensures spec_sum_of(n, f) == spec_sum_of(n, g)
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == g(n - 1));
            lemma_sum_of_ext(f, g, n - 1);
        }
    }

    /// Changing one term in the sum: the total changes by the difference.
    proof fn lemma_sum_of_change_one(n: int, old_f: spec_fn(int) -> nat, new_f: spec_fn(int) -> nat, k: int)
        requires
            0 <= k < n,
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] old_f(i) == new_f(i),
        ensures
            spec_sum_of(n, new_f) + old_f(k) == spec_sum_of(n, old_f) + new_f(k),
        decreases n,
    {
        if n > 0 {
            if k == n - 1 {
                lemma_sum_of_ext(old_f, new_f, n - 1);
            } else {
                assert(old_f(n - 1) == new_f(n - 1));
                lemma_sum_of_change_one(n - 1, old_f, new_f, k);
            }
        }
    }

    /// Lower bound: the sum of nats is at least any single term.
    proof fn lemma_sum_of_lower_bound(n: int, f: spec_fn(int) -> nat, k: int)
        requires 0 <= k < n
        ensures spec_sum_of(n, f) >= f(k)
        decreases n
    {
        if k == n - 1 {
        } else {
            lemma_sum_of_lower_bound(n - 1, f, k);
        }
    }

    /// Upper bound: if each f(i) ≤ bound, then sum ≤ n * bound.
    proof fn lemma_sum_of_bounded(n: int, f: spec_fn(int) -> nat, bound: nat)
        requires
            n >= 0,
            forall|i: int| 0 <= i < n ==> #[trigger] f(i) <= bound,
        ensures spec_sum_of(n, f) <= (n as nat) * bound
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) <= bound);
            lemma_sum_of_bounded(n - 1, f, bound);
            assert(spec_sum_of(n, f) <= bound + ((n - 1) as nat) * bound);
            assert((n as nat) * bound == bound + ((n - 1) as nat) * bound) by(nonlinear_arith)
                requires n >= 1;
        }
    }

    // 8. traits

    pub trait AdjSeqGraphStEphTrait: Sized {
        spec fn spec_adjseqgraphsteph_wf(&self) -> bool;
        spec fn spec_num_vertices(&self) -> nat;
        spec fn spec_degree(&self, u: int) -> nat
            recommends 0 <= u < self.spec_num_vertices();
        spec fn spec_neighbor(&self, u: int, j: int) -> usize
            recommends 0 <= u < self.spec_num_vertices(), 0 <= j < self.spec_degree(u);

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(n), Span O(n) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(n), Span Theta(n) — agrees; tabulate over n empty sequences.
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_adjseqgraphsteph_wf(),
                empty.spec_num_vertices() == n,
                forall|i: int| 0 <= i < n ==> #[trigger] empty.spec_degree(i) == 0;

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — wraps existing array.
        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (constructed: Self)
            requires
                forall|u: int, j: int|
                    0 <= u < adj.spec_len()
                    && 0 <= j < adj.spec_index(u).spec_len()
                    ==> #[trigger] adj.spec_index(u).spec_index(j) < adj.spec_len(),
                spec_sum_of(adj.spec_len() as int, |i: int| adj.spec_index(i).spec_len()) <= usize::MAX as nat,
            ensures
                constructed.spec_adjseqgraphsteph_wf(),
                constructed.spec_num_vertices() == adj.spec_len(),
                forall|i: int| 0 <= i < adj.spec_len() ==>
                    #[trigger] constructed.spec_degree(i) == adj.spec_index(i).spec_len(),
                forall|i: int, j: int| 0 <= i < adj.spec_len()
                    && 0 <= j < adj.spec_index(i).spec_len()
                    ==> #[trigger] constructed.spec_neighbor(i, j) == adj.spec_index(i).spec_index(j);

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; seq len
        fn num_vertices(&self) -> (n: usize)
            requires self.spec_adjseqgraphsteph_wf()
            ensures n as nat == self.spec_num_vertices();

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; cached field
        fn num_edges(&self) -> (m: usize)
            requires
                self.spec_adjseqgraphsteph_wf()
            ensures
                m as nat == spec_sum_of(
                    self.spec_num_vertices() as int,
                    |i: int| self.spec_degree(i),
                );

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(d_g(u)), Span O(lg d_g(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d_g(u)), Span O(d_g(u)) — matches APAS work; sequential scan
        fn has_edge(&self, u: usize, v: usize) -> (found: bool)
            requires self.spec_adjseqgraphsteph_wf(), u < self.spec_num_vertices()
            ensures found == exists|j: int|
                0 <= j < self.spec_degree(u as int)
                && #[trigger] self.spec_neighbor(u as int, j) == v;

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(d_g(v)), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d_g(v)), Span O(d_g(v)) — matches APAS work; tabulate copy
        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqStEphS<usize>)
            requires self.spec_adjseqgraphsteph_wf(), u < self.spec_num_vertices()
            ensures
                neighbors.spec_len() == self.spec_degree(u as int),
                forall|j: int| 0 <= j < neighbors.spec_len()
                    ==> #[trigger] neighbors.spec_index(j) == self.spec_neighbor(u as int, j);

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS; inner seq len
        fn out_degree(&self, u: usize) -> (d: usize)
            requires self.spec_adjseqgraphsteph_wf(), u < self.spec_num_vertices()
            ensures d as nat == self.spec_degree(u as int);

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; single array set.
        fn set_neighbors(&mut self, v: usize, neighbors: ArraySeqStEphS<usize>)
            requires
                old(self).spec_adjseqgraphsteph_wf(),
                v < old(self).spec_num_vertices(),
                forall|j: int| 0 <= j < neighbors.spec_len()
                    ==> #[trigger] neighbors.spec_index(j) < old(self).spec_num_vertices(),
                spec_sum_of(old(self).spec_num_vertices() as int, |i: int| old(self).spec_degree(i)) < usize::MAX as nat,
                spec_sum_of(old(self).spec_num_vertices() as int, |i: int| old(self).spec_degree(i))
                    - old(self).spec_degree(v as int)
                    + neighbors.spec_len() <= usize::MAX as nat,
            ensures
                self.spec_adjseqgraphsteph_wf(),
                self.spec_num_vertices() == old(self).spec_num_vertices(),
                self.spec_degree(v as int) == neighbors.spec_len(),
                forall|j: int| 0 <= j < neighbors.spec_len()
                    ==> #[trigger] self.spec_neighbor(v as int, j) == neighbors.spec_index(j),
                forall|i: int| 0 <= i < old(self).spec_num_vertices() && i != v as int
                    ==> #[trigger] self.spec_degree(i) == old(self).spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < old(self).spec_num_vertices() && i != v as int
                    && 0 <= j < old(self).spec_degree(i)
                    ==> #[trigger] self.spec_neighbor(i, j) == old(self).spec_neighbor(i, j);

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(n + deg(u)), Span O(n + deg(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + deg(u) — matches APAS
        /// - Claude-Opus-4.6: Delegates to set_edge(u, v, true).
        fn insert_edge(&mut self, u: usize, v: usize)
            requires
                old(self).spec_adjseqgraphsteph_wf(),
                u < old(self).spec_num_vertices(),
                v < old(self).spec_num_vertices(),
                spec_sum_of(old(self).spec_num_vertices() as int, |i: int| old(self).spec_degree(i)) < usize::MAX as nat,
            ensures
                self.spec_adjseqgraphsteph_wf(),
                self.spec_num_vertices() == old(self).spec_num_vertices(),
                forall|i: int| 0 <= i < old(self).spec_num_vertices() && i != u as int
                    ==> #[trigger] self.spec_degree(i) == old(self).spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < old(self).spec_num_vertices() && i != u as int
                    && 0 <= j < old(self).spec_degree(i)
                    ==> #[trigger] self.spec_neighbor(i, j) == old(self).spec_neighbor(i, j),
                exists|j: int|
                    0 <= j < self.spec_degree(u as int)
                    && #[trigger] self.spec_neighbor(u as int, j) == v;

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(n + deg(u)), Span O(n + deg(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + deg(u) — matches APAS
        /// - Claude-Opus-4.6: Delegates to set_edge(u, v, false).
        fn delete_edge(&mut self, u: usize, v: usize)
            requires
                old(self).spec_adjseqgraphsteph_wf(),
                u < old(self).spec_num_vertices(),
                v < old(self).spec_num_vertices(),
                spec_sum_of(old(self).spec_num_vertices() as int, |i: int| old(self).spec_degree(i)) < usize::MAX as nat,
            ensures
                self.spec_adjseqgraphsteph_wf(),
                self.spec_num_vertices() == old(self).spec_num_vertices(),
                forall|i: int| 0 <= i < old(self).spec_num_vertices() && i != u as int
                    ==> #[trigger] self.spec_degree(i) == old(self).spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < old(self).spec_num_vertices() && i != u as int
                    && 0 <= j < old(self).spec_degree(i)
                    ==> #[trigger] self.spec_neighbor(i, j) == old(self).spec_neighbor(i, j),
                forall|j: int|
                    0 <= j < self.spec_degree(u as int)
                    ==> #[trigger] self.spec_neighbor(u as int, j) != v;

        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(n), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + d_g(u)), Span O(n + d_g(u)) — DIFFERS: APAS says O(n), O(1); impl rebuilds neighbor list sequentially
        fn set_edge(&mut self, u: usize, v: usize, exists: bool)
            requires
                old(self).spec_adjseqgraphsteph_wf(),
                u < old(self).spec_num_vertices(),
                v < old(self).spec_num_vertices(),
                spec_sum_of(old(self).spec_num_vertices() as int, |i: int| old(self).spec_degree(i)) < usize::MAX as nat,
            ensures
                self.spec_adjseqgraphsteph_wf(),
                self.spec_num_vertices() == old(self).spec_num_vertices(),
                forall|i: int| 0 <= i < old(self).spec_num_vertices() && i != u as int
                    ==> #[trigger] self.spec_degree(i) == old(self).spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < old(self).spec_num_vertices() && i != u as int
                    && 0 <= j < old(self).spec_degree(i)
                    ==> #[trigger] self.spec_neighbor(i, j) == old(self).spec_neighbor(i, j),
                exists ==> (exists|j: int|
                    0 <= j < self.spec_degree(u as int)
                    && #[trigger] self.spec_neighbor(u as int, j) == v),
                !exists ==> forall|j: int|
                    0 <= j < self.spec_degree(u as int)
                    ==> #[trigger] self.spec_neighbor(u as int, j) != v;
    }

    // 9. impls

    impl AdjSeqGraphStEphTrait for AdjSeqGraphStEph {

        open spec fn spec_adjseqgraphsteph_wf(&self) -> bool {
            &&& (forall|u: int, j: int|
                0 <= u < self.adj.spec_len()
                && 0 <= j < self.adj.spec_index(u).spec_len()
                ==> #[trigger] self.adj.spec_index(u).spec_index(j) < self.adj.spec_len())
            &&& self.num_edges as nat == spec_sum_of(self.spec_num_vertices() as int, |i: int| self.spec_degree(i))
        }

        open spec fn spec_num_vertices(&self) -> nat {
            self.adj.spec_len()
        }

        open spec fn spec_degree(&self, u: int) -> nat {
            self.adj.spec_index(u).spec_len()
        }

        open spec fn spec_neighbor(&self, u: int, j: int) -> usize {
            self.adj.spec_index(u).spec_index(j)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn new(n: usize) -> (empty: Self) {
            let adj = ArraySeqStEphS::tabulate(
                &|_i: usize| -> (r: ArraySeqStEphS<usize>)
                    ensures r.spec_len() == 0
                {
                    ArraySeqStEphS::empty()
                },
                n,
            );
            let empty = AdjSeqGraphStEph { adj, num_edges: 0 };
            proof {
                let degree_fn = |i: int| empty.spec_degree(i);
                assert forall|i: int| 0 <= i < n implies #[trigger] degree_fn(i) == 0nat by {};
                lemma_sum_of_all_zero(degree_fn, n as int);
            }
            empty
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (constructed: Self) {
            let n = adj.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            let ghost degree_fn: spec_fn(int) -> nat = |k: int| adj.spec_index(k).spec_len();
            while i < n
                invariant
                    i <= n,
                    n as nat == adj.spec_len(),
                    count as nat == spec_sum_of(i as int, degree_fn),
                    degree_fn == (|k: int| adj.spec_index(k).spec_len()),
                    spec_sum_of(n as int, degree_fn) <= usize::MAX as nat,
                decreases n - i
            {
                proof {
                    lemma_sum_of_unfold(i as int, degree_fn);
                    lemma_sum_of_monotone(i as int + 1, n as int, degree_fn);
                }
                let deg = adj.nth(i).length();
                count = count + deg;
                i = i + 1;
            }
            let constructed = AdjSeqGraphStEph { adj, num_edges: count };
            proof {
                let wf_degree = |i: int| constructed.spec_degree(i);
                assert forall|i: int| 0 <= i < n implies #[trigger] degree_fn(i) == wf_degree(i) by {};
                lemma_sum_of_ext(degree_fn, wf_degree, n as int);
            }
            constructed
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> (n: usize) {
            self.adj.length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_edges(&self) -> (m: usize) {
            self.num_edges
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn has_edge(&self, u: usize, v: usize) -> (found: bool) {
            let neighbors = self.adj.nth(u);
            let len = neighbors.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    u < self.spec_num_vertices(),
                    len as nat == neighbors.spec_len(),
                    len as nat == self.spec_degree(u as int),
                    forall|j: int| 0 <= j < len as int
                        ==> #[trigger] neighbors.spec_index(j) == self.spec_neighbor(u as int, j),
                    forall|j: int| 0 <= j < i
                        ==> #[trigger] neighbors.spec_index(j) != v,
                decreases len - i
            {
                if *neighbors.nth(i) == v {
                    assert(self.spec_neighbor(u as int, i as int) == v);
                    return true;
                }
                i = i + 1;
            }
            false
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqStEphS<usize>) {
            let src = self.adj.nth(u);
            let len = src.length();
            ArraySeqStEphS::tabulate(
                &|i: usize| -> (r: usize)
                    requires i < len
                    ensures r == src.spec_index(i as int)
                {
                    *src.nth(i)
                },
                len,
            )
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn out_degree(&self, u: usize) -> (d: usize) {
            self.adj.nth(u).length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn set_neighbors(&mut self, v: usize, neighbors: ArraySeqStEphS<usize>) {
            let ghost n = self.spec_num_vertices();
            let old_deg = self.adj.nth(v).length();
            let new_deg = neighbors.length();
            let ghost old_degree_fn: spec_fn(int) -> nat = |i: int| self.spec_degree(i);
            let _ = self.adj.set(v, neighbors);
            assert forall|u: int, j: int|
                0 <= u < self.adj.spec_len()
                && 0 <= j < self.adj.spec_index(u).spec_len()
            implies #[trigger] self.adj.spec_index(u).spec_index(j) < self.adj.spec_len()
            by {
                if u != v as int {
                    assert(self.adj.spec_index(u) == old(self).adj.spec_index(u));
                }
            }
            // Prove overflow safety before subtraction.
            proof {
                lemma_sum_of_lower_bound(n as int, old_degree_fn, v as int);
            }
            // Compute updated num_edges: old_count - old_deg + new_deg.
            if new_deg >= old_deg {
                self.num_edges = self.num_edges + (new_deg - old_deg);
            } else {
                self.num_edges = self.num_edges - (old_deg - new_deg);
            }
            // Prove wf: self.num_edges == spec_sum_of(n, |i| self.spec_degree(i)).
            proof {
                let new_degree_fn: spec_fn(int) -> nat = |i: int| self.spec_degree(i);
                assert forall|i: int| 0 <= i < n as int && i != v as int
                    implies #[trigger] old_degree_fn(i) == new_degree_fn(i)
                by {
                    assert(self.adj.spec_index(i) == old(self).adj.spec_index(i));
                }
                assert(old_degree_fn(v as int) == old_deg as nat);
                assert(new_degree_fn(v as int) == new_deg as nat);
                lemma_sum_of_change_one(n as int, old_degree_fn, new_degree_fn, v as int);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn insert_edge(&mut self, u: usize, v: usize) {
            self.set_edge(u, v, true);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn delete_edge(&mut self, u: usize, v: usize) {
            self.set_edge(u, v, false);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn set_edge(&mut self, u: usize, v: usize, exists: bool) {
            let ghost old_degree_fn: spec_fn(int) -> nat = |i: int| self.spec_degree(i);
            let ghost adj_len = self.adj.spec_len();
            let ghost n = self.spec_num_vertices();

            if exists {
                let old_len = self.adj.nth(u).length();
                let mut found = false;
                let mut i: usize = 0;
                while i < old_len
                    invariant
                        i <= old_len,
                        u < self.spec_num_vertices(),
                        old_len as nat == self.spec_degree(u as int),
                        !found ==> forall|j: int| 0 <= j < i
                            ==> #[trigger] self.spec_neighbor(u as int, j) != v,
                        found ==> exists|j: int| 0 <= j < self.spec_degree(u as int)
                            && #[trigger] self.spec_neighbor(u as int, j) == v,
                    decreases old_len - i
                {
                    let elem = *self.adj.nth(u).nth(i);
                    if elem == v {
                        assert(self.spec_neighbor(u as int, i as int) == v);
                        found = true;
                        break;
                    }
                    i = i + 1;
                }

                if !found {
                    let mut new_vec = Vec::<usize>::new();
                    let mut j: usize = 0;
                    while j < old_len
                        invariant
                            j <= old_len,
                            u < self.spec_num_vertices(),
                            old_len as nat == self.spec_degree(u as int),
                            new_vec@.len() == j as int,
                            forall|k: int| 0 <= k < j
                                ==> #[trigger] new_vec@[k] == self.spec_neighbor(u as int, k),
                            adj_len == self.adj.spec_len(),
                            old(self).spec_adjseqgraphsteph_wf(),
                            forall|k: int| 0 <= k < new_vec@.len() as int
                                ==> new_vec@[k] < adj_len,
                        decreases old_len - j
                    {
                        new_vec.push(*self.adj.nth(u).nth(j));
                        j = j + 1;
                    }
                    new_vec.push(v);
                    let new_neighbors = ArraySeqStEphS::from_vec(new_vec);
                    let _ = self.adj.set(u, new_neighbors);
                    assert(self.spec_degree(u as int) == old_len as nat + 1);
                    assert(self.spec_neighbor(u as int, old_len as int) == v);
                    assert forall|u2: int, j2: int|
                        0 <= u2 < self.adj.spec_len()
                        && 0 <= j2 < self.adj.spec_index(u2).spec_len()
                    implies #[trigger] self.adj.spec_index(u2).spec_index(j2) < self.adj.spec_len()
                    by {
                        if u2 != u as int {
                            assert(self.adj.spec_index(u2) == old(self).adj.spec_index(u2));
                        }
                    }
                    self.num_edges = self.num_edges + 1;
                    // Re-assert postconditions after num_edges mutation.
                    assert(self.spec_degree(u as int) == old_len as nat + 1);
                    assert(self.spec_neighbor(u as int, old_len as int) == v);
                    proof {
                        let new_degree_fn: spec_fn(int) -> nat = |i: int| self.spec_degree(i);
                        assert forall|i: int| 0 <= i < n as int && i != u as int
                            implies #[trigger] old_degree_fn(i) == new_degree_fn(i)
                        by {
                            assert(self.adj.spec_index(i) == old(self).adj.spec_index(i));
                        }
                        assert(old_degree_fn(u as int) == old_len as nat);
                        assert(new_degree_fn(u as int) == old_len as nat + 1);
                        lemma_sum_of_change_one(n as int, old_degree_fn, new_degree_fn, u as int);
                    }
                }
                // If found, edge already present; num_edges unchanged.
            } else {
                let old_len = self.adj.nth(u).length();
                let mut new_vec = Vec::<usize>::new();
                let mut j: usize = 0;
                while j < old_len
                    invariant
                        j <= old_len,
                        u < self.spec_num_vertices(),
                        old_len as nat == self.spec_degree(u as int),
                        new_vec@.len() <= j as nat,
                        forall|k: int| 0 <= k < new_vec@.len() as int
                            ==> #[trigger] new_vec@[k] != v,
                        adj_len == self.adj.spec_len(),
                        old(self).spec_adjseqgraphsteph_wf(),
                        forall|k: int| 0 <= k < new_vec@.len() as int
                            ==> new_vec@[k] < adj_len,
                    decreases old_len - j
                {
                    let neighbor = *self.adj.nth(u).nth(j);
                    if neighbor != v {
                        new_vec.push(neighbor);
                    }
                    j = j + 1;
                }
                let new_len = new_vec.len();
                let new_neighbors = ArraySeqStEphS::from_vec(new_vec);
                let _ = self.adj.set(u, new_neighbors);
                assert forall|u2: int, j2: int|
                    0 <= u2 < self.adj.spec_len()
                    && 0 <= j2 < self.adj.spec_index(u2).spec_len()
                implies #[trigger] self.adj.spec_index(u2).spec_index(j2) < self.adj.spec_len()
                by {
                    if u2 != u as int {
                        assert(self.adj.spec_index(u2) == old(self).adj.spec_index(u2));
                    }
                }
                proof {
                    lemma_sum_of_lower_bound(n as int, old_degree_fn, u as int);
                    assert(new_len <= old_len);
                }
                self.num_edges = self.num_edges - (old_len - new_len);
                proof {
                    let new_degree_fn: spec_fn(int) -> nat = |i: int| self.spec_degree(i);
                    assert forall|i: int| 0 <= i < n as int && i != u as int
                        implies #[trigger] old_degree_fn(i) == new_degree_fn(i)
                    by {
                        assert(self.adj.spec_index(i) == old(self).adj.spec_index(i));
                    }
                    assert(old_degree_fn(u as int) == old_len as nat);
                    assert(new_degree_fn(u as int) == new_len as nat);
                    lemma_sum_of_change_one(n as int, old_degree_fn, new_degree_fn, u as int);
                }
            }
        }
    }

    // 11. derive impls in verus!

    impl Clone for AdjSeqGraphStEph {
        fn clone(&self) -> (out: Self) {
            AdjSeqGraphStEph { adj: self.adj.clone(), num_edges: self.num_edges }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for AdjSeqGraphStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjSeqGraphStEph")
                .field("adj", &self.adj)
                .field("num_edges", &self.num_edges)
                .finish()
        }
    }

    impl std::fmt::Display for AdjSeqGraphStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjSeqGraphStEph(n: {}, edges: {})", self.adj.length(), self.num_edges)
        }
    }
}
