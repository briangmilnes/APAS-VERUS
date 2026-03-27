// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod AdjTableGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    use crate::Chap42::TableStEph::TableStEph::*;
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
        pub adj: TableStEph<V, AVLTreeSetStEph<V>>,
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
        when m.dom().finite()
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

        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; creates empty table.
        fn empty() -> (out: Self)
            ensures out.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1)
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — wraps existing table.
        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self)
            requires
                forall|u: <V as View>::V, v: <V as View>::V|
                    table@.dom().contains(u)
                    && #[trigger] table@.index(u).contains(v)
                    ==> table@.dom().contains(v),
            ensures out.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(1), Span Theta(1) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(1), Span Theta(1) — agrees; table size.
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(|V| + |E|), Span Theta(|V| + |E|) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(|V| + |E|), Span Theta(|V| + |E|) — agrees; iterates all adjacency sets.
        fn num_edges(&self) -> (m: usize)
            requires self.spec_adjtablegraphsteph_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// - APAS: Work Theta(|V|), Span Theta(|V|) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(|V|), Span Theta(|V|) — agrees; builds set from domain.
        fn vertices(&self) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set find.
        fn has_edge(&self, u: &V, v: &V) -> bool
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; table find.
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V>
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; delegates to out_neighbors.
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphsteph_wf();
        /// - APAS: Work Theta(lg n), Span Theta(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n), Span Theta(lg n) — agrees; table insert.
        fn insert_vertex(&mut self, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), self.spec_adj().dom().contains(v@);
        /// - APAS: Work Theta((n + m) lg n), Span Theta((n + m) lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta((n + m) lg n), Span Theta((n + m) lg n) — agrees; removes from all neighbor sets.
        fn delete_vertex(&mut self, v: &V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures self.spec_adjtablegraphsteph_wf(), !self.spec_adj().dom().contains(v@);
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set insert.
        fn insert_edge(&mut self, u: V, v: V)
            requires old(self).spec_adjtablegraphsteph_wf()
            ensures
                self.spec_adjtablegraphsteph_wf(),
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@);
        /// - APAS: Work Theta(lg n + lg m), Span Theta(lg n + lg m) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Theta(lg n + lg m), Span Theta(lg n + lg m) — agrees; table find + set delete.
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
            let adj: TableStEph<V, AVLTreeSetStEph<V>> = TableStEph::empty();
            AdjTableGraphStEph { adj }
        }

        fn from_table(table: TableStEph<V, AVLTreeSetStEph<V>>) -> (out: Self) { AdjTableGraphStEph { adj: table } }

        fn num_vertices(&self) -> usize { self.adj.size() }

        fn num_edges(&self) -> (m: usize) {
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

        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
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

        fn out_degree(&self, u: &V) -> usize { self.out_neighbors(u).size() }

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
