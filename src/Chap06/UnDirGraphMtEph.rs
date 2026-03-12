//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Edge filtering (ng) and vertex map-reduce (ng_of_vertices) are parallel.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	11. top level coarse locking
//	12. derive impls in verus!
//	13. macros
//	14. derive impls outside verus!

//		1. module


pub mod UnDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    use vstd::rwlock::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;


    //		3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        // Veracity: added broadcast groups
        vstd::set_lib::group_set_lib_default,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct UnDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub V: SetStEph<V>,
        pub E: SetStEph<Edge<V>>,
    }


    //		5. view impls

    impl<V: StTInMtT + Hash + 'static> View for UnDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.E@ }
        }
    }


    //		6. spec fns

    pub open spec fn valid_key_type_for_graph<V: StTInMtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }


    //		8. traits

    pub trait UnDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {

        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_edges(&self) -> Set<(V::V, V::V)> { self@.A }

        open spec fn spec_ng_from_set(&self, v: V::V, subedges: Set<(V::V, V::V)>) -> Set<V::V> 
            recommends 
                spec_graphview_wf(self@),
                subedges <= self@.A,
        {
            Set::new(|w: V::V| subedges.contains((v, w)) || subedges.contains((w, v)))
        }

        open spec fn spec_ng_of_vertices_from_set(&self, subverts: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), subverts <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger subverts.contains(u)] subverts.contains(u) && self.spec_ng(u).contains(w))
        }
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures 
                spec_graphview_wf(g@),
                g@.V == Set::<<V as View>::V>::empty(), 
                g@.A == Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|V| + |E|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: Self)
            requires 
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                E@.finite(),
                forall |u: V::V, w: V::V| 
                    #[trigger] E@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures 
                spec_graphview_wf(g@),
                g@.V == V@, 
                g@.A == E@;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn edges(&self) -> (e: &SetStEph<Edge<V>>)
            ensures e@ == self@.A;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.V.len();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn sizeE(&self) -> (n: N)
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.A.len();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| self@.A.contains((v, w)) || self@.A.contains((w, v)))
        }

        /// - APAS: Work Θ(|E|), Span Θ(log |E|) — parallel
        /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(log |E|) — ParaPair! split edges
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        /// - APAS: Work Θ(|u_set| × |E|), Span Θ(log |u_set| + log |E|) — parallel
        /// - Claude-Opus-4.6: Work Θ(|u_set| × |E|), Span Θ(log |u_set| + log |E|) — ParaPair! split vertices
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// - APAS: Work Θ(|E|), Span Θ(log |E|) — parallel
        /// - Claude-Opus-4.6: Work Θ(|E|), Span Θ(log |E|) — calls ng
        fn degree(&self, v: &V) -> (n: N)
            requires
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_ng(v@).len();

        /// Parallel edge filtering for neighbors using set split.
        fn ng_par(&self, v: V, edges: SetStEph<Edge<V>>) -> (neighbors: SetStEph<V>)
            requires
                valid_key_type::<V>(),
                valid_key_type::<Edge<V>>(),
                spec_graphview_wf(self@),
                edges@ <= self@.A,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_from_set(v@, edges@),
                neighbors@ <= self.spec_ng(v@)
            decreases edges@.len();

        /// Parallel neighbors over a set of vertices using set split.
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


    //		9. impls

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
            self.ng_par(v.clone_plus(), edges)
        }

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            self.ng_of_vertices_par(u_set.clone())
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) {
            feq(&e.0, v) || feq(&e.1, v)
        }

        fn degree(&self, v: &V) -> (n: N) {
            self.ng(v).size()
        }

        fn ng_par(&self, v: V, edges: SetStEph<Edge<V>>) -> (neighbors: SetStEph<V>)
            decreases edges@.len()
        {
            let n = edges.size();
            if n == 0 {
                proof {
                    assert forall |w: V::V| !(edges@.contains((v@, w)) || edges@.contains((w, v@))) by {}
                    assert(self.spec_ng_from_set(v@, edges@) =~= Set::empty());
                }
                SetStEph::empty()
            }
            else if n == 1 {
                let Edge(a, b) = edges.choose();
                proof {
                    assert(edges@.len() == 1);
                    assert(edges@.contains((a@, b@)));
                    assert forall |e: (V::V, V::V)| edges@.contains(e) implies e == (a@, b@) by {
                        if edges@.contains(e) && e != (a@, b@) {
                            let s_minus = edges@.remove((a@, b@));
                            assert(s_minus.contains(e));
                            assert(edges@.len() == s_minus.len() + 1);
                        }
                    }
                    assert(edges@ =~= Set::empty().insert((a@, b@)));
                }
                if feq(&a, &v) {
                    proof {
                        assert forall |w: V::V| self.spec_ng_from_set(v@, edges@).contains(w) <==> w == b@ by {}
                        assert(self.spec_ng_from_set(v@, edges@) =~= Set::empty().insert(b@));
                    }
                    SetStEph::singleton(b.clone_plus())
                } else if feq(&b, &v) {
                    proof {
                        assert forall |w: V::V| self.spec_ng_from_set(v@, edges@).contains(w) <==> w == a@ by {}
                        assert(self.spec_ng_from_set(v@, edges@) =~= Set::empty().insert(a@));
                    }
                    SetStEph::singleton(a.clone_plus())
                } else {
                    proof {
                        assert forall |w: V::V| !self.spec_ng_from_set(v@, edges@).contains(w) by {}
                        assert(self.spec_ng_from_set(v@, edges@) =~= Set::empty());
                    }
                    SetStEph::empty()
                }
            }
            else {
                let mid = n / 2;
                let (left_edges, right_edges) = edges.split(mid);
                let v_left  = v.clone_plus();
                let v_right = v.clone_plus();
                let g_left  = self.clone_plus();
                let g_right = self.clone_plus();

                let f1 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_left.spec_ng_from_set(v_left@, left_edges@)
                { g_left.ng_par(v_left, left_edges) };

                let f2 = move || -> (out: SetStEph<V>)
                    ensures out.spec_setsteph_wf(), out@ == g_right.spec_ng_from_set(v_right@, right_edges@)
                { g_right.ng_par(v_right, right_edges) };

                let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);

                left_neighbors.union(&right_neighbors)
            }
        }

        fn ng_of_vertices_par(&self, verts: SetStEph<V>) -> (neighbors: SetStEph<V>)
            decreases verts@.len()
        {
            let n = verts.size();
            if n == 0 {
                SetStEph::empty()
            }
            else if n == 1 {
                let u = verts.choose();
                let neighbors = self.ng(&u);
                proof {
                    assert(verts@.len() == 1);
                    assert(verts@.contains(u@));
                    assert forall |v_any: V::V| verts@.contains(v_any) implies v_any == u@ by {
                        if verts@.contains(v_any) && v_any != u@ {
                            let s_minus_u = verts@.remove(u@);
                            assert(s_minus_u.contains(v_any));
                            assert(verts@.len() == s_minus_u.len() + 1);
                        }
                    }
                    assert(verts@ =~= Set::empty().insert(u@));
                    assert forall |w: V::V| self.spec_ng_of_vertices_from_set(verts@).contains(w)
                        <==> self.spec_ng(u@).contains(w) by {
                        if self.spec_ng_of_vertices_from_set(verts@).contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![trigger verts@.contains(v)] verts@.contains(v) && self.spec_ng(v).contains(w);
                            assert(v_wit == u@);
                        }
                    }
                }
                neighbors
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
                    assert(verts@ =~= left_verts@.union(right_verts@));
                    assert forall |w: V::V| #![trigger neighbors@.contains(w)] self.spec_ng_of_vertices_from_set(verts@).contains(w)
                        <==> neighbors@.contains(w) by {
                        if self.spec_ng_of_vertices_from_set(verts@).contains(w) {
                            let v_wit: V::V = choose |v: V::V| #![trigger verts@.contains(v)] verts@.contains(v) && self.spec_ng(v).contains(w);
                            assert(left_verts@.contains(v_wit) || right_verts@.contains(v_wit));
                            if left_verts@.contains(v_wit) {
                                assert(self.spec_ng_of_vertices_from_set(left_verts@).contains(w));
                            } else {
                                assert(self.spec_ng_of_vertices_from_set(right_verts@).contains(w));
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

    #[cfg(verus_keep_ghost)]
    impl<V: StTInMtT + Hash + 'static> PartialEqSpecImpl for UnDirGraphMtEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<'a, V: StTInMtT + Hash + 'static> std::iter::IntoIterator for &'a UnDirGraphMtEph<V> {
        type Item = &'a V;
        type IntoIter = SetStEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>(), spec_graphview_wf(self@)
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
        {
            self.vertices().iter()
        }
    }

    //		11. top level coarse locking

    pub struct UnDirGraphMtEphInv;

    impl<V: StTInMtT + Hash + 'static> RwLockPredicate<UnDirGraphMtEph<V>> for UnDirGraphMtEphInv {
        open spec fn inv(self, v: UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(v@) && valid_key_type_for_graph::<V>()
        }
    }

    #[verifier::reject_recursive_types(V)]
    pub struct LockedUnDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub(crate) locked_graph: RwLock<UnDirGraphMtEph<V>, UnDirGraphMtEphInv>,
        pub(crate) ghost_locked_graph: Ghost<GraphView<<V as View>::V>>,
    }

    impl<V: StTInMtT + Hash + 'static> LockedUnDirGraphMtEph<V> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_graphview_wf(self.ghost_locked_graph@)
        }

        pub closed spec fn spec_ghost_locked_graph(self) -> GraphView<<V as View>::V> {
            self.ghost_locked_graph@
        }
    }

    impl<V: StTInMtT + Hash + 'static> View for LockedUnDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V { self.spec_ghost_locked_graph() }
    }

    pub trait LockedUnDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {
        spec fn spec_lockedundirgraphmteph_wf(&self) -> bool;

        fn new(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (s: Self)
            requires
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                E@.finite(),
                forall |u: V::V, w: V::V|
                    #[trigger] E@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures
                s.spec_lockedundirgraphmteph_wf(),
                s@.V == V@,
                s@.A == E@;

        fn vertices(&self) -> (v: SetStEph<V>)
            requires self.spec_lockedundirgraphmteph_wf()
            ensures v@ == self@.V;

        fn edges(&self) -> (e: SetStEph<Edge<V>>)
            requires self.spec_lockedundirgraphmteph_wf()
            ensures e@ == self@.A;

        fn sizeV(&self) -> (n: N)
            requires self.spec_lockedundirgraphmteph_wf()
            ensures n == self@.V.len();

        fn sizeE(&self) -> (n: N)
            requires self.spec_lockedundirgraphmteph_wf()
            ensures n == self@.A.len();

        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires
                self.spec_lockedundirgraphmteph_wf(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires
                self.spec_lockedundirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ <= self@.V;

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                self.spec_lockedundirgraphmteph_wf(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ <= self@.V;
    }

    impl<V: StTInMtT + Hash + 'static> LockedUnDirGraphMtEphTrait<V> for LockedUnDirGraphMtEph<V> {
        open spec fn spec_lockedundirgraphmteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        fn new(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (s: Self) {
            let g = UnDirGraphMtEph::from_sets(V, E);
            let ghost gv = g@;
            LockedUnDirGraphMtEph {
                locked_graph: RwLock::new(g, Ghost(UnDirGraphMtEphInv)),
                ghost_locked_graph: Ghost(gv),
            }
        }

        fn vertices(&self) -> (v: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let v = read_handle.borrow().V.clone();
            proof { assume(v@ == self@.V); }
            read_handle.release_read();
            v
        }

        fn edges(&self) -> (e: SetStEph<Edge<V>>) {
            let read_handle = self.locked_graph.acquire_read();
            let e = read_handle.borrow().E.clone();
            proof { assume(e@ == self@.A); }
            read_handle.release_read();
            e
        }

        fn sizeV(&self) -> (n: N) {
            let read_handle = self.locked_graph.acquire_read();
            let n = read_handle.borrow().sizeV();
            proof { assume(n == self@.V.len()); }
            read_handle.release_read();
            n
        }

        fn sizeE(&self) -> (n: N) {
            let read_handle = self.locked_graph.acquire_read();
            let n = read_handle.borrow().sizeE();
            proof { assume(n == self@.A.len()); }
            read_handle.release_read();
            n
        }

        fn neighbor(&self, u: &V, v: &V) -> (b: B) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner@ == self@); }
            let b = inner.neighbor(u, v);
            proof { assume(b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)))); }
            read_handle.release_read();
            b
        }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner@ == self@); }
            let neighbors = inner.ng(v);
            proof { assume(neighbors@ <= self@.V); }
            read_handle.release_read();
            neighbors
        }

        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { assume(inner@ == self@); }
            let neighbors = inner.ng_of_vertices(u_set);
            proof { assume(neighbors@ <= self@.V); }
            read_handle.release_read();
            neighbors
        }
    }

    //		12. derive impls in verus!

    impl<V: StTInMtT + Hash + 'static> Clone for UnDirGraphMtEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            UnDirGraphMtEph { V: self.V.clone(), E: self.E.clone() }
        }
    }

    impl<V: StTInMtT + Hash + 'static> Eq for UnDirGraphMtEph<V> {}

    impl<V: StTInMtT + Hash + 'static> PartialEq for UnDirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let v_eq = self.V == other.V;
            let e_eq = self.E == other.E;
            proof {
                if v_eq && e_eq {
                    assert(self@ =~= other@);
                }
            }
            v_eq && e_eq
        }
    }

    } // verus!


    //		13. macros

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


    //		14. derive impls outside verus!

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
}
