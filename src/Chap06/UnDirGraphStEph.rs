// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod UnDirGraphStEph {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::into_iter_hash_keys;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::seq_set::*;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

verus!
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_Edge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
        vstd::set::group_set_lemmas,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct UnDirGraphStEph<V: StT + Hash> {
        pub V: SetStEph<V>,
        pub E: SetStEph<Edge<V>>,
    }

    //		Section 5. view impls


    impl<V: StT + Hash> View for UnDirGraphStEph<V> {
        type V = GraphView<<V as View>::V>;

        open spec fn view(&self) -> Self::V {
            GraphView { V: self.V@, A: self.E@ }
        }
    }

    //		Section 8. traits


    pub trait UnDirGraphStEphTrait<V: StT + Hash>:
    View<V = GraphView<<V as View>::V>> + Sized {

        spec fn spec_undirgraphsteph_wf(&self) -> bool;

        open spec fn spec_ng(&self, v: V::V) -> Set<V::V>
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        {
            self@.V.filter(|w: V::V| self@.A.contains((v, w)) || self@.A.contains((w, v)))
        }

        open spec fn spec_ng_of_vertices(&self, vertices: Set<V::V>) -> Set<V::V>
            recommends spec_graphview_wf(self@), vertices <= self@.V
        {
            self@.V.filter(|w: V::V| exists |u: V::V| #![trigger vertices.contains(u)] vertices.contains(u) && self.spec_ng(u).contains(w))
        }

        open spec fn spec_degree(&self, v: V::V) -> nat
            recommends spec_graphview_wf(self@), self@.V.contains(v)
        {
            self.spec_ng(v).len()
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: Self)
            requires valid_key_type_Edge::<V>()
            ensures
                g.spec_undirgraphsteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= Set::<<V as View>::V>::empty(),
                g@.A =~= Set::<(<V as View>::V, <V as View>::V)>::empty();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) -- sequential
        fn from_sets(vertices: SetStEph<V>, edges: SetStEph<Edge<V>>) -> (g: Self)
            requires
                forall |u: V::V, w: V::V|
                    #[trigger] edges@.contains((u, w)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                g.spec_undirgraphsteph_wf(),
                spec_graphview_wf(g@),
                g@.V =~= vertices@,
                g@.A =~= edges@;

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
            requires valid_key_type_Edge::<V>()
            ensures n == self@.V.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn sizeE(&self) -> (n: usize)
            requires valid_key_type_Edge::<V>()
            ensures n == self@.A.len();

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn neighbor(&self, u: &V, v: &V) -> (b: bool)
            requires
                spec_graphview_wf(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(u@),
                self@.V.contains(v@),
            ensures b == (self@.A.contains((u@, v@)) || self@.A.contains((v@, u@)));

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential filter
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>)
            requires
                spec_graphview_wf(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(v@),
            ensures
                neighbors@ == self.spec_ng(v@),
                neighbors@ <= self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|u_set| x |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|u_set| x |E|), Span O(|u_set| x |E|) -- nested iteration
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>)
            requires
                spec_graphview_wf(self@),
                valid_key_type_Edge::<V>(),
                vertices@ <= self@.V,
            ensures
                neighbors@ == self.spec_ng_of_vertices(vertices@),
                neighbors@ <= self@.V;

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool)
            requires valid_key_type_Edge::<V>()
            ensures b == (e@.0 == v@ || e@.1 == v@);

        /// - Alg Analysis: APAS (Ch06 Def 6.2): Work O(|E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential filter
        fn degree(&self, v: &V) -> (n: usize)
            requires
                spec_graphview_wf(self@),
                valid_key_type_Edge::<V>(),
                self@.V.contains(v@),
            ensures n == self.spec_degree(v@);
    }

    //		Section 9. impls


    impl<V: StT + Hash> UnDirGraphStEphTrait<V> for UnDirGraphStEph<V> {

        open spec fn spec_undirgraphsteph_wf(&self) -> bool {
            spec_graphview_wf(self@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (g: UnDirGraphStEph<V>)
            ensures g.spec_undirgraphsteph_wf()
        {
            UnDirGraphStEph { V: SetStEph::empty(), E: SetStEph::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_sets(V: SetStEph<V>, E: SetStEph<Edge<V>>) -> (g: UnDirGraphStEph<V>)
            ensures g.spec_undirgraphsteph_wf()
        {
            UnDirGraphStEph { V, E }
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
            self.E.mem(&Edge(u.clone_plus(), v.clone_plus())) ||
            self.E.mem(&Edge(v.clone_plus(), u.clone_plus()))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|) -- sequential scan of edges
        fn ng(&self, v: &V) -> (neighbors: SetStEph<V>) {
            let mut ng: SetStEph<V> = SetStEph::empty();
            let edges_iter = self.E.iter();
            let ghost edges_seq = into_iter_hash_keys(edges_iter);
            let ghost v_view = v@;
            let ghost edges_view = self@.A;
            let mut it = edges_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= edges_seq.len(),
                    IteratorSpec::remaining(&it).len() == edges_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == edges_seq[pos + i],
                    edges_seq.map(|i: int, e: Edge<V>| e@).to_set() == edges_view,
                    forall |w: V::V| #[trigger] ng@.contains(w) <==>
                        exists |i: int| #![trigger edges_seq[i]] 0 <= i < pos &&
                            ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                             (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w)),
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block (speed hint)
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] ng@.contains(w) implies
                                self.spec_ng(v_view).contains(w) by {
                                if ng@.contains(w) {
                                    let i = choose |i: int| #![trigger edges_seq[i]] 0 <= i < edges_seq.len() &&
                                        ((edges_seq[i]@.0 == v_view && edges_seq[i]@.1 == w) ||
                                         (edges_seq[i]@.1 == v_view && edges_seq[i]@.0 == w));
                                    lemma_seq_index_in_map_to_set(edges_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
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
                            assert(ng@ =~= self.spec_ng(v_view));
                        }
                        return ng;
                    },
                    Some(edge) => {
                        proof { pos = pos + 1; assert(edges_seq[old_pos] == *edge); }
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|S| * |E|), Span O(|S| * |E|) -- iterates vertices, calls ng for each
        fn ng_of_vertices(&self, vertices: &SetStEph<V>) -> (neighbors: SetStEph<V>) {
            let mut neighbors: SetStEph<V> = SetStEph::empty();
            let u_iter = vertices.iter();
            let ghost u_seq = into_iter_hash_keys(u_iter);
            let ghost vertices_view = vertices@;
            let mut it = u_iter;
            let ghost mut pos: int = 0;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_Edge::<V>(),
                    IteratorSpec::obeys_prophetic_iter_laws(&it),
                    IteratorSpec::decrease(&it) is Some,
                    0 <= pos <= u_seq.len(),
                    IteratorSpec::remaining(&it).len() == u_seq.len() - pos,
                    forall|i: int| 0 <= i < IteratorSpec::remaining(&it).len()
                        ==> *(#[trigger] IteratorSpec::remaining(&it)[i]) == u_seq[pos + i],
                    u_seq.map(|i: int, v: V| v@).to_set() == vertices_view,
                    neighbors@ <= self@.V,
                    forall |w: V::V| #[trigger] neighbors@.contains(w) <==>
                        exists |i: int| #![trigger u_seq[i]] 0 <= i < pos &&self.spec_ng(u_seq[i]@).contains(w),
                decreases IteratorSpec::decrease(&it)->0,
            {
                let ghost old_pos = pos;
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        // Veracity: NEEDED proof block
                        proof {
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] neighbors@.contains(w) implies
                                self.spec_ng_of_vertices(vertices_view).contains(w) by {
                                if neighbors@.contains(w) {
                                    let i = choose |i: int| #![trigger u_seq[i]] 0 <= i < u_seq.len() && self.spec_ng(u_seq[i]@).contains(w);
                                    lemma_seq_index_in_map_to_set(u_seq, i);
                                }
                            }
                            // Veracity: NEEDED assert
                            // Veracity: NEEDED assert
                            assert forall |w: V::V| #[trigger] self.spec_ng_of_vertices(vertices_view).contains(w) implies
                                neighbors@.contains(w) by {
                                if self.spec_ng_of_vertices(vertices_view).contains(w) {
                                    let u = choose |u: V::V| #![trigger vertices_view.contains(u)] vertices_view.contains(u) && self.spec_ng(u).contains(w);
                                    lemma_map_to_set_contains_index(u_seq, u);
                                }
                            }
                            assert(neighbors@ =~= self.spec_ng_of_vertices(vertices_view));
                        }
                        return neighbors;
                    },
                    // Veracity: NEEDED proof block
                    Some(u) => {
                        // Veracity: NEEDED proof block
                        proof {
                            pos = pos + 1;
                            // u comes from iterator at position old_pos
                            // u == u_seq[old_pos], so u@ is in the mapped set
                            lemma_seq_index_in_map_to_set(u_seq, old_pos);
                            // vertices@ <= self@.V, so u@ is in self@.V
                        }
                        let ng_u = self.ng(u);
                        neighbors = neighbors.union(&ng_u);
                    },
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn incident(&self, e: &Edge<V>, v: &V) -> (b: bool) {
            feq(&e.0, v) || feq(&e.1, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|E|), Span O(|E|)
        fn degree(&self, v: &V) -> (n: usize) { self.ng(v).size() }
    }

    //		Section 10. iterators


    // Delegated iteration over the vertices: the std hash-set iterator that vstd
    // specifies. An impl of an external trait method may not add `requires`, so
    // the contract is conditional on the vertex set's well-formedness.
    impl<'a, V: StT + Hash> std::iter::IntoIterator for &'a UnDirGraphStEph<V> {
        type Item = &'a V;
        type IntoIter = std::collections::hash_set::Iter<'a, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                self.V.spec_setsteph_wf() ==> {
                    &&& IteratorSpec::remaining(&it).unref().map(|i: int, k: V| k@).to_set() == self@.V
                    &&& IteratorSpec::remaining(&it).unref().no_duplicates()
                    &&& IteratorSpec::remaining(&it).len() == self@.V.len()
                    &&& into_iter_hash_keys(it) == IteratorSpec::remaining(&it).unref()
                    &&& IteratorSpec::decrease(&it) is Some
                },
        {
            (&self.V).into_iter()
        }
    }

    //		Section 12. derive impls in verus!


    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash> PartialEqSpecImpl for UnDirGraphStEph<V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<V: StT + Hash> Clone for UnDirGraphStEph<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            UnDirGraphStEph { V: self.V.clone(), E: self.E.clone() }
        }
    }

    impl<V: StT + Hash> Eq for UnDirGraphStEph<V> {}

    impl<V: StT + Hash> PartialEq for UnDirGraphStEph<V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            // Veracity: NEEDED proof block
            let v_eq = self.V == other.V;
            let e_eq = self.E == other.E;
            // Veracity: NEEDED proof block
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

    //		Section 14. derive impls outside verus!

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
}
