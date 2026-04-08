//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 52: Adjacency Matrix Graph (ephemeral, single-threaded).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod AdjMatrixGraphStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


    pub struct AdjMatrixGraphStEph {
        pub matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>,
        pub n: usize,
        pub num_edges: usize,
    }

    //		Section 5. view impls


    impl View for AdjMatrixGraphStEph {
        type V = Seq<Seq<bool>>;
        open spec fn view(&self) -> Self::V {
            self.matrix@
        }
    }

    //		Section 6. spec fns


    /// Count how many of f(0), f(1), ..., f(n-1) are true.
    pub open spec fn spec_count_true(f: spec_fn(int) -> bool, n: int) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else if f(n - 1) { 1 + spec_count_true(f, n - 1) }
        else { spec_count_true(f, n - 1) }
    }

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    /// A well-formed adjacency matrix: square n x n.
    pub open spec fn spec_adjmatrixgraphsteph_wf(g: AdjMatrixGraphStEph) -> bool {
        g.matrix.spec_len() == g.n
        && forall|i: int| 0 <= i < g.n ==>
            #[trigger] g.matrix.spec_index(i).spec_len() == g.n
    }

    //		Section 7. proof fns/broadcast groups


    proof fn lemma_count_true_monotone(f: spec_fn(int) -> bool, i: int, n: int)
        requires 0 <= i <= n
        ensures spec_count_true(f, i) <= spec_count_true(f, n)
        decreases n - i
    {
        if i < n {
            lemma_count_true_monotone(f, i, n - 1);
        }
    }

    proof fn lemma_sum_of_monotone(i: int, n: int, f: spec_fn(int) -> nat)
        requires 0 <= i <= n
        ensures spec_sum_of(i, f) <= spec_sum_of(n, f)
        decreases n - i
    {
        if i < n {
            lemma_sum_of_monotone(i, n - 1, f);
        }
    }

    proof fn lemma_count_true_bound(f: spec_fn(int) -> bool, n: int)
        requires n >= 0
        ensures spec_count_true(f, n) <= n as nat
        decreases n
    {
        if n > 0 {
            lemma_count_true_bound(f, n - 1);
        }
    }

    /// Count of an all-false predicate is zero.
    proof fn lemma_count_true_all_false(f: spec_fn(int) -> bool, n: int)
        requires forall|i: int| 0 <= i < n ==> !#[trigger] f(i)
        ensures spec_count_true(f, n) == 0
        decreases n
    {
        if n > 0 {
            assert(!f(n - 1));
            lemma_count_true_all_false(f, n - 1);
        }
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

    /// Extensionality for spec_count_true: identical predicates yield identical counts.
    proof fn lemma_count_true_ext(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, n: int)
        requires forall|i: int| 0 <= i < n ==> #[trigger] f(i) == g(i)
        ensures spec_count_true(f, n) == spec_count_true(g, n)
        decreases n
    {
        if n > 0 {
            assert(f(n - 1) == g(n - 1));
            lemma_count_true_ext(f, g, n - 1);
        }
    }

    /// Flipping one position from false to true increases count by 1.
    proof fn lemma_count_true_set_true(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, k: int, n: int)
        requires
            0 <= k < n,
            !f(k) && g(k),
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] f(i) == g(i),
        ensures spec_count_true(g, n) == spec_count_true(f, n) + 1
        decreases n
    {
        if n > 0 {
            if k == n - 1 {
                lemma_count_true_ext(f, g, n - 1);
            } else {
                assert(f(n - 1) == g(n - 1));
                lemma_count_true_set_true(f, g, k, n - 1);
            }
        }
    }

    /// Flipping one position from true to false decreases count by 1.
    proof fn lemma_count_true_set_false(f: spec_fn(int) -> bool, g: spec_fn(int) -> bool, k: int, n: int)
        requires
            0 <= k < n,
            f(k) && !g(k),
            forall|i: int| 0 <= i < n && i != k ==> #[trigger] f(i) == g(i),
        ensures spec_count_true(f, n) == spec_count_true(g, n) + 1
        decreases n
    {
        if n > 0 {
            if k == n - 1 {
                lemma_count_true_ext(f, g, n - 1);
            } else {
                assert(f(n - 1) == g(n - 1));
                lemma_count_true_set_false(f, g, k, n - 1);
            }
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

    /// Count is at least 1 if the predicate holds at some position.
    proof fn lemma_count_true_at_least_one(f: spec_fn(int) -> bool, k: int, n: int)
        requires 0 <= k < n, f(k)
        ensures spec_count_true(f, n) >= 1
        decreases n
    {
        if k == n - 1 {
        } else {
            lemma_count_true_at_least_one(f, k, n - 1);
        }
    }

    //		Section 8. traits


    pub trait AdjMatrixGraphStEphTrait: Sized {
        spec fn spec_adjmatrixgraphsteph_wf(&self) -> bool;
        spec fn spec_n(&self) -> nat;
        spec fn spec_edge(&self, u: int, v: int) -> bool
            recommends 0 <= u < self.spec_n(), 0 <= v < self.spec_n();

        /// - Alg Analysis: APAS (Ch52 CS 52.6): (no explicit new cost; matrix init is n^2)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n^2), Span Theta(n^2) — tabulate n rows of n booleans.
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_adjmatrixgraphsteph_wf(),
                empty.spec_n() == n,
                forall|u: int, v: int|
                    0 <= u < n && 0 <= v < n ==> !#[trigger] empty.spec_edge(u, v);

        /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Theta(n^2), Span Theta(n^2) — counts edges during construction.
        fn from_matrix(matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>) -> (constructed: Self)
            requires
                forall|i: int| 0 <= i < matrix.spec_len() ==>
                    #[trigger] matrix.spec_index(i).spec_len() == matrix.spec_len(),
                spec_sum_of(
                    matrix.spec_len() as int,
                    |u: int| spec_count_true(|v: int| matrix.spec_index(u).spec_index(v), matrix.spec_len() as int),
                ) <= usize::MAX as nat,
            ensures
                constructed.spec_adjmatrixgraphsteph_wf(),
                constructed.spec_n() == matrix.spec_len(),
                forall|u: int, v: int|
                    0 <= u < matrix.spec_len() && 0 <= v < matrix.spec_len()
                    ==> #[trigger] constructed.spec_edge(u, v) == matrix.spec_index(u).spec_index(v);

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); cached vertex count
        fn num_vertices(&self) -> (n: usize)
            requires self.spec_adjmatrixgraphsteph_wf()
            ensures n as nat == self.spec_n();

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); cached edge count
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjmatrixgraphsteph_wf()
            ensures
                m as nat == spec_sum_of(
                    self.spec_n() as int,
                    |u: int| spec_count_true(|v: int| self.spec_edge(u, v), self.spec_n() as int),
                );

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); direct matrix index
        fn has_edge(&self, u: usize, v: usize) -> (found: bool)
            requires self.spec_adjmatrixgraphsteph_wf(), u < self.spec_n(), v < self.spec_n()
            ensures found == self.spec_edge(u as int, v as int);

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) work; sequential row scan
        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqStEphS<usize>)
            requires self.spec_adjmatrixgraphsteph_wf(), u < self.spec_n()
            ensures
                forall|k: int| 0 <= k < neighbors.spec_len()
                    ==> #[trigger] neighbors.spec_index(k) < self.spec_n()
                        && self.spec_edge(u as int, neighbors.spec_index(k) as int),
                forall|v: int| 0 <= v < self.spec_n() && #[trigger] self.spec_edge(u as int, v)
                    ==> exists|k: int|
                        0 <= k < neighbors.spec_len() && #[trigger] neighbors.spec_index(k) == v as usize;

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) work; sequential count
        fn out_degree(&self, u: usize) -> (d: usize)
            requires self.spec_adjmatrixgraphsteph_wf(), u < self.spec_n()
            ensures d as nat == spec_count_true(
                |v: int| self.spec_edge(u as int, v),
                self.spec_n() as int,
            );

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); in-place matrix update
        fn set_edge(&mut self, u: usize, v: usize, exists: bool)
            requires
                old(self).spec_adjmatrixgraphsteph_wf(),
                u < old(self).spec_n(),
                v < old(self).spec_n(),
                old(self).spec_n() * old(self).spec_n() < usize::MAX as nat,
            ensures
                self.spec_adjmatrixgraphsteph_wf(),
                self.spec_n() == old(self).spec_n(),
                self.spec_edge(u as int, v as int) == exists,
                forall|i: int, j: int|
                    0 <= i < old(self).spec_n() && 0 <= j < old(self).spec_n()
                    && !(i == u as int && j == v as int)
                    ==> #[trigger] self.spec_edge(i, j) == old(self).spec_edge(i, j);

        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n^2), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) work; negate all entries
        fn complement(&self) -> (complemented: Self)
            requires
                self.spec_adjmatrixgraphsteph_wf(),
                self.spec_n() * self.spec_n() <= usize::MAX as nat,
            ensures
                complemented.spec_adjmatrixgraphsteph_wf(),
                complemented.spec_n() == self.spec_n(),
                forall|i: int, j: int|
                    0 <= i < self.spec_n() && 0 <= j < self.spec_n()
                    ==> #[trigger] complemented.spec_edge(i, j) == (i != j && !self.spec_edge(i, j));
    }

    //		Section 9. impls


    impl AdjMatrixGraphStEphTrait for AdjMatrixGraphStEph {

        open spec fn spec_adjmatrixgraphsteph_wf(&self) -> bool {
            &&& self.matrix.spec_len() == self.n
            &&& forall|i: int| 0 <= i < self.n ==>
                #[trigger] self.matrix.spec_index(i).spec_len() == self.n
            &&& self.num_edges as nat == spec_sum_of(
                self.spec_n() as int,
                |u: int| spec_count_true(|v: int| self.spec_edge(u, v), self.spec_n() as int),
            )
        }

        open spec fn spec_n(&self) -> nat { self.n as nat }

        open spec fn spec_edge(&self, u: int, v: int) -> bool {
            self.matrix.spec_index(u).spec_index(v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn new(n: usize) -> (empty: Self) {
            let false_row = ArraySeqStEphS::tabulate(
                &|_j: usize| -> (r: bool) ensures !r { false },
                n,
            );
            let matrix = ArraySeqStEphS::tabulate(
                &|_i: usize| -> (r: ArraySeqStEphS<bool>)
                    ensures
                        r.spec_len() == n,
                        forall|j: int| 0 <= j < n ==> !#[trigger] r.spec_index(j)
                {
                    ArraySeqStEphS::tabulate(
                        &|_j: usize| -> (r: bool) ensures !r { false },
                        n,
                    )
                },
                n,
            );
            let empty = AdjMatrixGraphStEph { matrix, n, num_edges: 0 };
            proof {
                // Each row has zero true entries, so each row count is 0.
                let row_count = |u: int| spec_count_true(|v: int| empty.spec_edge(u, v), empty.spec_n() as int);
                assert forall|u: int| 0 <= u < n implies #[trigger] row_count(u) == 0nat by {
                    let edge_fn = |v: int| empty.spec_edge(u, v);
                    assert forall|v: int| 0 <= v < n implies !#[trigger] edge_fn(v) by {};
                    lemma_count_true_all_false(edge_fn, n as int);
                };
                lemma_sum_of_all_zero(row_count, n as int);
            }
            empty
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — counts edges during construction
        fn from_matrix(matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>) -> (constructed: Self) {
            let n = matrix.length();
            let ghost row_count = |u: int| spec_count_true(|v: int| matrix.spec_index(u).spec_index(v), n as int);
            // Count total edges in the matrix.
            let mut total: usize = 0;
            let mut u: usize = 0;
            while u < n
                invariant
                    u <= n,
                    n as nat == matrix.spec_len(),
                    forall|i: int| 0 <= i < n ==> #[trigger] matrix.spec_index(i).spec_len() == n,
                    total as nat == spec_sum_of(u as int, row_count),
                    row_count == (|u: int| spec_count_true(|v: int| matrix.spec_index(u).spec_index(v), n as int)),
                    spec_sum_of(n as int, row_count) <= usize::MAX as nat,
                decreases n - u
            {
                proof { lemma_sum_of_monotone(u as int + 1, n as int, row_count); }
                let row = matrix.nth(u);
                let mut count: usize = 0;
                let mut v: usize = 0;
                let ghost edge_fn = |v: int| matrix.spec_index(u as int).spec_index(v);
                while v < n
                    invariant
                        v <= n,
                        n as nat == matrix.spec_len(),
                        u < n,
                        row.spec_len() == n,
                        forall|vi: int| 0 <= vi < n ==> #[trigger] row.spec_index(vi) == matrix.spec_index(u as int).spec_index(vi),
                        count as nat == spec_count_true(edge_fn, v as int),
                        edge_fn == (|v: int| matrix.spec_index(u as int).spec_index(v)),
                        spec_count_true(edge_fn, n as int) <= usize::MAX as nat,
                    decreases n - v
                {
                    proof { lemma_count_true_monotone(edge_fn, v as int + 1, n as int); }
                    if *row.nth(v) {
                        count = count + 1;
                    }
                    v = v + 1;
                }
                total = total + count;
                u = u + 1;
            }
            // Connect the loop's row_count (using matrix directly) to the wf's row_count (using spec_edge).
            let constructed = AdjMatrixGraphStEph { matrix, n, num_edges: total };
            proof {
                let wf_row_count = |u: int| spec_count_true(|v: int| constructed.spec_edge(u, v), constructed.spec_n() as int);
                assert forall|u: int| 0 <= u < n implies #[trigger] row_count(u) == wf_row_count(u) by {
                    let inner_mat = |v: int| matrix.spec_index(u).spec_index(v);
                    let inner_edge = |v: int| constructed.spec_edge(u, v);
                    assert forall|v: int| 0 <= v < n implies #[trigger] inner_mat(v) == inner_edge(v) by {};
                    lemma_count_true_ext(inner_mat, inner_edge, n as int);
                };
                lemma_sum_of_ext(row_count, wf_row_count, n as int);
            }
            constructed
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> (n: usize) { self.n }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); cached edge count
        fn num_edges(&self) -> (m: usize) {
            // wf directly contains: self.num_edges == spec_sum_of(spec_n, |u| count_true(|v| spec_edge(u,v), spec_n))
            self.num_edges
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn has_edge(&self, u: usize, v: usize) -> (found: bool) {
            *self.matrix.nth(u).nth(v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqStEphS<usize>) {
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut nvec = Vec::<usize>::new();
            let mut v: usize = 0;
            while v < n
                invariant
                    v <= n,
                    self.spec_adjmatrixgraphsteph_wf(),
                    n as nat == self.spec_n(),
                    u < self.spec_n(),
                    row.spec_len() == n,
                    forall|vi: int| 0 <= vi < n ==> #[trigger] row.spec_index(vi) == self.spec_edge(u as int, vi),
                    forall|k: int| 0 <= k < nvec@.len() as int
                        ==> #[trigger] nvec@[k] < n
                            && self.spec_edge(u as int, nvec@[k] as int),
                    forall|j: int| 0 <= j < v && #[trigger] self.spec_edge(u as int, j)
                        ==> exists|k: int|
                            0 <= k < nvec@.len() as int && #[trigger] nvec@[k] == j as usize,
                decreases n - v
            {
                let val = *row.nth(v);
                assert(val == self.spec_edge(u as int, v as int));
                let ghost pre_push = nvec@;
                let ghost old_nvec_len = nvec@.len();
                if val {
                    nvec.push(v);
                }
                proof {
                    assert forall|k: int| 0 <= k < old_nvec_len as int
                        implies nvec@[k] == #[trigger] pre_push[k]
                    by {};
                    assert forall|j: int| 0 <= j < (v as int + 1) && self.spec_edge(u as int, j)
                        implies exists|k: int| 0 <= k < nvec@.len() as int && nvec@[k] == j as usize
                    by {
                        if j < v as int {
                            let witness = choose|k: int| 0 <= k < old_nvec_len as int && pre_push[k] == j as usize;
                            assert(nvec@[witness] == j as usize);
                        } else {
                            assert(nvec@[old_nvec_len as int] == v as usize);
                        }
                    }
                }
                v = v + 1;
            }
            let ghost nvec_view = nvec@;
            let neighbors = ArraySeqStEphS::from_vec(nvec);
            proof {
                assert forall|j: int| 0 <= j < (n as int) && self.spec_edge(u as int, j)
                    implies exists|k: int| 0 <= k < neighbors.spec_len() && neighbors.spec_index(k) == j as usize
                by {
                    let witness = choose|k: int| 0 <= k < nvec_view.len() as int && nvec_view[k] == j as usize;
                    assert(neighbors.spec_index(witness) == nvec_view[witness]);
                }
            }
            neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn out_degree(&self, u: usize) -> (d: usize) {
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut count: usize = 0;
            let mut v: usize = 0;
            let ghost edge_fn = |v: int| self.spec_edge(u as int, v);
            proof { lemma_count_true_bound(edge_fn, n as int); }
            while v < n
                invariant
                    v <= n,
                    self.spec_adjmatrixgraphsteph_wf(),
                    n as nat == self.spec_n(),
                    row.spec_len() == n,
                    forall|vi: int| 0 <= vi < n ==> #[trigger] row.spec_index(vi) == self.spec_edge(u as int, vi),
                    count as nat == spec_count_true(edge_fn, v as int),
                    edge_fn == (|v: int| self.spec_edge(u as int, v)),
                    spec_count_true(edge_fn, n as int) <= n as nat,
                decreases n - v
            {
                proof { lemma_count_true_monotone(edge_fn, v as int + 1, n as int); }
                if *row.nth(v) {
                    count = count + 1;
                }
                v = v + 1;
            }
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn set_edge(&mut self, u: usize, v: usize, exists: bool) {
            let n = self.n;
            let old_val = *self.matrix.nth(u).nth(v);
            let ghost old_num_edges = self.num_edges;
            let ghost old_row_count = |r: int| spec_count_true(|c: int| self.spec_edge(r, c), self.spec_n() as int);
            let new_row = ArraySeqStEphS::tabulate(
                &|j: usize| -> (r: bool)
                    requires j < n
                    ensures
                        r == (if j == v {
                            exists
                        } else {
                            self.matrix.spec_index(u as int).spec_index(j as int)
                        })
                {
                    if j == v {
                        exists
                    } else {
                        *self.matrix.nth(u).nth(j)
                    }
                },
                n,
            );
            let matrix = ArraySeqStEphS::tabulate(
                &|i: usize| -> (r: ArraySeqStEphS<bool>)
                    requires i < n
                    ensures
                        r.spec_len() == n,
                        (i as int == u as int) ==> forall|j: int| 0 <= j < n ==>
                            #[trigger] r.spec_index(j) == (if j == v as int {
                                exists
                            } else {
                                self.matrix.spec_index(u as int).spec_index(j)
                            }),
                        (i as int != u as int) ==> forall|j: int| 0 <= j < n ==>
                            #[trigger] r.spec_index(j) == self.matrix.spec_index(i as int).spec_index(j)
                {
                    if i == u {
                        new_row.clone()
                    } else {
                        let row = self.matrix.nth(i);
                        ArraySeqStEphS::tabulate(
                            &|j: usize| -> (r: bool)
                                requires j < n
                                ensures r == row.spec_index(j as int)
                            { *row.nth(j) },
                            n,
                        )
                    }
                },
                n,
            );
            self.matrix = matrix;
            // Update cached edge count based on whether the edge changed.
            if old_val && !exists {
                proof {
                    // num_edges >= 1: edge(u,v) was true, so row u count >= 1, so sum >= 1.
                    let edge_u = |c: int| old(self).spec_edge(u as int, c);
                    lemma_count_true_at_least_one(edge_u, v as int, n as int);
                    lemma_sum_of_lower_bound(n as int, old_row_count, u as int);
                    assert(self.num_edges as nat >= 1nat);
                }
                self.num_edges = self.num_edges - 1;
            } else if !old_val && exists {
                proof {
                    // num_edges ≤ n² < usize::MAX, so +1 fits.
                    assert forall|r: int| 0 <= r < n implies #[trigger] old_row_count(r) <= n as nat by {
                        lemma_count_true_bound(|c: int| old(self).spec_edge(r, c), n as int);
                    };
                    lemma_sum_of_bounded(n as int, old_row_count, n as nat);
                    assert(self.num_edges as nat <= (n as nat) * (n as nat));
                }
                self.num_edges = self.num_edges + 1;
            }
            proof {
                let new_row_count = |r: int| spec_count_true(|c: int| self.spec_edge(r, c), self.spec_n() as int);
                // For rows other than u, the edge predicate is unchanged.
                assert forall|r: int| 0 <= r < n && r != u as int
                    implies #[trigger] new_row_count(r) == old_row_count(r) by {
                    let old_edge = |c: int| old(self).spec_edge(r, c);
                    let new_edge = |c: int| self.spec_edge(r, c);
                    assert forall|c: int| 0 <= c < n implies #[trigger] old_edge(c) == new_edge(c) by {};
                    lemma_count_true_ext(old_edge, new_edge, n as int);
                };
                // For row u, relate old and new counts.
                let old_edge_u = |c: int| old(self).spec_edge(u as int, c);
                let new_edge_u = |c: int| self.spec_edge(u as int, c);
                assert forall|c: int| 0 <= c < n && c != v as int
                    implies #[trigger] old_edge_u(c) == new_edge_u(c) by {};
                if old_val && !exists {
                    lemma_count_true_set_false(old_edge_u, new_edge_u, v as int, n as int);
                    assert(new_row_count(u as int) + 1 == old_row_count(u as int));
                    lemma_sum_of_change_one(n as int, old_row_count, new_row_count, u as int);
                } else if !old_val && exists {
                    lemma_count_true_set_true(old_edge_u, new_edge_u, v as int, n as int);
                    assert(new_row_count(u as int) == old_row_count(u as int) + 1);
                    lemma_sum_of_change_one(n as int, old_row_count, new_row_count, u as int);
                } else {
                    lemma_count_true_ext(old_edge_u, new_edge_u, n as int);
                    assert(new_row_count(u as int) == old_row_count(u as int));
                    lemma_sum_of_ext(old_row_count, new_row_count, n as int);
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2)
        fn complement(&self) -> (complemented: Self) {
            let n = self.n;
            let matrix = ArraySeqStEphS::tabulate(
                &|i: usize| -> (r: ArraySeqStEphS<bool>)
                    requires i < n
                    ensures
                        r.spec_len() == n,
                        forall|j: int| 0 <= j < n ==>
                            #[trigger] r.spec_index(j) == (i as int != j && !self.matrix.spec_index(i as int).spec_index(j))
                {
                    let row = self.matrix.nth(i);
                    ArraySeqStEphS::tabulate(
                        &|j: usize| -> (r: bool)
                            requires j < n
                            ensures r == (i as int != j as int && !row.spec_index(j as int))
                        {
                            i != j && !*row.nth(j)
                        },
                        n,
                    )
                },
                n,
            );
            // Count edges in complement matrix.
            let ghost comp_row_count = |u: int| spec_count_true(|v: int| matrix.spec_index(u).spec_index(v), n as int);
            proof {
                assert forall|r: int| 0 <= r < n implies #[trigger] comp_row_count(r) <= n as nat by {
                    lemma_count_true_bound(|v: int| matrix.spec_index(r).spec_index(v), n as int);
                };
                lemma_sum_of_bounded(n as int, comp_row_count, n as nat);
            }
            let mut total: usize = 0;
            let mut u: usize = 0;
            while u < n
                invariant
                    u <= n,
                    n as nat == matrix.spec_len(),
                    forall|i: int| 0 <= i < n ==> #[trigger] matrix.spec_index(i).spec_len() == n,
                    total as nat == spec_sum_of(u as int, comp_row_count),
                    comp_row_count == (|u: int| spec_count_true(|v: int| matrix.spec_index(u).spec_index(v), n as int)),
                    spec_sum_of(n as int, comp_row_count) <= (n as nat) * (n as nat),
                    (n as nat) * (n as nat) <= usize::MAX as nat,
                decreases n - u
            {
                proof { lemma_sum_of_monotone(u as int + 1, n as int, comp_row_count); }
                let row = matrix.nth(u);
                let mut count: usize = 0;
                let mut vi: usize = 0;
                let ghost edge_fn = |v: int| matrix.spec_index(u as int).spec_index(v);
                proof { lemma_count_true_bound(edge_fn, n as int); }
                while vi < n
                    invariant
                        vi <= n,
                        n as nat == matrix.spec_len(),
                        u < n,
                        row.spec_len() == n,
                        forall|j: int| 0 <= j < n ==> #[trigger] row.spec_index(j) == matrix.spec_index(u as int).spec_index(j),
                        count as nat == spec_count_true(edge_fn, vi as int),
                        edge_fn == (|v: int| matrix.spec_index(u as int).spec_index(v)),
                        spec_count_true(edge_fn, n as int) <= n as nat,
                    decreases n - vi
                {
                    proof { lemma_count_true_monotone(edge_fn, vi as int + 1, n as int); }
                    if *row.nth(vi) {
                        count = count + 1;
                    }
                    vi = vi + 1;
                }
                total = total + count;
                u = u + 1;
            }
            // Connect the loop's row_count to the wf's row_count via spec_edge.
            let complemented = AdjMatrixGraphStEph { matrix, n, num_edges: total };
            proof {
                let wf_row_count = |u: int| spec_count_true(|v: int| complemented.spec_edge(u, v), complemented.spec_n() as int);
                assert forall|u: int| 0 <= u < n implies #[trigger] comp_row_count(u) == wf_row_count(u) by {
                    let inner_mat = |v: int| matrix.spec_index(u).spec_index(v);
                    let inner_edge = |v: int| complemented.spec_edge(u, v);
                    assert forall|v: int| 0 <= v < n implies #[trigger] inner_mat(v) == inner_edge(v) by {};
                    lemma_count_true_ext(inner_mat, inner_edge, n as int);
                };
                lemma_sum_of_ext(comp_row_count, wf_row_count, n as int);
            }
            complemented
        }
    }

    //		Section 12. derive impls in verus!


    impl Clone for AdjMatrixGraphStEph {
        fn clone(&self) -> (out: Self) {
            AdjMatrixGraphStEph { matrix: self.matrix.clone(), n: self.n, num_edges: self.num_edges }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for AdjMatrixGraphStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjMatrixGraphStEph")
                .field("matrix", &self.matrix)
                .field("n", &self.n)
                .field("num_edges", &self.num_edges)
                .finish()
        }
    }

    impl std::fmt::Display for AdjMatrixGraphStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjMatrixGraphStEph(n: {}, edges: {})", self.n, self.num_edges)
        }
    }
}
