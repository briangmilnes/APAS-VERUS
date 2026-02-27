// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

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
//	8. traits
//	9. impls
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module

pub mod BoruvkaStEph {

    use vstd::prelude::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::float::float::{WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use rand::rngs::StdRng;
    #[cfg(not(verus_keep_ghost))]
    use rand::*;

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
        fn clone(&self) -> (s: Self)
            ensures s@ == self@
        {
            *self
        }
    }

    impl<V: PartialEq + Copy> PartialEq for LabeledEdge<V> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3;
            proof { accept(r == (self@ == other@)); }
            r
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
    pub open spec fn spec_valid_bridge<V: View<V = V> + Copy>(
        v: V, neighbor: V, weight: WrappedF64, label: usize,
        edges: Set<LabeledEdge<V>>,
    ) -> bool {
        edges.contains(LabeledEdge(v, neighbor, weight, label))
        || edges.contains(LabeledEdge(neighbor, v, weight, label))
    }

    /// All edge weights are finite.
    pub open spec fn spec_all_weights_finite<V: View<V = V> + Copy>(edges: Set<LabeledEdge<V>>) -> bool {
        forall|e: LabeledEdge<V>| edges.contains(e) ==> e.2.spec_is_finite()
    }

    //		8. traits

    pub trait BoruvkaStEphTrait {
        /// Find vertex bridges for Borůvka's algorithm.
        /// APAS: Work O(|E|), Span O(|E|)
        fn vertex_bridges<V: StT + Hash + Ord>(
            edges: &SetStEph<LabeledEdge<V>>,
        ) -> (bridges: HashMapWithViewPlus<V, (V, WrappedF64, usize)>)
            requires
                obeys_key_model::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
                spec_all_weights_finite(edges@),
            ensures
                forall|v: V| #[trigger] bridges@.contains_key(v@) ==> {
                    let (neighbor, weight, label) = bridges@[v@];
                    spec_valid_bridge(v, neighbor, weight, label, edges@)
                };

