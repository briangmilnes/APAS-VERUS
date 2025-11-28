//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges.

pub mod UnDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

verus! {

    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    pub struct UnDirGraphStEph<V: StT + Hash> {
        pub V: SetStEph<V>,
        pub E: SetStEph<Edge<V>>,
    }

    impl<V: StT + Hash> View for UnDirGraphStEph<V> {
        type V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V)>);
        
        open spec fn view(&self) -> Self::V {
            (self.V@, self.E@)
        }
    }

    pub trait UnDirGraphStEphTrait<V: StT + Hash>:
    View<V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V)>)> + Sized {

        /// A graph is finite if both its vertex set and edge set are finite
        open spec fn spec_finite(&self) -> bool {
            self@.0.finite() && self@.1.finite()
        }

        /// Neighbors of v: vertices adjacent via any edge containing v
        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> { 
            Set::new(|w: V::V| self@.1.contains((v, w)) || self@.1.contains((w, v)))
        }

        /// Neighbors of a set of vertices
        open spec fn spec_ng_of_vertices(&self, u_set: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![auto] u_set.contains(u) && self.spec_ng(u).contains(w))
        }

        /// Degree = |NG(v)|
        open spec fn spec_degree(&self, v: V::V) -> nat {
            self.spec_ng(v).len()
        }

        fn empty() -> (g: UnDirGraphStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures
                g@.0 =~= Set::<<V as View>::V>::empty(),
                g@.1 =~= Set::<(<V as View>::V, <V as View>::V)>::empty(),
                g@.0.finite(),
                g@.1.finite();

        fn FromSets(vertices: SetStEph<V>, edges: SetStEph<Edge<V>>) -> (g: UnDirGraphStEph<V>)
            ensures
                g@.0 =~= vertices@,
                g@.1 =~= edges@,
                g@.0.finite(),
                g@.1.finite();

        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.0;

        fn edges(&self) -> (e: &SetStEph<Edge<V>>)
            ensures e@ =~= self@.1;

        fn sizeV(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.0.len();

        fn sizeE(&self) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.1.len();

        fn Neighbor(&self, u: &V, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (self@.1.contains((u@, v@)) || self@.1.contains((v@, u@)));

        fn NG(&self, v: &V) -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_ng(v@);

        fn NGOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_ng_of_vertices(u_set@);

        fn Incident(&self, e: &Edge<V>, v: &V) -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        fn Degree(&self, v: &V) -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self.spec_degree(v@);
    }

    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {

        fn empty() -> (g: UnDirGraphStEph<V>) {
            UnDirGraphStEph { V: SetStEph::empty(), E: SetStEph::empty() }
        }

        fn FromSets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: UnDirGraphStEph<V>) { 
            UnDirGraphStEph { V, E } 
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        fn edges(&self) -> (e: &SetStEph<Edge<V>>) { &self.E }

        fn sizeV(&self) -> (n: N) { self.V.size() }

        fn sizeE(&self) -> (n: N) { self.E.size() }

        fn Neighbor(&self, u: &V, v: &V) -> (b: B) {
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) || 
            self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        fn NG(&self, v: &V) -> (result: SetStEph<V>) {
            let mut ng: SetStEph<V> = SetStEph::empty();
            let mut it = self.E.iter();
            let ghost edges_seq = it@.1;
            let ghost v_view = v@;
            let ghost edges_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= edges_seq.len(),
                    it@.1 == edges_seq,
                    edges_seq.map(|i: int, e: Edge<V>| e@).to_set() == edges_view,
                    ng@ == Set::new(|w: V::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && 
                            ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                             (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w))),
                decreases edges_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #![auto] ng@.contains(w) implies 
                                self.spec_ng(v_view).contains(w) by {
                                if ng@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < edges_seq.len() && 
                                        ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                                         (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w));
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(edges_seq, i);
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_ng(v_view).contains(w) implies 
                                ng@.contains(w) by {
                                if self.spec_ng(v_view).contains(w) {
                                    if edges_view.contains((v_view, w)) {
                                        crate::vstdplus::seq_set::lemma_map_to_set_contains_index(edges_seq, (v_view, w));
                                    } else {
                                        crate::vstdplus::seq_set::lemma_map_to_set_contains_index(edges_seq, (w, v_view));
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

        fn NGOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>) {
            let mut result: SetStEph<V> = SetStEph::empty();
            let mut it = u_set.iter();
            let ghost u_seq = it@.1;
            let ghost u_set_view = u_set@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= u_seq.len(),
                    it@.1 == u_seq,
                    u_seq.map(|i: int, v: V| v@).to_set() == u_set_view,
                    result@ == Set::new(|w: V::V| 
                        exists |i: int| #![auto] 0 <= i < it@.0 && self.spec_ng(u_seq[i]@).contains(w)),
                decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #![auto] result@.contains(w) implies 
                                self.spec_ng_of_vertices(u_set_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < u_seq.len() && self.spec_ng(u_seq[i]@).contains(w);
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(u_seq, i);
                                    assert(u_set_view.contains(u_seq[i]@));
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_ng_of_vertices(u_set_view).contains(w) implies 
                                result@.contains(w) by {
                                if self.spec_ng_of_vertices(u_set_view).contains(w) {
                                    let u = choose |u: V::V| #![auto] u_set_view.contains(u) && self.spec_ng(u).contains(w);
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        let ng_u = self.NG(u);
                        result = result.union(&ng_u);
                    },
                }
            }
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> (b: B) { 
            feq(&e.0, v) || feq(&e.1, v) 
        }

        fn Degree(&self, v: &V) -> (n: N) { self.NG(v).size() }
    }

} // verus!

    impl<V: StT + Hash> Clone for UnDirGraphStEph<V> {
        fn clone(&self) -> Self { UnDirGraphStEph { V: self.V.clone(), E: self.E.clone() } }
    }

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

    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }
    }

    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphStEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEph<_> as $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEphTrait<_> >::FromSets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEph<_> as $crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::UnDirGraphStEphTrait<_> >::FromSets(__V, __E)
        }};
    }
}
