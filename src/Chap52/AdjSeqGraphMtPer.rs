//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 52: Adjacency Sequence Graph (persistent, multi-threaded).


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

pub mod AdjSeqGraphMtPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    use crate::Chap52::AdjTableGraphSpecsAndLemmas::AdjTableGraphSpecsAndLemmas::*;

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


    pub struct AdjSeqGraphMtPer {
        pub adj: ArraySeqMtPerS<ArraySeqMtPerS<usize>>,
        pub num_edges: usize,
    }

    //		Section 5. view impls


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

    //		Section 6. spec fns


    //		Section 7. proof fns/broadcast groups


    //		Section 8. traits


    pub trait AdjSeqGraphMtPerTrait: Sized {
        spec fn spec_adjseqgraphmtper_wf(&self) -> bool;
        spec fn spec_num_vertices(&self) -> nat;
        spec fn spec_degree(&self, u: int) -> nat
            recommends 0 <= u < self.spec_num_vertices();
        spec fn spec_neighbor(&self, u: int, j: int) -> usize
            recommends 0 <= u < self.spec_num_vertices(), 0 <= j < self.spec_degree(u);

        /// Work Theta(n), Span Theta(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn new(n: usize) -> (empty: Self)
            ensures
                empty.spec_adjseqgraphmtper_wf(),
                empty.spec_num_vertices() == n,
                forall|i: int| 0 <= i < n ==> #[trigger] empty.spec_degree(i) == 0;

        /// Work Theta(n + m), Span Theta(n + m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — counting loop
        fn from_seq(adj: ArraySeqMtPerS<ArraySeqMtPerS<usize>>) -> (constructed: Self)
            requires
                forall|u: int, j: int|
                    0 <= u < adj.spec_len()
                    && 0 <= j < adj.spec_index(u).spec_len()
                    ==> #[trigger] adj.spec_index(u).spec_index(j) < adj.spec_len(),
                spec_sum_of(
                    adj.spec_len() as int,
                    |i: int| adj.spec_index(i).spec_len(),
                ) <= usize::MAX as nat,
            ensures
                constructed.spec_adjseqgraphmtper_wf(),
                constructed.spec_num_vertices() == adj.spec_len(),
                forall|i: int| 0 <= i < adj.spec_len() ==>
                    #[trigger] constructed.spec_degree(i) == adj.spec_index(i).spec_len(),
                forall|i: int, j: int| 0 <= i < adj.spec_len()
                    && 0 <= j < adj.spec_index(i).spec_len()
                    ==> #[trigger] constructed.spec_neighbor(i, j) == adj.spec_index(i).spec_index(j);

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); seq len
        fn num_vertices(&self) -> (n: usize)
            requires self.spec_adjseqgraphmtper_wf()
            ensures n as nat == self.spec_num_vertices();

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — cached field
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjseqgraphmtper_wf()
            ensures
                m as nat == spec_sum_of(
                    self.spec_num_vertices() as int,
                    |i: int| self.spec_degree(i),
                );

        /// Work Theta(deg(u)), Span Theta(deg(u))
        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(d_g(u)), Span O(lg d_g(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d_g(u)), Span O(d_g(u)) work; sequential scan
        fn has_edge(&self, u: usize, v: usize) -> (found: bool)
            requires self.spec_adjseqgraphmtper_wf(), u < self.spec_num_vertices()
            ensures found == exists|j: int|
                0 <= j < self.spec_degree(u as int)
                && #[trigger] self.spec_neighbor(u as int, j) == v;

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(d_g(v)), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); returns reference to inner seq
        fn out_neighbors(&self, u: usize) -> (neighbors: &ArraySeqMtPerS<usize>)
            requires self.spec_adjseqgraphmtper_wf(), u < self.spec_num_vertices()
            ensures
                neighbors.spec_len() == self.spec_degree(u as int),
                forall|j: int| 0 <= j < neighbors.spec_len()
                    ==> #[trigger] neighbors.spec_index(j) == self.spec_neighbor(u as int, j);

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.5): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); inner seq len
        fn out_degree(&self, u: usize) -> (d: usize)
            requires self.spec_adjseqgraphmtper_wf(), u < self.spec_num_vertices()
            ensures d as nat == self.spec_degree(u as int);

        /// Work Theta(n + deg(u)), Span Theta(n + deg(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn insert_edge(&self, u: usize, v: usize) -> (updated: Self)
            requires
                self.spec_adjseqgraphmtper_wf(),
                u < self.spec_num_vertices(),
                v < self.spec_num_vertices(),
                spec_sum_of(self.spec_num_vertices() as int, |i: int| self.spec_degree(i)) < usize::MAX as nat,
            ensures
                updated.spec_adjseqgraphmtper_wf(),
                updated.spec_num_vertices() == self.spec_num_vertices(),
                forall|i: int| 0 <= i < self.spec_num_vertices() && i != u as int
                    ==> #[trigger] updated.spec_degree(i) == self.spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < self.spec_num_vertices() && i != u as int
                    && 0 <= j < self.spec_degree(i)
                    ==> #[trigger] updated.spec_neighbor(i, j) == self.spec_neighbor(i, j),
                exists|j: int|
                    0 <= j < updated.spec_degree(u as int)
                    && #[trigger] updated.spec_neighbor(u as int, j) == v;

        /// Work Theta(n + deg(u)), Span Theta(n + deg(u))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn delete_edge(&self, u: usize, v: usize) -> (updated: Self)
            requires self.spec_adjseqgraphmtper_wf(), u < self.spec_num_vertices()
            ensures
                updated.spec_adjseqgraphmtper_wf(),
                updated.spec_num_vertices() == self.spec_num_vertices(),
                forall|i: int| 0 <= i < self.spec_num_vertices() && i != u as int
                    ==> #[trigger] updated.spec_degree(i) == self.spec_degree(i),
                forall|i: int, j: int|
                    0 <= i < self.spec_num_vertices() && i != u as int
                    && 0 <= j < self.spec_degree(i)
                    ==> #[trigger] updated.spec_neighbor(i, j) == self.spec_neighbor(i, j),
                forall|j: int|
                    0 <= j < updated.spec_degree(u as int)
                    ==> #[trigger] updated.spec_neighbor(u as int, j) != v;
    }

    //		Section 9. impls


    impl AdjSeqGraphMtPerTrait for AdjSeqGraphMtPer {

        open spec fn spec_adjseqgraphmtper_wf(&self) -> bool {
            &&& forall|u: int, j: int|
                0 <= u < self.adj.spec_len()
                && 0 <= j < self.adj.spec_index(u).spec_len()
                ==> #[trigger] self.adj.spec_index(u).spec_index(j) < self.adj.spec_len()
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
            let adj = ArraySeqMtPerS::tabulate(
                &|_i: usize| -> (r: ArraySeqMtPerS<usize>)
                    ensures r.spec_len() == 0
                {
                    ArraySeqMtPerS::empty()
                },
                n,
            );
            let empty = AdjSeqGraphMtPer { adj, num_edges: 0 };
            proof {
                let degree_fn = |i: int| empty.spec_degree(i);
                // Veracity: NEEDED assert (speed hint)
                assert forall|i: int| 0 <= i < n implies #[trigger] degree_fn(i) == 0nat by {};
                lemma_sum_of_all_zero(degree_fn, n as int);
            }
            empty
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m)
        fn from_seq(adj: ArraySeqMtPerS<ArraySeqMtPerS<usize>>) -> (constructed: Self) {
            let n = adj.length();
            let ghost degree_fn: spec_fn(int) -> nat = |i: int| adj.spec_index(i).spec_len();
            let mut count: usize = 0;
            let mut i: usize = 0;
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
            let constructed = AdjSeqGraphMtPer { adj, num_edges: count };
            proof {
                let wf_degree = |i: int| constructed.spec_degree(i);
                // Veracity: NEEDED assert (speed hint)
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
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.spec_neighbor(u as int, i as int) == v);
                    return true;
                }
                i = i + 1;
            }
            false
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn out_neighbors(&self, u: usize) -> (neighbors: &ArraySeqMtPerS<usize>) {
            self.adj.nth(u)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn out_degree(&self, u: usize) -> (d: usize) {
            self.adj.nth(u).length()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn insert_edge(&self, u: usize, v: usize) -> (updated: Self) {
            let n_v = self.adj.length();
            let src_u = self.adj.nth(u);
            let deg_u = src_u.length();

            // Check if v already in neighbor list.
            let mut found = false;
            let mut fi: usize = 0;
            while fi < deg_u
                invariant
                    fi <= deg_u,
                    u < self.spec_num_vertices(),
                    deg_u as nat == self.spec_degree(u as int),
                    deg_u as nat == src_u.spec_len(),
                    forall|j: int| 0 <= j < deg_u as int
                        ==> #[trigger] src_u.spec_index(j) == self.spec_neighbor(u as int, j),
                    !found ==> forall|j: int| 0 <= j < fi
                        ==> #[trigger] self.spec_neighbor(u as int, j) != v,
                    found ==> exists|j: int| 0 <= j < self.spec_degree(u as int)
                        && #[trigger] self.spec_neighbor(u as int, j) == v,
                decreases deg_u - fi
            {
                if *src_u.nth(fi) == v {
                    // Veracity: NEEDED assert (speed hint)
                    assert(self.spec_neighbor(u as int, fi as int) == v);
                    found = true;
                    break;
                }
                fi = fi + 1;
            }

            // Build new neighbor list for vertex u.
            let new_neighbors: ArraySeqMtPerS<usize>;
            let ghost mut witness: int = 0;
            if found {
                new_neighbors = ArraySeqMtPerS::tabulate(
                    &|i: usize| -> (r: usize)
                        requires i < deg_u
                        ensures r == src_u.spec_index(i as int)
                    { *src_u.nth(i) },
                    deg_u,
                );
                proof {
                    witness = choose|j: int| 0 <= j < self.spec_degree(u as int)
                        && self.spec_neighbor(u as int, j) == v;
// Veracity: UNNEEDED assert                     assert(new_neighbors.spec_index(witness) == src_u.spec_index(witness));
                    // Veracity: NEEDED assert (speed hint)
                    assert(src_u.spec_index(witness) == self.spec_neighbor(u as int, witness));
                }
            } else {
                let mut nvec = Vec::<usize>::new();
                let mut j: usize = 0;
                while j < deg_u
                    invariant
                        j <= deg_u,
                        u < self.spec_num_vertices(),
                        deg_u as nat == self.spec_degree(u as int),
                        deg_u as nat == src_u.spec_len(),
                        forall|k: int| 0 <= k < deg_u as int
                            ==> #[trigger] src_u.spec_index(k) == self.spec_neighbor(u as int, k),
                        nvec@.len() == j as int,
                        forall|k: int| 0 <= k < j
                            ==> #[trigger] nvec@[k] == self.spec_neighbor(u as int, k),
                    decreases deg_u - j
                {
                    nvec.push(*src_u.nth(j));
                    j = j + 1;
                }
                nvec.push(v);
                new_neighbors = ArraySeqMtPerS::from_vec(nvec);
                proof { witness = deg_u as int; }
            }
            // Veracity: NEEDED assert (speed hint)
            assert(0 <= witness < new_neighbors.spec_len() as int);
            // Veracity: NEEDED assert (speed hint)
            assert(new_neighbors.spec_index(witness) == v);

            // Build new adj: tabulate copies each row; row u gets new_neighbors.
            let result_adj = ArraySeqMtPerS::tabulate(
                &|k: usize| -> (r: ArraySeqMtPerS<usize>)
                    requires k < n_v
                    ensures
                        k as int != u as int ==> (
                            r.spec_len() == self.adj.spec_index(k as int).spec_len()
                            && forall|l: int| 0 <= l < r.spec_len()
                                ==> #[trigger] r.spec_index(l) == self.adj.spec_index(k as int).spec_index(l)
                        ),
                        k as int == u as int ==> (
                            r.spec_len() == new_neighbors.spec_len()
                            && forall|l: int| 0 <= l < r.spec_len()
                                ==> #[trigger] r.spec_index(l) == new_neighbors.spec_index(l)
                        )
                {
                    if k == u {
                        let nn_len = new_neighbors.length();
                        ArraySeqMtPerS::tabulate(
                            &|i: usize| -> (r: usize)
                                requires i < nn_len
                                ensures r == new_neighbors.spec_index(i as int)
                            { *new_neighbors.nth(i) },
                            nn_len,
                        )
                    } else {
                        let src = self.adj.nth(k);
                        let len = src.length();
                        ArraySeqMtPerS::tabulate(
                            &|i: usize| -> (r: usize)
                                requires i < len
                                ensures r == src.spec_index(i as int)
                            { *src.nth(i) },
                            len,
                        )
                    }
                },
                n_v,
            );

            // Compute new num_edges: self.num_edges + (0 or 1 depending on found).
            let new_num_edges: usize = if found { self.num_edges } else { self.num_edges + 1 };

            let updated = AdjSeqGraphMtPer { adj: result_adj, num_edges: new_num_edges };
// Veracity: UNNEEDED assert             assert(updated.spec_degree(u as int) == new_neighbors.spec_len());
            // Veracity: NEEDED assert (speed hint)
            assert(updated.spec_neighbor(u as int, witness) == new_neighbors.spec_index(witness));
            // Veracity: NEEDED assert (speed hint)
            assert(updated.spec_neighbor(u as int, witness) == v);

            proof {
                let old_degree_fn = |i: int| self.spec_degree(i);
                let new_degree_fn = |i: int| updated.spec_degree(i);
// Veracity: UNNEEDED assert                 assert(forall|i: int| 0 <= i < n_v as int && i != u as int
// Veracity: UNNEEDED assert                     ==> #[trigger] old_degree_fn(i) == new_degree_fn(i));
                lemma_sum_of_change_one(n_v as int, old_degree_fn, new_degree_fn, u as int);
                // Veracity: NEEDED assert (speed hint)
                assert(updated.spec_degree(u as int) == new_neighbors.spec_len());
                if found {
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_neighbors.spec_len() == deg_u as nat);
// Veracity: UNNEEDED assert                     assert(new_degree_fn(u as int) == old_degree_fn(u as int));
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_sum_of(n_v as int, new_degree_fn) == spec_sum_of(n_v as int, old_degree_fn));
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_num_edges as nat == self.num_edges as nat);
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_num_edges as nat == spec_sum_of(n_v as int, new_degree_fn));
                } else {
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_neighbors.spec_len() == deg_u + 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_degree_fn(u as int) == old_degree_fn(u as int) + 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(spec_sum_of(n_v as int, new_degree_fn) == spec_sum_of(n_v as int, old_degree_fn) + 1);
// Veracity: UNNEEDED assert                     assert(new_num_edges as nat == self.num_edges as nat + 1);
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_num_edges as nat == spec_sum_of(n_v as int, new_degree_fn));
                }
                // Veracity: NEEDED assert (speed hint)
                assert(updated.num_edges as nat == spec_sum_of(updated.spec_num_vertices() as int, |i: int| updated.spec_degree(i)));
            }

            updated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
        fn delete_edge(&self, u: usize, v: usize) -> (updated: Self) {
            let n_v = self.adj.length();
            let src_u = self.adj.nth(u);
            let deg_u = src_u.length();

            // Build filtered neighbors for vertex u (exclude v).
            let mut nvec = Vec::<usize>::new();
            let mut j: usize = 0;
            while j < deg_u
                invariant
                    j <= deg_u,
                    u < self.spec_num_vertices(),
                    deg_u as nat == self.spec_degree(u as int),
                    deg_u as nat == src_u.spec_len(),
                    nvec@.len() <= j,
                    forall|k: int| 0 <= k < nvec@.len() as int
                        ==> #[trigger] nvec@[k] != v,
                    self.spec_adjseqgraphmtper_wf(),
                    forall|k: int| 0 <= k < nvec@.len() as int
                        ==> nvec@[k] < self.adj.spec_len(),
                decreases deg_u - j
            {
                let neighbor = *self.adj.nth(u).nth(j);
                if neighbor != v {
                    nvec.push(neighbor);
                }
                j = j + 1;
            }
            let new_deg_u = nvec.len();
            let new_neighbors = ArraySeqMtPerS::from_vec(nvec);

            // Build new adj: tabulate copies each row; row u gets new_neighbors.
            let result_adj = ArraySeqMtPerS::tabulate(
                &|k: usize| -> (r: ArraySeqMtPerS<usize>)
                    requires k < n_v
                    ensures
                        k as int != u as int ==> (
                            r.spec_len() == self.adj.spec_index(k as int).spec_len()
                            && forall|l: int| 0 <= l < r.spec_len()
                                ==> #[trigger] r.spec_index(l) == self.adj.spec_index(k as int).spec_index(l)
                        ),
                        k as int == u as int ==> (
                            r.spec_len() == new_neighbors.spec_len()
                            && forall|l: int| 0 <= l < r.spec_len()
                                ==> #[trigger] r.spec_index(l) == new_neighbors.spec_index(l)
                        )
                {
                    if k == u {
                        let nn_len = new_neighbors.length();
                        ArraySeqMtPerS::tabulate(
                            &|i: usize| -> (r: usize)
                                requires i < nn_len
                                ensures r == new_neighbors.spec_index(i as int)
                            { *new_neighbors.nth(i) },
                            nn_len,
                        )
                    } else {
                        let src = self.adj.nth(k);
                        let len = src.length();
                        ArraySeqMtPerS::tabulate(
                            &|i: usize| -> (r: usize)
                                requires i < len
                                ensures r == src.spec_index(i as int)
                            { *src.nth(i) },
                            len,
                        )
                    }
                },
                n_v,
            );

            // Veracity: NEEDED assert
            assert forall|u2: int, j2: int|
                0 <= u2 < result_adj.spec_len()
                && 0 <= j2 < result_adj.spec_index(u2).spec_len()
            implies #[trigger] result_adj.spec_index(u2).spec_index(j2) < result_adj.spec_len()
            by {
                if u2 != u as int {
                    // Veracity: NEEDED assert (speed hint)
                    assert(result_adj.spec_index(u2).spec_index(j2) == self.adj.spec_index(u2).spec_index(j2));
                }
            }

            // Prove overflow safety before subtraction.
            proof {
                let old_degree_fn = |i: int| self.spec_degree(i);
                lemma_sum_of_lower_bound(n_v as int, old_degree_fn, u as int);
// Veracity: UNNEEDED assert                 assert(new_deg_u <= deg_u);
            }

            // Compute new num_edges: self.num_edges - (deg_u - new_deg_u).
            let new_num_edges: usize = self.num_edges - (deg_u - new_deg_u);

            let updated = AdjSeqGraphMtPer { adj: result_adj, num_edges: new_num_edges };

            proof {
                let old_degree_fn = |i: int| self.spec_degree(i);
                let new_degree_fn = |i: int| updated.spec_degree(i);
                // Veracity: NEEDED assert (speed hint)
                assert(updated.spec_degree(u as int) == new_neighbors.spec_len());
                // Veracity: NEEDED assert (speed hint)
                assert(new_neighbors.spec_len() == new_deg_u as nat);
                // Veracity: NEEDED assert (speed hint)
                assert(forall|i: int| 0 <= i < n_v as int && i != u as int
                    ==> #[trigger] old_degree_fn(i) == new_degree_fn(i));
                lemma_sum_of_change_one(n_v as int, old_degree_fn, new_degree_fn, u as int);
                // Veracity: NEEDED assert (speed hint)
                assert(spec_sum_of(n_v as int, new_degree_fn) + old_degree_fn(u as int)
                    == spec_sum_of(n_v as int, old_degree_fn) + new_degree_fn(u as int));
// Veracity: UNNEEDED assert                 assert(old_degree_fn(u as int) == deg_u as nat);
                // Veracity: NEEDED assert (speed hint)
                assert(new_degree_fn(u as int) == new_deg_u as nat);
// Veracity: UNNEEDED assert                 assert(new_deg_u <= deg_u);
                // Veracity: NEEDED assert (speed hint)
                assert(spec_sum_of(n_v as int, old_degree_fn) == self.num_edges as nat);
// Veracity: UNNEEDED assert                 assert(spec_sum_of(n_v as int, new_degree_fn)
// Veracity: UNNEEDED assert                     == self.num_edges as nat - (deg_u - new_deg_u) as nat);
// Veracity: UNNEEDED assert                 assert(new_num_edges as nat == spec_sum_of(n_v as int, new_degree_fn));
// Veracity: UNNEEDED assert                 assert(updated.num_edges as nat == spec_sum_of(updated.spec_num_vertices() as int, |i: int| updated.spec_degree(i)));
            }

            updated
        }
    }

    //		Section 12. derive impls in verus!


    impl Clone for AdjSeqGraphMtPer {
        fn clone(&self) -> (out: Self) {
            AdjSeqGraphMtPer { adj: self.adj.clone(), num_edges: self.num_edges }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for AdjSeqGraphMtPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AdjSeqGraphMtPer")
                .field("adj", &self.adj)
                .field("num_edges", &self.num_edges)
                .finish()
        }
    }

    impl std::fmt::Display for AdjSeqGraphMtPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjSeqGraphMtPer(n: {}, edges: {})", self.adj.length(), self.num_edges)
        }
    }
}
