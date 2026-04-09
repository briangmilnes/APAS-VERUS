//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod DirGraphStEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::seq_set::*;

verus! 
{


    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    //		Section 3. broadcast use


    // Broadcast groups for hash collections, sets, and our custom axioms
    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        vstd::set::group_set_axioms,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEph<V: StT + Hash> { pub V: SetStEph<V>, pub A: SetStEph<Edge<V>> }

    //		Section 5. view impls


    // View implementation: GraphView with named V and A fields
    impl<V: StT + Hash> View for DirGraphStEph<V> {
        type V = GraphView<<V as View>::V>;

        open spec fn view(&self) -> Self::V { GraphView { V: self.V@, A: self.A@ } }
    }

    //		Section 8. traits


    pub trait DirGraphStEphTrait<V: StT + Hash>:
         View<V = GraphView<<V as View>::V>> + Sized {

        spec fn spec_dirgraphsteph_wf(&self) -> bool;

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { Set::new(|w: V::V| self@.A.contains((v, w))) }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { Set::new(|u: V::V| self@.A.contains((u, v))) }

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { self.spec_n_plus(v).union(self.spec_n_minus(v)) }

        open spec fn spec_degree(&self, v: V::V) -> nat
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { self.spec_ng(v).len() }

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))
        }

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w))
        }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_Edge::<V>()
            ensures
                g.spec_dirgraphsteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|V| + |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|V| + |A|), Span O(|V| + |A|) — sequential
        fn from_sets(vertices: SetStEph<V>, arcs: SetStEph<Edge<V>>) -> (g: Self)
            requires
                forall |u: V::V, w: V::V|
                    #[trigger] arcs@.contains((u, w)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_dirgraphsteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= vertices@,
                g@.A =~= arcs@;

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>)
            ensures a@ =~= self@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeV(&self) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>()
            ensures n == self@.V.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeA(&self) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>()
            ensures n == self@.A.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>()
            ensures b == self@.A.contains((u@, v@));

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures neighbors@ == self.spec_ng(v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|vertices| × |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|vertices| × |A|), Span O(|vertices| × |A|) — nested iteration
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures neighbors@ == self.spec_ng_of_vertices(vertices@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures out_neighbors@ == self.spec_n_plus(v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures in_neighbors@ == self.spec_n_minus(v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|vertices| × |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|vertices| × |A|), Span O(|vertices| × |A|) — nested iteration
        fn n_plus_of_vertices(&self, vertices: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures out_neighbors@ == self.spec_n_plus_of_vertices(vertices@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|vertices| × |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|vertices| × |A|), Span O(|vertices| × |A|) — nested iteration
        fn n_minus_of_vertices(&self, vertices: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures in_neighbors@ == self.spec_n_minus_of_vertices(vertices@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn degree(&self, v: &V) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_degree(v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn in_degree(&self, v: &V) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_n_minus(v@).len();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential filter
        fn out_degree(&self, v: &V) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_n_plus(v@).len();
    }

    //		Section 9. impls


    impl<V: StT + Hash> DirGraphStEph<V> {
        /// Returns an iterator over the vertices
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        pub fn iter_vertices(&self) -> (it: SetStEphIter<'_, V>)
            requires valid_key_type_Edge::<V>(),
            ensures true,
       { self.V.iter() }

        /// Returns an iterator over the arcs
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        pub fn iter_arcs(&self) -> (it: SetStEphIter<'_, Edge<V>>)
            requires valid_key_type_Edge::<V>(),
            ensures true,
        { self.A.iter() }
    }

    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {

        open spec fn spec_dirgraphsteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: DirGraphStEph<V>)
            ensures g.spec_dirgraphsteph_wf()
        { DirGraphStEph { V: SetStEph::empty(), A: SetStEph::empty() } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>)
            ensures g.spec_dirgraphsteph_wf()
        { DirGraphStEph { V, A } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>) { &self.A }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeV(&self) -> (n: usize) { self.V.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeA(&self) -> (n: usize) { self.A.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool) { self.A.mem(&Edge(u.clone_plus(), v.clone_plus())) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|)
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            ensures neighbors@ == self.spec_ng(v@)
        { self.n_plus(v).union(&self.n_minus(v)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(|S| * |A|) -- iterates vertices, calls ng for each
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            ensures neighbors@ == self.spec_ng_of_vertices(vertices@)
        {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    spec_graphview_wf(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    neighbors@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_ng(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies
                            self.spec_ng_of_vertices(vertices_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_ng(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_ng_of_vertices(vertices_view).contains(w) implies
                            neighbors@.contains(w) by {
                                if self.spec_ng_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_ng(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(u) => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                        }
                        let ng_u = self.ng(u);
                        neighbors = neighbors.union(&ng_u);
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential scan of arcs
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            ensures out_neighbors@ == self.spec_n_plus(v@)
        {
            let mut out: SetStEph<V> = SetStEph::empty();
            let mut it = self.A.iter();
            let ghost arcs_seq = it@.1;
            let ghost v_view = v@;
            let ghost arcs_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= arcs_seq.len(),
                    it@.1 == arcs_seq,
                    arcs_seq.map(|i: int, e: Edge<V>| e@).to_set() == arcs_view,
                    out@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger arcs_seq[i]]
                        0 <= i < it@.0 && arcs_seq[i]@.0 == v_view && arcs_seq[i]@.1 == w),
                    decreases arcs_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] out@.contains(w) implies
                            self.spec_n_plus(v_view).contains(w) by {
                                if out@.contains(w) {
                                    let i = choose |i: int| #![trigger arcs_seq[i]] 0 <= i < arcs_seq.len() && arcs_seq[i]@.0 == v_view && arcs_seq[i]@.1 == w;
                                    lemma_seq_index_in_map_to_set(arcs_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_n_plus(v_view).contains(w) implies
                            out@.contains(w) by {
                                if self.spec_n_plus(v_view).contains(w) {
                                    lemma_map_to_set_contains_index(arcs_seq, (v_view, w));
                                }
                            }
                        }
                        return out;
                    },
                    Some(edge) => {
                        let x = &edge.0;
                        let y = edge.1.clone_plus();
                        if feq(x, v) {
                            let _ = out.insert(y);
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential scan of arcs
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            ensures in_neighbors@ == self.spec_n_minus(v@)
        {
            let mut inn: SetStEph<V> = SetStEph::empty();
            let mut it = self.A.iter();
            let ghost arcs_seq = it@.1;
            let ghost v_view = v@;
            let ghost arcs_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= arcs_seq.len(),
                    it@.1 == arcs_seq,
                    arcs_seq.map(|i: int, e: Edge<V>| e@).to_set() == arcs_view,
                    inn@ == Set::new(|u: V::V| exists |i: int|
                        #![trigger arcs_seq[i]]
                        0 <= i < it@.0 && arcs_seq[i]@.1 == v_view && arcs_seq[i]@.0 == u),
                    decreases arcs_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |u: V::V| #[trigger] inn@.contains(u) implies
                            self.spec_n_minus(v_view).contains(u) by {
                                if inn@.contains(u) {
                                    let i = choose |i: int| #![trigger arcs_seq[i]] 0 <= i < arcs_seq.len() && arcs_seq[i]@.1 == v_view && arcs_seq[i]@.0 == u;
                                    lemma_seq_index_in_map_to_set(arcs_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |u: V::V| #[trigger] self.spec_n_minus(v_view).contains(u) implies
                            inn@.contains(u) by {
                                if self.spec_n_minus(v_view).contains(u) {
                                    lemma_map_to_set_contains_index(arcs_seq, (u, v_view));
                                }
                            }
                        }
                        return inn;
                    },
                    Some(edge) => {
                        let x = edge.0.clone_plus();
                        let y = &edge.1;
                        if feq(y, v) {
                            let _ = inn.insert(x);
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(|S| * |A|) -- iterates vertices, calls n_plus for each
        fn n_plus_of_vertices(&self, vertices: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            ensures out_neighbors@ == self.spec_n_plus_of_vertices(vertices@)
        {
            let mut out_neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    spec_graphview_wf(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    out_neighbors@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_n_plus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] out_neighbors@.contains(w) implies
                            self.spec_n_plus_of_vertices(vertices_view).contains(w) by {
                                if out_neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_n_plus(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_n_plus_of_vertices(vertices_view).contains(w) implies
                            out_neighbors@.contains(w) by {
                                if self.spec_n_plus_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_n_plus(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return out_neighbors;
                    },
                    Some(u) => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                        }
                        let plus_u = self.n_plus(u);
                        out_neighbors = out_neighbors.union(&plus_u);
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(|S| * |A|) -- iterates vertices, calls n_minus for each
        fn n_minus_of_vertices(&self, vertices: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            ensures in_neighbors@ == self.spec_n_minus_of_vertices(vertices@)
        {
            let mut in_neighbors: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    spec_graphview_wf(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    in_neighbors@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_n_minus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] in_neighbors@.contains(w) implies
                            self.spec_n_minus_of_vertices(vertices_view).contains(w) by {
                                if in_neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_n_minus(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_n_minus_of_vertices(vertices_view).contains(w) implies
                            in_neighbors@.contains(w) by {
                                if self.spec_n_minus_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_n_minus(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return in_neighbors;
                    },
                    Some(u) => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                        }
                        let minus_u = self.n_minus(u);
                        in_neighbors = in_neighbors.union(&minus_u);
                    }
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool) { feq(&e.0, v) || feq(&e.1, v) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|)
        fn degree(&self, v: &V) -> (n: usize)
            ensures n == self.spec_degree(v@)
        { self.ng(v).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|)
        fn in_degree(&self, v: &V) -> (n: usize)
            ensures n == self.spec_n_minus(v@).len()
        { self.n_minus(v).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|)
        fn out_degree(&self, v: &V) -> (n: usize)
            ensures n == self.spec_n_plus(v@).len()
        { self.n_plus(v).size() }
    }

    //		Section 10. iterators


    /// Iterator wrapper for DirGraphStEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEphIter<'a, V: StT + Hash> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: StT + Hash> View for DirGraphStEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: StT + Hash>(it: &DirGraphStEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: StT + Hash> std::iter::Iterator for DirGraphStEphIter<'a, V> {
        type Item = &'a V;

        fn next(&mut self) -> (next: Option<&'a V>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEphGhostIterator<'a, V: StT + Hash> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: StT + Hash> vstd::pervasive::ForLoopGhostIteratorNew for DirGraphStEphIter<'a, V> {
        type GhostIter = DirGraphStEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> DirGraphStEphGhostIterator<'a, V> {
            DirGraphStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: StT + Hash> vstd::pervasive::ForLoopGhostIterator for DirGraphStEphGhostIterator<'a, V> {
        type ExecIter = DirGraphStEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &DirGraphStEphIter<'a, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<V> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &DirGraphStEphIter<'a, V>) -> DirGraphStEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: StT + Hash> View for DirGraphStEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: StT + Hash> std::iter::IntoIterator for &'a DirGraphStEph<V> {
        type Item = &'a V;
        type IntoIter = DirGraphStEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            DirGraphStEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash> PartialEqSpecImpl for DirGraphStEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<V: StT + Hash> Clone for DirGraphStEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            DirGraphStEph { V: self.V.clone(), A: self.A.clone() }
        }
    }

    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}

    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let v_eq = self.V == other.V;
            let a_eq = self.A == other.A;
            // Veracity: NEEDED proof block
            proof {
                if v_eq && a_eq {
                }
            }
            v_eq && a_eq
        }
    }

    } // verus!

    //		Section 13. macros


    // Macro defined outside verus! block
    #[macro_export]
    macro_rules! DirGraphStEphLit {
        () => {{
            let __V : $crate::Chap05::SetStEph::SetStEph::SetStEph<_>                             = $crate::SetLit![];
            let __A : $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::from_sets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V : $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __A : $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::from_sets(__V, __A)
        }}
    }

    //		Section 14. derive impls outside verus!

    impl<V: StT + Hash> Debug for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphStEph")
            .field("V", &self.V)
            .field("A", &self.A)
            .finish()
        }
    }

    impl<V: StT + Hash> Display for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<'a, V: StT + Hash> Debug for DirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphStEphIter") }
    }

    impl<'a, V: StT + Hash> Display for DirGraphStEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphStEphIter") }
    }

    impl<'a, V: StT + Hash> Debug for DirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphStEphGhostIterator") }
    }

    impl<'a, V: StT + Hash> Display for DirGraphStEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphStEphGhostIterator") }
    }
}