        /// Bridge-based star partition.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bridge_star_partition<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            bridges: &HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
            rng: &mut StdRng,
        ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>));

        /// Borůvka's MST algorithm.
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: SetStEph<usize>,
            rng: &mut StdRng,
        ) -> (mst: SetStEph<usize>);

        /// Borůvka's MST with random seed.
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst_with_seed<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            seed: u64,
        ) -> (mst: SetStEph<usize>);

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: &SetStEph<usize>,
        ) -> (total: WrappedF64);
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
        fn vertex_bridges<V: StT + Hash + Ord>(
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
        /// Performs star contraction along vertex bridges using randomized coin flips.
        /// Each vertex flips a coin (Heads/Tails). Edges from Tail to Head are contracted.
        ///
        /// - APAS: Work O(n), Span O(log n)
        /// - Sequential: Work O(n), Span O(n) — sequential iteration over vertices.
        #[verifier::external_body]
        fn bridge_star_partition<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            bridges: &HashMapWithViewPlus<V, (V, WrappedF64, usize)>,
            rng: &mut StdRng,
        ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, (V, WrappedF64, usize)>)) {
            // Coin flips for all vertices.
            let mut flips = HashMap::<V, bool>::new();
            for v in vertices.iter() {
                let is_heads = rng.random::<bool>();
                let _ = flips.insert(v.clone(), is_heads);
            }

            // Select edges from Tail to Head (Tail=false, Head=true).
            let mut partition = HashMapWithViewPlus { inner: HashMap::new() };
            for (u, (v, w, label)) in bridges.inner.iter() {
                let u_heads = flips.get(u).copied().unwrap_or(false);
                let v_heads = flips.get(v).copied().unwrap_or(false);

                // Contract if u is Tail and v is Head.
                if !u_heads && v_heads {
                    let _ = partition.inner.insert(u.clone(), (v.clone(), *w, *label));
                }
            }

            // Remaining vertices = all vertices minus contracted tails.
            let mut remaining = SetStEph::empty();
            for v in vertices.iter() {
                if !partition.inner.contains_key(v) {
                    let _ = remaining.insert(v.clone());
                }
            }

            (remaining, partition)
        }

        /// Algorithm 66.3: Borůvka's MST.
        ///
        /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
        /// Returns the set of edge labels in the MST.
        ///
        /// - APAS: Work O(m log n), Span O(log^2 n)
        /// - Sequential: Work O(m log n), Span O(m log n) — sequential; O(log n) rounds each O(m).
        #[verifier::external_body]
        fn boruvka_mst<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            mst_labels: SetStEph<usize>,
            rng: &mut StdRng,
        ) -> (mst: SetStEph<usize>) {
            if edges.size() == 0 {
                return mst_labels;
            }

            let bridges = Self::vertex_bridges(edges);
            let (remaining_vertices, partition) =
                Self::bridge_star_partition(vertices, &bridges, rng);

            // Collect new MST labels from partition.
            let mut new_mst_labels = mst_labels.clone();
            for (_, (_, _, label)) in partition.inner.iter() {
                let _ = new_mst_labels.insert(*label);
            }

            // Build full partition map (including identity for non-contracted vertices).
            let mut full_partition = HashMap::<V, V>::new();
            for (tail, (head, _, _)) in partition.inner.iter() {
                let _ = full_partition.insert(tail.clone(), head.clone());
            }
            for v in remaining_vertices.iter() {
                let _ = full_partition.insert(v.clone(), v.clone());
            }

            // Re-route edges to new endpoints, removing self-edges.
            let mut new_edges = SetStEph::empty();
            for LabeledEdge(u, v, w, label) in edges.iter() {
                let new_u = full_partition.get(u).cloned().unwrap_or_else(|| u.clone());
                let new_v = full_partition.get(v).cloned().unwrap_or_else(|| v.clone());

                if new_u != new_v {
                    let _ = new_edges.insert(LabeledEdge(new_u, new_v, *w, *label));
                }
            }

            Self::boruvka_mst(&remaining_vertices, &new_edges, new_mst_labels, rng)
        }

        /// Borůvka MST with a specific seed.
        /// Initializes RNG and delegates to `boruvka_mst`.
        ///
        /// - APAS: Work O(m log n), Span O(log^2 n)
        /// - Sequential: Work O(m log n), Span O(m log n) — delegates to sequential boruvka_mst.
        #[verifier::external_body]
        fn boruvka_mst_with_seed<V: StT + Hash + Ord>(
            vertices: &SetStEph<V>,
            edges: &SetStEph<LabeledEdge<V>>,
            seed: u64,
        ) -> (mst: SetStEph<usize>) {
            let mut rng = StdRng::seed_from_u64(seed);
            Self::boruvka_mst(vertices, edges, SetStEph::empty(), &mut rng)
        }

        /// Compute MST weight from edge labels.
        ///
        /// - APAS: N/A — utility function, not in prose.
        /// - Sequential: Work O(m), Span O(m) — sequential scan of edges.
        fn mst_weight<V: StT + Hash>(
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

    #[cfg(not(verus_keep_ghost))]
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> PartialOrd for LabeledEdge<V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Ord for LabeledEdge<V> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
                .then_with(|| self.1.cmp(&other.1))
                .then_with(|| self.2.val.partial_cmp(&other.2.val).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| self.3.cmp(&other.3))
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::hash::Hash> std::hash::Hash for LabeledEdge<V> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
            self.1.hash(state);
            self.2.val.to_bits().hash(state);
            self.3.hash(state);
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::fmt::Debug> std::fmt::Debug for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("LabeledEdge").field(&self.0).field(&self.1).field(&self.2.val).field(&self.3).finish()
        }
    }
    #[cfg(not(verus_keep_ghost))]
    impl<V: std::fmt::Display> std::fmt::Display for LabeledEdge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        }
    }
}
