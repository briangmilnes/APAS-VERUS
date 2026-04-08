//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Edge filtering (ng) and vertex map-reduce (ng_of_vertices) are parallel.

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


pub mod UnDirGraphMtEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};
    use crate::vstdplus::accept::accept;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    use vstd::rwlock::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
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
    pub struct UnDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub V: SetStEph<V>,
        pub E: SetStEph<Edge<V>>,
    }

    //		Section 5a. view impls


    impl<V: StTInMtT + Hash + 'static> View for UnDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.E@ }
        }
    }

    //		Section 6a. spec fns


    pub open spec fn valid_key_type_for_graph<V: StTInMtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }

    //		Section 8a. traits


    pub trait UnDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {

        spec fn spec_undirgraphmteph_wf(&self) -> bool;

        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_edges(&self) -> Set<(V::V, V::V)> { self@.A }

        open spec fn spec_degree(&self, v: V::V) -> nat
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        {
            self.spec_ng(v).len()
        }

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
        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures
                g.spec_undirgraphmteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: Self)
            requires
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                E@.finite(),
                forall |u: V::V, w: V::V|
                    #[trigger] E@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures
                g.spec_undirgraphmteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= V@,
                g@.A =~= E@;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> (e: &SetStEph<Edge<V>>)
            ensures e@ =~= self@.A;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeV(&self) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.V.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeE(&self) -> (n: usize)
            requires spec_graphview_wf(self@), valid_key_type_for_graph::<V>()
            ensures n == self@.A.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
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

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|E|), Span O(log |E|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) — ParaPair! split edges
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

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|u_set| × |E|), Span O(log |u_set| + log |E|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|u_set| × |E|), Span O(log |u_set| + log |E|) — ParaPair! split vertices
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires 
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|E|), Span O(log |E|) — parallel
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) — calls ng
        fn degree(&self, v: &V) -> (n: usize)
            requires
                spec_graphview_wf(self@),
                valid_key_type_for_graph::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_ng(v@).len();

        /// Parallel edge filtering for neighbors using set split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- parallel split on edges
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(log |S| * log |E|)
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


    impl<V: StTInMtT + Hash + 'static> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {

        open spec fn spec_undirgraphmteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: UnDirGraphMtEph<V>) {
            UnDirGraphMtEph {
                V: SetLit![],
                E: SetLit![],
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: UnDirGraphMtEph<V>) {
            UnDirGraphMtEph { V, E }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn edges(&self) -> (e: &SetStEph<Edge<V>>) { &self.E }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeV(&self) -> (n: usize) { self.V.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeE(&self) -> (n: usize) { self.E.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool) {
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) || self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- delegates to ng_par
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let edges = self.E.clone();
            self.ng_par(v.clone_plus(), edges)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(log |S| * log |E|)
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            self.ng_of_vertices_par(u_set.clone())
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool) {
            feq(&e.0, v) || feq(&e.1, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|)
        fn degree(&self, v: &V) -> (n: usize) {
            self.ng(v).size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- parallel split on edges
        fn ng_par(&self, v: V, edges: SetStEph<Edge<V>>) -> (neighbors: SetStEph<V>)
            decreases edges@.len()
        {
            let n = edges.size();
            if n == 0 {
                proof {
                }
                SetStEph::empty()
            }
            else if n == 1 {
                let Edge(a, b) = edges.choose();
                proof {
                }
                if feq(&a, &v) {
                    proof {
                    }
                    SetStEph::singleton(b.clone_plus())
                } else if feq(&b, &v) {
                    proof {
                    }
                    SetStEph::singleton(a.clone_plus())
                } else {
                    proof {
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(log |S| * log |E|)
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
                    // Veracity: NEEDED assert
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
                                // Veracity: NEEDED assert
                                assert(verts@.contains(v_wit));
                            } else {
                                let v_wit: V::V = choose |v: V::V| #![trigger right_verts@.contains(v)] right_verts@.contains(v) && self.spec_ng(v).contains(w);
                                // Veracity: NEEDED assert
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


    /// Iterator wrapper for UnDirGraphMtEph vertex iteration.
    #[verifier::reject_recursive_types(V)]
    pub struct UnDirGraphMtEphIter<'a, V: StTInMtT + Hash + 'static> {
        pub inner: SetStEphIter<'a, V>,
    }

    impl<'a, V: StTInMtT + Hash + 'static> View for UnDirGraphMtEphIter<'a, V> {
        type V = (int, Seq<V>);
        open spec fn view(&self) -> (int, Seq<V>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, V: StTInMtT + Hash + 'static>(it: &UnDirGraphMtEphIter<'a, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, V: StTInMtT + Hash + 'static> std::iter::Iterator for UnDirGraphMtEphIter<'a, V> {
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
    pub struct UnDirGraphMtEphGhostIterator<'a, V: StTInMtT + Hash + 'static> {
        pub pos: int,
        pub elements: Seq<V>,
        pub phantom: core::marker::PhantomData<&'a V>,
    }

    impl<'a, V: StTInMtT + Hash + 'static> vstd::pervasive::ForLoopGhostIteratorNew for UnDirGraphMtEphIter<'a, V> {
        type GhostIter = UnDirGraphMtEphGhostIterator<'a, V>;

        open spec fn ghost_iter(&self) -> UnDirGraphMtEphGhostIterator<'a, V> {
            UnDirGraphMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> vstd::pervasive::ForLoopGhostIterator for UnDirGraphMtEphGhostIterator<'a, V> {
        type ExecIter = UnDirGraphMtEphIter<'a, V>;
        type Item = V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &UnDirGraphMtEphIter<'a, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &UnDirGraphMtEphIter<'a, V>) -> UnDirGraphMtEphGhostIterator<'a, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> View for UnDirGraphMtEphGhostIterator<'a, V> {
        type V = Seq<V>;

        open spec fn view(&self) -> Seq<V> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, V: StTInMtT + Hash + 'static> std::iter::IntoIterator for &'a UnDirGraphMtEph<V> {
        type Item = &'a V;
        type IntoIter = UnDirGraphMtEphIter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires valid_key_type::<V>(), spec_graphview_wf(self@)
            ensures
                it@.0 == 0int,
                it@.1.map(|i: int, k: V| k@).to_set() == self@.V,
                it@.1.no_duplicates(),
                iter_invariant(&it),
        {
            UnDirGraphMtEphIter { inner: self.vertices().iter() }
        }
    }

    //		Section 4b. type definitions


    pub struct UnDirGraphMtEphInv;

    //		Section 4c. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct LockedUnDirGraphMtEph<V: StTInMtT + Hash + 'static> {
        pub(crate) locked_graph: RwLock<UnDirGraphMtEph<V>, UnDirGraphMtEphInv>,
        pub(crate) ghost_locked_graph: Ghost<GraphView<<V as View>::V>>,
    }

    //		Section 5c. view impls


    impl<V: StTInMtT + Hash + 'static> View for LockedUnDirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V { self.spec_ghost_locked_graph() }
    }

    //		Section 8c. traits


    pub trait LockedUnDirGraphMtEphTrait<V: StTInMtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {
        spec fn spec_undirgraphmteph_wf(&self) -> bool;

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        { Set::new(|w: V::V| self@.A.contains((v, w)) || self@.A.contains((w, v))) }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        { Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (s: Self)
            requires
                valid_key_type_for_graph::<V>(),
                V@.finite(),
                E@.finite(),
                forall |u: V::V, w: V::V|
                    #[trigger] E@.contains((u, w)) ==> V@.contains(u) && V@.contains(w),
            ensures
                s.spec_undirgraphmteph_wf(),
                s@.V == V@,
                s@.A == E@;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones vertex set under lock
        fn vertices(&self) -> (v: SetStEph<V>)
            requires self.spec_undirgraphmteph_wf()
            ensures v@ == self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- clones edge set under lock
        fn edges(&self) -> (e: SetStEph<Edge<V>>)
            requires self.spec_undirgraphmteph_wf()
            ensures e@ == self@.A;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeV(&self) -> (n: usize)
            requires self.spec_undirgraphmteph_wf()
            ensures n == self@.V.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn sizeE(&self) -> (n: usize)
            requires self.spec_undirgraphmteph_wf()
            ensures n == self@.A.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
            requires
                self.spec_undirgraphmteph_wf(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- RwLock wrapper
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires
                self.spec_undirgraphmteph_wf(),
                self@.V.contains(v@),
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(log |S| * log |E|) -- RwLock wrapper
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                self.spec_undirgraphmteph_wf(),
                u_set@ <= self@.V,
            ensures
                neighbors.spec_setsteph_wf(),
                neighbors@ == self.spec_ng_of_vertices(u_set@),
                neighbors@ <= self@.V;
    }

    //		Section 9c. impls


    impl<V: StTInMtT + Hash + 'static> LockedUnDirGraphMtEph<V> {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            spec_graphview_wf(self.ghost_locked_graph@)
        }

        pub closed spec fn spec_ghost_locked_graph(self) -> GraphView<<V as View>::V> {
            self.ghost_locked_graph@
        }
    }

    impl<V: StTInMtT + Hash + 'static> LockedUnDirGraphMtEphTrait<V> for LockedUnDirGraphMtEph<V> {
        open spec fn spec_undirgraphmteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) -- RwLock wrapper
        fn new(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (s: Self) {
            let g = UnDirGraphMtEph::from_sets(V, E);
            let ghost gv = g@;
            LockedUnDirGraphMtEph {
                locked_graph: RwLock::new(g, Ghost(UnDirGraphMtEphInv)),
                ghost_locked_graph: Ghost(gv),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|), Span O(|V|) -- clones under lock
        fn vertices(&self) -> (v: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let v = inner.V.clone();
            read_handle.release_read();
            v
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- clones under lock
        fn edges(&self) -> (e: SetStEph<Edge<V>>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let e = inner.E.clone();
            read_handle.release_read();
            e
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
        fn sizeE(&self) -> (n: usize) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let n = inner.sizeE();
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(log |E|) -- RwLock wrapper
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let read_handle = self.locked_graph.acquire_read();
            let inner = read_handle.borrow();
            proof { accept(inner@ == self@); }
            let neighbors = inner.ng(v);
            read_handle.release_read();
            neighbors
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(log |S| * log |E|) -- RwLock wrapper
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


    impl<V: StTInMtT + Hash + 'static> RwLockPredicate<UnDirGraphMtEph<V>> for UnDirGraphMtEphInv {
        open spec fn inv(self, v: UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(v@) && valid_key_type_for_graph::<V>()
        }
    }

    //		Section 12a. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<V: StTInMtT + Hash + 'static> PartialEqSpecImpl for UnDirGraphMtEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


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
                }
            }
            v_eq && e_eq
        }
    }

    } // verus!

    //		Section 13. macros


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

    //		Section 14a. derive impls outside verus!

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

    impl<'a, V: StTInMtT + Hash + 'static> Debug for UnDirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Display for UnDirGraphMtEphIter<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphIter") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Debug for UnDirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphGhostIterator") }
    }

    impl<'a, V: StTInMtT + Hash + 'static> Display for UnDirGraphMtEphGhostIterator<'a, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphGhostIterator") }
    }

    //		Section 14b. derive impls outside verus!

    impl Debug for UnDirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphInv") }
    }

    impl Display for UnDirGraphMtEphInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "UnDirGraphMtEphInv") }
    }

    //		Section 14c. derive impls outside verus!

    impl<V: StTInMtT + Hash + 'static> Debug for LockedUnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedUnDirGraphMtEph") }
    }

    impl<V: StTInMtT + Hash + 'static> Display for LockedUnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "LockedUnDirGraphMtEph") }
    }
}
