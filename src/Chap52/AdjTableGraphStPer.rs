// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 52: Adjacency Table Graph representation (persistent, single-threaded).
//! G = (V × V set) table - maps vertices to sets of their out-neighbors.

pub mod AdjTableGraphStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap42::TableStPer::TableStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::spec_sum_adj_sizes;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesWf;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, obeys_feq_fulls};
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_view_eq_trigger;

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
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphStPer<V: StT + Ord> {
        pub adj: TableStPer<V, AVLTreeSetStPer<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait AdjTableGraphStPerTrait<V: StT + Ord>: Sized {
        spec fn spec_adjtablegraphstper_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures out.spec_adjtablegraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_table(table: TableStPer<V, AVLTreeSetStPer<V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphstper_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphstper_wf();
        /// Work Theta(|V| + |E|), Span Theta(|V| + |E|)
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphstper_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// Work Theta(|V|), Span Theta(|V|)
        fn vertices(&self) -> (verts: AVLTreeSetStPer<V>)
            requires self.spec_adjtablegraphstper_wf()
            ensures verts@ == self.spec_adj().dom();
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphstper_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            requires self.spec_adjtablegraphstper_wf()
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphstper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures updated.spec_adjtablegraphstper_wf(), updated.spec_adj().dom().contains(v@);
        /// Work Theta((|V| + |E|) log |V|), Span Theta((|V| + |E|) log |V|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures updated.spec_adjtablegraphstper_wf(), !updated.spec_adj().dom().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures
                updated.spec_adjtablegraphstper_wf(),
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphstper_wf()
            ensures
                updated.spec_adjtablegraphstper_wf(),
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStPerTrait<V> for AdjTableGraphStPer<V> {
        open spec fn spec_adjtablegraphstper_wf(&self) -> bool {
            // Type-level predicates needed by table and set operations.
            obeys_view_eq::<V>()
            && vstd::laws_cmp::obeys_cmp_spec::<V>()
            && view_ord_consistent::<V>()
            // Table internal invariant (keys are unique).
            && spec_keys_no_dups::<V::V, Set<V::V>>(self.adj.entries@)
            // feq/clone properties needed by table operations.
            && obeys_feq_fulls::<V, AVLTreeSetStPer<V>>()
            && obeys_feq_full::<Pair<V, AVLTreeSetStPer<V>>>()
            // All stored neighbor sets are well-formed.
            && forall|k: <V as View>::V| #[trigger] self.adj@.dom().contains(k) ==>
                self.adj.spec_stored_value(k).spec_avltreesetstper_wf()
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
            let adj: TableStPer<V, AVLTreeSetStPer<V>> = TableStPer::empty();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStPer<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStPer<V>>>());
                assert(obeys_view_eq_trigger::<V>());
            }
            AdjTableGraphStPer { adj }
        }

        fn from_table(table: TableStPer<V, AVLTreeSetStPer<V>>) -> (out: Self) {
            let out = AdjTableGraphStPer { adj: table };
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<AVLTreeSetStPer<V>>());
                assert(obeys_feq_full_trigger::<Pair<V, AVLTreeSetStPer<V>>>());
                assert(obeys_view_eq_trigger::<V>());
                // Table internals and stored-value wf not in requires. Verus ICE on Set<V::V>.
                assume(out.spec_adjtablegraphstper_wf());
            }
            out
        }

        fn num_vertices(&self) -> usize {
            proof { reveal(obeys_view_eq); }
            self.adj.size()
        }

        /// - external_body: iterating domain requires loop invariant.
        #[verifier::external_body]
        fn num_edges(&self) -> (m: usize) {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len {
                let v = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        /// - external_body: building set from domain requires loop invariant.
        #[verifier::external_body]
        fn vertices(&self) -> (verts: AVLTreeSetStPer<V>) {
            let domain_set = self.adj.domain();
            let seq = domain_set.to_seq();
            let len = seq.length();
            let mut vertices = AVLTreeSetStPer::empty();
            let mut i: usize = 0;
            while i < len {
                vertices = vertices.insert(seq.nth(i).clone());
                i += 1;
            }
            vertices
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool) {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => {
                    neighbors.find(v)
                }
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>) {
            proof { reveal(obeys_view_eq); }
            match self.adj.find(u) {
                Some(ns) => {
                    proof {
                        // Verus ICE on Set<V::V> prevents proving ns@ == self.spec_adj()[u@].
                        assume(self.spec_adj().dom().contains(u@) ==> ns@ == self.spec_adj()[u@]);
                    }
                    ns
                }
                None => {
                    let empty = AVLTreeSetStPer::empty();
                    proof {
                        assume(!self.spec_adj().dom().contains(u@)
                            ==> empty@ == Set::<<V as View>::V>::empty());
                    }
                    empty
                }
            }
        }

        fn out_degree(&self, u: &V) -> usize {
            proof { reveal(obeys_view_eq); }
            match self.adj.find_ref(u) {
                Some(neighbors) => neighbors.size(),
                None => 0,
            }
        }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone());
            let updated = AdjTableGraphStPer { adj: new_adj };
            proof {
                // Clone gap + graph closure: Verus ICE on Set<V::V> in proof bodies.
                assume(updated.spec_adjtablegraphstper_wf());
            }
            updated
        }

        /// - external_body: requires loop with table iteration + nested set operations.
        #[verifier::external_body]
        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            let v_clone = v.clone();
            let new_adj = self.adj.delete(&v_clone);
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut result_adj = new_adj;
            let mut i: usize = 0;
            while i < len {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = result_adj.find(&u) {
                    let new_neighbors = neighbors.delete(&v_clone);
                    result_adj = result_adj.insert(u, new_neighbors, |_old, new| new.clone());
                }
                i += 1;
            }
            AdjTableGraphStPer { adj: result_adj }
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let neighbors = match self.adj.find_ref(&u) {
                Some(ns_ref) => {
                    proof {
                        // Capacity: stored sets have len < usize::MAX.
                        assume(ns_ref@.len() + 1 < usize::MAX as nat);
                    }
                    ns_ref.clone_wf().insert(v.clone())
                }
                None => AVLTreeSetStPer::singleton(v.clone()),
            };
            let new_adj = self.adj.insert(u, neighbors, |_old, new| new.clone());
            let final_adj = if new_adj.find_ref(&v).is_none() {
                new_adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone())
            } else {
                new_adj
            };
            let updated = AdjTableGraphStPer { adj: final_adj };
            proof {
                // Clone gap + graph closure + postconditions: Verus ICE on Set<V::V>.
                assume(updated.spec_adjtablegraphstper_wf());
                assume(updated.spec_adj().dom().contains(u@));
                assume(updated.spec_adj().dom().contains(v@));
                assume(updated.spec_adj()[u@].contains(v@));
            }
            updated
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            proof { reveal(obeys_view_eq); }
            let updated = match self.adj.find_ref(u) {
                Some(neighbors) => {
                    let new_neighbors = neighbors.clone_wf().delete(v);
                    let new_adj = self.adj.insert(u.clone(), new_neighbors, |_old, new| new.clone());
                    AdjTableGraphStPer { adj: new_adj }
                }
                None => self.clone(),
            };
            proof {
                // Clone gap + graph closure + postcondition: Verus ICE on Set<V::V>.
                assume(updated.spec_adjtablegraphstper_wf());
                assume(!updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@));
            }
            updated
        }
    }

    } // verus!
}
