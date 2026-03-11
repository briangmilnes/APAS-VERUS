// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Adjacency Table Graph representation (ephemeral, single-threaded).

pub mod AdjTableGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
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
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphStEph<V: StT + Ord> {
        pub adj: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 6. spec fns

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    /// Sum of neighbor-set sizes over map domain (recursive over dom).
    pub open spec fn spec_sum_adj_sizes<VV>(m: Map<VV, Set<VV>>) -> nat
        decreases m.dom().len()
    {
        if m.dom().is_empty() {
            0
        } else {
            let k = m.dom().choose();
            m[k].len() + spec_sum_adj_sizes(m.remove(k))
        }
    }

    // 7. proof fns

    proof fn lemma_sum_adj_sizes_monotone<VV>(m: Map<VV, Set<VV>>, sub: Set<VV>)
        requires m.dom().finite(), sub.finite(), sub.subset_of(m.dom())
        ensures spec_sum_adj_sizes(m) >= 0
    {
    }

    // 8. traits

    pub trait AdjTableGraphStEphTrait<V: StT + Ord>: Sized {
        spec fn spec_adjtablegraphsteph_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// Work Theta(1), Span Theta(1)
        fn empty() -> (out: Self)
            ensures out.spec_adjtablegraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphsteph_wf();
        /// Work Theta(1), Span Theta(1)
        fn num_vertices(&self) -> N
            requires self.spec_adjtablegraphsteph_wf();
        /// Work Theta(|V| + |E|), Span Theta(|V| + |E|)
        fn num_edges(&self) -> (m: N)
            requires self.spec_adjtablegraphsteph_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// Work Theta(|V|), Span Theta(|V|)
        fn vertices(&self) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn has_edge(&self, u: &V, v: &V) -> B
            requires self.spec_adjtablegraphsteph_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_degree(&self, u: &V) -> N
            requires self.spec_adjtablegraphsteph_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&mut self, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), self.spec_adj().dom().contains(v@);
        /// Work Theta((|V| + |E|) log |V|), Span Theta((|V| + |E|) log |V|)
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), !self.spec_adj().dom().contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn insert_edge(&mut self, u: V, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn delete_edge(&mut self, u: &V, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                !self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStEphTrait<V> for AdjTableGraphStEph<V> {
        open spec fn spec_adjtablegraphsteph_wf(&self) -> bool {
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
            AdjTableGraphStEph {
                adj: OrderedTableStEph::empty(),
            }
        }

        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self) { AdjTableGraphStEph { adj: table } }

        fn num_vertices(&self) -> N { self.adj.size() }

        fn num_edges(&self) -> (m: N) {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut count = 0;
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

        fn vertices(&self) -> (verts: AVLTreeSetStEph<V>)
            ensures verts@ == self.spec_adj().dom()
        {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut verts = AVLTreeSetStEph::empty();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == seq.spec_len(),
                    verts@.finite(),
                    verts@ == seq@.subrange(0, i as int).to_set(),
                decreases len - i
            {
                verts.insert(seq.nth(i).clone());
                i += 1;
            }
            verts
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.find(v),
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetStEph<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty(),
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.clone(),
                None => AVLTreeSetStEph::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V)
            ensures self.spec_adj().dom().contains(v@)
        {
            self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
        }

        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_adj().dom().contains(v@)
        {
            let v_clone = v.clone();
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();
            let mut i: usize = 0;
            self.adj.delete(&v_clone);
            while i < len
                invariant i <= len, len == seq.spec_len()
                decreases len - i
            {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&u) {
                    let mut neighbors = neighbors.clone();
                    neighbors.delete(&v_clone);
                    self.adj.insert(u, neighbors, |_old, new| new.clone());
                }
                i += 1;
            }
        }

        fn insert_edge(&mut self, u: V, v: V)
            ensures
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@),
        {
            let neighbors = match self.adj.find(&u) {
                Some(ns) => {
                    let mut ns = ns.clone();
                    ns.insert(v.clone());
                    ns
                }
                None => AVLTreeSetStEph::singleton(v.clone()),
            };
            self.adj.insert(u, neighbors, |_old, new| new.clone());
            if self.adj.find(&v).is_none() {
                self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _new| old.clone());
            }
        }

        fn delete_edge(&mut self, u: &V, v: &V)
            ensures
                !self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@),
        {
            if let Some(neighbors) = self.adj.find(u) {
                let mut neighbors = neighbors.clone();
                neighbors.delete(v);
                self.adj.insert(u.clone(), neighbors, |_old, new| new.clone());
            }
        }
    }

    } // verus!
}
