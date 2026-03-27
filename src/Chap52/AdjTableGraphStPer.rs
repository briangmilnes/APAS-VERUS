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
            forall|u: <V as View>::V, v: <V as View>::V|
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
            AdjTableGraphStPer { adj }
        }

        fn from_table(table: TableStPer<V, AVLTreeSetStPer<V>>) -> (out: Self) { AdjTableGraphStPer { adj: table } }

        fn num_vertices(&self) -> usize { self.adj.size() }

        fn num_edges(&self) -> (m: usize) {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len
                invariant i <= len, len == seq.spec_len()
                decreases len - i
            {
                let v = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        fn vertices(&self) -> (verts: AVLTreeSetStPer<V>)
            ensures verts@ == self.spec_adj().dom()
        {
            let domain_set = self.adj.domain();
            let seq = domain_set.to_seq();
            let len = seq.length();
            let mut vertices = AVLTreeSetStPer::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == seq.spec_len(),
                    vertices@.finite(),
                    vertices@ == seq@.subrange(0, i as int).to_set(),
                decreases len - i
            {
                vertices = vertices.insert(seq.nth(i).clone());
                i += 1;
            }
            vertices
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.find(v),
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStPer<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty(),
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.clone(),
                None => AVLTreeSetStPer::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> (updated: Self)
            ensures updated.spec_adj().dom().contains(v@)
        {
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone());
            AdjTableGraphStPer { adj: new_adj }
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.spec_adj().dom().contains(v@)
        {
            let v_clone = v.clone();
            let new_adj = self.adj.delete(&v_clone);
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut result_adj = new_adj;
            let mut i: usize = 0;
            while i < len
                invariant i <= len, len == seq.spec_len()
                decreases len - i
            {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = result_adj.find(&u) {
                    let new_neighbors = neighbors.delete(&v_clone);
                    result_adj = result_adj.insert(u, new_neighbors, |_old, new| new.clone());
                }
                i += 1;
            }
            AdjTableGraphStPer { adj: result_adj }
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            ensures
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@),
        {
            let neighbors = match self.adj.find(&u) {
                Some(ns) => ns.insert(v.clone()),
                None => AVLTreeSetStPer::singleton(v.clone()),
            };
            let new_adj = self.adj.insert(u, neighbors, |_old, new| new.clone());
            let final_adj = if new_adj.find(&v).is_none() {
                new_adj.insert(v, AVLTreeSetStPer::empty(), |old, _new| old.clone())
            } else {
                new_adj
            };
            AdjTableGraphStPer { adj: final_adj }
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            ensures
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@),
        {
            match self.adj.find(u) {
                Some(neighbors) => {
                    let new_neighbors = neighbors.delete(v);
                    let new_adj = self.adj.insert(u.clone(), new_neighbors, |_old, new| new.clone());
                    AdjTableGraphStPer { adj: new_adj }
                }
                None => self.clone(),
            }
        }
    }

    } // verus!
}
