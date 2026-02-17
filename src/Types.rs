//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Common types used across the crate.

pub mod Types {

    use std::fmt::{Formatter, Debug, Display};
    use std::hash::Hash;
    use std::ops::Add;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::SetIterAdditionalSpecFns;

    pub type N = usize;

    /// - Data Type 18.1 (Boolean) type used by APAS.
    /// - But we have converted to using Rust's built in base bool type.
    pub type B = bool;

    /// - Data Type 18.1 (Ordering) relationships used by APAS, using Rust's as it matches.
    /// - Enumerated values in `std::cmp::Ordering` are named: Less, Equal, Greater.
    pub use std::cmp::Ordering as O;

    // Note: bool already implements Display, Debug, Not, etc.
    // No custom implementations needed when B = bool

    verus! {

    /// Graph view struct: vertices and arcs/edges as spec sets.
    #[verifier::reject_recursive_types(V)]
    pub ghost struct GraphView<V> {
        pub V: Set<V>,
        pub A: Set<(V, V)>,
    }

    /// Well-formedness for unlabeled graph views: finite sets and arc endpoints in V.
    pub open spec fn wf_graph_view<V>(gv: GraphView<V>) -> bool {
        &&& gv.V.finite()
        &&& gv.A.finite()
        &&& forall |u: V, w: V| 
                #[trigger] gv.A.contains((u, w)) ==> 
                    gv.V.contains(u) && gv.V.contains(w)
    }
    /// Well-formedness is preserved when taking a subset of arcs.
    pub proof fn lemma_wf_graph_view_subset_arcs<V>(gv: GraphView<V>, arcs_subset: Set<(V, V)>)
        requires
            wf_graph_view(gv),
            arcs_subset <= gv.A,
        ensures
            wf_graph_view(GraphView { V: gv.V, A: arcs_subset }),
    {
    }

    /// Labeled graph view struct: vertices and labeled arcs/edges.
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    pub ghost struct LabGraphView<V, L> {
        pub V: Set<V>,
        pub A: Set<(V, V, L)>,
    }

    /// Well-formedness for labeled graph views: finite sets and arc endpoints in V.
    pub open spec fn wf_lab_graph_view<V, L>(gv: LabGraphView<V, L>) -> bool {
        &&& gv.V.finite()
        &&& gv.A.finite()
        &&& forall |u: V, w: V, l: L| 
                #[trigger] gv.A.contains((u, w, l)) ==> 
                    gv.V.contains(u) && gv.V.contains(w)
    }

    /// Well-formedness is preserved when taking a subset of arcs.
    pub proof fn lemma_wf_lab_graph_view_subset_arcs<V,L>(gv: LabGraphView<V,L>, arcs_subset: Set<(V, V, L)>)
        requires
            wf_lab_graph_view(gv),
            arcs_subset <= gv.A,
        ensures
            wf_lab_graph_view(LabGraphView { V: gv.V, A: arcs_subset }),
    {
    }

    /// Triple wrapper for three-element tuples.
    #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Triple<A, B, C>(pub A, pub B, pub C);

    /// Quadruple wrapper for four-element tuples.
    #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Quadruple<A, B, C, D>(pub A, pub B, pub C, pub D);

    /// Key-value struct with named fields.
    #[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct KeyVal<K, V> {
        pub key: K,
        pub val: V,
    }

    /// Single-threaded friendly elements: Eq + Clone + Display + Debug + Sized + View.
    pub trait StT: Eq + Clone + Display + Debug + Sized + vstd::prelude::View {}
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized + vstd::prelude::View {}

    /// Single-threaded predicate function (boolean function).
    pub trait PredSt<T>: Fn(&T) -> B {}
    impl<F, T> PredSt<T> for F where F: Fn(&T) -> B {}

    /// Type that can be hashed and ordered (for graph vertices).
    pub trait HashOrd: StT + Hash + Ord {}
    impl<T> HashOrd for T where T: StT + Hash + Ord {}

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

