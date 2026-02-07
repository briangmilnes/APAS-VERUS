//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges.

pub mod UnDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::seq_set::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };

    #[verifier::reject_recursive_types(V)]
    pub struct UnDirGraphStEph<V: StT + Hash> {
        pub V: SetStEph<V>,
        pub E: SetStEph<Edge<V>>,
    }

    impl<V: StT + Hash> View for UnDirGraphStEph<V> {
        type V = GraphView<<V as View>::V>;
        
        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.E@ }
        }
    }


    pub trait UnDirGraphStEphTrait<V: StT + Hash>:
    View<V = GraphView<<V as View>::V>> + Sized {

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> 
            recommends wf_graph_view(self@), self@.V.contains(v)
        { 
            Set::new(|w: V::V| self@.A.contains((v, w)) || self@.A.contains((w, v)))
        }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V> 
            recommends wf_graph_view(self@), vertices <= self@.V
        {
            Set::new(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        open spec fn spec_degree(&self, v: V::V) -> nat 
            recommends wf_graph_view(self@), self@.V.contains(v)
        {
            self.spec_ng(v).len()
        }

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: UnDirGraphStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures
                wf_graph_view(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        fn from_sets(vertices: SetStEph<V>, edges: SetStEph<Edge<V>>) -> (g: UnDirGraphStEph<V>)
            requires
                forall |u: V::V, w: V::V| 
                    #[trigger] edges@.contains((u, w)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_graph_view(g@),
                g@.V =~= vertices@,
                g@.A =~= edges@;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self) -> (e: &SetStEph<Edge<V>>)
            ensures e@ =~= self@.A;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.V.len();

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeE(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.A.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: B)
            requires 
                wf_graph_view(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(v@),
            ensures 
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        /// APAS: Work Θ(|u_set| × |E|), Span Θ(1)
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires 
                wf_graph_view(self@),
                valid_key_type_Edge::<V>(),
                vertices@ <= self@.V,
            ensures 
                neighbors@ == self.spec_ng_of_vertices(vertices@),
                neighbors@ <= self@.V;

        /// APAS: Work Θ(1), Span Θ(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// APAS: Work Θ(|E|), Span Θ(1)
        fn degree(&self, v: &V) -> (n: N)
            requires 
                wf_graph_view(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_degree(v@);
    }

    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {

        fn empty() -> (g: UnDirGraphStEph<V>) {
            UnDirGraphStEph { V: SetStEph::empty(), E: SetStEph::empty() }
        }

        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: UnDirGraphStEph<V>) { 
            UnDirGraphStEph { V, E } 
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        fn edges(&self) -> (e: &SetStEph<Edge<V>>) { &self.E }

        fn sizeV(&self) -> (n: N) { self.V.size() }

        fn sizeE(&self) -> (n: N) { self.E.size() }

        fn neighbor(&self, u: &V, v: &V) -> (b: B) {
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) || 
            self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let mut ng: SetStEph<V> = SetStEph::empty();
            let mut it = self.E.iter();
            let ghost edges_seq = it@.1;
            let ghost v_view = v@;
            let ghost edges_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= edges_seq.len(),
                    it@.1 == edges_seq,
                    edges_seq.map(|i: int, e: Edge<V>| e@).to_set() == edges_view,
                    ng@ == Set::new(|w: V::V| 
                        exists |i: int| #![trigger edges_seq[i]] 0 <= i < it@.0 && 
                            ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                             (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w))),
                decreases edges_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #[trigger] ng@.contains(w) implies 
                                self.spec_ng(v_view).contains(w) by {
                                if ng@.contains(w) {
                                    let i = choose |i: int| #![trigger edges_seq[i]] 0 <= i < edges_seq.len() && 
                                        ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                                         (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w));
                                    lemma_seq_index_in_map_to_set(edges_seq, i);
                                }
                            }
                            assert forall |w: V::V| #[trigger] self.spec_ng(v_view).contains(w) implies 
                                ng@.contains(w) by {
                                if self.spec_ng(v_view).contains(w) {
                                    if edges_view.contains((v_view, w)) {
                                        lemma_map_to_set_contains_index(edges_seq, (v_view, w));
                                    } else {
                                        lemma_map_to_set_contains_index(edges_seq, (w, v_view));
                                    }
                                }
                            }
                        }
                        return ng;
                    },
                    Some(edge) => {
                        let a = edge.0.clone_plus();
                        let b = edge.1.clone_plus();
                        if feq(&a, v) {
                            let _ = ng.insert(b);
                        } else if feq(&b, v) {
                            let _ = ng.insert(a);
                        }
                    },
                }
            }
        }

        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            let mut result: SetStEph<V> = SetStEph::empty();
            let mut it = vertices.iter();
            let ghost u_seq = it@.1;
            let ghost vertices_view = vertices@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    result@ == Set::new(|w: V::V| 
                        exists |i: int| #![trigger u_seq[i]] 0 <= i < it@.0 && self.spec_ng(u_seq[i]@).contains(w)),
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
                            // u comes from iterator at position (it@.0 - 1)
                            // u == u_seq[it@.0 - 1], so u@ is in the mapped set
                            let idx = (it@.0 - 1) as int;
                            assert(u_seq[idx] == u);
                            lemma_seq_index_in_map_to_set(u_seq, idx);
                            assert(vertices_view.contains(u@));
                            // vertices@ <= self@.V, so u@ is in self@.V
                            assert(self@.V.contains(u@));
                        }
                        let ng_u = self.ng(u);
                        result = result.union(&ng_u);
                    },
                }
            }
        }

        fn incident(&self, e: &Edge<V>, v: &V) -> (b: B) { 
            feq(&e.0, v) || feq(&e.1, v) 
        }

        fn degree(&self, v: &V) -> (n: N) { self.ng(v).size() }
    }

    impl<V: StT + Hash> Clone for UnDirGraphStEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            UnDirGraphStEph { V: self.V.clone(), E: self.E.clone() }
        }
    }

    impl<V: StT + Hash> PartialEqSpecImpl for UnDirGraphStEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}

    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
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

    impl<V: StT + Hash> Debug for UnDirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("UnDirGraphStEph")
                .field("V", &self.V)
                .field("E", &self.E)
                .finish()
        }
    }

    impl<V: StT + Hash> Display for UnDirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} E={:?}", self.V, self.E) }
    }

    #[macro_export]
    macro_rules! UnDirGraphStEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEph<_> as $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEphTrait<_> >::from_sets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEph<_> as $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEphTrait<_> >::from_sets(__V, __E)
        }};
    }
}
