//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Matrix Graph (ephemeral, multi-threaded).

pub mod AdjMatrixGraphMtEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjMatrixGraphMtEph {
        pub matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>,
        pub n: N,
    }

    // 5. view impls

    impl View for AdjMatrixGraphMtEph {
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
    pub open spec fn spec_wf(g: AdjMatrixGraphMtEph) -> bool {
        g.matrix.spec_len() == g.n
        && forall|i: int| #![auto] 0 <= i < g.n ==>
            g.matrix.spec_index(i).spec_len() == g.n
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

    // 8. traits

    pub trait AdjMatrixGraphMtEphTrait: Sized {
        spec fn spec_n(&self) -> nat;
        spec fn spec_edge(&self, u: int, v: int) -> bool
            recommends 0 <= u < self.spec_n(), 0 <= v < self.spec_n();

        /// Work Theta(n^2), Span Theta(n^2)
        fn new(n: N) -> (result: Self)
            ensures
                spec_wf(result),
                result.spec_n() == n,
                forall|u: int, v: int| #![auto]
                    0 <= u < n && 0 <= v < n ==> !result.spec_edge(u, v);

        /// Work Theta(1), Span Theta(1)
        fn from_matrix(matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>) -> (result: Self)
            requires
                forall|i: int| #![auto] 0 <= i < matrix.spec_len() ==>
                    matrix.spec_index(i).spec_len() == matrix.spec_len()
            ensures
                spec_wf(result),
                result.spec_n() == matrix.spec_len(),
                forall|u: int, v: int| #![auto]
                    0 <= u < matrix.spec_len() && 0 <= v < matrix.spec_len()
                    ==> result.spec_edge(u, v) == matrix.spec_index(u).spec_index(v);

        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> (n: N)
            requires spec_wf(*self)
            ensures n as nat == self.spec_n();

        /// Work Theta(n^2), Span Theta(n^2)
        fn num_edges(&self) -> (m: N)
            requires
                spec_wf(*self),
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
        fn has_edge(&self, u: N, v: N) -> (found: B)
            requires spec_wf(*self), u < self.spec_n(), v < self.spec_n()
            ensures found == self.spec_edge(u as int, v as int);

        /// Work Theta(n), Span Theta(n)
        fn out_neighbors(&self, u: N) -> (neighbors: ArraySeqMtEphS<N>)
            requires spec_wf(*self), u < self.spec_n()
            ensures
                forall|k: int| #![auto] 0 <= k < neighbors.spec_len()
                    ==> neighbors.spec_index(k) < self.spec_n()
                        && self.spec_edge(u as int, neighbors.spec_index(k) as int),
                forall|v: int| #![auto] 0 <= v < self.spec_n() && self.spec_edge(u as int, v)
                    ==> exists|k: int| #![auto]
                        0 <= k < neighbors.spec_len() && neighbors.spec_index(k) == v as N;

        /// Work Theta(n), Span Theta(n)
        fn out_degree(&self, u: N) -> (d: N)
            requires spec_wf(*self), u < self.spec_n()
            ensures d as nat == spec_count_true(
                |v: int| self.spec_edge(u as int, v),
                self.spec_n() as int,
            );

        /// Work Theta(1), Span Theta(1)
        fn set_edge(&mut self, u: N, v: N, exists: B)
            requires
                spec_wf(*old(self)),
                u < old(self).spec_n(),
                v < old(self).spec_n(),
            ensures
                spec_wf(*self),
                self.spec_n() == old(self).spec_n(),
                self.spec_edge(u as int, v as int) == exists,
                forall|i: int, j: int| #![auto]
                    0 <= i < old(self).spec_n() && 0 <= j < old(self).spec_n()
                    && !(i == u as int && j == v as int)
                    ==> self.spec_edge(i, j) == old(self).spec_edge(i, j);

        /// Work Theta(n^2), Span Theta(n^2)
        fn complement(&self) -> (result: Self)
            requires spec_wf(*self)
            ensures
                spec_wf(result),
                result.spec_n() == self.spec_n(),
                forall|i: int, j: int| #![auto]
                    0 <= i < self.spec_n() && 0 <= j < self.spec_n()
                    ==> result.spec_edge(i, j) == (i != j && !self.spec_edge(i, j));
    }

    // 9. impls

    impl AdjMatrixGraphMtEphTrait for AdjMatrixGraphMtEph {

        open spec fn spec_n(&self) -> nat { self.n as nat }

        open spec fn spec_edge(&self, u: int, v: int) -> bool {
            self.matrix.spec_index(u).spec_index(v)
        }

        fn new(n: N) -> (result: Self) {
            let false_row = ArraySeqMtEphS::tabulate(
                &|_j: usize| -> (r: bool) ensures !r { false },
                n,
            );
            let matrix = ArraySeqMtEphS::tabulate(
                &|_i: usize| -> (r: ArraySeqMtEphS<bool>)
                    ensures
                        r.spec_len() == n,
                        forall|j: int| #![auto] 0 <= j < n ==> !r.spec_index(j)
                {
                    ArraySeqMtEphS::tabulate(
                        &|_j: usize| -> (r: bool) ensures !r { false },
                        n,
                    )
                },
                n,
            );
            AdjMatrixGraphMtEph { matrix, n }
        }

        fn from_matrix(matrix: ArraySeqMtEphS<ArraySeqMtEphS<bool>>) -> (result: Self) {
            let n = matrix.length();
            AdjMatrixGraphMtEph { matrix, n }
        }

        fn num_vertices(&self) -> (n: N) { self.n }

        fn num_edges(&self) -> (m: N) {
            let n = self.n;
            let mut total: usize = 0;
            let mut u: usize = 0;
            let ghost row_count = |u: int| spec_count_true(|v: int| self.spec_edge(u, v), n as int);
            while u < n
                invariant
                    u <= n,
                    spec_wf(*self),
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
                        spec_wf(*self),
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

        fn has_edge(&self, u: N, v: N) -> (found: B) {
            *self.matrix.nth(u).nth(v)
        }

        fn out_neighbors(&self, u: N) -> (neighbors: ArraySeqMtEphS<N>) {
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut nvec = Vec::<N>::new();
            let mut v: usize = 0;
            while v < n
                invariant
                    v <= n,
                    spec_wf(*self),
                    u < self.spec_n(),
                    forall|k: int| #![auto] 0 <= k < nvec@.len() as int
                        ==> nvec@[k] < n
                            && self.spec_edge(u as int, nvec@[k] as int),
                    forall|j: int| #![auto] 0 <= j < v && self.spec_edge(u as int, j)
                        ==> exists|k: int| #![auto]
                            0 <= k < nvec@.len() as int && nvec@[k] == j as N,
                decreases n - v
            {
                if *row.nth(v) {
                    nvec.push(v);
                }
                v = v + 1;
            }
            ArraySeqMtEphS::from_vec(nvec)
        }

        fn out_degree(&self, u: N) -> (d: N) {
            let n = self.n;
            let row = self.matrix.nth(u);
            let mut count: usize = 0;
            let mut v: usize = 0;
            let ghost edge_fn = |v: int| self.spec_edge(u as int, v);
            while v < n
                invariant
                    v <= n,
                    spec_wf(*self),
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
            count
        }

        fn set_edge(&mut self, u: N, v: N, exists: B) {
            let mut row = self.matrix.nth(u).clone();
            let _ = row.set(v, exists);
            let _ = self.matrix.set(u, row);
        }

        fn complement(&self) -> (result: Self) {
            let n = self.n;
            let matrix = ArraySeqMtEphS::tabulate(
                &|i: usize| -> (r: ArraySeqMtEphS<bool>)
                    requires i < n
                    ensures
                        r.spec_len() == n,
                        forall|j: int| #![auto] 0 <= j < n ==>
                            r.spec_index(j) == (i as int != j && !self.matrix.spec_index(i as int).spec_index(j))
                {
                    let row = self.matrix.nth(i);
                    ArraySeqMtEphS::tabulate(
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
            AdjMatrixGraphMtEph { matrix, n }
        }
    }

    } // verus!
}
