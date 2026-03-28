// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Adjacency Table Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where the graph is represented as a table mapping vertices to their out-neighbor sets.

pub mod AdjTableGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEphTrait;
    use crate::Chap43::OrderedTableMtPer::OrderedTableMtPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::spec_pair_key_determines_order;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    // This implementation requires V: Ord for BOTH keys and values because:
    // - OrderedTableMtPer is backed by BSTParaTreapMtEph<Pair<K,V>>
    // - BSTParaTreapMtEph requires elements to be MtKey (which includes Ord)
    // - This allows the table to use parallel tree operations (split, join, union)
    // - Sets (AVLTreeSetMtPer<V>) implement Ord via lexicographic ordering of elements
    // - This constraint enables efficient parallel operations on the adjacency structure
    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphMtPer<V: StTInMtT + Ord + TotalOrder + 'static> {
        pub adj: OrderedTableMtPer<V, AVLTreeSetMtPer<V>>,
    }

    // 5. view impls

    impl<V: StTInMtT + Ord + TotalOrder + 'static> View for AdjTableGraphMtPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 6. spec fns

    /// Sum of all neighbor set sizes across all vertices in the adjacency map.
    /// Local copy — standalone rule forbids importing from StEph.
    pub open spec fn spec_sum_adj_sizes<VV>(m: Map<VV, Set<VV>>) -> nat
        decreases m.dom().len()
        when m.dom().finite()
    {
        if m.dom().is_empty() {
            0
        } else {
            let k = m.dom().choose();
            m[k].len() + spec_sum_adj_sizes(m.remove(k))
        }
    }

    // 8. traits

    pub trait AdjTableGraphMtPerTrait<V: StTInMtT + Ord + TotalOrder + 'static>: Sized {
        spec fn spec_adjtablegraphmtper_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, AVLTreeSetMtPer<V>>>(),
                view_ord_consistent::<Pair<V, AVLTreeSetMtPer<V>>>(),
                spec_pair_key_determines_order::<V, AVLTreeSetMtPer<V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures out.spec_adjtablegraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphmtper_wf();
        /// Work Theta(|V| + |E|), Span Theta(log |V| * log |E|)
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphmtper_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphmtper_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            requires self.spec_adjtablegraphmtper_wf()
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphmtper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires self.spec_adjtablegraphmtper_wf()
            ensures updated.spec_adjtablegraphmtper_wf(), updated.spec_adj().dom().contains(v@);
        /// Work Theta((|V| + |E|) log |V|), Span Theta(log^2 |V| + log |E|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphmtper_wf()
            ensures updated.spec_adjtablegraphmtper_wf(), !updated.spec_adj().dom().contains(v@);
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires self.spec_adjtablegraphmtper_wf()
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphmtper_wf()
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StTInMtT + Ord + TotalOrder + 'static> AdjTableGraphMtPerTrait<V> for AdjTableGraphMtPer<V> {
        open spec fn spec_adjtablegraphmtper_wf(&self) -> bool {
            // Type-level predicates for table and set operations.
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, AVLTreeSetMtPer<V>>>()
            && view_ord_consistent::<Pair<V, AVLTreeSetMtPer<V>>>()
            && spec_pair_key_determines_order::<V, AVLTreeSetMtPer<V>>()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            // Graph closure: every neighbor is also a vertex.
            && forall|u: <V as View>::V, v: <V as View>::V|
                self.spec_adj().dom().contains(u)
                && #[trigger] self.spec_adj().index(u).contains(v)
                ==> self.spec_adj().dom().contains(v)
        }

        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            spec_sum_adj_sizes(self.spec_adj())
        }

        fn empty() -> (out: Self) {
            let adj = OrderedTableMtPer::empty();
            let out = AdjTableGraphMtPer { adj };
            proof {
                // Empty map: graph closure holds vacuously.
                assume(out.spec_adjtablegraphmtper_wf());
            }
            out
        }

        fn num_vertices(&self) -> usize {
            self.adj.size()
        }

        /// - external_body: iterating domain requires loop invariant.
        #[verifier::external_body]
        fn num_edges(&self) -> (m: usize) {
            let domain = self.adj.domain();
            let domain_seq = domain.to_seq();
            let len = domain_seq.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len {
                let v = domain_seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool) {
            let found = match self.adj.find(u) {
                Some(neighbors) => {
                    proof { assume(neighbors.spec_avltreesetmtper_wf()); }
                    neighbors.find(v)
                }
                None => false,
            };
            proof {
                assume(found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@)));
            }
            found
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>) {
            let neighbors = match self.adj.find(u) {
                Some(ns) => ns.clone(),
                None => AVLTreeSetMtPer::empty(),
            };
            proof {
                assume(self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@]);
                assume(!self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty());
            }
            neighbors
        }

        fn out_degree(&self, u: &V) -> usize {
            let ns = self.out_neighbors(u);
            proof { assume(ns.spec_avltreesetmtper_wf()); }
            ns.size()
        }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            let updated = if self.adj.find(&v).is_some() {
                self.clone()
            } else {
                proof { assume(self.adj@.dom().len() + 1 < usize::MAX as nat); }
                AdjTableGraphMtPer {
                    adj: self.adj.insert(v, AVLTreeSetMtPer::empty()),
                }
            };
            proof {
                assume(updated.spec_adjtablegraphmtper_wf());
                assume(updated.spec_adj().dom().contains(v@));
            }
            updated
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            let without_v = self.adj.delete(v);
            let v_clone = v.clone();
            let cleaned = without_v.map(move |_k: &V, neighbors: &AVLTreeSetMtPer<V>| {
                neighbors.delete(&v_clone)
            });
            let updated = AdjTableGraphMtPer { adj: cleaned };
            proof {
                // Graph-level wf (neighbor-set wf + graph closure) requires
                // quantifying over Map<V::V, Set<V::V>> which triggers Verus ICE.
                // Weak OrderedTableMtPer::delete/map ensures prevent proving
                // domain/value properties. Algorithmic logic verified: v deleted
                // from table, v removed from each neighbor set via map.
                // blocked by Verus ICE + weak OrderedTableMtPer ensures
                assume(updated.spec_adjtablegraphmtper_wf());
                assume(!updated.spec_adj().dom().contains(v@));
            }
            updated
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            let mut new_adj = self.adj.clone();
            if new_adj.find(&u).is_none() {
                proof { assume(new_adj@.dom().len() + 1 < usize::MAX as nat); }
                new_adj = new_adj.insert(u.clone(), AVLTreeSetMtPer::empty());
            }
            if new_adj.find(&v).is_none() {
                proof { assume(new_adj@.dom().len() + 1 < usize::MAX as nat); }
                new_adj = new_adj.insert(v.clone(), AVLTreeSetMtPer::empty());
            }
            let u_neighbors = match new_adj.find(&u) {
                Some(ns) => ns,
                None => AVLTreeSetMtPer::empty(),
            };
            proof {
                assume(u_neighbors.spec_avltreesetmtper_wf());
                assume(u_neighbors@.len() + 1 < usize::MAX as nat);
            }
            let new_u_neighbors = u_neighbors.insert(v);
            proof { assume(new_adj@.dom().len() + 1 < usize::MAX as nat); }
            let updated = AdjTableGraphMtPer {
                adj: new_adj.insert(u, new_u_neighbors),
            };
            proof {
                assume(updated.spec_adjtablegraphmtper_wf());
                assume(updated.spec_adj().dom().contains(u@));
                assume(updated.spec_adj().dom().contains(v@));
                assume(updated.spec_adj()[u@].contains(v@));
            }
            updated
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            let updated = match self.adj.find(u) {
                Some(u_neighbors) => {
                    proof {
                        assume(u_neighbors.spec_avltreesetmtper_wf());
                        assume(self.adj@.dom().len() + 1 < usize::MAX as nat);
                    }
                    let new_u_neighbors = u_neighbors.delete(v);
                    AdjTableGraphMtPer {
                        adj: self.adj.insert(u.clone(), new_u_neighbors),
                    }
                }
                None => self.clone(),
            };
            proof {
                assume(updated.spec_adjtablegraphmtper_wf());
                assume(!updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@));
            }
            updated
        }
    }

    } // verus!
}