    /// - Weighted Labelled Edge wrapper for edges with both a label and a weight.
    /// - This is a quadruple: (from, to, label, weight).
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(L)]
    #[verifier::reject_recursive_types(W)]
    #[derive(Copy, PartialEq, Eq, Hash, Debug)]
    pub struct WeightedLabEdge<V: StT, L: StT + Hash, W: StT + Hash>(pub V, pub V, pub L, pub W);

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

    impl<V: StT, L: StT + Hash, W: StT + Hash> vstd::prelude::View for WeightedLabEdge<V, L, W> {
        type V = (V::V, V::V, L::V, W::V);

        open spec fn view(&self) -> (V::V, V::V, L::V, W::V) {(self.0@, self.1@, self.2@, self.3@)}
    }

    impl<A: vstd::prelude::View, B: vstd::prelude::View, C: vstd::prelude::View> vstd::prelude::View for Triple<A, B, C> {
        type V = (A::V, B::V, C::V);

        open spec fn view(&self) -> (A::V, B::V, C::V) {(self.0@, self.1@, self.2@)}
    }

    impl<A: vstd::prelude::View, B: vstd::prelude::View, C: vstd::prelude::View, D: vstd::prelude::View> vstd::prelude::View for Quadruple<A, B, C, D> {
        type V = (A::V, B::V, C::V, D::V);

        open spec fn view(&self) -> (A::V, B::V, C::V, D::V) {(self.0@, self.1@, self.2@, self.3@)}
    }

    impl<K: vstd::prelude::View, V: vstd::prelude::View> vstd::prelude::View for KeyVal<K, V> {
        type V = (K::V, V::V);

        open spec fn view(&self) -> (K::V, V::V) {(self.key@, self.val@)}
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

    pub broadcast group group_Pair_axioms {
        axiom_Pair_view_injective,
        axiom_Pair_feq,
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
        // Also require Triple and Pair for edge collections and neighbor results
        &&& obeys_key_model::<Triple<V, V, W>>() && obeys_feq_full::<Triple<V, V, W>>()
        &&& obeys_key_model::<Pair<V, W>>() && obeys_feq_full::<Pair<V, W>>()
    }

    pub open spec fn WeightedLabEdge_feq_trigger<V: StT + Hash, L: StT + Hash, W: StT + Hash>() -> bool { true }

    pub broadcast proof fn axiom_WeightedLabEdge_feq<V: StT + Hash, L: StT + Hash, W: StT + Hash>()
        requires #[trigger] WeightedLabEdge_feq_trigger::<V, L, W>()
        ensures obeys_feq_full::<WeightedLabEdge<V, L, W>>()
    { admit(); }

    pub broadcast proof fn axiom_WeightedLabEdge_key_model<V: StT + Hash, L: StT + Hash, W: StT + Hash>()
        requires #[trigger] WeightedLabEdge_feq_trigger::<V, L, W>()
        ensures obeys_key_model::<WeightedLabEdge<V, L, W>>()
    { admit(); }

    pub broadcast group group_WeightedLabEdge_axioms {
        axiom_WeightedLabEdge_feq,
        axiom_WeightedLabEdge_key_model,
    }

    pub open spec fn valid_key_type_WeightedLabEdge<V: StT + Hash, L: StT + Hash, W: StT + Hash>() -> bool {
        &&& obeys_key_model::<V>() && obeys_key_model::<L>() && obeys_key_model::<W>() && obeys_key_model::<WeightedLabEdge<V, L, W>>()
        &&& obeys_feq_full::<V>() && obeys_feq_full::<L>() && obeys_feq_full::<W>() && obeys_feq_full::<WeightedLabEdge<V, L, W>>()
    }

    pub open spec fn Triple_feq_trigger<A: StT + Hash, B: StT + Hash, C: StT + Hash>() -> bool { true }

    pub broadcast proof fn axiom_Triple_feq<A: StT + Hash, B: StT + Hash, C: StT + Hash>()
        requires #[trigger] Triple_feq_trigger::<A, B, C>()
        ensures obeys_feq_full::<Triple<A, B, C>>()
    { admit(); }

    pub broadcast proof fn axiom_Triple_key_model<A: StT + Hash, B: StT + Hash, C: StT + Hash>()
        requires #[trigger] Triple_feq_trigger::<A, B, C>()
        ensures obeys_key_model::<Triple<A, B, C>>()
    { admit(); }

    pub broadcast group group_Triple_axioms {
        axiom_Triple_feq,
        axiom_Triple_key_model,
    }

    pub open spec fn valid_key_type_Triple<A: StT + Hash, B: StT + Hash, C: StT + Hash>() -> bool {
        &&& obeys_key_model::<A>() && obeys_key_model::<B>() && obeys_key_model::<C>() && obeys_key_model::<Triple<A, B, C>>()
        &&& obeys_feq_full::<A>() && obeys_feq_full::<B>() && obeys_feq_full::<C>() && obeys_feq_full::<Triple<A, B, C>>()
    }

    /// - Newtype wrapper for Pair iterator to implement ForLoopGhostIterator (orphan rule).
    /// - Currently unused due to Verus limitation - for loops don't recognize ForLoopGhostIteratorNew
    ///   on newtype wrappers. Kept for future use when this is supported.
    pub struct PairIter<'a, K: 'a, V: 'a>(pub std::collections::hash_set::Iter<'a, Pair<K, V>>);

    /// Ghost iterator for iterating over Pair<K, V> in hash sets.
    pub struct PairIterGhostIterator<'a, K, V> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
        pub phantom: core::marker::PhantomData<&'a ()>,
    }

    impl<'a, K: 'a, V: 'a> vstd::pervasive::ForLoopGhostIteratorNew for PairIter<'a, K, V> {
        type GhostIter = PairIterGhostIterator<'a, K, V>;

        open spec fn ghost_iter(&self) -> PairIterGhostIterator<'a, K, V> {
            PairIterGhostIterator { pos: self.0@.0, elements: self.0@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, K: 'a, V: 'a> vstd::pervasive::ForLoopGhostIterator for PairIterGhostIterator<'a, K, V> {
        type ExecIter = PairIter<'a, K, V>;

        type Item = &'a Pair<K, V>;

        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &PairIter<'a, K, V>) -> bool {
            &&& self.pos == exec_iter.0@.0
            &&& self.elements == exec_iter.0@.1
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

        open spec fn ghost_peek_next(&self) -> Option<&'a Pair<K, V>> {
            if 0 <= self.pos < self.elements.len() {
                Some(&self.elements[self.pos as int])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &PairIter<'a, K, V>) -> PairIterGhostIterator<'a, K, V> {
            PairIterGhostIterator { pos: self.pos + 1, ..*self }
        }
    }

    } // verus!

    // View impl for OrderedFloat removed — orphan rule prevents impl for external type.
    // Chapters using OrderedFloat (56–66) remain gated when they have too many errors.

    /// Type supporting arithmetic operations (for reductions). Must be outside verus! block because Default is not supported.
    pub trait ArithmeticT: StT + Add<Output = Self> + Default + Copy {}
    impl<T> ArithmeticT for T where T: StT + Add<Output = T> + Default + Copy {}

    // Re-export MT traits from Concurrency (canonical definitions)
    pub use crate::Concurrency::Concurrency::{
        StTInMtT, MtT, MtKey, MtVal, MtFn, MtFnClone, MtReduceFn, PredMt, Pred, PredVal,
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

    impl<V: StT, L: StT + Hash, W: StT + Hash> Display for WeightedLabEdge<V, L, W> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3) }
    }

    impl<V: StT, L: StT + Hash, W: StT + Hash> From<(V, V, L, W)> for WeightedLabEdge<V, L, W> {
        fn from(t: (V, V, L, W)) -> Self { WeightedLabEdge(t.0, t.1, t.2, t.3) }
    }

    impl<V: StT, L: StT + Hash, W: StT + Hash> From<WeightedLabEdge<V, L, W>> for (V, V, L, W) {
        fn from(e: WeightedLabEdge<V, L, W>) -> (V, V, L, W) { (e.0, e.1, e.2, e.3) }
    }

    // Import OrderedFloat from the ordered-float crate (not available during Verus compilation)
    #[cfg(not(verus_keep_ghost))]
    pub use ordered_float::OrderedFloat;
    #[cfg(not(verus_keep_ghost))]
    pub type OrderedF32 = OrderedFloat<f32>;
    #[cfg(not(verus_keep_ghost))]
    pub type OrderedF64 = OrderedFloat<f64>;


    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }
    }

    impl<A, B> From<Pair<A, B>> for (A, B) {
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }
    }

    impl<A: Display, B: Display, C: Display> Display for Triple<A, B, C> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.0, self.1, self.2) }
    }
    impl<A: Display, B: Display, C: Display, D: Display> Display for Quadruple<A, B, C, D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }
    impl<K: Display, V: Display> Display for KeyVal<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{key: {}, val: {}}}", self.key, self.val)
        }
    }

    // Implement Iterator for PairIter to enable for loops (must be outside verus! block)
    impl<'a, K: 'a, V: 'a> Iterator for PairIter<'a, K, V> {
        type Item = &'a Pair<K, V>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    // Implement Deref for easier access
    impl<'a, K: 'a, V: 'a> std::ops::Deref for PairIter<'a, K, V> {
        type Target = std::collections::hash_set::Iter<'a, Pair<K, V>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a, K: 'a, V: 'a> std::ops::DerefMut for PairIter<'a, K, V> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // Clone implementations (outside verus! block to avoid Verus autoderive warnings)
    impl<A: Clone, B: Clone, C: Clone> Clone for Triple<A, B, C> {
        fn clone(&self) -> Self {
            Triple(self.0.clone(), self.1.clone(), self.2.clone())
        }
    }

    impl<A: Clone, B: Clone, C: Clone, D: Clone> Clone for Quadruple<A, B, C, D> {
        fn clone(&self) -> Self {
            Quadruple(self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone())
        }
    }

    impl<K: Clone, V: Clone> Clone for KeyVal<K, V> {
        fn clone(&self) -> Self {
            KeyVal { key: self.key.clone(), val: self.val.clone() }
        }
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

    impl<V: StT, L: StT + Hash, W: StT + Hash> Clone for WeightedLabEdge<V, L, W> {
        fn clone(&self) -> Self {
            WeightedLabEdge(self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone())
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
    macro_rules! EdgeLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Edge($a, $b)
        };
    }

    #[macro_export]
    macro_rules! PairLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Pair($a, $b)
        };
    }

    #[macro_export]
    macro_rules! EdgeList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::EdgeLit!($a, $b) ),* ]
        };
    }

    #[macro_export]
    macro_rules! PairList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::PairLit!($a, $b) ),* ]
        };
    }
}
