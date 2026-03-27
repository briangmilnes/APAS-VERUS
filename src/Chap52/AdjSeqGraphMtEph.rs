//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (ephemeral, multi-threaded).

pub mod AdjSeqGraphMtEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    const SEQUENTIAL_CUTOFF: usize = 1;

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

    pub struct AdjSeqGraphMtEph {
        pub adj: ArraySeqMtEphS<ArraySeqMtEphS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphMtEph {
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

    /// Unfolding one step: spec_sum_of(i+1, f) == f(i) + spec_sum_of(i, f).
    proof fn lemma_sum_of_unfold(i: int, f: spec_fn(int) -> nat)
        requires i >= 0
        ensures spec_sum_of(i + 1, f) == f(i) + spec_sum_of(i, f)
    {
    }

    // 8. traits

    pub trait AdjSeqGraphMtEphTrait: Sized {
        spec fn spec_adjseqgraphmteph_wf(&self) -> bool;
        spec fn spec_num_vertices(&self) -> nat;
        spec fn spec_degree(&self, u: int) -> nat
            recommends 0 <= u < self.spec_num_vertices();
        spec fn spec_neighbor(&self, u: int, j: int) -> N
            recommends 0 <= u < self.spec_num_vertices(), 0 <= j < self.spec_degree(u);

        /// Work Theta(n), Span Theta(1)
        fn new(n: N) -> (empty: Self)
            ensures
                empty.spec_adjseqgraphmteph_wf(),
                empty.spec_num_vertices() == n,
                forall|i: int| 0 <= i < n ==> #[trigger] empty.spec_degree(i) == 0;

        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> (n: N)
            requires self.spec_adjseqgraphmteph_wf()
            ensures n as nat == self.spec_num_vertices();

        /// Work Theta(n + m), Span Theta(lg n)
        fn num_edges(&self) -> (m: N)
            requires
                self.spec_adjseqgraphmteph_wf(),
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
        fn has_edge(&self, u: N, v: N) -> (found: bool)
            requires self.spec_adjseqgraphmteph_wf(), u < self.spec_num_vertices()
            ensures found == exists|j: int|
                0 <= j < self.spec_degree(u as int)
                && #[trigger] self.spec_neighbor(u as int, j) == v;

        /// Work Theta(1), Span Theta(1)
        fn out_neighbors(&self, u: N) -> (neighbors: ArraySeqMtEphS<N>)
            requires self.spec_adjseqgraphmteph_wf(), u < self.spec_num_vertices()
            ensures
                neighbors.spec_len() == self.spec_degree(u as int),
                forall|j: int| 0 <= j < neighbors.spec_len()
                    ==> #[trigger] neighbors.spec_index(j) == self.spec_neighbor(u as int, j);

        /// Work Theta(1), Span Theta(1)
        fn out_degree(&self, u: N) -> (d: N)
            requires self.spec_adjseqgraphmteph_wf(), u < self.spec_num_vertices()
            ensures d as nat == self.spec_degree(u as int);

        /// Work Theta(deg(u)), Span Theta(deg(u))
        fn set_edge(&mut self, u: N, v: N, exists: bool)
            requires
                old(self).spec_adjseqgraphmteph_wf(),
                u < old(self).spec_num_vertices(),
                v < old(self).spec_num_vertices(),
            ensures
                self.spec_adjseqgraphmteph_wf(),
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

    impl AdjSeqGraphMtEphTrait for AdjSeqGraphMtEph {

        open spec fn spec_adjseqgraphmteph_wf(&self) -> bool {
            forall|u: int, j: int|
                0 <= u < self.adj.spec_len()
                && 0 <= j < self.adj.spec_index(u).spec_len()
                ==> #[trigger] self.adj.spec_index(u).spec_index(j) < self.adj.spec_len()
        }

        open spec fn spec_num_vertices(&self) -> nat {
            self.adj.spec_len()
        }

        open spec fn spec_degree(&self, u: int) -> nat {
            self.adj.spec_index(u).spec_len()
        }

        open spec fn spec_neighbor(&self, u: int, j: int) -> N {
            self.adj.spec_index(u).spec_index(j)
        }

        fn new(n: N) -> (empty: Self) {
            let adj = ArraySeqMtEphS::tabulate(
                &|_i: usize| -> (r: ArraySeqMtEphS<N>)
                    ensures r.spec_len() == 0
                {
                    ArraySeqMtEphS::empty()
                },
                n,
            );
            AdjSeqGraphMtEph { adj }
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

        fn has_edge(&self, u: N, v: N) -> (found: bool) {
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

        fn out_neighbors(&self, u: N) -> (neighbors: ArraySeqMtEphS<N>) {
            let src = self.adj.nth(u);
            let len = src.length();
            ArraySeqMtEphS::tabulate(
                &|i: usize| -> (r: N)
                    requires i < len
                    ensures r == src.spec_index(i as int)
                {
                    *src.nth(i)
                },
                len,
            )
        }

        fn out_degree(&self, u: N) -> (d: N) {
            self.adj.nth(u).length()
        }

        fn set_edge(&mut self, u: N, v: N, exists: bool) {
            let ghost old_degree = self.spec_degree(u as int);
            let ghost old_neighbors_view = Seq::new(old_degree, |j: int| self.spec_neighbor(u as int, j));
            let ghost adj_len = self.adj.spec_len();

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
                    let mut new_vec = Vec::<N>::new();
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
                            old(self).spec_adjseqgraphmteph_wf(),
                            forall|k: int| 0 <= k < new_vec@.len() as int
                                ==> new_vec@[k] < adj_len,
                        decreases old_len - j
                    {
                        new_vec.push(*self.adj.nth(u).nth(j));
                        j = j + 1;
                    }
                    new_vec.push(v);
                    let new_neighbors = ArraySeqMtEphS::from_vec(new_vec);
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
                }
            } else {
                let old_len = self.adj.nth(u).length();
                let mut new_vec = Vec::<N>::new();
                let mut j: usize = 0;
                while j < old_len
                    invariant
                        j <= old_len,
                        u < self.spec_num_vertices(),
                        old_len as nat == self.spec_degree(u as int),
                        forall|k: int| 0 <= k < new_vec@.len() as int
                            ==> #[trigger] new_vec@[k] != v,
                        adj_len == self.adj.spec_len(),
                        old(self).spec_adjseqgraphmteph_wf(),
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
                let new_neighbors = ArraySeqMtEphS::from_vec(new_vec);
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
            }
        }
    }

    // 11. derive impls in verus!

    impl Clone for AdjSeqGraphMtEph {
        fn clone(&self) -> (out: Self) {
            AdjSeqGraphMtEph { adj: self.adj.clone() }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl std::fmt::Debug for AdjSeqGraphMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjSeqGraphMtEph").field("adj", &self.adj).finish()
        }
    }
}
