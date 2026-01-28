//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Edge filtering (ng) and vertex map-reduce (ng_of_vertices) are parallel.

pub mod UnDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};

    verus! {

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::valid_key_type;

    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::wf_graph_view;

    broadcast use {
        vstd::set::group_set_axioms,
        crate::Types::Types::group_Edge_axioms,
    };

    pub open spec fn valid_key_type_for_graph<V: StTInMtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }

    #[verifier::reject_recursive_types(V)]
    #[derive(Clone)]
    pub struct UnDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        V: SetStEph<V>,
        E: SetStEph<Edge<V>>,
    }

    impl<V: StTInMtT + Hash + 'static> View for UnDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.E@ }
        }
    }

    impl<V: StTInMtT + Hash + 'static> UnDirGraphMtEph<V> {
        pub open spec fn spec_vertices(&self) -> Set<V::V> { self.V@ }
        pub open spec fn spec_edges(&self) -> Set<(V::V, V::V)> { self.E@ }

        /// Spec for ng computed from a subset of edges
        open spec fn spec_ng_from_set(&self, v: V::V, subedges: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                wf_graph_view(self@),
                subedges <= self@.A,
        {
            Set::new(|w: V::V| subedges.contains((v, w)) || subedges.contains((w, v)))
        }

        /// Spec for ng_of_vertices computed from a subset of vertices
        open spec fn spec_ng_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_ng(u).contains(w))
        }
    }

    pub trait UnDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures 
                wf_graph_view(g@),
                g@.V == Set::<<V as View>::V>::empty(), 
                g@.A == Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: Self)
            requires 
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                E@.finite(),
                forall |u: V::V, w: V::V| 
                    #[trigger] E@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures 
                wf_graph_view(g@),
                g@.V == V@, 
                g@.A == E@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn edges(&self) -> (e: &SetStEph<Edge<V>>)
            ensures e@ == self@.A;

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            ensures n == self@.V.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeE(&self) -> (n: N)
            ensures n == self@.A.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| self@.A.contains((v, w)) || self@.A.contains((w, v)))
        }

        /// APAS: Work Θ(|E|), Span Θ(log |E|) - parallel
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures 
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        /// APAS: Work Θ(|u_set| × |E|), Span Θ(log |u_set| + log |E|) - parallel
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures 
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// APAS: Work Θ(|E|), Span Θ(log |E|) - parallel
        fn degree(&self, v: &V) -> (n: N)
            requires 
                wf_graph_view(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_ng(v@).len();
    }

    /// Parallel edge filtering for neighbors using set split.
    fn ng_parallel<V: StTInMtT + Hash + 'static>(g: &UnDirGraphMtEph<V>, v: V, edges: SetStEph<Edge<V>>) 
                                                  -> (neighbors: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            wf_graph_view(g@),
            edges@ <= g@.A,
        ensures 
            neighbors@ == g.spec_ng_from_set(v@, edges@),
            neighbors@ <= g.spec_ng(v@)
        decreases edges.size()
    {
        let n = edges.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let Edge(a, b) = edges.choose();
            if feq(&a, &v) {
                SetStEph::singleton(b.clone_plus())
            } else if feq(&b, &v) {
                SetStEph::singleton(a.clone_plus())
            } else {
                SetStEph::empty()
            }
        }
        else {
            let mid = n / 2;
            let (left_edges, right_edges) = edges.split(mid);
            let v_left  = v.clone_plus();
            let v_right = v.clone_plus();
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_ng_from_set(v_left@, left_edges@)
            { ng_parallel(&g_left, v_left, left_edges) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_ng_from_set(v_right@, right_edges@)
            { ng_parallel(&g_right, v_right, right_edges) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            left_neighbors.union(&right_neighbors)
        }
    }

    /// Parallel neighbors over a set of vertices using set split.
    fn ng_of_vertices_parallel<V: StTInMtT + Hash + 'static>(
        g: &UnDirGraphMtEph<V>,
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
        decreases verts.size()
    {
        let n = verts.size();
        if n == 0 {
            SetStEph::empty()
        }
        else if n == 1 {
            let u = verts.choose();
            let result = g.ng(&u);
            proof {
                assert(verts@ =~= Set::empty().insert(u@));
                assert forall |w: V::V| #![auto] g.spec_ng_of_vertices_from_set(verts@).contains(w)
                    <==> g.spec_ng(u@).contains(w) by {
                    if g.spec_ng_of_vertices_from_set(verts@).contains(w) {
                        let v_wit: V::V = choose |v: V::V| #![auto] verts@.contains(v) && g.spec_ng(v).contains(w);
                        assert(v_wit == u@);
                    }
                }
            }
            result
        }
        else {
            let mid = n / 2;
            let (left_verts, right_verts) = verts.split(mid);
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_ng_of_vertices_from_set(left_verts@)
            { ng_of_vertices_parallel(&g_left, left_verts) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_ng_of_vertices_from_set(right_verts@)
            { ng_of_vertices_parallel(&g_right, right_verts) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            let result = left_neighbors.union(&right_neighbors);
            proof {
                assert(verts@ =~= left_verts@.union(right_verts@));
                assert forall |w: V::V| #![auto] g.spec_ng_of_vertices_from_set(verts@).contains(w)
                    <==> result@.contains(w) by {
                    if g.spec_ng_of_vertices_from_set(verts@).contains(w) {
                        let v_wit: V::V = choose |v: V::V| #![auto] verts@.contains(v) && g.spec_ng(v).contains(w);
                        assert(left_verts@.contains(v_wit) || right_verts@.contains(v_wit));
                        if left_verts@.contains(v_wit) {
                            assert(g.spec_ng_of_vertices_from_set(left_verts@).contains(w));
                        } else {
                            assert(g.spec_ng_of_vertices_from_set(right_verts@).contains(w));
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

    impl<V: StTInMtT + Hash + 'static> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {
        fn empty() -> (g: UnDirGraphMtEph<V>) {
            UnDirGraphMtEph {
                V: SetLit![],
                E: SetLit![],
            }
        }

        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: UnDirGraphMtEph<V>) { 
            UnDirGraphMtEph { V, E } 
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        fn edges(&self) -> (e: &SetStEph<Edge<V>>) { &self.E }

        fn sizeV(&self) -> (n: N) { self.V.size() }

        fn sizeE(&self) -> (n: N) { self.E.size() }

        fn neighbor(&self, u: &V, v: &V) -> (b: B) {
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) || self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let edges = self.E.clone();
            ng_parallel(self, v.clone_plus(), edges)
        }

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            ng_of_vertices_parallel(self, u_set.clone())
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) { 
            &e.0 == v || &e.1 == v 
        }

        fn degree(&self, v: &V) -> (n: N) { 
            self.ng(v).size() 
        }
    }

    } // verus!

    impl<V: StTInMtT + Hash + 'static> Debug for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("UnDirGraphMtEph")
                .field("V", &self.V)
                .field("E", &self.E)
                .finish()
        }
    }

    impl<V: StTInMtT + Hash + 'static> Display for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.E) }
    }

    impl<V: StTInMtT + Hash + 'static> PartialEq for UnDirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }
    }
    impl<V: StTInMtT + Hash + 'static> Eq for UnDirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::from_sets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::from_sets(__V, __E)
        }};
    }
}
