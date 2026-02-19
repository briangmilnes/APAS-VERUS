//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled arc filtering (n_plus, n_minus) are parallel.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	12. macros
//	13. derive impls outside verus!

//		1. module


pub mod LabDirGraphMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Concurrency::Concurrency::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::{ParaPair, SetLit};

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::seq_set::*;
    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::*;


    //		3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        crate::Types::Types::group_LabEdge_axioms,
        crate::Chap05::SetStEph::SetStEph::group_set_st_eph_lemmas,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub struct LabDirGraphMtEph<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> {
        pub vertices: SetStEph<V>,
        pub labeled_arcs: SetStEph<LabEdge<V, L>>,
    }


    //		5. view impls

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> View for LabDirGraphMtEph<V, L> {
        type V = LabGraphView<<V as View>::V, <L as View>::V>;
        open spec fn view(&self) -> Self::V {
            LabGraphView { V: self.vertices@, A: self.labeled_arcs@ }
        }
    }


    //		6. spec fns

    pub open spec fn valid_key_type_for_lab_graph<V: StTInMtT + Hash, L: StTInMtT + Hash>() -> bool {
        valid_key_type_LabEdge::<V, L>()
    }


    //		8. traits

    pub trait LabDirGraphMtEphTrait<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> 
        : View<V = LabGraphView<<V as View>::V, <L as View>::V>> + Sized 
    {
        open spec fn spec_vertices(&self) -> Set<V::V> { self@.V }
        open spec fn spec_labeled_arcs(&self) -> Set<(V::V, V::V, L::V)> { self@.A }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn empty() -> (g: Self)
            requires valid_key_type_for_lab_graph::<V, L>()
            ensures
                wf_lab_graph_view(g@),
                g@.V == Set::<<V as View>::V>::empty(),
                g@.A == Set::<(<V as View>::V, <V as View>::V, <L as View>::V)>::empty();

        /// - APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|V| + |A|), Span Θ(1)
        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self)
            requires
                valid_key_type_for_lab_graph::<V, L>(),
                vertices@.finite(),
                labeled_arcs@.finite(),
                forall |u: V::V, w: V::V, l: L::V|
                    #[trigger] labeled_arcs@.contains((u, w, l)) ==> vertices@.contains(u) && vertices@.contains(w),
            ensures
                wf_lab_graph_view(g@),
                g@.V == vertices@,
                g@.A == labeled_arcs@;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> (v: &SetStEph<V>)
            ensures v@ == self@.V;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>)
            ensures a@ == self@.A;

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential map
        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>(), valid_key_type_Edge::<V>()
            ensures forall |u: V::V, w: V::V| arcs@.contains((u, w)) == 
                (exists |l: L::V| #![trigger self@.A.contains((u, w, l))] self@.A.contains((u, w, l)));

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn add_vertex(&mut self, v: V)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures wf_lab_graph_view(self@), self@.V == old(self)@.V.insert(v@), self@.A == old(self)@.A;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L)
            requires wf_lab_graph_view(old(self)@), valid_key_type_for_lab_graph::<V, L>()
            ensures 
                wf_lab_graph_view(self@),
                self@.V == old(self)@.V.insert(from@).insert(to@),
                self@.A == old(self)@.A.insert((from@, to@, label@));

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential search
        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures match label {
                Some(l) => self@.A.contains((from@, to@, l@)),
                None => forall |l: L::V| !self@.A.contains((from@, to@, l)),
            };

        /// - APAS: Work Θ(|A|), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(|A|) — sequential search
        fn has_arc(&self, from: &V, to: &V) -> (b: bool)
            requires wf_lab_graph_view(self@), valid_key_type_for_lab_graph::<V, L>()
            ensures b == (exists |l: L::V| #![trigger self@.A.contains((from@, to@, l))] self@.A.contains((from@, to@, l)));

        open spec fn spec_n_plus(&self, v: V::V) -> Set<V::V>
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        {
            Set::new(|w: V::V| exists |l: L::V| self@.A.contains((v, w, l)))
        }

        open spec fn spec_n_plus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V, L::V)>) -> Set<V::V> 
            recommends 
                wf_lab_graph_view(self@),
                subarcs <= self@.A,
        {
            Set::new(|w: V::V| exists |l: L::V| subarcs.contains((v, w, l)))
        }

        open spec fn spec_n_minus(&self, v: V::V) -> Set<V::V>
            recommends wf_lab_graph_view(self@), self@.V.contains(v)
        {
            Set::new(|u: V::V| exists |l: L::V| self@.A.contains((u, v, l)))
        }

        open spec fn spec_n_minus_from_set(&self, v: V::V, subarcs: Set<(V::V, V::V, L::V)>) -> Set<V::V> 
            recommends 
                wf_lab_graph_view(self@),
                subarcs <= self@.A,
        {
            Set::new(|u: V::V| exists |l: L::V| subarcs.contains((u, v, l)))
        }

        /// out-neighbors
        /// - APAS: Work Θ(|A|), Span Θ(log |A|) — parallel
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(log |A|) — ParaPair! split arcs
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>)
            requires 
                wf_lab_graph_view(self@), 
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures 
                n_plus@ == self.spec_n_plus(v@),
                n_plus@ <= self@.V;

        /// in-neighbors
        /// - APAS: Work Θ(|A|), Span Θ(log |A|) — parallel
        /// - Claude-Opus-4.6: Work Θ(|A|), Span Θ(log |A|) — ParaPair! split arcs
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>)
            requires 
                wf_lab_graph_view(self@), 
                valid_key_type_for_lab_graph::<V, L>(),
                self@.V.contains(v@),
            ensures 
                n_minus@ == self.spec_n_minus(v@),
                n_minus@ <= self@.V;
    }


    //		9. impls

    /// out-neighbors: Parallel arc filtering using set split.
    fn n_plus_par<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
        g: &LabDirGraphMtEph<V, L>, 
        v: V, 
        arcs: SetStEph<LabEdge<V, L>>
    ) -> (n_plus: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type_LabEdge::<V, L>(),
            wf_lab_graph_view(g@),
            arcs@ <= g@.A,
        ensures 
            n_plus@ == g.spec_n_plus_from_set(v@, arcs@),
            n_plus@ <= g.spec_n_plus(v@)
        decreases arcs@.len()
    {
        let n = arcs.size();
        if n == 0 {
            proof {
                assert(g.spec_n_plus_from_set(v@, arcs@) =~= Set::empty());
            }
            SetStEph::empty()
        }
        else if n == 1 {
            let LabEdge(from, to, label) = arcs.choose();
            // arcs@ contains (from@, to@, label@)
            if feq(&from, &v) {
                proof {
                    // from@ == v@ by feq correctness
                    // arcs@ contains (v@, to@, label@)
                    // spec_n_plus_from_set(v@, arcs@) = {w | exists l. arcs@.contains((v@, w, l))}
                    // Since arcs@.len() == 1 and contains (v@, to@, label@), the only w is to@
                    
                    assert forall |w: V::V| #![trigger Set::empty().insert(to@).contains(w)] Set::empty().insert(to@).contains(w) implies 
                        g.spec_n_plus_from_set(v@, arcs@).contains(w) by {
                        assert(arcs@.contains((from@, to@, label@)));
                    }
                    assert forall |w: V::V| #![trigger Set::empty().insert(to@).contains(w)] g.spec_n_plus_from_set(v@, arcs@).contains(w) implies
                        Set::empty().insert(to@).contains(w) by {
                        let l = choose |l: L::V| arcs@.contains((v@, w, l));
                        assert(arcs@.remove((from@, to@, label@)).len() == 0);
                        if (v@, w, l) != (from@, to@, label@) {
                            assert(arcs@.remove((from@, to@, label@)).contains((v@, w, l)));
                        }
                    }
                    assert(Set::empty().insert(to@) =~= g.spec_n_plus_from_set(v@, arcs@));
                }
                SetStEph::singleton(to.clone_plus())
            } else {
                proof {
                    // from@ != v@ by feq correctness
                    assert forall |w: V::V| g.spec_n_plus_from_set(v@, arcs@).contains(w) implies false by {
                        let l = choose |l: L::V| arcs@.contains((v@, w, l));
                        assert(arcs@.remove((from@, to@, label@)).len() == 0);
                        if (v@, w, l) != (from@, to@, label@) {
                            assert(arcs@.remove((from@, to@, label@)).contains((v@, w, l)));
                        }
                        // So (v@, w, l) == (from@, to@, label@), meaning v@ == from@
                        // But from@ != v@, contradiction
                    }
                    assert(g.spec_n_plus_from_set(v@, arcs@) =~= Set::empty());
                }
                SetStEph::empty()
            }
        }
        else {
            let mid = n / 2;
            let (left_arcs, right_arcs) = arcs.split(mid);
            let v_left  = v.clone_plus();
            let v_right = v.clone_plus();
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_plus_from_set(v_left@, left_arcs@)
            { n_plus_par(&g_left, v_left, left_arcs) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_plus_from_set(v_right@, right_arcs@)
            { n_plus_par(&g_right, v_right, right_arcs) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            proof {
                // Prove subset in one direction
                assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] left_neighbors@.union(right_neighbors@).contains(w) implies
                    g.spec_n_plus_from_set(v@, arcs@).contains(w) by {
                    if left_neighbors@.contains(w) {
                        let l = choose |l: L::V| left_arcs@.contains((v@, w, l));
                        assert(arcs@.contains((v@, w, l)));
                    } else {
                        let l = choose |l: L::V| right_arcs@.contains((v@, w, l));
                        assert(arcs@.contains((v@, w, l)));
                    }
                }
                
                // Prove subset in other direction
                assert forall |w: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(w)] g.spec_n_plus_from_set(v@, arcs@).contains(w) implies
                    left_neighbors@.union(right_neighbors@).contains(w) by {
                    let l = choose |l: L::V| arcs@.contains((v@, w, l));
                    if left_arcs@.contains((v@, w, l)) {
                        assert(left_neighbors@.contains(w));
                    } else {
                        assert(right_arcs@.contains((v@, w, l)));
                        assert(right_neighbors@.contains(w));
                    }
                }
                
                assert(left_neighbors@.union(right_neighbors@) =~= g.spec_n_plus_from_set(v@, arcs@));
            }
            
            left_neighbors.union(&right_neighbors)
        }
    }

    /// in-neighbors: Parallel arc filtering using set split.
    fn n_minus_par<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
        g: &LabDirGraphMtEph<V, L>, 
        v: V, 
        arcs: SetStEph<LabEdge<V, L>>
    ) -> (n_minus: SetStEph<V>)
        requires
            valid_key_type::<V>(),
            valid_key_type_LabEdge::<V, L>(),
            wf_lab_graph_view(g@),
            arcs@ <= g@.A,
        ensures 
            n_minus@ == g.spec_n_minus_from_set(v@, arcs@),
            n_minus@ <= g.spec_n_minus(v@)
        decreases arcs@.len()
    {
        let n = arcs.size();
        if n == 0 {
            proof {
                assert(g.spec_n_minus_from_set(v@, arcs@) =~= Set::empty());
            }
            SetStEph::empty()
        }
        else if n == 1 {
            let LabEdge(from, to, label) = arcs.choose();
            // arcs@ contains (from@, to@, label@)
            if feq(&to, &v) {
                proof {
                    // to@ == v@ by feq correctness
                    // arcs@ contains (from@, v@, label@)
                    // spec_n_minus_from_set(v@, arcs@) = {u | exists l. arcs@.contains((u, v@, l))}
                    // Since arcs@.len() == 1 and contains (from@, v@, label@), the only u is from@
                    
                    // Prove: singleton(from)@ == spec_n_minus_from_set(v@, arcs@)
                    assert forall |u: V::V| #![trigger Set::empty().insert(from@).contains(u)] Set::empty().insert(from@).contains(u) implies 
                        g.spec_n_minus_from_set(v@, arcs@).contains(u) by {
                        // u == from@, and arcs contains (from@, v@, label@)
                        assert(arcs@.contains((from@, to@, label@)));
                    }
                    assert forall |u: V::V| #![trigger Set::empty().insert(from@).contains(u)] g.spec_n_minus_from_set(v@, arcs@).contains(u) implies
                        Set::empty().insert(from@).contains(u) by {
                        // exists l such that arcs@ contains (u, v@, l)
                        let l = choose |l: L::V| arcs@.contains((u, v@, l));
                        // arcs@.len() == 1 and arcs@.contains((from@, to@, label@))
                        // arcs@.contains((u, v@, l)) and len==1 implies (u, v@, l) == (from@, to@, label@)
                        // Use set length 1 + contains property
                        assert(arcs@.remove((from@, to@, label@)).len() == 0);
                        if (u, v@, l) != (from@, to@, label@) {
                            assert(arcs@.remove((from@, to@, label@)).contains((u, v@, l)));
                        }
                    }
                    assert(Set::empty().insert(from@) =~= g.spec_n_minus_from_set(v@, arcs@));
                }
                SetStEph::singleton(from.clone_plus())
            } else {
                proof {
                    // to@ != v@ by feq correctness
                    // arcs@ contains (from@, to@, label@)
                    // spec_n_minus_from_set(v@, arcs@) = {u | exists l. arcs@.contains((u, v@, l))}
                    // Since arcs@ has only (from@, to@, label@) and to@ != v@, no u satisfies the condition
                    
                    assert forall |u: V::V| g.spec_n_minus_from_set(v@, arcs@).contains(u) implies false by {
                        let l = choose |l: L::V| arcs@.contains((u, v@, l));
                        // arcs@.len() == 1 and contains (from@, to@, label@)
                        // arcs@.contains((u, v@, l)) and len==1 implies (u, v@, l) == (from@, to@, label@)
                        assert(arcs@.remove((from@, to@, label@)).len() == 0);
                        if (u, v@, l) != (from@, to@, label@) {
                            assert(arcs@.remove((from@, to@, label@)).contains((u, v@, l)));
                        }
                        // So (u, v@, l) == (from@, to@, label@), meaning v@ == to@
                        // But to@ != v@, contradiction
                    }
                    assert(g.spec_n_minus_from_set(v@, arcs@) =~= Set::empty());
                }
                SetStEph::empty()
            }
        }
        else {
            let mid = n / 2;
            let (left_arcs, right_arcs) = arcs.split(mid);
            let v_left  = v.clone_plus();
            let v_right = v.clone_plus();
            let g_left  = g.clone_plus();
            let g_right = g.clone_plus();
            
            let f1 = move || -> (out: SetStEph<V>)
                ensures out@ == g_left.spec_n_minus_from_set(v_left@, left_arcs@)
            { n_minus_par(&g_left, v_left, left_arcs) };
            
            let f2 = move || -> (out: SetStEph<V>)
                ensures out@ == g_right.spec_n_minus_from_set(v_right@, right_arcs@)
            { n_minus_par(&g_right, v_right, right_arcs) };
            
            let Pair(left_neighbors, right_neighbors) = ParaPair!(f1, f2);
            
            proof {
                // Prove: left_neighbors.union(right_neighbors)@ == spec_n_minus_from_set(v@, arcs@)
                // We know: left_arcs@.union(right_arcs@) == arcs@
                // And: left_neighbors@ == spec_n_minus_from_set(v@, left_arcs@)
                //      right_neighbors@ == spec_n_minus_from_set(v@, right_arcs@)
                
                // Prove subset in one direction
                assert forall |u: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(u)] left_neighbors@.union(right_neighbors@).contains(u) implies
                    g.spec_n_minus_from_set(v@, arcs@).contains(u) by {
                    if left_neighbors@.contains(u) {
                        let l = choose |l: L::V| left_arcs@.contains((u, v@, l));
                        assert(arcs@.contains((u, v@, l)));
                    } else {
                        let l = choose |l: L::V| right_arcs@.contains((u, v@, l));
                        assert(arcs@.contains((u, v@, l)));
                    }
                }
                
                // Prove subset in other direction
                assert forall |u: V::V| #![trigger left_neighbors@.union(right_neighbors@).contains(u)] g.spec_n_minus_from_set(v@, arcs@).contains(u) implies
                    left_neighbors@.union(right_neighbors@).contains(u) by {
                    let l = choose |l: L::V| arcs@.contains((u, v@, l));
                    if left_arcs@.contains((u, v@, l)) {
                        assert(left_neighbors@.contains(u));
                    } else {
                        assert(right_arcs@.contains((u, v@, l)));
                        assert(right_neighbors@.contains(u));
                    }
                }
                
                assert(left_neighbors@.union(right_neighbors@) =~= g.spec_n_minus_from_set(v@, arcs@));
            }
            
            left_neighbors.union(&right_neighbors)
        }
    }

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEphTrait<V, L>
        for LabDirGraphMtEph<V, L>
    {
        fn empty() -> (g: Self) {
            LabDirGraphMtEph {
                vertices: SetStEph::empty(),
                labeled_arcs: SetStEph::empty(),
            }
        }

        fn from_vertices_and_labeled_arcs(vertices: SetStEph<V>, labeled_arcs: SetStEph<LabEdge<V, L>>) -> (g: Self) {
            LabDirGraphMtEph { vertices, labeled_arcs }
        }

        fn vertices(&self) -> (v: &SetStEph<V>) { &self.vertices }

        fn labeled_arcs(&self) -> (a: &SetStEph<LabEdge<V, L>>) { &self.labeled_arcs }

        fn arcs(&self) -> (arcs: SetStEph<Edge<V>>) {
            let mut arcs: SetStEph<Edge<V>> = SetStEph::empty();
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    arcs@ == Set::new(|e: (V::V, V::V)| 
                        exists |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |e: (V::V, V::V)| #[trigger] arcs@.contains(e) implies 
                                (exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l))) by {
                                if arcs@.contains(e) {
                                    let i = choose |i: int| #![trigger la_seq[i]] 0 <= i < la_seq.len() && la_seq[i]@.0 == e.0 && la_seq[i]@.1 == e.1;
                                    lemma_seq_index_in_map_to_set(la_seq, i);
                                }
                            }
                            assert forall |e: (V::V, V::V)| 
                                (exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l))) implies 
                                arcs@.contains(e) by {
                                if exists |l: L::V| #![trigger self@.A.contains((e.0, e.1, l))] self@.A.contains((e.0, e.1, l)) {
                                    let l = choose |l: L::V| #![trigger la_view.contains((e.0, e.1, l))] la_view.contains((e.0, e.1, l));
                                    lemma_map_to_set_contains_index(la_seq, (e.0, e.1, l));
                                }
                            }
                        }
                        return arcs;
                    },
                    Some(labeled_arc) => {
                        let _ = arcs.insert(Edge(labeled_arc.0.clone_plus(), labeled_arc.1.clone_plus()));
                    },
                }
            }
        }

        fn add_vertex(&mut self, v: V) { let _ = self.vertices.insert(v); }

        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            let _ = self.vertices.insert(from.clone_plus());
            let _ = self.vertices.insert(to.clone_plus());
            let _ = self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        fn get_arc_label(&self, from: &V, to: &V) -> (label: Option<&L>) {
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return None;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(la_seq, idx);
                            }
                            return Some(&labeled_arc.2);
                        }
                    },
                }
            }
        }

        fn has_arc(&self, from: &V, to: &V) -> (b: bool) {
            let mut it = self.labeled_arcs.iter();
            let ghost la_seq = it@.1;
            let ghost la_view = self@.A;
            let ghost from_view = from@;
            let ghost to_view = to@;

            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            loop
                invariant
                    valid_key_type_LabEdge::<V, L>(),
                    it@.0 <= la_seq.len(),
                    it@.1 == la_seq,
                    la_seq.map(|i: int, e: LabEdge<V, L>| e@).to_set() == la_view,
                    forall |i: int| #![trigger la_seq[i]] 0 <= i < it@.0 ==> !(la_seq[i]@.0 == from_view && la_seq[i]@.1 == to_view),
                decreases la_seq.len() - it@.0,
            {
                match it.next() {
                    None => {
                        proof {
                            assert forall |l: L::V| !la_view.contains((from_view, to_view, l)) by {
                                if la_view.contains((from_view, to_view, l)) {
                                    lemma_map_to_set_contains_index(la_seq, (from_view, to_view, l));
                                }
                            }
                        }
                        return false;
                    },
                    Some(labeled_arc) => {
                        if feq(&labeled_arc.0, from) && feq(&labeled_arc.1, to) {
                            proof {
                                let idx = it@.0 - 1;
                                lemma_seq_index_in_map_to_set(la_seq, idx);
                            }
                            return true;
                        }
                    },
                }
            }
        }

        /// out-neighbors
        fn n_plus(&self, v: &V) -> (n_plus: SetStEph<V>) {
            let arcs = self.labeled_arcs.clone();
            n_plus_par(self, v.clone_plus(), arcs)
        }

        /// in-neighbors
        fn n_minus(&self, v: &V) -> (n_minus: SetStEph<V>) {
            let arcs = self.labeled_arcs.clone();
            n_minus_par(self, v.clone_plus(), arcs)
        }
    }


    //		11. derive impls in verus!

    impl<V: StTInMtT + Hash + 'static, L: StTInMtT + Hash + 'static> Clone for LabDirGraphMtEph<V, L> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            LabDirGraphMtEph { vertices: self.vertices.clone(), labeled_arcs: self.labeled_arcs.clone() }
        }
    }

    } // verus!


    //		13. derive impls outside verus!

    impl<V: StTInMtT + Hash, L: StTInMtT + Hash> Display for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StTInMtT + Hash, L: StTInMtT + Hash> Debug for LabDirGraphMtEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabDirGraph {{ vertices: {:?}, labeled_arcs: {:?} }}",
                self.vertices, self.labeled_arcs
            )
        }
    }


    //		12. macros

    #[macro_export]
    macro_rules! LabDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }
}
