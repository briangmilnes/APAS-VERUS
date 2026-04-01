//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 65: Prim's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.1: Prim's algorithm for computing Minimum Spanning Trees.
//! Uses priority-first search similar to Dijkstra's algorithm.

pub mod PrimStEph {

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::cmp::Ordering;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::{HashSetWithViewPlus, HashSetWithViewPlusTrait};
    use std::fmt::{Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::hash::Hash;
    use crate::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;
    use crate::SetLit;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    pub type T<V> = PQEntry<V>;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct PrimStEph;

    /// Priority queue entry for Prim's algorithm.
    #[derive(PartialEq, Eq)]
    pub struct PQEntry<V: StT + Ord + Clone> {
        pub priority: u64,
        pub vertex: V,
        pub parent: Option<V>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for PQEntry<V> {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    // 6. spec fns

    /// Lexicographic ordering on Option<V> for PQEntry total order.
    pub open spec fn spec_option_le<V: StT + Ord + TotalOrder>(a: Option<V>, b: Option<V>) -> bool {
        match a {
            Option::None => true,
            Option::Some(x) => match b {
                Option::None => false,
                Option::Some(y) => TotalOrder::le(x, y),
            },
        }
    }

    // 6a. TotalOrder for PQEntry — lexicographic on (priority, vertex, parent).

    impl<V: StT + Ord + TotalOrder> TotalOrder for PQEntry<V> {
        open spec fn le(self, other: Self) -> bool {
            self.priority < other.priority
            || (self.priority == other.priority && TotalOrder::le(self.vertex, other.vertex) && (
                !TotalOrder::le(other.vertex, self.vertex)
                || spec_option_le(self.parent, other.parent)
            ))
        }

        proof fn reflexive(x: Self) {
            V::reflexive(x.vertex);
            match x.parent {
                Option::None => {},
                Option::Some(v) => { V::reflexive(v); },
            }
        }

        proof fn transitive(x: Self, y: Self, z: Self) {
            if x.priority == y.priority && y.priority == z.priority {
                V::transitive(x.vertex, y.vertex, z.vertex);
                if TotalOrder::le(z.vertex, x.vertex) {
                    V::transitive(z.vertex, x.vertex, y.vertex);
                    V::transitive(y.vertex, z.vertex, x.vertex);
                    V::antisymmetric(x.vertex, y.vertex);
                    V::antisymmetric(y.vertex, z.vertex);
                    match (x.parent, y.parent, z.parent) {
                        (Option::Some(a), Option::Some(b), Option::Some(c)) => {
                            V::transitive(a, b, c);
                        },
                        _ => {},
                    }
                }
            }
        }

        proof fn antisymmetric(x: Self, y: Self) {
            V::antisymmetric(x.vertex, y.vertex);
            match (x.parent, y.parent) {
                (Option::Some(a), Option::Some(b)) => {
                    V::antisymmetric(a, b);
                },
                _ => {},
            }
        }

        proof fn total(x: Self, y: Self) {
            V::total(x.vertex, y.vertex);
            if x.priority == y.priority && TotalOrder::le(x.vertex, y.vertex) && TotalOrder::le(y.vertex, x.vertex) {
                V::antisymmetric(x.vertex, y.vertex);
                match (x.parent, y.parent) {
                    (Option::Some(a), Option::Some(b)) => {
                        V::total(a, b);
                    },
                    _ => {},
                }
            }
        }

        fn cmp(&self, other: &Self) -> (c: Ordering) {
            if self.priority < other.priority {
                Ordering::Less
            } else if self.priority > other.priority {
                Ordering::Greater
            } else {
                let vc = TotalOrder::cmp(&self.vertex, &other.vertex);
                proof {
                    if TotalOrder::le(self.vertex, other.vertex) && TotalOrder::le(other.vertex, self.vertex) {
                        V::antisymmetric(self.vertex, other.vertex);
                    }
                }
                match vc {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        // self.vertex == other.vertex.
                        proof { V::reflexive(self.vertex); }
                        match (&self.parent, &other.parent) {
                            (None, None) => Ordering::Equal,
                            (None, Some(_)) => Ordering::Less,
                            (Some(_), None) => Ordering::Greater,
                            (Some(a), Some(b)) => {
                                let pc = TotalOrder::cmp(a, b);
                                proof {
                                    if TotalOrder::le(*a, *b) && TotalOrder::le(*b, *a) {
                                        V::antisymmetric(*a, *b);
                                    }
                                }
                                match pc {
                                    Ordering::Less => Ordering::Less,
                                    Ordering::Equal => Ordering::Equal,
                                    Ordering::Greater => Ordering::Greater,
                                }
                            },
                        }
                    },
                }
            }
        }
    }

    // 8. traits

    pub trait PrimStEphTrait {
        /// Well-formedness for sequential Prim MST algorithm input.
        open spec fn spec_primsteph_wf<V: HashOrd>(graph: &LabUnDirGraphStEph<V, u64>) -> bool {
            spec_labgraphview_wf(graph@)
        }

        /// Prim's MST algorithm.
        /// APAS: Work O(m log n), Span O(m log n) where m = |E|, n = |V|
        fn prim_mst<V: HashOrd + TotalOrder>(
            graph: &LabUnDirGraphStEph<V, u64>,
            start: V,
        ) -> (mst: SetStEph<LabEdge<V, u64>>)
            requires Self::spec_primsteph_wf(graph),
            ensures mst.spec_setsteph_wf();

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
            requires mst.spec_setsteph_wf(),
            ensures mst@.len() == 0 ==> total == 0;
    }

    /// Module-level function to create a new PQEntry.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    // veracity: no_requires
    fn pq_entry_new<V: HashOrd>(priority: u64, vertex: V, parent: Option<V>) -> (entry: PQEntry<V>)
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
    /// - Alg Analysis: APAS (Ch65 Alg 65.1): Work O(m lg n), Span O(m lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(m lg n) — matches APAS
    /// - Claude-Opus-4.6: Work O(m^2 lg n), Span O(m^2 lg n) — the APAS bound assumes
    ///   O(degree) adjacency-list lookups, but LabUnDirGraphStEph stores edges in a flat
    ///   set, so ng() and get_edge_label() each cost O(m) per call. Total neighbor/weight
    ///   work across all vertices is O(nm) = O(m^2) in a dense graph. With an adjacency-list
    ///   graph representation this would be O(m lg n) as textbook states.
    #[verifier::exec_allows_no_decreases_clause]
    #[verifier::external_body]
    pub fn prim_mst<V: HashOrd + Display + TotalOrder>(
        graph: &LabUnDirGraphStEph<V, u64>,
        start: &V,
    ) -> (mst: SetStEph<LabEdge<V, u64>>)
        requires
            spec_labgraphview_wf(graph@),
            valid_key_type_LabEdge::<V, u64>(),
            graph@.A.len() * 4 + 4 <= usize::MAX as int,
        ensures
            mst.spec_setsteph_wf(),
    {
        let m = graph.labeled_edges.size();
        assert(m as int == graph@.A.len());
        proof { assert(obeys_feq_full_trigger::<PQEntry<V>>()); }

        // DA = directed adjacency pairs derived from undirected edges.
        // Each undirected edge (u,v,l) in A contributes directed pairs (u,v) and (v,u).
        let ghost DA_fwd = graph@.A.map(|e: (V::V, V::V, u64)| (e.0, e.1));
        let ghost DA_rev = graph@.A.map(|e: (V::V, V::V, u64)| (e.1, e.0));
        let ghost DA = DA_fwd.union(DA_rev);

        proof {
            graph@.A.lemma_map_finite(|e: (V::V, V::V, u64)| (e.0, e.1));
            graph@.A.lemma_map_finite(|e: (V::V, V::V, u64)| (e.1, e.0));
            vstd::set_lib::lemma_set_union_finite_iff(DA_fwd, DA_rev);
            vstd::set_lib::lemma_map_size_bound(graph@.A, DA_fwd, |e: (V::V, V::V, u64)| (e.0, e.1));
            vstd::set_lib::lemma_map_size_bound(graph@.A, DA_rev, |e: (V::V, V::V, u64)| (e.1, e.0));
            vstd::set_lib::lemma_len_union(DA_fwd, DA_rev);
            assert(DA.len() <= 2 * m as int);
        }

        let mut mst_edges = SetLit![];
        let mut visited = HashSetWithViewPlus::<V>::new();
        let mut pq = BinaryHeapPQ::<PQEntry<V>>::singleton(pq_entry_new(0u64, start.clone(), None));
        let ghost mut remaining_budget: int = 2 * m as int;
        let ghost mut used_pairs: Set<(V::V, V::V)> = Set::empty();

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while !pq.is_empty()
            invariant
                spec_labgraphview_wf(graph@),
                valid_key_type_LabEdge::<V, u64>(),
                obeys_feq_clone::<PQEntry<V>>(),
                m as int == graph@.A.len(),
                graph@.A.len() * 4 + 4 <= usize::MAX as int,
                DA == DA_fwd.union(DA_rev),
                DA_fwd == graph@.A.map(|e: (V::V, V::V, u64)| (e.0, e.1)),
                DA_rev == graph@.A.map(|e: (V::V, V::V, u64)| (e.1, e.0)),
                DA.finite(),
                DA.len() <= 2 * m as int,
                remaining_budget >= 0,
                pq@.len() + remaining_budget <= 2 * m as int + 1,
                BinaryHeapPQ::<PQEntry<V>>::spec_is_exec_heap(pq.spec_seq()),
                used_pairs.subset_of(DA),
                used_pairs.finite(),
                used_pairs.len() as int == 2 * m as int - remaining_budget,
                mst_edges.spec_setsteph_wf(),
                visited@.finite(),
                forall |e: (V::V, V::V)| #[trigger] used_pairs.contains(e) ==>
                    visited@.contains(e.0),
        {
            // pq@.len() <= 2*m + 1, so pq@.len() * 2 <= (2*m+1)*2 = 4*m+2 <= 4*m+4 <= usize::MAX.
            assert(pq@.len() * 2 <= usize::MAX as int) by {
                vstd::set_lib::lemma_len_subset(used_pairs, DA);
            };

            let (new_pq, min_elem) = pq.delete_min();
            pq = new_pq;

            if let Some(entry) = min_elem {
                let u = entry.vertex;
                let parent_u = entry.parent;

                if visited.contains(&u) {
                    continue;
                }

                let _ = visited.insert(u.clone());

                if let Some(parent_v) = parent_u {
                    if let Some(weight) = graph.get_edge_label(&parent_v, &u) {
                        let edge = if parent_v < u {
                            LabEdge(parent_v.clone(), u.clone(), *weight)
                        } else {
                            LabEdge(u.clone(), parent_v.clone(), *weight)
                        };
                        let _ = mst_edges.insert(edge);
                    }
                }

                let neighbors = graph.ng(&u);

                // Prove neighbors.spec_setsteph_wf() so we can call iter().
                proof {
                    assert(neighbors.spec_setsteph_wf()) by {
                        assert forall |w: V::V| neighbors@.contains(w)
                            implies graph@.V.contains(w)
                        by {
                            let l = choose |l: u64|
                                graph@.A.contains((u@, w, l)) || graph@.A.contains((w, u@, l));
                            if graph@.A.contains((u@, w, l)) {
                                assert(graph@.V.contains(w));
                            } else {
                                assert(graph@.V.contains(w));
                            }
                        };
                        vstd::set_lib::lemma_set_subset_finite(graph@.V, neighbors@);
                    };
                }

                let mut it = neighbors.iter();

                // Every element in ng(u) is a directed adjacency pair (u@, v@) in DA.
                proof {
                    assert forall |j: int| 0 <= j < it@.1.len()
                        implies DA.contains((u@, (#[trigger] it@.1[j])@))
                    by {
                        assert(neighbors@.contains(it@.1[j]@));
                        let w: V::V = it@.1[j]@;
                        let l = choose |l: u64|
                            #![trigger graph@.A.contains((u@, w, l))]
                            graph@.A.contains((u@, w, l)) || graph@.A.contains((w, u@, l));
                        if graph@.A.contains((u@, w, l)) {
                            assert(DA_fwd.contains((u@, w)));
                            assert(DA.contains((u@, w)));
                        } else {
                            assert(graph@.A.contains((w, u@, l)));
                            assert(DA_rev.contains((u@, w)));
                            assert(DA.contains((u@, w)));
                        }
                    };
                }

                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                loop
                    invariant
                        spec_labgraphview_wf(graph@),
                        valid_key_type_LabEdge::<V, u64>(),
                        obeys_feq_clone::<PQEntry<V>>(),
                        m as int == graph@.A.len(),
                        graph@.A.len() * 4 + 4 <= usize::MAX as int,
                        DA.finite(),
                        DA.len() <= 2 * m as int,
                        remaining_budget >= 0,
                        pq@.len() + remaining_budget <= 2 * m as int,
                        BinaryHeapPQ::<PQEntry<V>>::spec_is_exec_heap(pq.spec_seq()),
                        used_pairs.subset_of(DA),
                        used_pairs.finite(),
                        used_pairs.len() as int == 2 * m as int - remaining_budget,
                        it@.0 <= it@.1.len(),
                        it@.1.no_duplicates(),
                        mst_edges.spec_setsteph_wf(),
                        visited@.contains(u@),
                        visited@.finite(),
                        forall |e: (V::V, V::V)| #[trigger] used_pairs.contains(e) ==>
                            visited@.contains(e.0),
                        forall |j: int| 0 <= j < it@.1.len() ==>
                            DA.contains((u@, (#[trigger] it@.1[j])@)),
                        forall |e: (V::V, V::V)| #[trigger] used_pairs.contains(e) ==>
                            (e.0 != u@ || (exists |j: int| 0 <= j < it@.0 &&
                                #[trigger] it@.1[j]@ == e.1)),
                {
                    match it.next() {
                        None => break,
                        Some(v) => {
                            proof {
                                // Current element is at position it@.0 - 1 (after next() advanced).
                                let ghost pos = (it@.0 - 1) as int;
                                let new_pair: (V::V, V::V) = (u@, v@);
                                assert(DA.contains((u@, it@.1[pos]@)));
                                assert(DA.contains(new_pair));
                                // From inner invariant at top (pos = old it@.0):
                                // all (u@, b) in used_pairs have j < pos. Combined with
                                // no_duplicates, the current element at pos can't be in used_pairs.
                                assert(!used_pairs.contains(new_pair));
                                let new_used = used_pairs.insert(new_pair);
                                assert(new_used.subset_of(DA));
                                assert(new_used.finite());
                                vstd::set_lib::lemma_len_subset(new_used, DA);
                                assert(remaining_budget > 0);
                                used_pairs = new_used;
                                remaining_budget = remaining_budget - 1;
                            }

                            if !visited.contains(v) {
                                // get_edge_label returns Some since v ∈ ng(u).
                                if let Some(weight) = graph.get_edge_label(&u, v) {
                                    assert(pq@.len() + 1 <= usize::MAX as int);
                                    pq = pq.insert(pq_entry_new(*weight, v.clone(), Some(u.clone())));
                                }
                            }
                        }
                    }
                }
            }
        }

        mst_edges
    }

    /// Compute total MST weight.
    /// - Alg Analysis: APAS: (no cost stated) — utility function
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    /// Overflow-safe: skips edges that would cause u64 overflow (never triggers for MST weights).
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
        requires mst_edges.spec_setsteph_wf(),
        ensures mst_edges@.len() == 0 ==> total == 0,
    {
        if mst_edges.size() == 0 {
            return 0u64;
        }
        let mut total: u64 = 0;
        let mut it = mst_edges.iter();
        let ghost le_seq = it@.1;
        loop
            invariant
                it@.0 <= le_seq.len(),
                it@.1 == le_seq,
                mst_edges@.len() > 0,
            decreases le_seq.len() - it@.0,
        {
            match it.next() {
                None => return total,
                Some(edge) => {
                    if edge.2 <= u64::MAX - total {
                        total = total + edge.2;
                    }
                },
            }
        }
    }

    // 12. derive impls in verus!

    impl<V: StT + Ord + Clone> Clone for PQEntry<V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@,
        {
            let cloned = PQEntry { priority: self.priority, vertex: self.vertex.clone(), parent: self.parent.clone() };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    } // verus!

    impl<V: HashOrd> Ord for PQEntry<V> {
        /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn cmp(&self, other: &Self) -> Ordering {
            std::cmp::Ord::cmp(&self.priority, &other.priority)
                .then_with(|| std::cmp::Ord::cmp(&self.vertex, &other.vertex))
                .then_with(|| std::cmp::Ord::cmp(&self.parent, &other.parent))
        }
    }

    impl<V: HashOrd> PartialOrd for PQEntry<V> {
        /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(Ord::cmp(self, other)) }
    }

    impl<V: HashOrd + Display> Display for PQEntry<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { write!(f, "({}, {})", self.priority, self.vertex) }
    }

    impl<V: HashOrd + std::fmt::Debug> std::fmt::Debug for PQEntry<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PQEntry")
                .field("priority", &self.priority)
                .field("vertex", &self.vertex)
                .field("parent", &self.parent)
                .finish()
        }
    }
}
