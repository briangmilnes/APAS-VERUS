//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (persistent, multi-threaded).
//! Verified sequential implementation (no parallel helpers).

pub mod AdjMatrixGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
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

    pub struct AdjMatrixGraphMtPer {
        pub matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>,
        pub n: usize,
    }

    // 5. view impls

    impl View for AdjMatrixGraphMtPer {
        type V = Seq<Seq<bool>>;
        open spec fn view(&self) -> Self::V {
            self.matrix@
        }
    }

    // 6. spec fns

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
    pub open spec fn spec_adjmatrixgraphmtper_wf(g: AdjMatrixGraphMtPer) -> bool {
        g.matrix.spec_len() == g.n
        && forall|i: int| 0 <= i < g.n ==>
            #[trigger] g.matrix.spec_index(i).spec_len() == g.n
    }

    // 7. proof fns

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

    // 8. traits

    pub trait AdjMatrixGraphMtPerTrait: Sized {
        spec fn spec_adjmatrixgraphmtper_wf(&self) -> bool;
        spec fn spec_n(&self) -> nat;
        spec fn spec_edge(&self, u: int, v: int) -> bool
            recommends 0 <= u < self.spec_n(), 0 <= v < self.spec_n();

        /// Work Theta(n^2), Span Theta(n^2)
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_adjmatrixgraphmtper_wf(),
                empty.spec_n() == n,
                forall|u: int, v: int|
                    0 <= u < n && 0 <= v < n ==> !#[trigger] empty.spec_edge(u, v);

        /// Work Theta(1), Span Theta(1)
        fn from_matrix(matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>) -> (constructed: Self)
            requires
                forall|i: int| 0 <= i < matrix.spec_len() ==>
                    #[trigger] matrix.spec_index(i).spec_len() == matrix.spec_len()
            ensures
                constructed.spec_adjmatrixgraphmtper_wf(),
                constructed.spec_n() == matrix.spec_len(),
                forall|u: int, v: int|
                    0 <= u < matrix.spec_len() && 0 <= v < matrix.spec_len()
                    ==> #[trigger] constructed.spec_edge(u, v) == matrix.spec_index(u).spec_index(v);

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn num_vertices(&self) -> (n: usize)
            requires self.spec_adjmatrixgraphmtper_wf()
            ensures n as nat == self.spec_n();

        /// Work Theta(n^2), Span Theta(n^2)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — DIFFERS: nested sequential loops counting true entries
        fn num_edges(&self) -> (m: usize)
            requires
                self.spec_adjmatrixgraphmtper_wf(),
                spec_sum_of(
                    self.spec_n() as int,
                    |u: int| spec_count_true(|v: int| self.spec_edge(u, v), self.spec_n() as int),
                ) <= usize::MAX as nat
            ensures
                m as nat == spec_sum_of(
                    self.spec_n() as int,
                    |u: int| spec_count_true(|v: int| self.spec_edge(u, v), self.spec_n() as int),
                );

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn has_edge(&self, u: usize, v: usize) -> (found: bool)
            requires self.spec_adjmatrixgraphmtper_wf()
            ensures
                u < self.spec_n() && v < self.spec_n() ==> found == self.spec_edge(u as int, v as int),
                (u >= self.spec_n() || v >= self.spec_n()) ==> !found;

        /// Work Theta(n), Span Theta(n)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: span = work, sequential row scan
        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqMtPerS<usize>)
            requires self.spec_adjmatrixgraphmtper_wf()
            ensures
                u < self.spec_n() ==> (
                    (forall|k: int| 0 <= k < neighbors.spec_len()
                        ==> #[trigger] neighbors.spec_index(k) < self.spec_n()
                            && self.spec_edge(u as int, neighbors.spec_index(k) as int))
                    && (forall|v: int| 0 <= v < self.spec_n() && #[trigger] self.spec_edge(u as int, v)
                        ==> exists|k: int|
                            0 <= k < neighbors.spec_len() && #[trigger] neighbors.spec_index(k) == v as usize)
                ),
                u >= self.spec_n() ==> neighbors.spec_len() == 0;

        /// Work Theta(n), Span Theta(n)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: span = work, sequential count over row
        fn out_degree(&self, u: usize) -> (d: usize)
            requires self.spec_adjmatrixgraphmtper_wf()
            ensures
                u < self.spec_n() ==> d as nat == spec_count_true(
                    |v: int| self.spec_edge(u as int, v),
                    self.spec_n() as int,
                ),
                u >= self.spec_n() ==> d == 0;

        /// Work Theta(n), Span Theta(n)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential tabulate rebuilds entire row, not O(1) in-place
        fn set_edge(&self, u: usize, v: usize, exists: bool) -> (updated: Self)
            requires
                self.spec_adjmatrixgraphmtper_wf(),
                u < self.spec_n(),
                v < self.spec_n(),
            ensures
                updated.spec_adjmatrixgraphmtper_wf(),
                updated.spec_n() == self.spec_n(),
                updated.spec_edge(u as int, v as int) == exists,
                forall|i: int, j: int|
                    0 <= i < self.spec_n() && 0 <= j < self.spec_n()
                    && !(i == u as int && j == v as int)
                    ==> #[trigger] updated.spec_edge(i, j) == self.spec_edge(i, j);

        /// Work Theta(n^2), Span Theta(n^2)
        /// - Alg Analysis: APAS (Ch52 CS 52.6): Work O(n^2), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^2), Span O(n^2) — DIFFERS: span = work, sequential nested tabulate
        fn complement(&self) -> (complemented: Self)
            requires self.spec_adjmatrixgraphmtper_wf()
            ensures
                complemented.spec_adjmatrixgraphmtper_wf(),
                complemented.spec_n() == self.spec_n(),
                forall|i: int, j: int|
                    0 <= i < self.spec_n() && 0 <= j < self.spec_n()
                    ==> #[trigger] complemented.spec_edge(i, j) == (i != j && !self.spec_edge(i, j));
    }

    // 9. impls

    impl AdjMatrixGraphMtPerTrait for AdjMatrixGraphMtPer {

        open spec fn spec_adjmatrixgraphmtper_wf(&self) -> bool {
            self.matrix.spec_len() == self.n
            && forall|i: int| 0 <= i < self.n ==>
                #[trigger] self.matrix.spec_index(i).spec_len() == self.n
        }

        open spec fn spec_n(&self) -> nat { self.n as nat }

        open spec fn spec_edge(&self, u: int, v: int) -> bool {
            self.matrix.spec_index(u).spec_index(v)
        }

        fn new(n: usize) -> (empty: Self) {
            let matrix = ArraySeqMtPerS::tabulate(
                &|_i: usize| -> (r: ArraySeqMtPerS<bool>)
                    ensures
                        r.spec_len() == n,
                        forall|j: int| 0 <= j < n ==> !#[trigger] r.spec_index(j)
                {
                    ArraySeqMtPerS::tabulate(
                        &|_j: usize| -> (r: bool) ensures !r { false },
                        n,
                    )
                },
                n,
            );
            AdjMatrixGraphMtPer { matrix, n }
        }

        fn from_matrix(matrix: ArraySeqMtPerS<ArraySeqMtPerS<bool>>) -> (constructed: Self) {
            let n = matrix.length();
            AdjMatrixGraphMtPer { matrix, n }
        }

        fn num_vertices(&self) -> (n: usize) { self.n }

        fn num_edges(&self) -> (m: usize) {
            let n = self.n;
            let mut total: usize = 0;
            let mut u: usize = 0;
            let ghost row_count = |u: int| spec_count_true(|v: int| self.spec_edge(u, v), n as int);
            while u < n
                invariant
                    u <= n,
                    self.spec_adjmatrixgraphmtper_wf(),
                    n as nat == self.spec_n(),
                    total as nat == spec_sum_of(u as int, row_count),
                    row_count == (|u: int| spec_count_true(|v: int| self.spec_edge(u, v), n as int)),
                    spec_sum_of(n as int, row_count) <= usize::MAX as nat,
                decreases n - u
            {
                proof { lemma_sum_of_monotone(u as int + 1, n as int, row_count); }
                let row = self.matrix.nth(u);
                let mut count: usize = 0;
                let mut v: usize = 0;
                let ghost edge_fn = |v: int| self.spec_edge(u as int, v);
                while v < n
                    invariant
                        v <= n,
                        self.spec_adjmatrixgraphmtper_wf(),
                        n as nat == self.spec_n(),
                        u < n,
                        row.spec_len() == n,
                        forall|vi: int| 0 <= vi < n ==> #[trigger] row.spec_index(vi) == self.spec_edge(u as int, vi),
                        count as nat == spec_count_true(edge_fn, v as int),
                        edge_fn == (|v: int| self.spec_edge(u as int, v)),
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
            total
        }

        fn has_edge(&self, u: usize, v: usize) -> (found: bool) {
            if u >= self.n || v >= self.n {
                return false;
            }
            *self.matrix.nth(u).nth(v)
        }

        fn out_neighbors(&self, u: usize) -> (neighbors: ArraySeqMtPerS<usize>) {
            if u >= self.n {
                return ArraySeqMtPerS::empty();
            }
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut nvec = Vec::<usize>::new();
            let mut v: usize = 0;
            while v < n
                invariant
                    v <= n,
                    self.spec_adjmatrixgraphmtper_wf(),
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
            let neighbors = ArraySeqMtPerS::from_vec(nvec);
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

        fn out_degree(&self, u: usize) -> (d: usize) {
            if u >= self.n {
                return 0;
            }
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut count: usize = 0;
            let mut v: usize = 0;
            let ghost edge_fn = |v: int| self.spec_edge(u as int, v);
            proof { lemma_count_true_bound(edge_fn, n as int); }
            while v < n
                invariant
                    v <= n,
                    self.spec_adjmatrixgraphmtper_wf(),
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

        fn set_edge(&self, u: usize, v: usize, exists: bool) -> (updated: Self) {
            let n = self.n;
            let new_row = ArraySeqMtPerS::tabulate(
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
            let matrix = ArraySeqMtPerS::tabulate(
                &|i: usize| -> (r: ArraySeqMtPerS<bool>)
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
                        ArraySeqMtPerS::tabulate(
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
            AdjMatrixGraphMtPer { matrix, n }
        }

        fn complement(&self) -> (complemented: Self) {
            let n = self.n;
            let matrix = ArraySeqMtPerS::tabulate(
                &|i: usize| -> (r: ArraySeqMtPerS<bool>)
                    requires i < n
                    ensures
                        r.spec_len() == n,
                        forall|j: int| 0 <= j < n ==>
                            #[trigger] r.spec_index(j) == (i as int != j && !self.matrix.spec_index(i as int).spec_index(j))
                {
                    let row = self.matrix.nth(i);
                    ArraySeqMtPerS::tabulate(
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
            AdjMatrixGraphMtPer { matrix, n }
        }
    }

    // 11. derive impls in verus!

    impl Clone for AdjMatrixGraphMtPer {
        fn clone(&self) -> (out: Self) {
            AdjMatrixGraphMtPer { matrix: self.matrix.clone(), n: self.n }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for AdjMatrixGraphMtPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjMatrixGraphMtPer")
                .field("matrix", &self.matrix)
                .field("n", &self.n)
                .finish()
        }
    }
}
