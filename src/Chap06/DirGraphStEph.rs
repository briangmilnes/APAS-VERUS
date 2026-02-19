//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod DirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    use crate::vstdplus::seq_set::*;

verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;


    //		3. broadcast use

    // Broadcast groups for hash collections, sets, and our custom axioms
    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEph<V: StT + Hash> { pub V: SetStEph<V>, pub A: SetStEph<Edge<V>> }


    //		5. view impls

    // View implementation: GraphView with named V and A fields
    impl<V: StT + Hash> View for DirGraphStEph<V> {
        type V = GraphView<<V as View>::V>;

        open spec fn view(&self) -> Self::V { GraphView { V: self.V@, A: self.A@ } }
    }


    //		8. traits

    pub trait DirGraphStEphTrait<V: StT + Hash>:
         View<V = GraphView<<V as View>::V>> + Sized {

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends wf_graph_view(self@), self@.V.contains(v)
        { Set::new(|w: V::V| self@.A.contains((v, w))) }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends wf_graph_view(self@), self@.V.contains(v)
        { Set::new(|u: V::V| self@.A.contains((u, v))) }

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends wf_graph_view(self@), self@.V.contains(v)
        { self.spec_n_plus(v).union(self.spec_n_minus(v)) }

        open spec fn spec_degree(&self, v: V::V) -> nat
            recommends wf_graph_view(self@), self@.V.contains(v)
        { self.spec_ng(v).len() }

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))
        }

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w))
        }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn empty() -> (g: DirGraphStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures
                wf_graph_view(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|V| + |A|), Span Θ(|V| + |A|) — sequential
        fn from_sets(vertices: SetStEph<V>, arcs: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>)
            requires
                forall |u: V::V, w: V::V| 
                    #[trigger] arcs@.contains((u, w)) ==> 
                        vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_graph_view(g@),
                g@.V =~= vertices@,
                g@.A =~= arcs@;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>)
            ensures a@ =~= self@.A;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>()
            ensures n == self@.V.len();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn sizeA(&self) -> (n: N)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>()
            ensures n == self@.A.len();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>()
            ensures b == self@.A.contains((u@, v@));

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures neighbors@ == self.spec_ng(v@);

        /// - APAS: Work Θ(|vertices| × |A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|vertices| × |A|), Span Θ(|vertices| × |A|) — nested iteration
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures neighbors@ == self.spec_ng_of_vertices(vertices@);

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures out_neighbors@ == self.spec_n_plus(v@);

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures in_neighbors@ == self.spec_n_minus(v@);

        /// - APAS: Work Θ(|vertices| × |A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|vertices| × |A|), Span Θ(|vertices| × |A|) — nested iteration
        fn n_plus_of_vertices(&self, vertices: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures out_neighbors@ == self.spec_n_plus_of_vertices(vertices@);

        /// - APAS: Work Θ(|vertices| × |A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|vertices| × |A|), Span Θ(|vertices| × |A|) — nested iteration
        fn n_minus_of_vertices(&self, vertices: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), vertices@ <= self@.V
            ensures in_neighbors@ == self.spec_n_minus_of_vertices(vertices@);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn degree(&self, v: &V) -> (n: N)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_degree(v@);

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn in_degree(&self, v: &V) -> (n: N)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_n_minus(v@).len();

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential filter
        fn out_degree(&self, v: &V) -> (n: N)
            requires wf_graph_view(self@), valid_key_type_Edge::<V>(), self@.V.contains(v@)
            ensures n == self.spec_n_plus(v@).len();
    }


    //		9. impls

    impl<V: StT + Hash> DirGraphStEph<V> {
        /// Returns an iterator over the vertices
        pub fn iter_vertices(&self) -> (it: SetStEphIter<'_, V>)
            requires valid_key_type_Edge::<V>() 
       { self.V.iter() }

        /// Returns an iterator over the arcs
        pub fn iter_arcs(&self) -> (it: SetStEphIter<'_, Edge<V>>)
            requires valid_key_type_Edge::<V>()
        { self.A.iter() }
    }

    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {

        fn empty() -> (g: DirGraphStEph<V>) { DirGraphStEph { V: SetStEph::empty(), A: SetStEph::empty() } }

        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>) { DirGraphStEph { V, A } }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        fn arcs(&self) -> (a: &SetStEph<Edge<V>>) { &self.A }

        fn sizeV(&self) -> (n: N) { self.V.size() }

        fn sizeA(&self) -> (n: N) { self.A.size() }

        fn neighbor(&self, u: &V, v: &V) -> (b: B) { self.A.mem(&Edge(u.clone_plus(), v.clone_plus())) }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            ensures neighbors@ == self.spec_ng(v@)
        { self.n_plus(v).union(&self.n_minus(v)) }

        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            ensures neighbors@ == self.spec_ng_of_vertices(vertices@)
        {
            let mut result: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    wf_graph_view(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_ng(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] result@.contains(w) implies
                            self.spec_ng_of_vertices(vertices_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_ng(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_ng_of_vertices(vertices_view).contains(w) implies
                            result@.contains(w) by {
                                if self.spec_ng_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_ng(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                            assert(vertices_view.contains(u@));
                            assert(self@.V.contains(u@));
                        }
                        let ng_u = self.ng(u);
                        result = result.union(&ng_u);
                    }
                }
            }
        }

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
                        proof {
                            assert forall |w: V::V| #[trigger] out@.contains(w) implies
                            self.spec_n_plus(v_view).contains(w) by {
                                if out@.contains(w) {
                                    let i = choose |i: int| #![trigger arcs_seq[i]] 0 <= i < arcs_seq.len() && arcs_seq[i]@.0 == v_view && arcs_seq[i]@.1 == w;
                                    lemma_seq_index_in_map_to_set(arcs_seq, i);
                                }
                            }
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
                        proof {
                            assert forall |u: V::V| #[trigger] inn@.contains(u) implies
                            self.spec_n_minus(v_view).contains(u) by {
                                if inn@.contains(u) {
                                    let i = choose |i: int| #![trigger arcs_seq[i]] 0 <= i < arcs_seq.len() && arcs_seq[i]@.1 == v_view && arcs_seq[i]@.0 == u;
                                    lemma_seq_index_in_map_to_set(arcs_seq, i);
                                }
                            }
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

        fn n_plus_of_vertices(&self, vertices: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            ensures out_neighbors@ == self.spec_n_plus_of_vertices(vertices@)
        {
            let mut result: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    wf_graph_view(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_n_plus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] result@.contains(w) implies
                            self.spec_n_plus_of_vertices(vertices_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_n_plus(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_n_plus_of_vertices(vertices_view).contains(w) implies
                            result@.contains(w) by {
                                if self.spec_n_plus_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_n_plus(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                            assert(vertices_view.contains(u@));
                            assert(self@.V.contains(u@));
                        }
                        let plus_u = self.n_plus(u);
                        result = result.union(&plus_u);
                    }
                }
            }
        }

        fn n_minus_of_vertices(&self, vertices: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            ensures in_neighbors@ == self.spec_n_minus_of_vertices(vertices@)
        {
            let mut result: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    wf_graph_view(self@),
                    vertices_view <= self@.V,
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![trigger u_seq[i]]
                        0 <= i < it@.0 && self.spec_n_minus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] result@.contains(w) implies
                            self.spec_n_minus_of_vertices(vertices_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_n_minus(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_n_minus_of_vertices(vertices_view).contains(w) implies
                            result@.contains(w) by {
                                if self.spec_n_minus_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_n_minus(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        proof {
                            lemma_seq_index_in_map_to_set(u_seq, it@.0 - 1);
                            assert(vertices_view.contains(u@));
                            assert(self@.V.contains(u@));
                        }
                        let minus_u = self.n_minus(u);
                        result = result.union(&minus_u);
                    }
                }
            }
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) { feq(&e.0, v) || feq(&e.1, v) }

        fn degree(&self, v: &V) -> (n: N)
            ensures n == self.spec_degree(v@)
        { self.ng(v).size() }

        fn in_degree(&self, v: &V) -> (n: N)
            ensures n == self.spec_n_minus(v@).len()
        { self.n_minus(v).size() }

        fn out_degree(&self, v: &V) -> (n: N)
            ensures n == self.spec_n_plus(v@).len()
        { self.n_plus(v).size() }
    }

    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash> PartialEqSpecImpl for DirGraphStEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    impl<'a, V: StT + Hash> std::iter::IntoIterator for &'a DirGraphStEph<V> {
        type Item = &'a V;
        type IntoIter = SetStEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
        {
            self.vertices().iter()
        }
    }

    // Ghost view for graph vertex iterator: (visited, current, remaining) sets
    #[verifier::reject_recursive_types(V)]
    pub ghost struct DirGraphVertexIterView<V> {
        pub visited: Set<V>,
        pub current: Option<V>,
        pub remaining: Set<V>,
    }

    // Ghost view for graph arc iterator: (visited, current, remaining) sets  
    #[verifier::reject_recursive_types(V)]
    pub ghost struct DirGraphArcIterView<V> {
        pub visited: Set<(V, V)>,
        pub current: Option<(V, V)>,
        pub remaining: Set<(V, V)>,
    }


    //		11. derive impls in verus!

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
            proof {
                if v_eq && a_eq {
                    assert(self@ =~= other@);
                }
            }
            v_eq && a_eq
        }
    }

    } // verus!


    //		13. derive impls outside verus!

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


    //		12. macros

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
}
