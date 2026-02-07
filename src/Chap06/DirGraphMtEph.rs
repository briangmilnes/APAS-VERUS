//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (n_plus, n_minus) and vertex map-reduce (ng_of_vertices, etc.) are parallel.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod DirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Concurrency::Concurrency::*;
    use crate::{ParaPair, ParaPairDisjoint, SetLit};

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;


    //		3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub V: SetStEph<V>,
        pub A: SetStEph<Edge<V>>,
    }


    //		5. view impls

    impl<V: StTInMtT + Hash + 'static> View for DirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.A@ }
        }
    }


    //		6. spec fns

    pub open spec fn valid_key_type_for_graph<V: StTInMtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }


    //		8. traits

    pub trait DirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures 
                wf_graph_view(g@),
                g@.V == Set::<<V as View>::V>::empty(), 
                g@.A == Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: Self)
            requires 
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                A@.finite(),
                forall |u: V::V, w: V::V| 
                    #[trigger] A@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures 
                wf_graph_view(g@),
                g@.V == V@, 
                g@.A == A@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>)
            ensures a@ == self@.A;

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            ensures n == self@.V.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeA(&self) -> (n: N)
            ensures n == self@.A.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == self@.A.contains((u@, v@));

        /// APAS: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V> 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| self@.A.contains((v, w)))
        }

        open spec fn spec_n_plus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                wf_graph_view(self@),
                subarcs <= self@.A,
        {
            Set::new(|w: V::V| subarcs.contains((v, w)))
        }

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures 
                out_neighbors@ == self.spec_n_plus(v@),
                out_neighbors@ <= self@.V;

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn out_degree(&self, v: &V) -> (n: N)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_n_plus(v@).len();

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V> 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|u: V::V| self@.A.contains((u, v))) 
        }

        open spec fn spec_n_minus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                wf_graph_view(self@),
                subarcs <= self@.A,
        {
            Set::new(|u: V::V| subarcs.contains((u, v)))
        }

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures 
                in_neighbors@ == self.spec_n_minus(v@),
                in_neighbors@ <= self@.V;

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn in_degree(&self, v: &V) -> (n: N)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_n_minus(v@).len();

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            self.spec_n_plus(v).union(self.spec_n_minus(v)) 
        }

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures 
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        open spec fn spec_degree(&self, v: V::V) -> nat 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            self.spec_ng(v).len() 
        }

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn degree(&self, v: &V) -> (n: N)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_degree(v@);

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))
        }

        open spec fn spec_n_plus_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_n_plus(u).contains(w))
        }

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures 
                out_neighbors@ == self.spec_n_plus_of_vertices(u_set@),
                out_neighbors@ <= self@.V;

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w))
        }

        open spec fn spec_n_minus_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_n_minus(u).contains(w))
        }

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures 
                in_neighbors@ == self.spec_n_minus_of_vertices(u_set@),
                in_neighbors@ <= self@.V;

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        open spec fn spec_ng_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_ng(u).contains(w))
        }

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures 
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;
    }


    //		9. impls

    impl<V: StTInMtT + Hash + 'static> DirGraphMtEph<V> {
        /// Convenience accessor for vertices view
        pub open spec fn spec_vertices(&self) -> Set<V::V> { self.V@ }
        /// Convenience accessor for arcs view
        pub open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> { self.A@ }
    }

    /// Parallel arc filtering for out-neighbors using set split.
    fn n_plus_par<V: StTInMtT + Hash + 'static>(g: &DirGraphMtEph<V>, v: V, arcs: SetStEph<Edge<V>>) 
                                                     -> (out_neighbors: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            arcs@ <= g@.A,
        ensures 
            out_neighbors@ == g.spec_n_plus_from_set(v@, arcs@),
            out_neighbors@ <= g.spec_n_plus(v@)
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
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_plus_from_set(v_left@, left_arcs@)
            { n_plus_par(&g_left, v_left, left_arcs) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_plus_from_set(v_right@, right_arcs@)
            { n_plus_par(&g_right, v_right, right_arcs) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            left_neighbors.union(&right_neighbors)
        }
    }

    /// Parallel arc filtering for in-neighbors using set split.
    fn n_minus_par<V: StTInMtT + Hash + 'static>(g: &DirGraphMtEph<V>, v: V, arcs: SetStEph<Edge<V>>) 
                                                      -> (in_neighbors: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            arcs@ <= g@.A,
        ensures 
            in_neighbors@ == g.spec_n_minus_from_set(v@, arcs@),
            in_neighbors@ <= g.spec_n_minus(v@)
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
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_minus_from_set(v_left@, left_arcs@)
            { n_minus_par(&g_left, v_left, left_arcs) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_minus_from_set(v_right@, right_arcs@)
            { n_minus_par(&g_right, v_right, right_arcs) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            left_neighbors.union(&right_neighbors)
        }
    }

    /// Parallel out-neighbors over a set of vertices using set split.
    fn n_plus_of_vertices_par<V: StTInMtT + Hash + 'static>(
        g: &DirGraphMtEph<V>,
        verts: SetStEph<V>,
    ) -> (out_neighbors: SetStEph<V>)
        requires 
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            verts@ <= g@.V,
        ensures 
            out_neighbors@ == g.spec_n_plus_of_vertices_from_set(verts@),
            out_neighbors@ <= g@.V
        decreases verts@.len()
    {
        let n = verts.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let u = verts.choose();
            let result = n_plus_par(g, u, g.A.clone());
            result
        }
        else {
            let mid = n / 2;
            let (left_verts, right_verts) = verts.split(mid);
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_plus_of_vertices_from_set(left_verts@)
            { n_plus_of_vertices_par(&g_left, left_verts) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_plus_of_vertices_from_set(right_verts@)
            { n_plus_of_vertices_par(&g_right, right_verts) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            let result = left_neighbors.union(&right_neighbors);
            proof {
                // split guarantees: left_verts@ ∪ right_verts@ == verts@
                // Prove set equality for the union
                assert forall |w: V::V| #![auto] g.spec_n_plus_of_vertices_from_set(verts@).contains(w)
                    <==> result@.contains(w) by {
                    if g.spec_n_plus_of_vertices_from_set(verts@).contains(w) {
                        let v_wit: V::V = choose |v: V::V| #![auto] verts@.contains(v) && g.spec_n_plus(v).contains(w);
                        if left_verts@.contains(v_wit) {
                        } else {
                        }
                    }
                    if result@.contains(w) {
                        if left_neighbors@.contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![auto] left_verts@.contains(v) && g.spec_n_plus(v).contains(w);
                            assert(verts@.contains(v_wit));
                        } else {
                            let v_wit: V::V = choose |v: V::V| #![auto] right_verts@.contains(v) && g.spec_n_plus(v).contains(w);
                            assert(verts@.contains(v_wit));
                        }
                    }
                }
            }
            result
        }
    }

    /// Parallel in-neighbors over a set of vertices using set split.
    fn n_minus_of_vertices_par<V: StTInMtT + Hash + 'static>(
        g: &DirGraphMtEph<V>,
        verts: SetStEph<V>,
    ) -> (in_neighbors: SetStEph<V>)
        requires 
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            verts@ <= g@.V,
        ensures 
            in_neighbors@ == g.spec_n_minus_of_vertices_from_set(verts@),
            in_neighbors@ <= g@.V
        decreases verts@.len()
    {
        let n = verts.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let u = verts.choose();
            let result = n_minus_par(g, u, g.A.clone());
            result
        }
        else {
            let mid = n / 2;
            let (left_verts, right_verts) = verts.split(mid);
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_minus_of_vertices_from_set(left_verts@)
            { n_minus_of_vertices_par(&g_left, left_verts) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_minus_of_vertices_from_set(right_verts@)
            { n_minus_of_vertices_par(&g_right, right_verts) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            let result = left_neighbors.union(&right_neighbors);
            proof {
                // split guarantees: left_verts@ ∪ right_verts@ == verts@
                // Prove set equality for the union
                assert forall |w: V::V| #![auto] g.spec_n_minus_of_vertices_from_set(verts@).contains(w)
                    <==> result@.contains(w) by {
                    if g.spec_n_minus_of_vertices_from_set(verts@).contains(w) {
                        let v_wit: V::V = choose |v: V::V| #![auto] verts@.contains(v) && g.spec_n_minus(v).contains(w);
                        if left_verts@.contains(v_wit) {
                        } else {
                        }
                    }
                    if result@.contains(w) {
                        if left_neighbors@.contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![auto] left_verts@.contains(v) && g.spec_n_minus(v).contains(w);
                            assert(verts@.contains(v_wit));
                        } else {
                            let v_wit: V::V = choose |v: V::V| #![auto] right_verts@.contains(v) && g.spec_n_minus(v).contains(w);
                            assert(verts@.contains(v_wit));
                        }
                    }
                }
            }
            result
        }
    }

    /// Parallel all-neighbors over a set of vertices using set split.
    fn ng_of_vertices_par<V: StTInMtT + Hash + 'static>(
        g: &DirGraphMtEph<V>,
        verts: SetStEph<V>,
    ) -> (neighbors: SetStEph<V>)
        requires 
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            verts@ <= g@.V,
        ensures 
            neighbors@ == g.spec_ng_of_vertices_from_set(verts@),
            neighbors@ <= g@.V
        decreases verts@.len()
    {
        let n = verts.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let u = verts.choose();
            let result = g.ng(&u);
            result
        }
        else {
            let mid = n / 2;
            let (left_verts, right_verts) = verts.split(mid);
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_ng_of_vertices_from_set(left_verts@)
            { ng_of_vertices_par(&g_left, left_verts) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_ng_of_vertices_from_set(right_verts@)
            { ng_of_vertices_par(&g_right, right_verts) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            let result = left_neighbors.union(&right_neighbors);
            proof {
                // split guarantees: left_verts@ ∪ right_verts@ == verts@
                // Prove set equality for the union
                assert forall |w: V::V| #![auto] g.spec_ng_of_vertices_from_set(verts@).contains(w)
                    <==> result@.contains(w) by {
                    if g.spec_ng_of_vertices_from_set(verts@).contains(w) {
                        let v_wit: V::V = choose |v: V::V| #![auto] verts@.contains(v) && g.spec_ng(v).contains(w);
                        if left_verts@.contains(v_wit) {
                        } else {
                        }
                    }
                    if result@.contains(w) {
                        if left_neighbors@.contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![auto] left_verts@.contains(v) && g.spec_ng(v).contains(w);
                            assert(verts@.contains(v_wit));
                        } else {
                            let v_wit: V::V = choose |v: V::V| #![auto] right_verts@.contains(v) && g.spec_ng(v).contains(w);
                            assert(verts@.contains(v_wit));
                        }
                    }
                }
            }
            result
        }
    }

    impl<V: StTInMtT + Hash + 'static> DirGraphMtEph<V> {
        /// Returns an iterator over the vertices
        pub fn iter_vertices(&self) -> (it: SetStEphIter<'_, V>)
            requires valid_key_type_for_graph::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
        { self.V.iter() }

        /// Returns an iterator over the arcs
        pub fn iter_arcs(&self) -> (it: SetStEphIter<'_, Edge<V>>)
            requires valid_key_type_for_graph::<V>()
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: Edge<V>| k@).to_set() == self@.A,
                it@.1.no_duplicates(),
        { self.A.iter() }
    }

    impl<V: StTInMtT + Hash + 'static> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {
        fn empty() -> (g: DirGraphMtEph<V>) {
            DirGraphMtEph { V: SetStEph::empty(), A: SetStEph::empty() }
        }

        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: DirGraphMtEph<V>) {
            DirGraphMtEph { V, A }
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }
        fn arcs(&self) -> (a: &SetStEph<Edge<V>>) { &self.A }
        fn sizeV(&self) -> (n: N) { self.V.size() }
        fn sizeA(&self) -> (n: N) { self.A.size() }

        fn neighbor(&self, u: &V, v: &V) -> (b: B) {
            self.A.mem(&Edge(u.clone_plus(), v.clone_plus()))
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) { feq(&e.0, v) || feq(&e.1, v) }

        fn n_plus(&self, v: &V) -> SetStEph<V> { 
            let arcs = self.A.clone();
            n_plus_par(self, v.clone_plus(), arcs)
        }

        fn out_degree(&self, v: &V) -> (n: N) { self.n_plus(v).size() }

        fn n_minus(&self, v: &V) -> SetStEph<V> { 
            let arcs = self.A.clone();
            n_minus_par(self, v.clone_plus(), arcs)
        }
        fn in_degree(&self, v: &V) -> (n: N) { self.n_minus(v).size() }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) { self.n_plus(v).union(&self.n_minus(v)) }
        fn degree(&self, v: &V) -> (n: N) { self.ng(v).size() }

        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { n_plus_of_vertices_par(self, u_set.clone()) }
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { n_minus_of_vertices_par(self, u_set.clone()) }
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { ng_of_vertices_par(self, u_set.clone()) }
    }

    impl<V: StTInMtT + Hash + 'static> PartialEqSpecImpl for DirGraphMtEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		11. derive impls in verus!

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
                    assert(self@ =~= other@);
                }
            }
            v_eq && a_eq
        }
    }

    } // verus!


    //		13. derive impls outside verus!

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


    //		12. macros

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
}
