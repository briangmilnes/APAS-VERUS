//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Common types used across the crate.

pub mod Types {

    use std::fmt::{Formatter, Debug, Display};
    use std::hash::Hash;
    use vstd::prelude::*;


    // Note: bool already implements Display, Debug, Not, etc.

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    /// Graph view struct: vertices and arcs/edges as spec sets.
    #[verifier::reject_recursive_types(V)]
    pub ghost struct GraphView<V> {
        pub V: Set<V>,
        pub A: Set<(V, V)>,
    }

    /// Well-formedness for unlabeled graph views: finite sets and arc endpoints in V.
    pub open spec fn spec_graphview_wf<V>(gv: GraphView<V>) -> bool {
        &&& gv.V.finite()
        &&& gv.A.finite()
        &&& forall |u: V, w: V|
                #[trigger] gv.A.contains((u, w)) ==>
                    gv.V.contains(u) && gv.V.contains(w)
    }

    /// Labeled graph view struct: vertices and labeled arcs/edges.
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub ghost struct LabGraphView<V, L> {
        pub V: Set<V>,
        pub A: Set<(V, V, L)>,
    }

    /// Well-formedness for labeled graph views: finite sets and arc endpoints in V.
    pub open spec fn spec_labgraphview_wf<V, L>(gv: LabGraphView<V, L>) -> bool {
        &&& gv.V.finite()
        &&& gv.A.finite()
        &&& forall |u: V, w: V, l: L|
                #[trigger] gv.A.contains((u, w, l)) ==>
                    gv.V.contains(u) && gv.V.contains(w)
    }

    /// Single-threaded friendly elements: Eq + Clone + Display + Debug + Sized + View.
    pub trait StT: Eq + PartialEq + Clone + Display + Debug + Sized + vstd::prelude::View {}
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized + vstd::prelude::View {}

    /// Single-threaded predicate function (boolean function).
    pub trait PredSt<T>: Fn(&T) -> bool {}
    impl<F, T> PredSt<T> for F where F: Fn(&T) -> bool {}

    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

    /// Type that can be hashed and ordered (for graph vertices).
    pub trait HashOrd: StT + Hash + Ord + ClonePreservesView {}
    impl<T> HashOrd for T where T: StT + Hash + Ord + ClonePreservesView {}

    /// Edge wrapper to enable Display/Debug for pairs (V,V) under baseline bounds.
    #[verifier::reject_recursive_types(V)]
    #[derive(Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Edge<V: StT>(pub V, pub V);

    /// Labeled Edge wrapper to enable edges with labels.
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    #[derive(Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LabEdge<V: StT, L: StT + Hash>(pub V, pub V, pub L);

