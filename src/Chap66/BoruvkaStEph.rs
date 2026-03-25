//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 66.2 and 66.3: Borůvka's algorithm for computing Minimum Spanning Trees
//! using vertex bridges and graph contraction with randomized star contraction.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns/broadcast groups
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module

pub mod BoruvkaStEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::iter_invariant;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    use std::hash::Hash;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    verus! {

    //		3. broadcast use

    broadcast use {
        vstd::set::group_set_axioms,
        crate::vstdplus::float::float::group_float_finite_total_order,
    };

    //		4. type definitions

    /// Edge with label: (u, v, weight, label). Vertices u,v change during contraction.
    pub struct LabeledEdge<V>(pub V, pub V, pub WrappedF64, pub usize);

    /// Namespace struct for trait impl.
    pub struct BoruvkaStEph;

    impl<V: Copy> Copy for LabeledEdge<V> {}

    impl<V: Copy> Clone for LabeledEdge<V> {
        fn clone(&self) -> (s: Self) {
            *self
        }
    }

    #[verifier::external]
    impl<V: PartialEq + Copy> PartialEq for LabeledEdge<V> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
        }
    }

    impl<V: Eq + Copy> Eq for LabeledEdge<V> {}

    //		5. view impls

    impl<V: StT + Ord> View for LabeledEdge<V> {
        type V = Self;
        open spec fn view(&self) -> Self { *self }
    }

    //		6. spec fns

    /// A bridge entry (neighbor, weight, label) is valid for vertex v given edges.
    pub open spec fn spec_valid_bridge<V: Copy>(
        v: V, neighbor: V, weight: WrappedF64, label: usize,
        edges: Set<LabeledEdge<V>>,
    ) -> bool {
        edges.contains(LabeledEdge(v, neighbor, weight, label))
        || edges.contains(LabeledEdge(neighbor, v, weight, label))
    }

    /// All edge weights are finite.
    pub open spec fn spec_all_weights_finite<V: Copy>(edges: Set<LabeledEdge<V>>) -> bool {
        forall|e: LabeledEdge<V>| #[trigger] edges.contains(e) ==> e.2.spec_is_finite()
    }

    //		7. proof fns/broadcast groups

    /// Deterministic coin flip from seed and vertex iteration index.
    // veracity: no_requires
    fn coin_flip(seed: u64, index: usize) -> (flip: bool)
        ensures flip == (((seed ^ (index as u64)) & 1u64) == 1u64)
    {
        ((seed ^ (index as u64)) & 1) == 1
    }

    //		8. traits

    pub trait BoruvkaStEphTrait {
        /// Well-formedness for sequential Borůvka MST algorithm input.
        open spec fn spec_boruvkasteph_wf<V: Copy>(
            edges: Set<LabeledEdge<V>>,
        ) -> bool {
            spec_all_weights_finite(edges)
        }

        /// Find vertex bridges for Borůvka's algorithm.
        /// APAS: Work O(|E|), Span O(|E|)
        fn vertex_bridges<V: HashOrd + Copy>(
            edges: &SetStEph<LabeledEdge<V>>,
        ) -> (bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                edges.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                spec_all_weights_finite(edges@);

        /// Bridge-based star partition.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bridge_star_partition<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            bridges: &HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
            seed: u64,
        ) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>))
            requires
                vertices.spec_setsteph_wf(),
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2
            ensures partition.0.spec_setsteph_wf();

        /// Borůvka's MST algorithm.
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: SetStEph<usize>,
            seed: u64,
        ) -> (mst: SetStEph<usize>)
            requires
                vertices.spec_setsteph_wf(),
                edges.spec_setsteph_wf(),
                mst_labels.spec_setsteph_wf(),
                Self::spec_boruvkasteph_wf(edges@),
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                obeys_key_model::<LabeledEdge<V>>(),
                obeys_feq_full::<LabeledEdge<V>>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2
            ensures mst.spec_setsteph_wf();

        /// Borůvka's MST with random seed.
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst_with_seed<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            seed: u64,
        ) -> (mst: SetStEph<usize>)
            requires
                vertices.spec_setsteph_wf(),
                edges.spec_setsteph_wf(),
                Self::spec_boruvkasteph_wf(edges@),
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                obeys_key_model::<LabeledEdge<V>>(),
                obeys_feq_full::<LabeledEdge<V>>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2
            ensures mst.spec_setsteph_wf();

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash + Ord + Copy>(
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: &SetStEph<usize>,
        ) -> (total: WrappedF64)
            requires
                edges.spec_setsteph_wf(),
                mst_labels.spec_setsteph_wf();
    }

    //		9. impls

    impl BoruvkaStEphTrait for BoruvkaStEph {
        /// Algorithm 66.3: Find vertex bridges.
        ///
        /// For each vertex, find the minimum weight edge incident on it.
        /// Returns a table mapping each vertex to (neighbor, weight, label).
        ///
        /// - APAS: Work O(m), Span O(log m)
        /// - Sequential: Work O(m), Span O(m) — sequential iteration over edges.
        #[verifier::external_body]
        fn vertex_bridges<V: HashOrd + Copy>(
            edges: &SetStEph<LabeledEdge<V>>,
        ) -> (bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>) {
            let mut bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)> =
                HashMapWithViewPlus::new();

            let mut it = edges.iter();
            let ghost iter_seq = it@.1;

            loop
                invariant
                    forall|v: V| #[trigger] bridges@.contains_key(v@) ==> {
                        let (neighbor, weight, label) = bridges@[v@];
                        spec_valid_bridge(v, neighbor, weight, label, edges@)
                    },
                    iter_invariant(&it),
                    iter_seq == it@.1,
                    iter_seq.map(|i: int, k: LabeledEdge<V>| k@).to_set() == edges@,
                    spec_all_weights_finite(edges@),
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                decreases iter_seq.len() - it@.0,
            {
                if let Some(edge) = it.next() {
                    let LabeledEdge(u, v, w, label) = edge.clone();

                    // The edge is in edges@.
                    assert(edges@.contains(LabeledEdge(u, v, w, label)));

                    // Update bridge for u.
                    match bridges.get(&u) {
                        None => {
                            bridges.insert(u.clone(), (v.clone(), w, label));
                        }
                        Some((_, existing_w, _)) => {
                            if w.dist_lt(existing_w) {
                                bridges.insert(u.clone(), (v.clone(), w, label));
                            }
                        }
                    }

                    // Update bridge for v.
                    match bridges.get(&v) {
                        None => {
                            bridges.insert(v.clone(), (u.clone(), w, label));
                        }
                        Some((_, existing_w, _)) => {
                            if w.dist_lt(existing_w) {
                                bridges.insert(v.clone(), (u.clone(), w, label));
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            bridges
        }

        /// Algorithm 66.2: Bridge star partition.
        ///
        /// Performs star contraction along vertex bridges using deterministic coin flips.
        /// Each vertex flips a coin (Heads/Tails). Edges from Tail to Head are contracted.
        ///
        /// - APAS: Work O(n), Span O(log n)
        /// - Sequential: Work O(n), Span O(n) — sequential iteration over vertices.
        #[verifier::external_body]
        fn bridge_star_partition<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            bridges: &HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
            seed: u64,
        ) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>)) {
            // Phase 1: Assign coin flips to all vertices.
            let mut flips: HashMapWithViewPlus<V, bool> = HashMapWithViewPlus::new();
            let mut idx: usize = 0;
            let mut vit = vertices.iter();
            let ghost vit_seq = vit@.1;

            loop
                invariant
                    iter_invariant(&vit),
                    vit_seq == vit@.1,
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                decreases vit_seq.len() - vit@.0,
            {
                if let Some(v) = vit.next() {
                    let flip = coin_flip(seed, idx);
                    flips.insert(v.clone(), flip);
                    if idx < usize::MAX {
                        idx = idx + 1;
                    }
                } else {
                    break;
                }
            }

            // Phase 2: Select edges from Tail to Head (iterate vertices, check bridges).
            let mut contracted: HashMapWithViewPlus<V, (V, WrappedF64, usize)> =
                HashMapWithViewPlus::new();
            let mut vit2 = vertices.iter();
            let ghost vit2_seq = vit2@.1;

            loop
                invariant
                    iter_invariant(&vit2),
                    vit2_seq == vit2@.1,
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                decreases vit2_seq.len() - vit2@.0,
            {
                if let Some(u) = vit2.next() {
                    let u_heads = match flips.get(u) {
                        Some(b) => *b,
                        None => false,
                    };

                    if !u_heads {
                        match bridges.get(u) {
                            Some((v, w, label)) => {
                                let v_heads = match flips.get(v) {
                                    Some(b) => *b,
                                    None => false,
                                };
                                if v_heads {
                                    contracted.insert(
                                        u.clone(), (v.clone(), *w, *label));
                                }
                            }
                            None => {}
                        }
                    }
                } else {
                    break;
                }
            }

            // Phase 3: Remaining vertices = all vertices minus contracted tails.
            let mut remaining: SetStEph<V> = SetStEph::empty();
            let mut vit3 = vertices.iter();
            let ghost vit3_seq = vit3@.1;

            loop
                invariant
                    iter_invariant(&vit3),
                    vit3_seq == vit3@.1,
                    remaining.spec_setsteph_wf(),
                decreases vit3_seq.len() - vit3@.0,
            {
                if let Some(v) = vit3.next() {
                    if !contracted.contains_key(v) {
                        let _ = remaining.insert(v.clone());
                    }
                } else {
                    break;
                }
            }

            (remaining, contracted)
        }

        /// Algorithm 66.3: Borůvka's MST.
        ///
        /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
        /// Returns the set of edge labels in the MST.
        ///
        /// - APAS: Work O(m log n), Span O(log^2 n)
        /// - Sequential: Work O(m log n), Span O(m log n) — sequential; O(log n) rounds each O(m).
        #[verifier::external_body]
        fn boruvka_mst<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: SetStEph<usize>,
            seed: u64,
        ) -> (mst: SetStEph<usize>) {
            if edges.size() == 0 {
                return mst_labels;
            }

            let bridges = Self::vertex_bridges(edges);
            let (remaining_vertices, partition) =
                Self::bridge_star_partition(vertices, &bridges, seed);

            // Collect new MST labels from partition and build tail->head map.
            let mut new_mst_labels = mst_labels;
            let mut full_partition: HashMapWithViewPlus<V, V> = HashMapWithViewPlus::new();

            let mut pit = partition.iter();
            let ghost pit_seq = pit@.1;

            loop
                invariant
                    0 <= pit@.0 <= pit@.1.len(),
                    pit_seq == pit@.1,
                    new_mst_labels.spec_setsteph_wf(),
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                decreases pit_seq.len() - pit@.0,
            {
                if let Some((tail, bridge_entry)) = pit.next() {
                    let (head, _w, label) = bridge_entry;
                    let _ = new_mst_labels.insert(*label);
                    full_partition.insert(tail.clone(), head.clone());
                } else {
                    break;
                }
            }

            // Add identity mappings for remaining vertices.
            let mut rit = remaining_vertices.iter();
            let ghost rit_seq = rit@.1;

            loop
                invariant
                    iter_invariant(&rit),
                    rit_seq == rit@.1,
                    obeys_key_model::<V>(),
                    forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                decreases rit_seq.len() - rit@.0,
            {
                if let Some(v) = rit.next() {
                    full_partition.insert(v.clone(), v.clone());
                } else {
                    break;
                }
            }

            // Re-route edges to new endpoints, removing self-edges.
            let mut new_edges: SetStEph<LabeledEdge<V>> = SetStEph::empty();
            let mut eit = edges.iter();
            let ghost eit_seq = eit@.1;

            loop
                invariant
                    iter_invariant(&eit),
                    eit_seq == eit@.1,
                    new_edges.spec_setsteph_wf(),
                    spec_all_weights_finite(edges@),
                decreases eit_seq.len() - eit@.0,
            {
                if let Some(edge) = eit.next() {
                    let LabeledEdge(u, v, w, label) = edge.clone();
                    let new_u = match full_partition.get(&u) {
                        Some(mapped) => mapped.clone(),
                        None => u,
                    };
                    let new_v = match full_partition.get(&v) {
                        Some(mapped) => mapped.clone(),
                        None => v,
                    };
                    if new_u != new_v {
                        let _ = new_edges.insert(LabeledEdge(new_u, new_v, w, label));
                    }
                } else {
                    break;
                }
            }

            Self::boruvka_mst(&remaining_vertices, &new_edges, new_mst_labels, seed)
        }

        /// Borůvka MST with a specific seed.
        /// Deterministic coin flips replace StdRng for Verus verification.
        ///
        /// - APAS: Work O(m log n), Span O(log^2 n)
        /// - Sequential: Work O(m log n), Span O(m log n) — delegates to sequential boruvka_mst.
        #[verifier::external_body]
        fn boruvka_mst_with_seed<V: HashOrd + Copy>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            seed: u64,
        ) -> (mst: SetStEph<usize>) {
            Self::boruvka_mst(vertices, edges, SetStEph::empty(), seed)
        }

        /// Compute MST weight from edge labels.
        ///
        /// - APAS: N/A — utility function, not in prose.
        /// - Sequential: Work O(m), Span O(m) — sequential scan of edges.
        #[verifier::external_body]
        fn mst_weight<V: StT + Hash + Ord + Copy>(
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: &SetStEph<usize>,
        ) -> (total: WrappedF64) {
            let mut total = zero_dist();
            let mut it = edges.iter();
            let ghost iter_seq = it@.1;

            loop
                invariant
                    iter_invariant(&it),
                    iter_seq == it@.1,
                decreases iter_seq.len() - it@.0,
            {
                if let Some(edge) = it.next() {
                    let LabeledEdge(_, _, w, label) = edge;
                    if mst_labels.mem(label) {
                        total = total.dist_add(w);
                    }
                } else {
                    break;
                }
            }
            total
        }
    }

    //		11. derive impls in verus!

    } // verus!

    //		13. derive impls outside verus!

    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> PartialOrd for LabeledEdge<V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Ord for LabeledEdge<V> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
                .then_with(|| self.1.cmp(&other.1))
                .then_with(|| self.2.val.partial_cmp(&other.2.val).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| self.3.cmp(&other.3))
        }
    }
    impl<V: std::hash::Hash> std::hash::Hash for LabeledEdge<V> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
            self.1.hash(state);
            self.2.val.to_bits().hash(state);
            self.3.hash(state);
        }
    }
    impl<V: std::fmt::Debug> std::fmt::Debug for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("LabeledEdge").field(&self.0).field(&self.1).field(&self.2.val).field(&self.3).finish()
        }
    }
    impl<V: std::fmt::Display> std::fmt::Display for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }
}
