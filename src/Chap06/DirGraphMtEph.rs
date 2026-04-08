//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (n_plus, n_minus) and vertex map-reduce (ng_of_vertices, etc.) are parallel.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 6a. spec fns
//	Section 8a. traits
//	Section 9a. impls
//	Section 10a. iterators
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 5c. view impls
//	Section 8c. traits
//	Section 9c. impls
//	Section 11b. top level coarse locking
//	Section 12a. derive impls in verus!
//	Section 13. macros
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module


pub mod DirGraphMtEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::{ParaPair, ParaPairDisjoint, SetLit};
    use crate::vstdplus::accept::accept;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    use vstd::rwlock::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::set_lib::group_set_lib_default,
    };

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub V: SetStEph<V>,
        pub A: SetStEph<Edge<V>>,
    }

    //		Section 5a. view impls


    impl<V: StTInMtT + Hash + 'static> View for DirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.A@ }
        }
    }

    //		Section 6a. spec fns


    pub open spec fn valid_key_type_for_graph<V: StTInMtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }

    //		Section 8a. traits


    pub trait DirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {

        spec fn spec_dirgraphmteph_wf(&self) -> bool;

        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> { self@.A }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures
                g.spec_dirgraphmteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|V| + |A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |A|), Span O(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: Self)
            requires
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                A@.finite(),
                forall |u: V::V, w: V::V|
                    #[trigger] A@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures
                g.spec_dirgraphmteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= V@,
                g@.A =~= A@;

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
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.V.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeA(&self) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.A.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == self@.A.contains((u@, v@));

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V> 
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| self@.A.contains((v, w)))
        }

        open spec fn spec_n_plus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                spec_graphview_wf(self@),
                subarcs <= self@.A,
        {
            Set::new(|w: V::V| subarcs.contains((v, w)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — ParaPair! split arcs
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == self.spec_n_plus(v@),
                out_neighbors@ <= self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — calls n_plus
        fn out_degree(&self, v: &V) -> (n: usize)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_n_plus(v@).len();

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V> 
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|u: V::V| self@.A.contains((u, v))) 
        }

        open spec fn spec_n_minus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                spec_graphview_wf(self@),
                subarcs <= self@.A,
        {
            Set::new(|u: V::V| subarcs.contains((u, v)))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — ParaPair! split arcs
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == self.spec_n_minus(v@),
                in_neighbors@ <= self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — calls n_minus
        fn in_degree(&self, v: &V) -> (n: usize)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_n_minus(v@).len();

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { 
            self.spec_n_plus(v).union(self.spec_n_minus(v)) 
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — n_plus + n_minus
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        open spec fn spec_degree(&self, v: V::V) -> nat 
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { 
            self.spec_ng(v).len() 
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|A|), Span O(log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) — calls ng
        fn degree(&self, v: &V) -> (n: usize)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_degree(v@);

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))
        }

        open spec fn spec_n_plus_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_n_plus(u).contains(w))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — ParaPair! split vertices
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == self.spec_n_plus_of_vertices(u_set@),
                out_neighbors@ <= self@.V;

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w))
        }

        open spec fn spec_n_minus_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_n_minus(u).contains(w))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — ParaPair! split vertices
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == self.spec_n_minus_of_vertices(u_set@),
                in_neighbors@ <= self@.V;

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        open spec fn spec_ng_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_ng(u).contains(w))
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.1): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|u_set| × |A|), Span O(log |u_set| + log |A|) — ParaPair! split vertices
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;

        /// Parallel arc filtering for out-neighbors using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on arcs
        fn n_plus_par(&self, v: V, arcs: SetStEph<Edge<V>>) -> (out_neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                arcs@ <= self@.A,
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == self.spec_n_plus_from_set(v@, arcs@),
                out_neighbors@ <= self.spec_n_plus(v@)
            decreases arcs@.len();

        /// Parallel arc filtering for in-neighbors using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on arcs
        fn n_minus_par(&self, v: V, arcs: SetStEph<Edge<V>>) -> (in_neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                arcs@ <= self@.A,
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == self.spec_n_minus_from_set(v@, arcs@),
                in_neighbors@ <= self.spec_n_minus(v@)
            decreases arcs@.len();

        /// Parallel out-neighbors over a set of vertices using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_plus_of_vertices_par(&self, verts: SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                verts@ <= self@.V,
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == self.spec_n_plus_of_vertices_from_set(verts@),
                out_neighbors@ <= self@.V
            decreases verts@.len();

        /// Parallel in-neighbors over a set of vertices using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_minus_of_vertices_par(&self, verts: SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                verts@ <= self@.V,
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == self.spec_n_minus_of_vertices_from_set(verts@),
                in_neighbors@ <= self@.V
            decreases verts@.len();

        /// Parallel all-neighbors over a set of vertices using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn ng_of_vertices_par(&self, verts: SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                verts@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices_from_set(verts@),
                neighbors@ <= self@.V
            decreases verts@.len();
    }

    //		Section 9a. impls


    impl<V: StTInMtT + Hash + 'static> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {

        open spec fn spec_dirgraphmteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: DirGraphMtEph<V>) {
            DirGraphMtEph { V: SetStEph::empty(), A: SetStEph::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: DirGraphMtEph<V>) {
            DirGraphMtEph { V, A }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>) { &self.A }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeV(&self) -> (n: usize) { self.V.size() }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeA(&self) -> (n: usize) { self.A.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool) {
            self.A.mem(&Edge(u.clone_plus(), v.clone_plus()))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool) { feq(&e.0, v) || feq(&e.1, v) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- delegates to n_plus_par
        fn n_plus(&self, v: &V) -> SetStEph<V> {
            let arcs = self.A.clone();
            self.n_plus_par(v.clone_plus(), arcs)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|)
        fn out_degree(&self, v: &V) -> (n: usize) { self.n_plus(v).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- delegates to n_minus_par
        fn n_minus(&self, v: &V) -> SetStEph<V> {
            let arcs = self.A.clone();
            self.n_minus_par(v.clone_plus(), arcs)
        }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|)
        fn in_degree(&self, v: &V) -> (n: usize) { self.n_minus(v).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|)
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) { self.n_plus(v).union(&self.n_minus(v)) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|)
        fn degree(&self, v: &V) -> (n: usize) { self.ng(v).size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { self.n_plus_of_vertices_par(u_set.clone()) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { self.n_minus_of_vertices_par(u_set.clone()) }
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { self.ng_of_vertices_par(u_set.clone()) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on arcs
        fn n_plus_par(&self, v: V, arcs: SetStEph<Edge<V>>) -> (out_neighbors: SetStEph<V>)
            decreases arcs@.len()
        {
            let n = arcs.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let Edge(x, y) = arcs.choose();
                if feq(&x, &v) {
                    SetStEph::singleton(y.clone_plus())
                } else {
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_arcs, right_arcs) = arcs.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_plus_from_set(v_left@, left_arcs@)
                { g_left.n_plus_par(v_left, left_arcs) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_plus_from_set(v_right@, right_arcs@)
                { g_right.n_plus_par(v_right, right_arcs) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                left_neighbors.union(&right_neighbors)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- parallel split on arcs
        fn n_minus_par(&self, v: V, arcs: SetStEph<Edge<V>>) -> (in_neighbors: SetStEph<V>)
            decreases arcs@.len()
        {
            let n = arcs.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let Edge(x, y) = arcs.choose();
                if feq(&y, &v) {
                    SetStEph::singleton(x.clone_plus())
                } else {
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_arcs, right_arcs) = arcs.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_minus_from_set(v_left@, left_arcs@)
                { g_left.n_minus_par(v_left, left_arcs) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_minus_from_set(v_right@, right_arcs@)
                { g_right.n_minus_par(v_right, right_arcs) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                left_neighbors.union(&right_neighbors)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_plus_of_vertices_par(&self, verts: SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            decreases verts@.len()
        {
            let n = verts.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let u = verts.choose();
                self.n_plus_par(u, self.A.clone())
            }
            else {
                let mid = n / 2;
                let (left_verts, right_verts) = verts.split(mid);
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_plus_of_vertices_from_set(left_verts@)
                { g_left.n_plus_of_vertices_par(left_verts) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_plus_of_vertices_from_set(right_verts@)
                { g_right.n_plus_of_vertices_par(right_verts) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                let out_neighbors = left_neighbors.union(&right_neighbors);
                proof {
                    assert forall |w: V::V| #![trigger out_neighbors@.contains(w)] self.spec_n_plus_of_vertices_from_set(verts@).contains(w)
                        <==> out_neighbors@.contains(w) by {
                        if self.spec_n_plus_of_vertices_from_set(verts@).contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![trigger verts@.contains(v)] verts@.contains(v) && self.spec_n_plus(v).contains(w);
                            if left_verts@.contains(v_wit) {
                            } else {
                            }
                        }
                        if out_neighbors@.contains(w) {
                            if left_neighbors@.contains(w) {
                                let v_wit: V::V = choose |v: V::V| #![trigger left_verts@.contains(v)] left_verts@.contains(v) && self.spec_n_plus(v).contains(w);
                                assert(verts@.contains(v_wit));
                            } else {
                                let v_wit: V::V = choose |v: V::V| #![trigger right_verts@.contains(v)] right_verts@.contains(v) && self.spec_n_plus(v).contains(w);
                                assert(verts@.contains(v_wit));
                            }
                        }
                    }
                }
                out_neighbors
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn n_minus_of_vertices_par(&self, verts: SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            decreases verts@.len()
        {
            let n = verts.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let u = verts.choose();
                self.n_minus_par(u, self.A.clone())
            }
            else {
                let mid = n / 2;
                let (left_verts, right_verts) = verts.split(mid);
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_n_minus_of_vertices_from_set(left_verts@)
                { g_left.n_minus_of_vertices_par(left_verts) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_n_minus_of_vertices_from_set(right_verts@)
                { g_right.n_minus_of_vertices_par(right_verts) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                let in_neighbors = left_neighbors.union(&right_neighbors);
                proof {
                    assert forall |w: V::V| #![trigger in_neighbors@.contains(w)] self.spec_n_minus_of_vertices_from_set(verts@).contains(w)
                        <==> in_neighbors@.contains(w) by {
                        if self.spec_n_minus_of_vertices_from_set(verts@).contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![trigger verts@.contains(v)] verts@.contains(v) && self.spec_n_minus(v).contains(w);
                            if left_verts@.contains(v_wit) {
                            } else {
                            }
                        }
                        if in_neighbors@.contains(w) {
                            if left_neighbors@.contains(w) {
                                let v_wit: V::V = choose |v: V::V| #![trigger left_verts@.contains(v)] left_verts@.contains(v) && self.spec_n_minus(v).contains(w);
                                assert(verts@.contains(v_wit));
                            } else {
                                let v_wit: V::V = choose |v: V::V| #![trigger right_verts@.contains(v)] right_verts@.contains(v) && self.spec_n_minus(v).contains(w);
                                assert(verts@.contains(v_wit));
                            }
                        }
                    }
                }
                in_neighbors
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|)
        fn ng_of_vertices_par(&self, verts: SetStEph<V>) -> (neighbors: SetStEph<V>)
            decreases verts@.len()
        {
            let n = verts.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let u = verts.choose();
                self.ng(&u)
            }
            else {
                let mid = n / 2;
                let (left_verts, right_verts) = verts.split(mid);
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_ng_of_vertices_from_set(left_verts@)
                { g_left.ng_of_vertices_par(left_verts) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_ng_of_vertices_from_set(right_verts@)
                { g_right.ng_of_vertices_par(right_verts) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                let neighbors = left_neighbors.union(&right_neighbors);
                proof {
                    assert forall |w: V::V| #![trigger neighbors@.contains(w)] self.spec_ng_of_vertices_from_set(verts@).contains(w)
                        <==> neighbors@.contains(w) by {
                        if self.spec_ng_of_vertices_from_set(verts@).contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![trigger verts@.contains(v)] verts@.contains(v) && self.spec_ng(v).contains(w);
                            if left_verts@.contains(v_wit) {
                            } else {
                            }
                        }
                        if neighbors@.contains(w) {
                            if left_neighbors@.contains(w) {
                                let v_wit: V::V = choose |v: V::V| #![trigger left_verts@.contains(v)] left_verts@.contains(v) && self.spec_ng(v).contains(w);
                                assert(verts@.contains(v_wit));
                            } else {
                                let v_wit: V::V = choose |v: V::V| #![trigger right_verts@.contains(v)] right_verts@.contains(v) && self.spec_ng(v).contains(w);
                                assert(verts@.contains(v_wit));
                            }
                        }
                    }
                }
                neighbors
            }
        }
    }

    //		Section 10a. iterators


    /// Iterator wrapper for DirGraphMtEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphMtEphIter<'a, V: StTInMtT + Hash + 'static> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: StTInMtT + Hash + 'static> View for DirGraphMtEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: StTInMtT + Hash + 'static>(it: &DirGraphMtEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: StTInMtT + Hash + 'static> std::iter::Iterator for DirGraphMtEphIter<'a, V> {
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
    pub struct DirGraphMtEphGhostIterator<'a, V: StTInMtT + Hash + 'static> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: StTInMtT + Hash + 'static> vstd::pervasive::ForLoopGhostIteratorNew for DirGraphMtEphIter<'a, V> {
        type GhostIter = DirGraphMtEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> DirGraphMtEphGhostIterator<'a, V> {
            DirGraphMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> vstd::pervasive::ForLoopGhostIterator for DirGraphMtEphGhostIterator<'a, V> {
        type ExecIter = DirGraphMtEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &DirGraphMtEphIter<'a, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &DirGraphMtEphIter<'a, V>) -> DirGraphMtEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> View for DirGraphMtEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> std::iter::IntoIterator for &'a DirGraphMtEph<V> {
        type Item = &'a V;
        type IntoIter = DirGraphMtEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>(), spec_graphview_wf(self@)
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            DirGraphMtEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 4b. type definitions


    pub struct DirGraphMtEphInv;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct LockedDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub(crate) locked_graph: RwLock<DirGraphMtEph<V>, DirGraphMtEphInv>,
        pub(crate) ghost_locked_graph: Ghost<GraphView<<V as View>::V>>,
    }

    //		Section 5c. view impls


    impl<V: StTInMtT + Hash + 'static> View for LockedDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V { self.spec_ghost_locked_graph() }
    }

    //		Section 8c. traits


    pub trait LockedDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {
        spec fn spec_dirgraphmteph_wf(&self) -> bool;

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { Set::new(|w: V::V| self@.A.contains((v, w))) }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { Set::new(|u: V::V| self@.A.contains((u, v))) }

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { self.spec_n_plus(v).union(self.spec_n_minus(v)) }

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        { Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w)) }

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        { Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w)) }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        { Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (s: Self)
            requires
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                A@.finite(),
                forall |u: V::V, w: V::V|
                    #[trigger] A@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures
                s.spec_dirgraphmteph_wf(),
                s@.V == V@,
                s@.A == A@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones vertex set under lock
        fn vertices(&self) -> (v: SetStEph<V>)
            requires self.spec_dirgraphmteph_wf()
            ensures v@ == self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- clones arc set under lock
        fn arcs(&self) -> (a: SetStEph<Edge<V>>)
            requires self.spec_dirgraphmteph_wf()
            ensures a@ == self@.A;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeV(&self) -> (n: usize)
            requires self.spec_dirgraphmteph_wf()
            ensures n == self@.V.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeA(&self) -> (n: usize)
            requires self.spec_dirgraphmteph_wf()
            ensures n == self@.A.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
            requires
                self.spec_dirgraphmteph_wf(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == self@.A.contains((u@, v@));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == Set::new(|w: V::V| self@.A.contains((v@, w))),
                out_neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == Set::new(|u: V::V| self@.A.contains((u, v@))),
                in_neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                u_set@ <= self@.V,
            ensures
                out_neighbors.spec_setsteph_wf(),
                out_neighbors@ == self.spec_n_plus_of_vertices(u_set@),
                out_neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                u_set@ <= self@.V,
            ensures
                in_neighbors.spec_setsteph_wf(),
                in_neighbors@ == self.spec_n_minus_of_vertices(u_set@),
                in_neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                self.spec_dirgraphmteph_wf(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;
    }

    //		Section 9c. impls


    impl<V: StTInMtT + Hash + 'static> LockedDirGraphMtEph<V> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_graphview_wf(self.ghost_locked_graph@)
        }

        pub closed spec fn spec_ghost_locked_graph(self) -> GraphView<<V as View>::V> {
            self.ghost_locked_graph@
        }
    }

    impl<V: StTInMtT + Hash + 'static> LockedDirGraphMtEphTrait<V> for LockedDirGraphMtEph<V> {
        open spec fn spec_dirgraphmteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (s: Self) {
            let g = DirGraphMtEph::from_sets(V, A);
            let ghost gv = g@;
            LockedDirGraphMtEph {
                locked_graph: RwLock::new(g, Ghost(DirGraphMtEphInv)),
                ghost_locked_graph: Ghost(gv),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones vertex set under lock
        fn vertices(&self) -> (v: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let v = inner.V.clone();
            read_handle.release_read();
            v
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(|A|) -- clones arc set under lock
        fn arcs(&self) -> (a: SetStEph<Edge<V>>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let a = inner.A.clone();
            read_handle.release_read();
            a
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeV(&self) -> (n: usize) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let n = inner.sizeV();
            read_handle.release_read();
            n
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeA(&self) -> (n: usize) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let n = inner.sizeA();
            read_handle.release_read();
            n
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn neighbor(&self, u: &V, v: &V) -> (b: bool) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let b = inner.neighbor(u, v);
            read_handle.release_read();
            b
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let out_neighbors = inner.n_plus(v);
            read_handle.release_read();
            out_neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let in_neighbors = inner.n_minus(v);
            read_handle.release_read();
            in_neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|A|), Span O(log |A|) -- RwLock wrapper
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let neighbors = inner.ng(v);
            read_handle.release_read();
            neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> (out_neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let out_neighbors = inner.n_plus_of_vertices(u_set);
            read_handle.release_read();
            out_neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> (in_neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let in_neighbors = inner.n_minus_of_vertices(u_set);
            read_handle.release_read();
            in_neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |A|), Span O(log |S| * log |A|) -- RwLock wrapper
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let neighbors = inner.ng_of_vertices(u_set);
            read_handle.release_read();
            neighbors
        }
    }

    //		Section 11b. top level coarse locking


    impl<V: StTInMtT + Hash + 'static> RwLockPredicate<DirGraphMtEph<V>> for DirGraphMtEphInv {
        open spec fn inv(self, v: DirGraphMtEph<V>) -> bool {
            spec_graphview_wf(v@) && valid_key_type_for_graph::<V>()
        }
    }

    //		Section 12a. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<V: StTInMtT + Hash + 'static> PartialEqSpecImpl for DirGraphMtEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<V: StTInMtT + Hash + 'static> Clone for DirGraphMtEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            DirGraphMtEph { V: self.V.clone(), A: self.A.clone() }
        }
    }

    impl<V: StTInMtT + Hash + 'static> Eq for DirGraphMtEph<V> {}

    impl<V: StTInMtT + Hash + 'static> PartialEq for DirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let v_eq = self.V == other.V;
            let a_eq = self.A == other.A;
            proof {
                if v_eq && a_eq {
                }
            }
            v_eq && a_eq
        }
    }

    } // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! DirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::from_sets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::from_sets(__V, __A)
        }}
    }

    //		Section 14a. derive impls outside verus!

    impl<V: StTInMtT + Hash + 'static> Debug for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphMtEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StTInMtT + Hash + 'static> Display for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Debug for DirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Display for DirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Debug for DirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphGhostIterator") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Display for DirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphGhostIterator") }
    }

    //		Section 14b. derive impls outside verus!

    impl Debug for DirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphInv") }
    }

    impl Display for DirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "DirGraphMtEphInv") }
    }

    //		Section 14c. derive impls outside verus!

    impl<V: StTInMtT + Hash + 'static> Debug for LockedDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedDirGraphMtEph") }
    }

    impl<V: StTInMtT + Hash + 'static> Display for LockedDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedDirGraphMtEph") }
    }
}
