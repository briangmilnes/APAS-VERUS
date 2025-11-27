//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

pub mod DirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::feq;

verus! {

    // Broadcast groups for hash collections, sets, and our custom axioms
    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
    };

    #[verifier::reject_recursive_types(V)]
    pub struct DirGraphStEph<V: StT + Hash> { pub V: SetStEph<V>, pub A: SetStEph<Edge<V>> }

    impl<V: StT + Hash> DirGraphStEph<V> {
        /// Returns an iterator over the vertices
        pub fn iter_vertices(&self) -> (it: SetStEphIter<V>)
            requires valid_key_type_Edge::<V>() 
       { self.V.iter() }

        /// Returns an iterator over the arcs
        pub fn iter_arcs(&self) -> (it: SetStEphIter<Edge<V>>)
            requires valid_key_type_Edge::<V>()
        { self.A.iter() }
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

    // Helper: convert Option<V> to Set<V>
    pub open spec fn option_to_set<V>(opt: Option<V>) -> Set<V> {
        match opt {
            None => Set::empty(),
            Some(v) => Set::empty().insert(v),
        }
    }

    // View implementation: (vertices, arcs) as spec sets
    impl<V: StT + Hash> View for DirGraphStEph<V> {
        type V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V)>);

        open spec fn view(&self) -> Self::V { (self.V@, self.A@) }
    }

    // View type: (vertices, arcs) as spec sets
    pub trait DirGraphStEphTrait<V: StT + Hash>:
    View<V = (Set<<V as View>::V>, Set<(<V as View>::V, <V as View>::V)>)> + Sized {

        /// Out-neighbors: vertices w such that (v, w) is an arc
        open spec fn spec_nplus(&self, v: V::V) -> Set<V::V> { Set::new(|w: V::V| self@.1.contains((v, w))) }

        /// In-neighbors: vertices u such that (u, v) is an arc
        open spec fn spec_nminus(&self, v: V::V) -> Set<V::V> { Set::new(|u: V::V| self@.1.contains((u, v))) }

        /// All neighbors: union of in and out neighbors
        open spec fn spec_ng(&self, v: V::V) -> Set<V::V> { self.spec_nplus(v).union(self.spec_nminus(v)) }

        /// Out-neighbors of a set of vertices
        open spec fn spec_nplus_of_vertices(&self, u_set: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![auto] u_set.contains(u) && self.spec_nplus(u).contains(w))
        }

        /// In-neighbors of a set of vertices
        open spec fn spec_nminus_of_vertices(&self, u_set: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![auto] u_set.contains(u) && self.spec_nminus(u).contains(w))
        }

        /// All neighbors of a set of vertices
        open spec fn spec_ng_of_vertices(&self, u_set: Set<V::V>) -> Set<V::V> {
            Set::new(|w: V::V| exists |u: V::V| #![auto] u_set.contains(u) && self.spec_ng(u).contains(w))
        }

        /// Degree of a vertex = |NG(v)| = size of neighborhood
        open spec fn spec_degree(&self, v: V::V) -> nat { self.spec_ng(v).len() }

        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> (g: DirGraphStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures
                g@.0 =~= Set::<<V as View>::V>::empty(),
                g@.1 =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        fn FromSets(vertices: SetStEph<V>, arcs: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>)
            ensures
                g@.0 =~= vertices@,
                g@.1 =~= arcs@;

        /// APAS: Work Θ(1), Span Θ(1)
        fn vertices(&self)                              -> (v: &SetStEph<V>)
            ensures v@ == self@.0;

        /// APAS: Work Θ(1), Span Θ(1)
        fn arcs(&self)                                  -> (a: &SetStEph<Edge<V>>)
            ensures a@ =~= self@.1;

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeV(&self)                                 -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.0.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn sizeA(&self)                                 -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.1.len();

        /// APAS: Work Θ(1), Span Θ(1)
        fn Neighbor(&self, u: &V, v: &V)                -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == self@.1.contains((u@, v@));

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NG(&self, v: &V)                             -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_ng(v@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NGOfVertices(&self, u_set: &SetStEph<V>)     -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_ng_of_vertices(u_set@);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NPlus(&self, v: &V)                          -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_nplus(v@);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn NMinus(&self, v: &V)                         -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_nminus(v@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NPlusOfVertices(&self, u_set: &SetStEph<V>)  -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_nplus_of_vertices(u_set@);

        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NMinusOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>)
            requires valid_key_type_Edge::<V>()
            ensures result@ == self.spec_nminus_of_vertices(u_set@);

        /// APAS: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Edge<V>, v: &V)          -> (b: B)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn Degree(&self, v: &V)                         -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self.spec_degree(v@);

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn InDegree(&self, v: &V)                       -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self.spec_nminus(v@).len();

        /// APAS: Work Θ(|A|), Span Θ(1)
        fn OutDegree(&self, v: &V)                      -> (n: N)
            requires valid_key_type_Edge::<V>()
            ensures n == self.spec_nplus(v@).len();
    }

    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {

        fn empty() -> (g: DirGraphStEph<V>) { DirGraphStEph { V: SetStEph::empty(), A: SetStEph::empty() } }

        fn FromSets(V: SetStEph<V>, A: SetStEph<Edge<V>>) -> (g: DirGraphStEph<V>) { DirGraphStEph { V, A } }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.V }

        fn arcs(&self) -> (a: &SetStEph<Edge<V>>) { &self.A }

        fn sizeV(&self) -> (n: N) { self.V.size() }

        fn sizeA(&self) -> (n: N) { self.A.size() }

        fn Neighbor(&self, u: &V, v: &V) -> (b: B) { self.A.mem(&Edge(u.clone_plus(), v.clone_plus())) }

        fn NG(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_ng(v@)
        { self.NPlus(v).union(&self.NMinus(v)) }

        fn NGOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>)
            ensures result@ == self.spec_ng_of_vertices(u_set@)
        {
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
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![auto]
                        0 <= i < it@.0 && self.spec_ng(u_seq[i]@).contains(w)),
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
                    }
                }
            }
        }

        fn NPlus(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nplus(v@)
        {
            let mut out: SetStEph<V> = SetStEph::empty();
            let mut it = self.A.iter();
            let ghost arcs_seq = it@.1;
            let ghost v_view = v@;
            let ghost arcs_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= arcs_seq.len(),
                    it@.1 == arcs_seq,
                    arcs_seq.map(|i: int, e: Edge<V>| e@).to_set() == arcs_view,
                    out@ == Set::new(|w: V::V| exists |i: int|
                        #![auto]
                        0 <= i < it@.0 && arcs_seq[i]@.0 == v_view && arcs_seq[i]@.1 == w),
                    decreases arcs_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            // Connect invariant to postcondition
                            assert forall |w: V::V| #![auto] out@.contains(w) implies
                            self.spec_nplus(v_view).contains(w) by {
                                if out@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < arcs_seq.len() && arcs_seq[i]@.0 == v_view && arcs_seq[i]@.1 == w;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(arcs_seq, i);
                                    assert(arcs_view.contains((v_view, w)));
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_nplus(v_view).contains(w) implies
                            out@.contains(w) by {
                                if self.spec_nplus(v_view).contains(w) {
                                    assert(arcs_view.contains((v_view, w)));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(arcs_seq, (v_view, w));
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

        fn NMinus(&self, v: &V) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nminus(v@)
        {
            let mut inn: SetStEph<V> = SetStEph::empty();
            let mut it = self.A.iter();
            let ghost arcs_seq = it@.1;
            let ghost v_view = v@;
            let ghost arcs_view = self@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    it@.0 <= arcs_seq.len(),
                    it@.1 == arcs_seq,
                    arcs_seq.map(|i: int, e: Edge<V>| e@).to_set() == arcs_view,
                    inn@ == Set::new(|u: V::V| exists |i: int|
                        #![auto]
                        0 <= i < it@.0 && arcs_seq[i]@.1 == v_view && arcs_seq[i]@.0 == u),
                    decreases arcs_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |u: V::V| #![auto] inn@.contains(u) implies
                            self.spec_nminus(v_view).contains(u) by {
                                if inn@.contains(u) {
                                    let i = choose |i: int| #![auto] 0 <= i < arcs_seq.len() && arcs_seq[i]@.1 == v_view && arcs_seq[i]@.0 == u;
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(arcs_seq, i);
                                    assert(arcs_view.contains((u, v_view)));
                                }
                            }
                            assert forall |u: V::V| #![auto] self.spec_nminus(v_view).contains(u) implies
                            inn@.contains(u) by {
                                if self.spec_nminus(v_view).contains(u) {
                                    assert(arcs_view.contains((u, v_view)));
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(arcs_seq, (u, v_view));
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

        fn NPlusOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nplus_of_vertices(u_set@)
        {
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
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![auto]
                        0 <= i < it@.0 && self.spec_nplus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #![auto] result@.contains(w) implies
                            self.spec_nplus_of_vertices(u_set_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < u_seq.len() && self.spec_nplus(u_seq[i]@).contains(w);
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(u_seq, i);
                                    assert(u_set_view.contains(u_seq[i]@));
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_nplus_of_vertices(u_set_view).contains(w) implies
                            result@.contains(w) by {
                                if self.spec_nplus_of_vertices(u_set_view).contains(w) {
                                    let u = choose |u: V::V| #![auto] u_set_view.contains(u) && self.spec_nplus(u).contains(w);
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        let plus_u = self.NPlus(u);
                        result = result.union(&plus_u);
                    }
                }
            }
        }

        fn NMinusOfVertices(&self, u_set: &SetStEph<V>) -> (result: SetStEph<V>)
            ensures result@ == self.spec_nminus_of_vertices(u_set@)
        {
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
                    result@ == Set::new(|w: V::V| exists |i: int|
                        #![auto]
                        0 <= i < it@.0 && self.spec_nminus(u_seq[i]@).contains(w)),
                    decreases u_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |w: V::V| #![auto] result@.contains(w) implies
                            self.spec_nminus_of_vertices(u_set_view).contains(w) by {
                                if result@.contains(w) {
                                    let i = choose |i: int| #![auto] 0 <= i < u_seq.len() && self.spec_nminus(u_seq[i]@).contains(w);
                                    crate::vstdplus::seq_set::lemma_seq_index_in_map_to_set(u_seq, i);
                                    assert(u_set_view.contains(u_seq[i]@));
                                }
                            }
                            assert forall |w: V::V| #![auto] self.spec_nminus_of_vertices(u_set_view).contains(w) implies
                            result@.contains(w) by {
                                if self.spec_nminus_of_vertices(u_set_view).contains(w) {
                                    let u = choose |u: V::V| #![auto] u_set_view.contains(u) && self.spec_nminus(u).contains(w);
                                    crate::vstdplus::seq_set::lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                        }
                        return result;
                    },
                    Some(u) => {
                        let minus_u = self.NMinus(u);
                        result = result.union(&minus_u);
                    }
                }
            }
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> (b: B) { feq(&e.0, v) || feq(&e.1, v) }

        fn Degree(&self, v: &V) -> (n: N)
            ensures n == self.spec_degree(v@)
        { self.NG(v).size() }

        fn InDegree(&self, v: &V) -> (n: N)
            ensures n == self.spec_nminus(v@).len()
        { self.NMinus(v).size() }

        fn OutDegree(&self, v: &V) -> (n: N)
            ensures n == self.spec_nplus(v@).len()
        { self.NPlus(v).size() }
    }

 } // verus!

    // Clone implementation (outside verus! block)
    impl<V: StT + Hash> Clone for DirGraphStEph<V> {
        fn clone(&self) -> Self { DirGraphStEph { V: self.V.clone(), A: self.A.clone() } }
    }

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

    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }

    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}

    // Macro defined outside verus! block
    #[macro_export]
    macro_rules! DirGraphStEphLit {
        () => {{
            let __V : $crate::Chap05::SetStEph::SetStEph::SetStEph<_>                             = $crate::SetLit![];
            let __A : $crate::Chap05::SetStEph::SetStEph::SetStEph<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V : $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = $crate::SetLit![ $( $v ),* ];
            let __A : $crate::Chap05::SetStEph::SetStEph::SetStEph<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }}
    }
}