    /// - Weighted Edge wrapper to enable edges with weights.
    /// - Structurally identical to LabEdge but semantically distinct.
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(W)]
    #[derive(Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeightedEdge<V: StT, W: StT + Hash>(pub V, pub V, pub W);

    /// Newtype wrapper for key-value pairs with better Display than tuples
    #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pair<K, V>(pub K, pub V);

    impl<K: vstd::prelude::View, V: vstd::prelude::View> vstd::prelude::View for Pair<K, V> {
        type V = (K::V, V::V);

        open spec fn view(&self) -> (K::V, V::V) {(self.0@, self.1@)}
    }

    impl<V: StT> vstd::prelude::View for Edge<V> {
        type V = (V::V, V::V);

        open spec fn view(&self) -> (V::V, V::V) {(self.0@, self.1@)}
    }

    impl<V: StT, L: StT + Hash> vstd::prelude::View for LabEdge<V, L> {
        type V = (V::V, V::V, L::V);

        open spec fn view(&self) -> (V::V, V::V, L::V) {(self.0@, self.1@, self.2@)}
    }

    impl<V: StT, W: StT + Hash> vstd::prelude::View for WeightedEdge<V, W> {
        type V = (V::V, V::V, W::V);

        open spec fn view(&self) -> (V::V, V::V, W::V) {(self.0@, self.1@, self.2@)}
    }

    /// - Axiom that Pair's view is injective (needed for hash collections)
    /// - If two pairs have the same view, they are equal
    pub broadcast proof fn axiom_Pair_view_injective<K: vstd::prelude::View, V: vstd::prelude::View>(p1: Pair<K, V>, p2: Pair<K, V>)
        requires
            #[trigger] p1@ == #[trigger] p2@,
        ensures
            p1 == p2,
    {
        admit();
    }

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    pub open spec fn Pair_feq_trigger<K, V>() -> bool { true }

    pub broadcast proof fn axiom_Pair_feq<K: Eq + vstd::prelude::View + Clone + Sized, V: Eq + vstd::prelude::View + Clone + Sized>()
        requires #[trigger] Pair_feq_trigger::<K, V>()
        ensures obeys_feq_full::<Pair<K, V>>()
    { admit(); }

    pub broadcast proof fn axiom_Pair_key_model<K: Eq + vstd::prelude::View + Clone + Sized + Hash, V: Eq + vstd::prelude::View + Clone + Sized + Hash>()
        requires #[trigger] Pair_feq_trigger::<K, V>()
        ensures obeys_key_model::<Pair<K, V>>()
    { admit(); }

    pub broadcast group group_Pair_axioms {
        axiom_Pair_view_injective,
        axiom_Pair_feq,
        axiom_Pair_key_model,
    }

    /// For Verus wrapped hash tables we need obeys_key_model and for full equality we need obeys_feq_full.
    pub open spec fn valid_key_type_Pair<K: Eq + View + Clone + Sized + Hash, V: Eq + View + Clone + Sized + Hash>() -> bool {
        &&& obeys_key_model::<K>() && obeys_key_model::<V>() && obeys_key_model::<Pair<K, V>>()
        &&& obeys_feq_full::<K>() && obeys_feq_full::<V>() && obeys_feq_full::<Pair<K, V>>()
    }

    pub open spec fn obeys_feq_full_Pair<K: Eq + View + Clone + Sized, V: Eq + View + Clone + Sized>() -> bool {
        obeys_feq_full::<K>() && obeys_feq_full::<V>() && obeys_feq_full::<Pair<K, V>>()
    }

    pub open spec fn Edge_feq_trigger<V>() -> bool { true }

    pub broadcast proof fn axiom_Edge_feq<V: StT>()
        requires #[trigger] Edge_feq_trigger::<V>()
        ensures obeys_feq_full::<Edge<V>>()
    { admit(); }

    pub broadcast proof fn axiom_Edge_key_model<V: StT + Hash>()
        requires #[trigger] Edge_feq_trigger::<V>()
        ensures obeys_key_model::<Edge<V>>()
    { admit(); }

    pub broadcast group group_Edge_axioms {
        axiom_Edge_feq,
        axiom_Edge_key_model,
    }

    /// For Verus wrapped hash tables we need obeys_key_model and for full equality we need obeys_feq_full.
    pub open spec fn valid_key_type_Edge<V: StT + Hash>() -> bool {
        &&& obeys_key_model::<V>() && obeys_key_model::<Edge<V>>()
        &&& obeys_feq_full::<V>() && obeys_feq_full::<Edge<V>>()
    }

    pub open spec fn obeys_feq_full_Edge<V: StT>() -> bool {
        obeys_feq_full::<V>() && obeys_feq_full::<Edge<V>>()
    }

    pub open spec fn LabEdge_feq_trigger<V: StT + Hash, L: StT + Hash>() -> bool { true }

    pub broadcast proof fn axiom_LabEdge_feq<V: StT + Hash, L: StT + Hash>()
        requires #[trigger] LabEdge_feq_trigger::<V, L>()
        ensures obeys_feq_full::<LabEdge<V, L>>()
    { admit(); }

    pub broadcast proof fn axiom_LabEdge_key_model<V: StT + Hash, L: StT + Hash>()
        requires #[trigger] LabEdge_feq_trigger::<V, L>()
        ensures obeys_key_model::<LabEdge<V, L>>()
    { admit(); }

    pub broadcast group group_LabEdge_axioms {
        axiom_LabEdge_feq,
        axiom_LabEdge_key_model,
    }

    pub open spec fn valid_key_type_LabEdge<V: StT + Hash, L: StT + Hash>() -> bool {
        &&& obeys_key_model::<V>() && obeys_key_model::<L>() && obeys_key_model::<LabEdge<V, L>>()
        &&& obeys_feq_full::<V>() && obeys_feq_full::<L>() && obeys_feq_full::<LabEdge<V, L>>()
    }

    pub open spec fn WeightedEdge_feq_trigger<V: StT + Hash, W: StT + Hash>() -> bool { true }

    pub broadcast proof fn axiom_WeightedEdge_feq<V: StT + Hash, W: StT + Hash>()
        requires #[trigger] WeightedEdge_feq_trigger::<V, W>()
        ensures obeys_feq_full::<WeightedEdge<V, W>>()
    { admit(); }

    pub broadcast proof fn axiom_WeightedEdge_key_model<V: StT + Hash, W: StT + Hash>()
        requires #[trigger] WeightedEdge_feq_trigger::<V, W>()
        ensures obeys_key_model::<WeightedEdge<V, W>>()
    { admit(); }

    pub broadcast group group_WeightedEdge_axioms {
        axiom_WeightedEdge_feq,
        axiom_WeightedEdge_key_model,
    }

    pub open spec fn valid_key_type_WeightedEdge<V: StT + Hash, W: StT + Hash>() -> bool {
        &&& obeys_key_model::<V>() && obeys_key_model::<W>() && obeys_key_model::<WeightedEdge<V, W>>()
        &&& obeys_feq_full::<V>() && obeys_feq_full::<W>() && obeys_feq_full::<WeightedEdge<V, W>>()
        // Also require LabEdge since WeightedGraph implementations use LabDirGraph internally
        &&& obeys_key_model::<LabEdge<V, W>>() && obeys_feq_full::<LabEdge<V, W>>()
        // Also require Pair for neighbor results
        &&& obeys_key_model::<Pair<V, W>>() && obeys_feq_full::<Pair<V, W>>()
    }

    //      12. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash + ClonePreservesView> ClonePreservesView for Edge<V> {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            Edge(self.0.clone_view(), self.1.clone_view())
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash + ClonePreservesView, L: StT + Hash + ClonePreservesView> ClonePreservesView for LabEdge<V, L> {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            LabEdge(self.0.clone_view(), self.1.clone_view(), self.2.clone_view())
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<V: StT + Hash + ClonePreservesView, W: StT + Hash + ClonePreservesView> ClonePreservesView for WeightedEdge<V, W> {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            WeightedEdge(self.0.clone_view(), self.1.clone_view(), self.2.clone_view())
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<K: vstd::prelude::View + Clone + ClonePreservesView, V: vstd::prelude::View + Clone + ClonePreservesView> ClonePreservesView for Pair<K, V> {
        fn clone_view(&self) -> (result: Self)
            ensures result@ == self@,
        {
            Pair(self.0.clone_view(), self.1.clone_view())
        }
    }

    } // verus!

    // Re-export MT traits from Concurrency (canonical definitions)
    pub use crate::Concurrency::Concurrency::{
        StTInMtT, MtT, MtKey, MtVal, MtReduceFn, Pred,
    };


    impl<V: StT> Display for Edge<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.0, self.1) }
    }

    impl<V: StT> From<(V, V)> for Edge<V> {
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }
    }

    impl<V: StT> From<Edge<V>> for (V, V) {
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }
    }

    impl<V: StT, L: StT + Hash> Display for LabEdge<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.0, self.1, self.2) }
    }

    impl<V: StT, L: StT + Hash> From<(V, V, L)> for LabEdge<V, L> {
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }
    }

    impl<V: StT, L: StT + Hash> From<LabEdge<V, L>> for (V, V, L) {
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }
    }

    impl<V: StT, W: StT + Hash> Display for WeightedEdge<V, W> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.0, self.1, self.2) }
    }

    impl<V: StT, W: StT + Hash> From<(V, V, W)> for WeightedEdge<V, W> {
        fn from(t: (V, V, W)) -> Self { WeightedEdge(t.0, t.1, t.2) }
    }

    impl<V: StT, W: StT + Hash> From<WeightedEdge<V, W>> for (V, V, W) {
        fn from(e: WeightedEdge<V, W>) -> (V, V, W) { (e.0, e.1, e.2) }
    }


    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }
    }

    impl<A, B> From<Pair<A, B>> for (A, B) {
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }
    }

    impl<V: StT> Clone for Edge<V> {
        fn clone(&self) -> Self {
            Edge(self.0.clone(), self.1.clone())
        }
    }

    impl<V: StT, L: StT + Hash> Clone for LabEdge<V, L> {
        fn clone(&self) -> Self {
            LabEdge(self.0.clone(), self.1.clone(), self.2.clone())
        }
    }

    impl<V: StT, W: StT + Hash> Clone for WeightedEdge<V, W> {
        fn clone(&self) -> Self {
            WeightedEdge(self.0.clone(), self.1.clone(), self.2.clone())
        }
    }

    impl<K: Clone, V: Clone> Clone for Pair<K, V> {
        fn clone(&self) -> Self {
            Pair(self.0.clone(), self.1.clone())
        }
    }

    // Display implementation for Pair (outside verus! block)
    impl<K: Display, V: Display> Display for Pair<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({} -> {})", self.0, self.1)
        }
    }

    // Macros are defined outside verus! blocks to allow importing via `use crate::MacroName;` from other modules.
    #[macro_export]
    macro_rules! PairLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Pair($a, $b)
        };
    }
}
