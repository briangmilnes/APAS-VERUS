//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 6 Weighted Directed Graph (ephemeral) with f64 weights - Single-threaded version.
//! Uses WrappedF64 for Verus-compatible float weights.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 13. macros

//		Section 1. module

pub mod WeightedDirGraphStEphF64 {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::float::float::*;
    use crate::vstdplus::seq_set::*;

verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        vstd::set_lib::group_set_lib_default,
        vstd::set::group_set_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };

    //		Section 4. type definitions


    pub type WeightedDirGraphStEphF64<V> = LabDirGraphStEph<V, WrappedF64>;

    //		Section 8. traits


    pub trait WeightedDirGraphStEphF64Trait<V: StT + Hash>:
        View<V = LabGraphView<<V as View>::V, f64>> + Sized {

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|V| + |E|), Span O(|V| + |E|) -- sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, WrappedF64>>) -> (g: WeightedDirGraphStEphF64<V>)
            requires
                valid_key_type_WeightedEdge::<V, WrappedF64>(),
                edges@.finite(),
                forall |u: V::V, w: V::V, weight: f64|
                    #[trigger] edges@.contains((u, w, weight)) ==>
                        vertices@.contains(u) && vertices@.contains(w),
            ensures spec_labgraphview_wf(g@), g@.V =~= vertices@, g@.A =~= edges@;

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: WrappedF64)
            requires valid_key_type_WeightedEdge::<V, WrappedF64>()
            ensures
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, weight@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential search
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<WrappedF64>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, WrappedF64>()
            ensures
                weight.is_some() == (exists |w: f64| #![trigger self@.A.contains((from@, to@, w))] self@.A.contains((from@, to@, w))),
                weight.is_some() ==> self@.A.contains((from@, to@, weight.unwrap()@));

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential iteration
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, WrappedF64>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, WrappedF64>()
            ensures
                forall |t: (V::V, V::V, f64)| #[trigger] weighed_edges@.contains(t) == self@.A.contains(t);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential filter
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, WrappedF64>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, WrappedF64>()
            ensures
                forall |p: (V::V, f64)| out_neighbors@.contains(p) ==
                    (exists |w: f64| #![trigger self@.A.contains((v@, p.0, w))] self@.A.contains((v@, p.0, w)) && p.1 == w);

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) -- sequential filter
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, WrappedF64>>)
            requires spec_labgraphview_wf(self@), valid_key_type_WeightedEdge::<V, WrappedF64>()
            ensures
                forall |p: (V::V, f64)| in_neighbors@.contains(p) ==
                    (exists |w: f64| #![trigger self@.A.contains((p.0, v@, w))] self@.A.contains((p.0, v@, w)) && p.1 == w);
    }

    //		Section 9. impls


    impl<V: StT + Hash> WeightedDirGraphStEphF64Trait<V> for WeightedDirGraphStEphF64<V> {

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|V| + |E|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|V| + |E|), Span O(|V| + |E|) — sequential
        fn from_weighed_edges(vertices: SetStEph<V>, edges: SetStEph<WeightedEdge<V, WrappedF64>>) -> (g: WeightedDirGraphStEphF64<V>) {
            let mut edge_set: SetStEph<LabEdge<V, WrappedF64>> = SetStEph::empty();
            let mut it = edges.iter();
            let ghost edge_seq = it@.1;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, WrappedF64>(),
                    edge_set.spec_setsteph_wf(),
                    it@.0 <= edge_seq.len(),
                    it@.1 == edge_seq,
                    edge_seq.map(|i: int, e: WeightedEdge<V, WrappedF64>| e@).to_set() == edges@,
                    forall |u: V::V, w: V::V, weight: f64|
                        #[trigger] edge_set@.contains((u, w, weight)) ==>
                            vertices@.contains(u) && vertices@.contains(w),
                    forall |t: (V::V, V::V, f64)| #[trigger] edge_set@.contains(t) <==>
                        (exists |j: int| #![trigger edge_seq[j]] 0 <= j < it@.0 && edge_seq[j]@ == t),
                decreases edge_seq.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(triple) => {
                        // Veracity: NEEDED proof block
                        proof {
                            lemma_seq_index_in_map_to_set(edge_seq, it@.0 - 1);
                        }
                        let _ = edge_set.insert(LabEdge(triple.0.clone_plus(), triple.1.clone_plus(), triple.2));
                    },
                }
            }

            // Veracity: NEEDED proof block
            proof {
            }

            LabDirGraphStEph::from_vertices_and_labeled_arcs(vertices, edge_set)
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn add_weighed_edge(&mut self, from: V, to: V, weight: WrappedF64) {
            self.add_labeled_arc(from, to, weight);
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential
        fn get_edge_weight(&self, from: &V, to: &V) -> (weight: Option<WrappedF64>) {
            match self.get_arc_label(from, to) {
                Some(w) => Some(*w),
                None => None,
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential
        fn weighed_edges(&self) -> (weighed_edges: SetStEph<WeightedEdge<V, WrappedF64>>) {
            let mut edges: SetStEph<WeightedEdge<V, WrappedF64>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, WrappedF64>(),
                    edges.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, WrappedF64>| e@).to_set() == wa_view,
                    forall |t: (V::V, V::V, f64)| edges@.contains(t) ==
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@ == t),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
                        }
                        return edges;
                    },
                    Some(labeled_edge) => {
                        let _ = edges.insert(WeightedEdge(labeled_edge.0.clone_plus(), labeled_edge.1.clone_plus(), labeled_edge.2));
                    },
                }
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential
        fn out_neighbors_weighed(&self, v: &V) -> (out_neighbors: SetStEph<Pair<V, WrappedF64>>) {
            let mut neighbors: SetStEph<Pair<V, WrappedF64>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, WrappedF64>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, WrappedF64>| e@).to_set() == wa_view,
                    forall |p: (V::V, f64)| neighbors@.contains(p) ==
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |p: (V::V, f64)| neighbors@.contains(p) implies
                                (exists |w: f64| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.0 == v_view && wa_seq[i]@.1 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |p: (V::V, f64)| (exists |w: f64| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w) implies
                                neighbors@.contains(p) by {
                                if exists |w: f64| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w {
                                    let w = choose |w: f64| #![trigger wa_view.contains((v_view, p.0, w))] wa_view.contains((v_view, p.0, w)) && p.1 == w;
                                    lemma_map_to_set_contains_index(wa_seq, (v_view, p.0, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.0, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.1.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }

        /// - Alg Analysis: APAS (Ch06 Def 6.17): Work O(|A|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O(|A|), Span O(|A|) — sequential
        fn in_neighbors_weighed(&self, v: &V) -> (in_neighbors: SetStEph<Pair<V, WrappedF64>>) {
            let mut neighbors: SetStEph<Pair<V, WrappedF64>> = SetStEph::empty();
            let mut it = self.labeled_arcs().iter();
            let ghost wa_seq = it@.1;
            let ghost v_view = v@;
            let ghost wa_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_WeightedEdge::<V, WrappedF64>(),
                    neighbors.spec_setsteph_wf(),
                    it@.0 <= wa_seq.len(),
                    it@.1 == wa_seq,
                    wa_seq.map(|i: int, e: LabEdge<V, WrappedF64>| e@).to_set() == wa_view,
                    forall |p: (V::V, f64)| neighbors@.contains(p) ==
                        (exists |i: int| #![trigger wa_seq[i]] 0 <= i < it@.0 && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1),
                decreases wa_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        // Veracity: NEEDED proof block
                        proof {
// Veracity: NEEDED assert
assert forall |p: (V::V, f64)| neighbors@.contains(p) implies
                                (exists |w: f64| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) by {
                                if neighbors@.contains(p) {
                                    let i = choose |i: int| #![trigger wa_seq[i]] 0 <= i < wa_seq.len() && wa_seq[i]@.1 == v_view && wa_seq[i]@.0 == p.0 && wa_seq[i]@.2 == p.1;
                                    lemma_seq_index_in_map_to_set(wa_seq, i);
                                }
                            }
// Veracity: NEEDED assert
assert forall |p: (V::V, f64)| (exists |w: f64| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w) implies
                                neighbors@.contains(p) by {
                                if exists |w: f64| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w {
                                    let w = choose |w: f64| #![trigger wa_view.contains((p.0, v_view, w))] wa_view.contains((p.0, v_view, w)) && p.1 == w;
                                    lemma_map_to_set_contains_index(wa_seq, (p.0, v_view, w));
                                }
                            }
                        }
                        return neighbors;
                    },
                    Some(labeled_edge) => {
                        if feq(&labeled_edge.1, v) {
                            let _ = neighbors.insert(Pair(labeled_edge.0.clone_plus(), labeled_edge.2));
                        }
                    },
                }
            }
        }
    }

} // verus!

    //		Section 13. macros


    #[macro_export]
    macro_rules! WeightedDirGraphStEphF64Lit {
        () => {{
            $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( $edge:expr ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let edges = $crate::SetLit![ $( $edge ),* ];
            <$crate::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64<_> as $crate::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64Trait<_>>::from_weighed_edges(vertices, edges)
        }};
    }
}
