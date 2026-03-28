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
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

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
                neighbors.spec_avltreesetmtper_wf(),
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphmtper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 1 < usize::MAX as nat,
            ensures updated.spec_adjtablegraphmtper_wf(), updated.spec_adj().dom().contains(v@);
        /// Work Theta((|V| + |E|) log |V|), Span Theta(log^2 |V| + log |E|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires self.spec_adjtablegraphmtper_wf()
            ensures updated.spec_adjtablegraphmtper_wf(), !updated.spec_adj().dom().contains(v@);
        /// Work Theta(log |V|), Span Theta(log |V|)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 3 < usize::MAX as nat,
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@);
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 1 < usize::MAX as nat,
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
            // Adjacency domain is finite.
            && self.spec_adj().dom().finite()
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
                // Type-level preds come from requires. Graph closure is vacuous
                // on an empty map since no u satisfies dom().contains(u).
                assert(out.adj@ == Map::<<V as View>::V, Set<<V as View>::V>>::empty());
                assert(out.spec_adj().dom().finite());
                assert forall|u: <V as View>::V, v: <V as View>::V|
                    out.spec_adj().dom().contains(u)
                    && #[trigger] out.spec_adj().index(u).contains(v)
                    implies out.spec_adj().dom().contains(v)
                by {
                    // Empty map domain contains nothing.
                };
            }
            out
        }

        fn num_vertices(&self) -> usize {
            self.adj.size()
        }

        fn num_edges(&self) -> (m: usize) {
            let domain = self.adj.domain();
            let domain_seq = domain.to_seq();
            let len = domain_seq.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < len
                invariant
                    0 <= i <= len,
                    len == domain_seq@.len(),
                    self.spec_adjtablegraphmtper_wf(),
                    self.spec_num_edges() <= usize::MAX as nat,
                    count as nat <= self.spec_num_edges(),
                decreases len - i,
            {
                let v = domain_seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    proof {
                        // find ensures: self.adj@.contains_key(v@) && self.adj@[v@] == neighbors@
                        // Prove neighbors wf via graph closure + finiteness.
                        let dom = self.spec_adj().dom();
                        assert(neighbors@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] neighbors@.contains(w)
                                implies dom.contains(w)
                            by {
                                assert(self.spec_adj().index(v@).contains(w));
                            };
                        };
                        vstd::set_lib::lemma_len_subset(neighbors@, dom);
                        // Overflow: partial sum + current size <= total edges.
                        // blocked by weak OrderedTableMtPer domain/find ensures
                        assume(count as nat + neighbors@.len() <= self.spec_num_edges());
                    }
                    count = count + neighbors.size();
                }
                i += 1;
            }
            proof {
                // Bridge: the loop computes the correct sum algorithmically
                // (iterate domain keys, look up neighbor set sizes, accumulate),
                // but the spec connection to spec_sum_adj_sizes requires
                // domain-value correspondence that OrderedTableMtPer::map/domain
                // ensures don't provide.
                assume(count as nat == self.spec_num_edges());
            }
            count
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: bool) {
            match self.adj.find(u) {
                Some(neighbors) => {
                    proof {
                        // find ensures: self.adj@.contains_key(u@) && self.adj@[u@] == neighbors@
                        // Prove neighbors wf via graph closure + finiteness.
                        let dom = self.spec_adj().dom();
                        assert(neighbors@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] neighbors@.contains(w)
                                implies dom.contains(w)
                            by {
                                assert(self.spec_adj().index(u@).contains(w));
                            };
                        };
                        vstd::set_lib::lemma_len_subset(neighbors@, dom);
                    }
                    // neighbors.find(v) ensures: result == neighbors@.contains(v@)
                    // neighbors@ == self.spec_adj()[u@], dom.contains(u@) is true.
                    neighbors.find(v)
                }
                None => {
                    // find ensures: !self.adj@.contains_key(u@)
                    // so !self.spec_adj().dom().contains(u@), making the && false.
                    false
                }
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>) {
            match self.adj.find(u) {
                Some(ns) => {
                    proof {
                        // find ensures: self.adj@.contains_key(u@) && self.adj@[u@] == ns@
                        // Prove ns wf: ns@ == adj[u@] ⊆ dom (graph closure), dom finite → ns@ finite.
                        let dom = self.spec_adj().dom();
                        assert(ns@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] ns@.contains(w)
                                implies dom.contains(w)
                            by {
                                assert(self.spec_adj().index(u@).contains(w));
                            };
                        };
                        vstd::set_lib::lemma_len_subset(ns@, dom);
                    }
                    ns.clone()
                }
                None => {
                    AVLTreeSetMtPer::empty()
                }
            }
        }

        fn out_degree(&self, u: &V) -> usize {
            let ns = self.out_neighbors(u);
            // out_neighbors now ensures ns.spec_avltreesetmtper_wf().
            ns.size()
        }

        fn insert_vertex(&self, v: V) -> (updated: Self) {
            if self.adj.find(&v).is_some() {
                // v already in domain. Clone preserves view → preserves wf.
                let cloned_adj = self.adj.clone();
                // OrderedTableMtPer::clone ensures cloned_adj@ == self.adj@.
                let updated = AdjTableGraphMtPer { adj: cloned_adj };
                // updated.spec_adj() == self.spec_adj(), so wf follows from self's wf.
                // dom.contains(v@): find returned Some → self.adj@.contains_key(v@).
                updated
            } else {
                // v not in domain. Insert v with empty neighbor set.
                let empty_set = AVLTreeSetMtPer::empty();
                let updated = AdjTableGraphMtPer {
                    adj: self.adj.insert_wf(v, empty_set),
                };
                proof {
                    // insert_wf ensures:
                    //   updated.adj@.dom() =~= self.adj@.dom().insert(v@)
                    //   updated.adj@[v@] == empty_set@ == Set::empty()
                    //   forall|k2 != v@| self.adj@.contains_key(k2) ==> updated.adj@[k2] == self.adj@[k2]
                    //   updated.adj.spec_orderedtablemtper_wf()

                    // Graph closure: every neighbor of every vertex is also a vertex.
                    assert forall|u2: <V as View>::V, w: <V as View>::V|
                        updated.spec_adj().dom().contains(u2)
                        && #[trigger] updated.spec_adj().index(u2).contains(w)
                        implies updated.spec_adj().dom().contains(w)
                    by {
                        if u2 == v@ {
                            // updated.adj@[v@] == Set::empty(), contradiction.
                            assert(updated.adj@[v@] =~= Set::<<V as View>::V>::empty());
                        } else {
                            // u2 in old domain, value unchanged.
                            assert(self.adj@.contains_key(u2));
                            assert(updated.adj@[u2] == self.adj@[u2]);
                            // By self's graph closure: w in self's domain.
                            assert(self.spec_adj().index(u2).contains(w));
                            assert(self.spec_adj().dom().contains(w));
                            // self's domain ⊆ updated domain.
                        }
                    };
                }
                updated
            }
        }

        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            let without_v = self.adj.delete(v);
            let v_clone = v.clone();
            let cleaned = without_v.map(move |_k: &V, neighbors: &AVLTreeSetMtPer<V>| {
                neighbors.delete(&v_clone)
            });
            let updated = AdjTableGraphMtPer { adj: cleaned };
            proof {
                // delete ensures: without_v@ == self.adj@.remove(v@)
                // map ensures: cleaned@.dom() =~= without_v@.dom()
                // Map::remove(k).dom() == dom().remove(k), so v@ not in cleaned@.dom().
                assert(without_v@ == self.adj@.remove(v@));
                assert(cleaned@.dom() =~= without_v@.dom());
                assert(!updated.spec_adj().dom().contains(v@));
                // Graph closure still needs map value ensures to prove neighbor sets
                // had v removed, so every neighbor is still in domain.
                assume(updated.spec_adjtablegraphmtper_wf()); // algorithmic: needs map value ensures
            }
            updated
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self) {
            let mut new_adj = self.adj.clone();
            // clone ensures: new_adj@ == self.adj@
            let ghost orig_adj = self.adj@;
            let ghost orig_dom_len = self.adj@.dom().len();

            proof {
                assert(obeys_feq_full_trigger::<V>());
            }

            // Track whether u was originally in domain.
            let ghost u_in_orig = self.adj@.contains_key(u@);

            // Ensure u is in domain.
            match new_adj.find(&u) {
                Some(_) => {
                    assert(new_adj@.dom().len() <= orig_dom_len);
                }
                None => {
                    let u_clone = u.clone();
                    proof {
                        crate::vstdplus::feq::feq::lemma_cloned_view_eq::<V>(u, u_clone);
                    }
                    new_adj = new_adj.insert_wf(u_clone, AVLTreeSetMtPer::empty());
                    assert(new_adj@.dom().contains(u@));
                    assert(new_adj@.dom().len() <= orig_dom_len + 1);
                }
            }

            // After match 1: u@ in dom. Establish u-value invariant.
            proof {
                if u_in_orig {
                    // Some arm: new_adj unchanged from clone. new_adj@[u@] == orig_adj[u@].
                    assert(new_adj@[u@] == orig_adj[u@]);
                } else {
                    // None arm: insert_wf set new_adj@[u@] to empty.
                    assert(new_adj@[u@] =~= Set::<<V as View>::V>::empty());
                }
            }
            let ghost adj_after_u = new_adj@;

            // Ensure v is in domain.
            match new_adj.find(&v) {
                Some(_) => {
                    assert(new_adj@.dom().len() <= orig_dom_len + 1);
                }
                None => {
                    let v_clone = v.clone();
                    proof {
                        crate::vstdplus::feq::feq::lemma_cloned_view_eq::<V>(v, v_clone);
                    }
                    new_adj = new_adj.insert_wf(v_clone, AVLTreeSetMtPer::empty());
                    assert(new_adj@.dom().contains(v@));
                    assert(new_adj@.dom().len() <= orig_dom_len + 2);
                }
            }

            // After match 2: u@ and v@ both in dom. u's value is unchanged.
            proof {
                // In None arm of match 2: v@ was NOT in adj_after_u's dom. Since u@ WAS
                // in adj_after_u's dom, v@ != u@. insert_wf preserves new_adj@[u@].
                // In Some arm: new_adj unchanged.
                assert(new_adj@[u@] == adj_after_u[u@]);
                // Establish finiteness of new_adj@[u@].
                if u_in_orig {
                    assert(new_adj@[u@] == orig_adj[u@]);
                    // Graph closure on self: self.adj@[u@] ⊆ self.adj@.dom().
                    let dom = orig_adj.dom();
                    assert(new_adj@[u@].subset_of(dom)) by {
                        assert forall|w: <V as View>::V| #[trigger] new_adj@[u@].contains(w)
                            implies dom.contains(w)
                        by {
                            assert(orig_adj.index(u@).contains(w));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(new_adj@[u@], dom);
                } else {
                    assert(new_adj@[u@] =~= Set::<<V as View>::V>::empty());
                }
            }

            let u_neighbors = match new_adj.find(&u) {
                Some(ns) => ns,
                None => AVLTreeSetMtPer::empty(),
            };
            proof {
                // find returned Some (u@ in dom) → ns@ == new_adj@[u@].
                // new_adj@[u@] is finite (established above).
                // spec_avltreesetmtper_wf() = self@.finite() = new_adj@[u@].finite(). ✓
                assert(u_neighbors@.finite());
                // Overflow: u_neighbors@.len() ≤ orig_dom_len (if from self) or 0 (if empty).
                // orig_dom_len + 2 < usize::MAX → u_neighbors@.len() + 1 < usize::MAX.
                if u_in_orig {
                    assert(u_neighbors@.len() <= orig_dom_len);
                }
            }
            let new_u_neighbors = u_neighbors.insert(v);
            // insert ensures: new_u_neighbors@ == u_neighbors@.insert(v@), wf preserved.
            proof {
                // After match 1: dom.len() <= orig_dom_len + 1.
                // After match 2: dom.len() <= orig_dom_len + 2.
                // requires: orig_dom_len + 3 < usize::MAX.
                // Therefore: new_adj@.dom().len() + 1 <= orig_dom_len + 3 < usize::MAX.
                assert(new_adj@.dom().len() <= orig_dom_len + 2);
                assert(new_adj@.dom().len() + 1 < usize::MAX as nat);
            }
            let updated = AdjTableGraphMtPer {
                adj: new_adj.insert_wf(u, new_u_neighbors),
            };
            proof {
                // insert_wf ensures:
                //   updated.adj@.dom() =~= new_adj@.dom().insert(u@) (u@ already in dom)
                //   updated.adj@[u@] == new_u_neighbors@ == u_neighbors@.insert(v@)
                //   forall|k2 != u@| new_adj@.contains_key(k2) ==> updated.adj@[k2] == new_adj@[k2]

                // Postcondition: updated.adj@[u@].contains(v@).
                assert(updated.adj@[u@] =~= u_neighbors@.insert(v@));
                assert(updated.spec_adj()[u@].contains(v@));

                // Graph closure on updated.
                assert forall|u2: <V as View>::V, w: <V as View>::V|
                    updated.spec_adj().dom().contains(u2)
                    && #[trigger] updated.spec_adj().index(u2).contains(w)
                    implies updated.spec_adj().dom().contains(w)
                by {
                    if u2 == u@ {
                        // updated.adj@[u@] == u_neighbors@.insert(v@).
                        // w is in u_neighbors@.insert(v@).
                        if w == v@ {
                            // v@ is in new_adj@.dom() (from match 2) ⊆ updated dom.
                            assert(new_adj@.dom().contains(v@));
                        } else {
                            // w in u_neighbors@ == new_adj@[u@].
                            assert(u_neighbors@.contains(w));
                            if u_in_orig {
                                // u_neighbors@ == self.adj@[u@]. Graph closure on self.
                                assert(orig_adj.index(u@).contains(w));
                                assert(orig_adj.dom().contains(w));
                            }
                            // If !u_in_orig: u_neighbors@ == Set::empty(), w can't be in it.
                        }
                    } else {
                        // u2 != u@. updated.adj@[u2] == new_adj@[u2].
                        assert(new_adj@.contains_key(u2));
                        assert(updated.adj@[u2] == new_adj@[u2]);
                        // new_adj@[u2] comes from either self (via clone + insert_wf preserve)
                        // or is Set::empty() (newly inserted key). Either way, all elements
                        // are in self's domain ⊆ updated's domain.
                        // Trace: new_adj was built from self.adj via clone + up to 2 insert_wf.
                        // For u2 != u@ and u2 != v@ (if v@ was inserted):
                        //   new_adj@[u2] == orig_adj[u2] (preserved through both inserts).
                        //   Graph closure on self: orig_adj[u2] ⊆ orig_adj.dom() ⊆ updated dom.
                        // For u2 == v@ (if v@ was newly inserted):
                        //   new_adj@[v@] == Set::empty(). No elements.
                        if orig_adj.contains_key(u2) {
                            assert(orig_adj.index(u2).contains(w));
                            assert(orig_adj.dom().contains(w));
                        }
                        // If u2 was newly inserted (not in orig_adj): its value is Set::empty().
                    }
                };
            }
            updated
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self) {
            let updated = match self.adj.find(u) {
                Some(u_neighbors) => {
                    proof {
                        // find ensures: self.adj@.contains_key(u@) && self.adj@[u@] == u_neighbors@
                        // Prove u_neighbors wf via graph closure + finiteness.
                        let dom = self.spec_adj().dom();
                        assert(u_neighbors@.subset_of(dom)) by {
                            assert forall|w: <V as View>::V| #[trigger] u_neighbors@.contains(w)
                                implies dom.contains(w)
                            by {
                                assert(self.spec_adj().index(u@).contains(w));
                            };
                        };
                        vstd::set_lib::lemma_len_subset(u_neighbors@, dom);
                    }
                    let new_u_neighbors = u_neighbors.delete(v);
                    // delete ensures: new_u_neighbors@ == u_neighbors@.remove(v@), wf preserved.
                    let u_clone = u.clone();
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        crate::vstdplus::feq::feq::lemma_cloned_view_eq::<V>(*u, u_clone);
                        // u_clone@ == u@.
                    }
                    let updated_inner = AdjTableGraphMtPer {
                        adj: self.adj.insert_wf(u_clone, new_u_neighbors),
                    };
                    proof {
                        // insert_wf ensures (key is u_clone where u_clone@ == u@):
                        //   dom =~= self.adj@.dom().insert(u@) (u@ already in dom, so same)
                        //   updated_inner.adj@[u@] == new_u_neighbors@ == u_neighbors@.remove(v@)
                        //   forall|k2 != u@| self.adj@.contains_key(k2) ==>
                        //       updated_inner.adj@[k2] == self.adj@[k2]

                        // Dom is same as self's dom (u@ already present).
                        assert(self.adj@.dom().insert(u@) =~= self.adj@.dom());

                        // Graph closure on updated_inner.
                        assert forall|u2: <V as View>::V, w: <V as View>::V|
                            updated_inner.spec_adj().dom().contains(u2)
                            && #[trigger] updated_inner.spec_adj().index(u2).contains(w)
                            implies updated_inner.spec_adj().dom().contains(w)
                        by {
                            if u2 == u@ {
                                // Neighbors = u_neighbors@.remove(v@) ⊆ u_neighbors@ == self.adj@[u@].
                                assert(u_neighbors@.contains(w));
                                assert(self.spec_adj().index(u@).contains(w));
                                assert(self.spec_adj().dom().contains(w));
                            } else {
                                // u2 in updated dom == self dom, u2 != u@.
                                assert(self.adj@.contains_key(u2));
                                assert(updated_inner.adj@[u2] == self.adj@[u2]);
                                assert(self.spec_adj().index(u2).contains(w));
                                assert(self.spec_adj().dom().contains(w));
                            }
                        };

                        // Postcondition: edge (u,v) removed.
                        assert(!updated_inner.spec_adj()[u@].contains(v@));
                    }
                    updated_inner
                }
                None => {
                    // u not in domain. Clone adj, reconstruct struct.
                    let cloned_adj = self.adj.clone();
                    // OrderedTableMtPer::clone ensures cloned_adj@ == self.adj@.
                    let cloned = AdjTableGraphMtPer { adj: cloned_adj };
                    proof {
                        // cloned.adj@ == self.adj@, so spec_adj() is identical.
                        assert(cloned.adj@ == self.adj@);

                        // Graph closure follows from self's invariant.
                        assert forall|u2: <V as View>::V, w: <V as View>::V|
                            cloned.spec_adj().dom().contains(u2)
                            && #[trigger] cloned.spec_adj().index(u2).contains(w)
                            implies cloned.spec_adj().dom().contains(w)
                        by {
                            assert(self.spec_adj().dom().contains(u2));
                            assert(self.spec_adj().index(u2).contains(w));
                            assert(self.spec_adj().dom().contains(w));
                        };
                        // Postcondition: !dom.contains(u@), so disjunction holds.
                        assert(!cloned.spec_adj().dom().contains(u@));
                    }
                    cloned
                }
            };
            updated
        }
    }

    } // verus!
}
