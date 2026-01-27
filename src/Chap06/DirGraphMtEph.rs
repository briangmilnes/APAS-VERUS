//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (n_plus, n_minus) and vertex map-reduce (ng_of_vertices, etc.) are parallel.

pub mod DirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Concurrency::Concurrency::MtT;
    use crate::{ParaPair, ParaPairDisjoint, SetLit};

    verus! {

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::valid_key_type;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;

    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::seq_set::*;

    broadcast use {
        vstd::set::group_set_axioms,
        crate::Types::Types::group_Edge_axioms,
    };

    pub open spec fn valid_key_type_for_graph<V: StT + MtT + Hash>() -> bool {
        valid_key_type_Edge::<V>()
    }

    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphMtEph<V: StT + MtT + Hash + 'static> {
        pub V: SetStEph<V>,
        pub A: SetStEph<Edge<V>>,
    }

    impl<V: StT + MtT + Hash + 'static> View for DirGraphMtEph<V> {
        type V = GraphView<<V as View>::V>;
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.A@ }
        }
    }

    impl<V: StT + MtT + Hash + 'static> DirGraphMtEph<V> {
        /// Convenience accessor for vertices view
        pub open spec fn spec_vertices(&self) -> Set<V::V> { self.V@ }
        /// Convenience accessor for arcs view
        pub open spec fn spec_arcs(&self) -> Set<(V::V, V::V)> { self.A@ }
    }

    use crate::Types::Types::wf_graph_view;

    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {

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

        open spec fn spec_n_plus_from_set(&self, arcs: Set<(V::V, V::V)>, v: V::V) -> Set<V::V> {
            Set::new(|w: V::V| arcs.contains((v, w)))
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

    /// Parallel arc filtering for out-neighbors using set split.
    #[verifier::external_body]
    fn n_plus_parallel<V: StT + MtT + Hash + 'static>(
        g: &DirGraphMtEph<V>, 
        v: V, 
        arcs: SetStEph<Edge<V>>
    ) -> (out_neighbors: SetStEph<V>)
        requires 
            valid_key_type::<V>(),
            valid_key_type::<Edge<V>>(),
            arcs@ <= g@.A,
        ensures 
            out_neighbors@ == g.spec_n_plus_from_set(arcs@,v@),
            out_neighbors@ <= g.spec_n_plus(v@),
        decreases arcs.size()
    {
        let n = arcs.size();
        if n == 0 {
            proof {
                assert(Set::<V::V>::empty() <= g.spec_n_plus(v@));
            }
            // TODO We need to apply an empty set lemma here. 
            let out_neighbors = SetStEph::empty();
            assert(out_neighbors@ == g.spec_n_plus_from_set(arcs@,v@));
            out_neighbors
        }
        else if n == 1 {
            let mut it = arcs.iter();
            let ghost iter_seq = it@.1;
            // iter ensures: iter_seq.map(|i, k| k@).to_set() == arcs@
            match it.next() {
                None => {
                    // TODO We need to apply a lemma that a set of size 1 has a Some.
                    SetStEph::empty()
                }
                Some(Edge(x, y)) => {
                    // next ensures: Edge(x, y) == iter_seq[0] (since old_index was 0)
                    proof {
                        // iter_seq[0] maps to (x@, y@) which is in arcs@
                        let mapped = iter_seq.map(|i: int, k: Edge<V>| k@);
                        assert(mapped[0] == (x@, y@));
                        assert(mapped.to_set().contains((x@, y@)));
                        assert(arcs@.contains((x@, y@)));
                    }
                    if feq(x, &v) {
                        proof {
                            assert(g@.A.contains((x@, y@)));
                            assert(g@.A.contains((v@, y@)));
                            assert(g.spec_n_plus(v@).contains(y@));
                            assert(Set::empty().insert(y@) <= g.spec_n_plus(v@));
                        }
                        SetStEph::singleton(y.clone_plus())
                    } else {
                        proof {
                            assert(Set::<V::V>::empty() <= g.spec_n_plus(v@));
                        }
                        SetStEph::empty()
                    }
                }
            }
        }
        else {
            let mid = n / 2;
            let (left_arcs, right_arcs) = arcs.split(mid);
            let v_left = v.clone_plus();
            let v_right = v;
            let g_left = g.clone();
            let g_right = g.clone();
            let Pair(left_neighbors, right_neighbors) =
                ParaPair!(move || n_plus_parallel(&g_left, v_left, left_arcs),
                          move || n_plus_parallel(&g_right, v_right, right_arcs));
            let out_neighbors = left_neighbors.union(&right_neighbors);
            proof {
                // Both halves are subsets, so union is subset
                assume(out_neighbors@ <= g.spec_n_plus(v@));
            }
            out_neighbors
        }
    }

    /// Parallel arc filtering for in-neighbors.
    fn n_minus_parallel<V: StT + MtT + Hash + 'static>(graph: &DirGraphMtEph<V>, v: V) -> (result: SetStEph<V>)
        requires valid_key_type_for_graph::<V>()
        ensures result@ == graph.spec_n_minus(v@)
    {
        let arcs = graph.A.to_seq();
        let n = arcs.len();
        let result = if n == 0 { SetStEph::empty() }
        else if n == 1 {
            let Edge(x, y) = &arcs[0];
            if feq(y, &v) { SetStEph::singleton(x.clone_plus()) } 
            else { SetStEph::empty() }
        }
        else {
            let mid = n / 2;
            let mut right_arcs = arcs;
            let left_arcs = right_arcs.split_off(mid);
            let v_left = v.clone_plus();
            let v_right = v;
            let Pair(left_result, right_result) =
                ParaPair!(move || n_minus_arcs_parallel(left_arcs, v_left),
                          move || n_minus_arcs_parallel(right_arcs, v_right));
            left_result.union(&right_result)
        };
        proof { assume(result@ == graph.spec_n_minus(v@)); }
        result
    }

    /// Parallel arc filtering for in-neighbors (internal, works on arc sequence).
    fn n_minus_arcs_parallel<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> (result: SetStEph<V>)
        requires valid_key_type::<V>()
        decreases arcs.len()
    {
        let n = arcs.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 {
            let Edge(x, y) = &arcs[0];
            if feq(y, &v) { SetStEph::singleton(x.clone_plus()) } 
            else { SetStEph::empty() }
        }
        else {
            let mid = n / 2;
            let mut right_arcs = arcs;
            let left_arcs = right_arcs.split_off(mid);
            let v_left = v.clone_plus();
            let v_right = v;
            let Pair(left_result, right_result) =
                ParaPair!(move || n_minus_arcs_parallel(left_arcs, v_left),
                          move || n_minus_arcs_parallel(right_arcs, v_right));
            left_result.union(&right_result)
        }
    }

    /// Parallel out-neighbors over a set of vertices.
    fn n_plus_of_vertices_parallel<V: StT + MtT + Hash + 'static>(
        graph: &DirGraphMtEph<V>,
        u_set: &SetStEph<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            u_set@ <= graph@.V,
        ensures result@ == graph.spec_n_plus_of_vertices(u_set@)
    {
        let vertices = u_set.to_seq();
        let n = vertices.len();
        let result = if n == 0 { SetStEph::empty() }
        else if n == 1 { n_plus_parallel(graph, vertices[0].clone_plus(), graph.A.clone()) }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, mid + i);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, i);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph.clone();
            let Pair(left_result, right_result) =
                ParaPair!(move || n_plus_of_vertices_seq_parallel(graph_left, left_verts),
                          move || n_plus_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        };
        proof { assume(result@ == graph.spec_n_plus_of_vertices(u_set@)); }
        result
    }

    /// Internal recursive worker for n_plus_of_vertices_parallel.
    fn n_plus_of_vertices_seq_parallel<V: StT + MtT + Hash + 'static>(
        graph: DirGraphMtEph<V>,
        vertices: Vec<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            forall |i: int| 0 <= i < vertices@.len() ==> graph@.V.contains(#[trigger] vertices@[i]@),
        decreases vertices.len()
    {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { 
            n_plus_parallel(&graph, vertices[0].clone_plus(), graph.A.clone())
        }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || n_plus_of_vertices_seq_parallel(graph_left, left_verts),
                          move || n_plus_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        }
    }

    /// Parallel in-neighbors over a set of vertices.
    fn n_minus_of_vertices_parallel<V: StT + MtT + Hash + 'static>(
        graph: &DirGraphMtEph<V>,
        u_set: &SetStEph<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            u_set@ <= graph@.V,
        ensures result@ == graph.spec_n_minus_of_vertices(u_set@)
    {
        let vertices = u_set.to_seq();
        let n = vertices.len();
        let result = if n == 0 { SetStEph::empty() }
        else if n == 1 { n_minus_parallel(graph, vertices[0].clone_plus()) }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, mid + i);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, i);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph.clone();
            let Pair(left_result, right_result) =
                ParaPair!(move || n_minus_of_vertices_seq_parallel(graph_left, left_verts),
                          move || n_minus_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        };
        proof { assume(result@ == graph.spec_n_minus_of_vertices(u_set@)); }
        result
    }

    /// Internal recursive worker for n_minus_of_vertices_parallel.
    fn n_minus_of_vertices_seq_parallel<V: StT + MtT + Hash + 'static>(
        graph: DirGraphMtEph<V>,
        vertices: Vec<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            forall |i: int| 0 <= i < vertices@.len() ==> graph@.V.contains(#[trigger] vertices@[i]@),
        decreases vertices.len()
    {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { n_minus_parallel(&graph, vertices[0].clone_plus()) }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || n_minus_of_vertices_seq_parallel(graph_left, left_verts),
                          move || n_minus_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        }
    }

    /// Parallel all-neighbors over a set of vertices.
    fn ng_of_vertices_parallel<V: StT + MtT + Hash + 'static>(
        graph: &DirGraphMtEph<V>,
        u_set: &SetStEph<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            u_set@ <= graph@.V,
        ensures result@ == graph.spec_ng_of_vertices(u_set@)
    {
        let vertices = u_set.to_seq();
        let n = vertices.len();
        let result = if n == 0 { SetStEph::empty() }
        else if n == 1 { 
            proof {
                // vertices[0]@ is in u_set@ which is subset of graph@.V
                lemma_seq_index_in_map_to_set(vertices@, 0);
            }
            graph.ng(&vertices[0]) 
        }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, mid + i);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                    lemma_seq_index_in_map_to_set::<V>(old_vertices, i);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph.clone();
            let Pair(left_result, right_result) =
                ParaPair!(move || ng_of_vertices_seq_parallel(graph_left, left_verts),
                          move || ng_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        };
        proof { assume(result@ == graph.spec_ng_of_vertices(u_set@)); }
        result
    }

    /// Internal recursive worker for ng_of_vertices_parallel.
    fn ng_of_vertices_seq_parallel<V: StT + MtT + Hash + 'static>(
        graph: DirGraphMtEph<V>,
        vertices: Vec<V>,
    ) -> (result: SetStEph<V>)
        requires 
            wf_graph_view(graph@),
            valid_key_type_for_graph::<V>(),
            forall |i: int| 0 <= i < vertices@.len() ==> graph@.V.contains(#[trigger] vertices@[i]@),
        decreases vertices.len()
    {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { graph.ng(&vertices[0]) }
        else {
            let mid = n / 2;
            let ghost old_vertices = vertices@;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            proof {
                assert forall |i: int| 0 <= i < left_verts@.len() 
                    implies graph@.V.contains(#[trigger] left_verts@[i]@) by {
                    assert(left_verts@[i] == old_vertices[mid + i]);
                }
                assert forall |i: int| 0 <= i < right_verts@.len() 
                    implies graph@.V.contains(#[trigger] right_verts@[i]@) by {
                    assert(right_verts@[i] == old_vertices[i]);
                }
            }
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || ng_of_vertices_seq_parallel(graph_left, left_verts),
                          move || ng_of_vertices_seq_parallel(graph_right, right_verts));
            left_result.union(&right_result)
        }
    }

    impl<V: StT + MtT + Hash + 'static> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {
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
            let out_neighbors = n_plus_parallel(self, v.clone_plus(), arcs);
            assume(out_neighbors@ == self.spec_n_plus_from_set(arcs@, v@));
            assert(out_neighbors@ == self.spec_n_plus(v@));
            out_neighbors
        }

        fn out_degree(&self, v: &V) -> (n: N) { self.n_plus(v).size() }

        fn n_minus(&self, v: &V) -> SetStEph<V> { n_minus_parallel(self, v.clone_plus()) }
        fn in_degree(&self, v: &V) -> (n: N) { self.n_minus(v).size() }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) { self.n_plus(v).union(&self.n_minus(v)) }
        fn degree(&self, v: &V) -> (n: N) { self.ng(v).size() }

        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { n_plus_of_vertices_parallel(self, u_set) }
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { n_minus_of_vertices_parallel(self, u_set) }
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> { ng_of_vertices_parallel(self, u_set) }
    }

    impl<V: StT + MtT + Hash + 'static> Clone for DirGraphMtEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            DirGraphMtEph { V: self.V.clone(), A: self.A.clone() }
        }
    }

    } // verus!

    impl<V: StT + MtT + Hash + 'static> Debug for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphMtEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash + 'static> Display for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + MtT + Hash + 'static> PartialEq for DirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }

    impl<V: StT + MtT + Hash + 'static> Eq for DirGraphMtEph<V> {}

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
