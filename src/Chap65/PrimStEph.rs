//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Prim's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.1: Prim's algorithm for computing Minimum Spanning Trees.
//! Uses priority-first search similar to Dijkstra's algorithm.

pub mod PrimStEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::cmp::Ordering;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::{HashSetWithViewPlus, HashSetWithViewPlusTrait};
    use std::fmt::{Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::Hash;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::SetLit;

    pub type T<V> = PQEntry<V>;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct PrimStEph;

    /// Priority queue entry for Prim's algorithm.
    #[derive(Clone, PartialEq, Eq)]
    pub struct PQEntry<V: StT + Ord + Clone> {
        pub priority: WrappedF64,
        pub vertex: V,
        pub parent: Option<V>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for PQEntry<V> {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    // 8. traits

    pub trait PrimStEphTrait {
        /// Well-formedness for sequential Prim MST algorithm input.
        open spec fn spec_primsteph_wf<V: StT + Hash>(graph: &LabUnDirGraphStEph<V, WrappedF64>) -> bool {
            spec_labgraphview_wf(graph@)
        }

        /// Prim's MST algorithm.
        /// APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
        fn prim_mst<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
            start: V,
        ) -> (result: SetStEph<LabEdge<V, WrappedF64>>)
            requires Self::spec_primsteph_wf(graph),
            ensures result.spec_setsteph_wf();

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, WrappedF64>>) -> WrappedF64
            requires mst.spec_setsteph_wf();
    }

    /// Module-level function to create a new PQEntry.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    fn pq_entry_new<V: HashOrd>(priority: WrappedF64, vertex: V, parent: Option<V>) -> (entry: PQEntry<V>)
        ensures entry.priority == priority, entry.vertex == vertex, entry.parent == parent,
    {
        PQEntry {
            priority,
            vertex,
            parent,
        }
    }

    /// Algorithm 65.1: Prim's MST Algorithm
    ///
    /// Computes the Minimum Spanning Tree using priority-first search.
    /// Similar to Dijkstra's, but priority is minimum edge weight to visited set X.
    ///
    /// Priority: p(v) = min_{x in X} w(x,v)
    ///
    /// - APAS: Work O(m lg n), Span O(m lg n)
    /// - Claude-Opus-4.6: Work O(m^2 lg n), Span O(m^2 lg n) — the APAS bound assumes
    ///   O(degree) adjacency-list lookups, but LabUnDirGraphStEph stores edges in a flat
    ///   set, so ng() and get_edge_label() each cost O(m) per call. Total neighbor/weight
    ///   work across all vertices is O(nm) = O(m^2) in a dense graph. With an adjacency-list
    ///   graph representation this would be O(m lg n) as textbook states.
    #[verifier::external_body]
    pub fn prim_mst<V: HashOrd + Display>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
        start: &V,
    ) -> (result: SetStEph<LabEdge<V, WrappedF64>>)
        requires
            spec_labgraphview_wf(graph@),
            obeys_key_model::<V>(),
            valid_key_type_LabEdge::<V, WrappedF64>(),
        ensures
            result.spec_setsteph_wf(),
    {
        let mut mst_edges = SetLit![];
        let mut visited = HashSetWithViewPlus::<V>::new();

        let mut pq = BinaryHeapPQ::<PQEntry<V>>::singleton(pq_entry_new(zero_dist(), start.clone(), None));

        while !pq.is_empty() {
            let (new_pq, entry_opt) = pq.delete_min();
            pq = new_pq;

            let entry = match entry_opt {
                | Some(e) => e,
                | None => break,
            };

            let u = entry.vertex;
            let parent_u = entry.parent;

            if visited.contains(&u) {
                continue;
            }

            let _ = visited.insert(u.clone());

            if let Some(parent_v) = parent_u {
                if let Some(weight) = graph.get_edge_label(&parent_v, &u) {
                    let edge = if parent_v < u {
                        LabEdge(parent_v, u.clone(), *weight)
                    } else {
                        LabEdge(u.clone(), parent_v, *weight)
                    };
                    let _ = mst_edges.insert(edge);
                }
            }

            let neighbors = graph.ng(&u);
            for v in neighbors.iter() {
                if !visited.contains(v) {
                    if let Some(weight) = graph.get_edge_label(&u, v) {
                        pq = pq.insert(pq_entry_new(*weight, v.clone(), Some(u.clone())));
                    }
                }
            }
        }

        mst_edges
    }

    /// Compute total MST weight.
    /// - APAS: (no cost stated) — utility function
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, WrappedF64>>) -> (total: WrappedF64)
        requires mst_edges.spec_setsteph_wf(),
        ensures mst_edges@.len() == 0 ==> total@ == 0.0f64,
    {
        if mst_edges.size() == 0 {
            return WrappedF64 { val: 0.0 };
        }
        let mut total = WrappedF64 { val: 0.0 };
        let mut it = mst_edges.iter();
        let ghost le_seq = it@.1;
        loop
            invariant
                it@.0 <= le_seq.len(),
                it@.1 == le_seq,
            decreases le_seq.len() - it@.0,
        {
            match it.next() {
                None => return total,
                Some(edge) => {
                    total = WrappedF64 { val: total.val + edge.2.val };
                },
            }
        }
    }

    } // verus!

    impl<V: HashOrd> Ord for PQEntry<V> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn cmp(&self, other: &Self) -> Ordering { self.priority.cmp(&other.priority) }
    }

    impl<V: HashOrd> PartialOrd for PQEntry<V> {
        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
    }

    impl<V: HashOrd + Display> Display for PQEntry<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.priority, self.vertex) }
    }

    impl<V: HashOrd + std::fmt::Debug> std::fmt::Debug for PQEntry<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PQEntry")
                .field("priority", &self.priority.val)
                .field("vertex", &self.vertex)
                .field("parent", &self.parent)
                .finish()
        }
    }
}
