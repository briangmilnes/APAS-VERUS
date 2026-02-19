//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).

pub mod AdjSeqGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
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
    pub struct AdjSeqGraphMtPer {
        pub adj: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphMtPer {
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

    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    // 7. proof fns

    proof fn lemma_sum_of_monotone(i: int, n: int, f: spec_fn(int) -> nat)
        requires 0 <= i <= n
        ensures spec_sum_of(i, f) <= spec_sum_of(n, f)
        decreases n - i
    {
        if i < n {
            lemma_sum_of_monotone(i, n - 1, f);
        }
    }

    proof fn lemma_sum_of_unfold(i: int, f: spec_fn(int) -> nat)
        requires i >= 0
        ensures spec_sum_of(i + 1, f) == f(i) + spec_sum_of(i, f)
    {
    }

    // 8. traits

    pub trait AdjSeqGraphMtPerTrait: Sized {
        spec fn spec_num_vertices(&self) -> nat;
        spec fn spec_degree(&self, u: int) -> nat
            recommends 0 <= u < self.spec_num_vertices();
        spec fn spec_neighbor(&self, u: int, j: int) -> N
            recommends 0 <= u < self.spec_num_vertices(), 0 <= j < self.spec_degree(u);

        /// Work Theta(n), Span Theta(log n)
        fn new(n: N) -> (result: Self)
            ensures
                result.spec_num_vertices() == n,
                forall|i: int| #![auto] 0 <= i < n ==> result.spec_degree(i) == 0;

        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> (n: N)
            ensures n as nat == self.spec_num_vertices();

        /// Work Theta(n + m), Span Theta(lg n)
        fn num_edges(&self) -> (m: N)
            requires
                spec_sum_of(
                    self.spec_num_vertices() as int,
                    |i: int| self.spec_degree(i),
                ) <= usize::MAX as nat
            ensures
                m as nat == spec_sum_of(
                    self.spec_num_vertices() as int,
                    |i: int| self.spec_degree(i),
                );

        /// Work Theta(deg(u)), Span Theta(deg(u))
        fn has_edge(&self, u: N, v: N) -> (found: B)
            requires u < self.spec_num_vertices()
            ensures found == exists|j: int|
                #![auto] 0 <= j < self.spec_degree(u as int)
                && self.spec_neighbor(u as int, j) == v;

        /// Work Theta(1), Span Theta(1)
        fn out_neighbors(&self, u: N) -> (neighbors: &ArraySeqMtPerS<N>)
            requires u < self.spec_num_vertices()
            ensures
                neighbors.spec_len() == self.spec_degree(u as int),
                forall|j: int| #![auto] 0 <= j < neighbors.spec_len()
                    ==> neighbors.spec_index(j) == self.spec_neighbor(u as int, j);

        /// Work Theta(1), Span Theta(1)
        fn out_degree(&self, u: N) -> (d: N)
            requires u < self.spec_num_vertices()
            ensures d as nat == self.spec_degree(u as int);
    }

    // 9. impls

    impl AdjSeqGraphMtPerTrait for AdjSeqGraphMtPer {

        open spec fn spec_num_vertices(&self) -> nat {
            self.adj.spec_len()
        }

        open spec fn spec_degree(&self, u: int) -> nat {
            self.adj.spec_index(u).spec_len()
        }

        open spec fn spec_neighbor(&self, u: int, j: int) -> N {
            self.adj.spec_index(u).spec_index(j)
        }

        fn new(n: N) -> (result: Self) {
            let adj = ArraySeqMtPerS::tabulate(
                &|_i: usize| -> (r: ArraySeqMtPerS<N>)
                    ensures r.spec_len() == 0
                {
                    ArraySeqMtPerS::empty()
                },
                n,
            );
            AdjSeqGraphMtPer { adj }
        }

        fn num_vertices(&self) -> (n: N) {
            self.adj.length()
        }

        fn num_edges(&self) -> (m: N) {
            let n = self.adj.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            let ghost degree_fn: spec_fn(int) -> nat = |k: int| self.spec_degree(k);
            while i < n
                invariant
                    i <= n,
                    n as nat == self.spec_num_vertices(),
                    count as nat == spec_sum_of(i as int, degree_fn),
                    degree_fn == (|k: int| self.spec_degree(k)),
                    spec_sum_of(n as int, degree_fn) <= usize::MAX as nat,
                decreases n - i
            {
                proof {
                    lemma_sum_of_unfold(i as int, degree_fn);
                    lemma_sum_of_monotone(i as int + 1, n as int, degree_fn);
                }
                let deg = self.adj.nth(i).length();
                count = count + deg;
                i = i + 1;
            }
            count
        }

        fn has_edge(&self, u: N, v: N) -> (found: B) {
            let neighbors = self.adj.nth(u);
            let len = neighbors.length();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    u < self.spec_num_vertices(),
                    len as nat == neighbors.spec_len(),
                    len as nat == self.spec_degree(u as int),
                    forall|j: int| #![auto] 0 <= j < len as int
                        ==> neighbors.spec_index(j) == self.spec_neighbor(u as int, j),
                    forall|j: int| #![auto] 0 <= j < i
                        ==> neighbors.spec_index(j) != v,
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

        fn out_neighbors(&self, u: N) -> (neighbors: &ArraySeqMtPerS<N>) {
            self.adj.nth(u)
        }

        fn out_degree(&self, u: N) -> (d: N) {
            self.adj.nth(u).length()
        }
    }

    } // verus!
}
