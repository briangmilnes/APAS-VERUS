//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (n_plus, n_minus) and vertex map-reduce (ng_of_vertices, etc.) are parallel.

pub mod DirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Concurrency::Concurrency::MtT;
    use crate::{ParaPair, ParaPairDisjoint, SetLit};
    use crate::Types::Types::{*, GraphView};

    verus! {

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::valid_key_type;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::vstdplus::feq::feq::feq;

    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

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

    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash + 'static> : View<V = GraphView<<V as View>::V>> + Sized {
        open spec fn spec_finite(&self) -> bool {
            self@.V.finite() && self@.A.finite()
        }

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V> { Set::new(|w: V::V| self@.A.contains((v, w))) }
        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V> { Set::new(|u: V::V| self@.A.contains((u, v))) }
        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> { self.spec_n_plus(v).union(self.spec_n_minus(v)) }
        open spec fn spec_degree(&self, v: V::V) -> nat { self.spec_ng(v).len() }

        open spec fn spec_n_plus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_plus(u).contains(w))
        }

        open spec fn spec_n_minus_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_n_minus(u).contains(w))
        }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures g@.V == Set::<<V as View>::V>::empty(), g@.A == Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        fn from_sets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: Self)
            requires valid_key_type_for_graph::<V>()
            ensures g@.V == V@, g@.A == A@;

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
            requires valid_key_type_for_graph::<V>()
            ensures b == self@.A.contains((u@, v@));

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures neighbors@ == self.spec_ng(v@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures neighbors@ == self.spec_ng_of_vertices(u_set@);

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn n_plus(&self, v: &V) -> (out_neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures out_neighbors@ == self.spec_n_plus(v@);

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn n_minus(&self, v: &V) -> (in_neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures in_neighbors@ == self.spec_n_minus(v@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> (out_neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures out_neighbors@ == self.spec_n_plus_of_vertices(u_set@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|) - parallel
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> (in_neighbors: SetStEph<V>)
            requires valid_key_type_for_graph::<V>()
            ensures in_neighbors@ == self.spec_n_minus_of_vertices(u_set@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_for_graph::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn degree(&self, v: &V) -> (n: N)
            requires valid_key_type_for_graph::<V>()
            ensures n == self.spec_degree(v@);

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn in_degree(&self, v: &V) -> (n: N)
            requires valid_key_type_for_graph::<V>()
            ensures n == self.spec_n_minus(v@).len();

        /// APAS: Work Θ(|A|), Span Θ(log |A|) - parallel
        fn out_degree(&self, v: &V) -> (n: N)
            requires valid_key_type_for_graph::<V>()
            ensures n == self.spec_n_plus(v@).len();
    }

    /// Spec: filter arcs for out-neighbors of v
    pub open spec fn spec_filter_n_plus<V: View>(arcs: Seq<(V::V, V::V)>, v: V::V) -> Set<V::V> {
        Set::new(|w: V::V| exists |i: int| #![trigger arcs[i]] 0 <= i < arcs.len() && arcs[i].0 == v && arcs[i].1 == w)
    }

    #[verifier::external_body]
    fn parallel_n_plus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> (result: SetStEph<V>)
        requires valid_key_type::<V>()
        ensures result@ == spec_filter_n_plus::<V>(arcs@.map(|i, e: Edge<V>| e@), v@)
    {
        let n = arcs.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 {
            let Edge(x, y) = &arcs[0];
            if feq(x, &v) {
                let mut s = SetStEph::empty();
                s.insert(y.clone_plus());
                s
            } else {
                SetStEph::empty()
            }
        }
        else {
            let mid = n / 2;
            let mut right_arcs = arcs;
            let left_arcs = right_arcs.split_off(mid);
            let v_left = v.clone_plus();
            let v_right = v;
            let Pair(left_result, right_result) =
                ParaPair!(move || parallel_n_plus(left_arcs, v_left),
                          move || parallel_n_plus(right_arcs, v_right));
            left_result.union(&right_result)
        }
    }

    #[verifier::external_body]
    fn parallel_n_minus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> (result: SetStEph<V>) {
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
                ParaPair!(move || parallel_n_minus(left_arcs, v_left),
                          move || parallel_n_minus(right_arcs, v_right));
            left_result.union(&right_result)
        }
    }

    #[verifier::external_body]
    fn parallel_n_plus_of_vertices<V: StT + MtT + Hash + 'static>(
        vertices: Vec<V>,
        graph: DirGraphMtEph<V>,
    ) -> (result: SetStEph<V>) {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { graph.n_plus(&vertices[0]) }
        else {
            let mid = n / 2;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || parallel_n_plus_of_vertices(left_verts, graph_left),
                          move || parallel_n_plus_of_vertices(right_verts, graph_right));
            left_result.union(&right_result)
        }
    }

    #[verifier::external_body]
    fn parallel_n_minus_of_vertices<V: StT + MtT + Hash + 'static>(
        vertices: Vec<V>,
        graph: DirGraphMtEph<V>,
    ) -> (result: SetStEph<V>) {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { graph.n_minus(&vertices[0]) }
        else {
            let mid = n / 2;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || parallel_n_minus_of_vertices(left_verts, graph_left),
                          move || parallel_n_minus_of_vertices(right_verts, graph_right));
            left_result.union(&right_result)
        }
    }

    #[verifier::external_body]
    fn parallel_ng_of_vertices<V: StT + MtT + Hash + 'static>(
        vertices: Vec<V>,
        graph: DirGraphMtEph<V>,
    ) -> (result: SetStEph<V>) {
        let n = vertices.len();
        if n == 0 { SetStEph::empty() }
        else if n == 1 { graph.ng(&vertices[0]) }
        else {
            let mid = n / 2;
            let mut right_verts = vertices;
            let left_verts = right_verts.split_off(mid);
            let graph_left = graph.clone();
            let graph_right = graph;
            let Pair(left_result, right_result) =
                ParaPair!(move || parallel_ng_of_vertices(left_verts, graph_left),
                          move || parallel_ng_of_vertices(right_verts, graph_right));
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

        #[verifier::external_body]
        fn n_plus(&self, v: &V) -> SetStEph<V> {
            parallel_n_plus(self.A.to_seq(), v.clone_plus())
        }

        #[verifier::external_body]
        fn n_minus(&self, v: &V) -> SetStEph<V> {
            parallel_n_minus(self.A.to_seq(), v.clone_plus())
        }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            self.n_plus(v).union(&self.n_minus(v))
        }

        #[verifier::external_body]
        fn n_plus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            parallel_n_plus_of_vertices(u_set.to_seq(), self.clone())
        }

        #[verifier::external_body]
        fn n_minus_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            parallel_n_minus_of_vertices(u_set.to_seq(), self.clone())
        }

        #[verifier::external_body]
        fn ng_of_vertices(&self, u_set: &SetStEph<V>) -> SetStEph<V> {
            parallel_ng_of_vertices(u_set.to_seq(), self.clone())
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) { feq(&e.0, v) || feq(&e.1, v) }

        fn in_degree(&self, v: &V) -> (n: N) { self.n_minus(v).size() }

        fn out_degree(&self, v: &V) -> (n: N) { self.n_plus(v).size() }

        fn degree(&self, v: &V) -> (n: N) { self.ng(v).size() }
    }

    } // verus!

    // Non-Verus impls
    impl<V: StT + MtT + Hash + 'static> Clone for DirGraphMtEph<V> {
        fn clone(&self) -> Self {
            DirGraphMtEph { V: self.V.clone(), A: self.A.clone() }
        }
    }

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

