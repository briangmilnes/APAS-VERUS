//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 52: Adjacency Table Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where the graph is represented as a table mapping vertices to their out-neighbor sets.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod AdjTableGraphMtPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEphTrait;
    use crate::Chap43::OrderedTableMtPer::OrderedTableMtPer::*;
    use crate::Chap52::AdjTableGraphSpecsAndLemmas::AdjTableGraphSpecsAndLemmas::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::spec_pair_key_determines_order;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    //		Section 4. type definitions


    // This implementation requires V: Ord for BOTH keys and values because:
    // - OrderedTableMtPer is backed by BSTParaTreapMtEph<Pair<K,V>>
    // - BSTParaTreapMtEph requires elements to be MtKey (which includes Ord)
    // - This allows the table to use parallel tree operations (split, join, union)
    // - Sets (AVLTreeSetMtPer<V>) implement Ord via lexicographic ordering of elements
    // - This constraint enables efficient parallel operations on the adjacency structure
    #[verifier::reject_recursive_types(V)]
    pub struct AdjTableGraphMtPer<V: StTInMtT + Ord + TotalOrder + 'static> {
        pub adj: OrderedTableMtPer<V, AVLTreeSetMtPer<V>>,
        pub num_edges: usize,
    }

    //		Section 5. view impls


    impl<V: StTInMtT + Ord + TotalOrder + 'static> View for AdjTableGraphMtPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    //		Section 7. proof fns/broadcast groups


    //		Section 8. traits


    pub trait AdjTableGraphMtPerTrait<V: StTInMtT + Ord + TotalOrder + 'static>: Sized {
        spec fn spec_adjtablegraphmtper_wf(&self) -> bool;
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, AVLTreeSetMtPer<V>>>(),
                view_ord_consistent::<Pair<V, AVLTreeSetMtPer<V>>>(),
                spec_pair_key_determines_order::<V, AVLTreeSetMtPer<V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures
                out.spec_adjtablegraphmtper_wf(),
                out.spec_num_edges() == spec_sum_adj_sizes(out.spec_adj());
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); table size
        fn num_vertices(&self) -> usize
            requires self.spec_adjtablegraphmtper_wf();
        /// Work Theta(1), Span Theta(1)
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); cached field
        fn num_edges(&self) -> (m: usize)
            where V: crate::vstdplus::total_order::total_order::TotalOrder
            requires self.spec_adjtablegraphmtper_wf(), self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find + set find
        fn has_edge(&self, u: &V, v: &V) -> (found: bool)
            requires self.spec_adjtablegraphmtper_wf()
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n + d_g(v)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find returns neighbor set
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            requires self.spec_adjtablegraphmtper_wf()
            ensures
                neighbors.spec_avltreesetmtper_wf(),
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: APAS (Ch52 CS 52.3): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n); table find + set len
        fn out_degree(&self, u: &V) -> usize
            requires self.spec_adjtablegraphmtper_wf();
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 1 < usize::MAX as nat,
                self.spec_num_edges() == spec_sum_adj_sizes(self.spec_adj()),
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                updated.spec_adj().dom().contains(v@),
                updated.spec_num_edges() == spec_sum_adj_sizes(updated.spec_adj());
        /// Work Theta((|V| + |E|) log |V|), Span Theta(log^2 |V| + log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * (log n + d)), Span O(n * (log n + d))
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_num_edges() == spec_sum_adj_sizes(self.spec_adj()),
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                !updated.spec_adj().dom().contains(v@),
                updated.spec_num_edges() == spec_sum_adj_sizes(updated.spec_adj());
        /// Work Theta(log |V|), Span Theta(log |V|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 3 < usize::MAX as nat,
                self.spec_num_edges() < usize::MAX as nat,
                self.spec_num_edges() == spec_sum_adj_sizes(self.spec_adj()),
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@),
                updated.spec_num_edges() == spec_sum_adj_sizes(updated.spec_adj());
        /// Work Theta(log |V| + log |E|), Span Theta(log |V| + log |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            requires
                self.spec_adjtablegraphmtper_wf(),
                self.spec_adj().dom().len() + 1 < usize::MAX as nat,
                self.spec_num_edges() == spec_sum_adj_sizes(self.spec_adj()),
            ensures
                updated.spec_adjtablegraphmtper_wf(),
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@),
                updated.spec_num_edges() == spec_sum_adj_sizes(updated.spec_adj());
    }

    //		Section 9. impls


    /// Count all edges in a table that satisfies the graph closure property.
    /// Used by operations that cannot cheaply compute the new edge count incrementally.
    fn count_table_edges<V: StTInMtT + Ord + TotalOrder + 'static>(
        table: &OrderedTableMtPer<V, AVLTreeSetMtPer<V>>,
    ) -> (count: usize)
        where V: TotalOrder
        requires
            table.spec_orderedtablemtper_wf(),
            forall|u: <V as View>::V, w: <V as View>::V|
                table@.dom().contains(u) && #[trigger] table@.index(u).contains(w)
                ==> table@.dom().contains(w),
            spec_sum_adj_sizes(table@) <= usize::MAX as nat,
        ensures
            count as nat == spec_sum_adj_sizes(table@)
    {
        proof { reveal(spec_sum_adj_sizes); }
        let mut remaining = table.clone();
        let mut count: usize = 0;
        let mut n = remaining.size();
        while n > 0
            invariant
                remaining.spec_orderedtablemtper_wf(),
                n as nat == remaining@.dom().len(),
                count as nat + spec_sum_adj_sizes(remaining@) == spec_sum_adj_sizes(table@),
                remaining@.dom().subset_of(table@.dom()),
                forall|k: <V as View>::V| #[trigger] remaining@.dom().contains(k)
                    ==> remaining@[k] == table@[k],
                forall|u: <V as View>::V, w: <V as View>::V|
                    table@.dom().contains(u) && #[trigger] table@.index(u).contains(w)
                    ==> table@.dom().contains(w),
                table@.dom().finite(),
                spec_sum_adj_sizes(table@) <= usize::MAX as nat,
                count as nat <= spec_sum_adj_sizes(table@),
            decreases n,
        {
            let first = remaining.first_key();
            match first {
                None => {
                    proof { assert(false); }
                }
                Some(v_key) => {
                    let ghost old_remaining_view = remaining@;
                    match remaining.find(&v_key) {
                        None => {
                            proof { assert(false); }
                        }
                        Some(neighbors) => {
                            proof {
                                assert(remaining@.dom().contains(v_key@));
                                assert(neighbors@ == table@[v_key@]);
                                let dom = table@.dom();
                                assert(neighbors@.subset_of(dom)) by {
                                    assert forall|w: <V as View>::V|
                                        #[trigger] neighbors@.contains(w)
                                        implies dom.contains(w)
                                    by {
                                        assert(table@.index(v_key@).contains(w));
                                    };
                                };
                                vstd::set_lib::lemma_len_subset(neighbors@, dom);
                                lemma_sum_adj_remove(remaining@, v_key@);
                            }
                            let neighbor_count = neighbors.size();
                            count = count + neighbor_count;
                            remaining = remaining.delete(&v_key);
                            n = remaining.size();
                            proof {
                                assert(remaining@ == old_remaining_view.remove(v_key@));
                                assert(remaining@.dom().subset_of(table@.dom())) by {
                                    assert forall|k: <V as View>::V|
                                        #[trigger] remaining@.dom().contains(k)
                                        implies table@.dom().contains(k)
                                    by {
                                        assert(old_remaining_view.dom().contains(k));
                                    };
                                };
                                assert forall|k: <V as View>::V|
                                    #[trigger] remaining@.dom().contains(k)
                                    implies remaining@[k] == table@[k]
                                by {
                                    assert(old_remaining_view.dom().contains(k));
                                    assert(k != v_key@);
                                };
                            }
                        }
                    }
                }
            }
        }
        proof {
            assert(remaining@.dom().is_empty());
        }
        count
    }


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
            // Edge count invariant proved per-operation (not in wf due to Z3 matching loops).
            // See ensures on each operation for: num_edges == spec_sum_adj_sizes(adj@).
        }

        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            self.num_edges as nat
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (out: Self) {
            let adj = OrderedTableMtPer::empty();
            let out = AdjTableGraphMtPer { adj, num_edges: 0 };
            proof {
                reveal(spec_sum_adj_sizes);
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn num_vertices(&self) -> usize {
            self.adj.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1); cached field
        fn num_edges(&self) -> (m: usize)
            where V: crate::vstdplus::total_order::total_order::TotalOrder
        {
            self.num_edges
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(d), Span O(d)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn out_degree(&self, u: &V) -> usize {
            let ns = self.out_neighbors(u);
            // out_neighbors now ensures ns.spec_avltreesetmtper_wf().
            ns.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert_vertex(&self, v: V) -> (updated: Self) {
            if self.adj.find(&v).is_some() {
                // v already in domain. Clone preserves view → preserves wf.
                let cloned_adj = self.adj.clone();
                // OrderedTableMtPer::clone ensures cloned_adj@ == self.adj@.
                let updated = AdjTableGraphMtPer { adj: cloned_adj, num_edges: self.num_edges };
                // updated.spec_adj() == self.spec_adj(), so wf follows from self's wf.
                // dom.contains(v@): find returned Some → self.adj@.contains_key(v@).
                updated
            } else {
                // v not in domain. Insert v with empty neighbor set.
                let empty_set = AVLTreeSetMtPer::empty();
                let new_adj = self.adj.insert_wf(v, empty_set);
                let updated = AdjTableGraphMtPer {
                    adj: new_adj,
                    num_edges: self.num_edges,
                };
                proof {
                    // insert_wf ensures:
                    //   updated.adj@.dom() =~= self.adj@.dom().insert(v@)
                    //   updated.adj@[v@] == empty_set@ == Set::empty()
                    //   forall|k2 != v@| self.adj@.contains_key(k2) ==> updated.adj@[k2] == self.adj@[k2]
                    //   updated.adj.spec_orderedtablemtper_wf()

                    // Edge count: inserting v with empty set adds 0 to the sum.
                    // spec_sum_adj_sizes(updated.adj@)
                    //   = updated.adj@[v@].len() + spec_sum_adj_sizes(updated.adj@.remove(v@))
                    //   = 0 + spec_sum_adj_sizes(self.adj@)
                    //   = self.num_edges
                    reveal(spec_sum_adj_sizes);
                    lemma_sum_adj_remove(updated.adj@, v@);
                    assert(updated.adj@[v@] =~= Set::<<V as View>::V>::empty());
                    assert(updated.adj@.remove(v@) =~= self.adj@);

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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * (log n + d)), Span O(n * (log n + d))
        fn delete_vertex(&self, v: &V) -> (updated: Self) {
            let without_v = self.adj.delete(v);
            let ghost v_view = v@;
            let v_clone = v.clone();
            proof {
                assert(obeys_feq_full_trigger::<V>());
                crate::vstdplus::feq::feq::lemma_cloned_view_eq::<V>(*v, v_clone);
                assert(v_clone@ == v_view);
            }
            let cleaned = without_v.map(
                move |neighbors: &AVLTreeSetMtPer<V>| -> (r: AVLTreeSetMtPer<V>)
                    ensures r@ == neighbors@.remove(v_clone@)
                {
                    assert_avltreesetmtper_always_wf(neighbors);
                    neighbors.delete(&v_clone)
                },
                Ghost(|ns: Set<<V as View>::V>| -> Set<<V as View>::V> { ns.remove(v_view) }),
            );
            proof {
                // delete ensures: without_v@ == self.adj@.remove(v@)
                // map ensures: cleaned@.dom() =~= without_v@.dom(),
                //   forall|k| without_v@.contains_key(k) ==> cleaned@[k] == without_v@[k].remove(v@)
                assert(without_v@ == self.adj@.remove(v@));
                assert(cleaned@.dom() =~= without_v@.dom());

                // Graph closure on cleaned.
                assert forall|u: <V as View>::V, w: <V as View>::V|
                    cleaned@.dom().contains(u)
                    && #[trigger] cleaned@.index(u).contains(w)
                    implies cleaned@.dom().contains(w)
                by {
                    assert(cleaned@.dom().contains(u));
                    assert(cleaned@.index(u).contains(w));
                    assert(without_v@.contains_key(u));
                    assert(without_v@.contains_key(u));
                    assert(cleaned@[u] =~= without_v@[u].remove(v_view));
                    assert(u != v@);
                    assert(self.adj@.contains_key(u));
                    assert(without_v@[u] =~= self.adj@[u]);
                    assert(self.adj@.index(u).contains(w));
                    assert(w != v_view);
                    assert(v_view == v@);
                    assert(self.adj@.dom().contains(w));
                    assert(without_v@.dom().contains(w));
                    assert(cleaned@.dom().contains(w));
                };

                // Prove spec_sum_adj_sizes(cleaned@) <= self.num_edges for overflow bound.
                reveal(spec_sum_adj_sizes);
                // cleaned@ values are subsets of without_v@ values (removing an element).
                // without_v@ values equal self.adj@ values for keys in without_v@.dom().
                // Monotonicity: smaller sets → smaller sum.
                assert forall|k: <V as View>::V| #[trigger] cleaned@.dom().contains(k)
                    implies cleaned@[k].len() <= without_v@[k].len()
                by {
                    assert(without_v@.contains_key(k));
                    assert(cleaned@[k] =~= without_v@[k].remove(v_view));
                    // S.remove(x) ⊆ S, so |S.remove(x)| <= |S|.
                    // Prove without_v@[k] is finite for len() to be defined.
                    assert(k != v@);
                    assert(self.adj@.contains_key(k));
                    assert(without_v@[k] =~= self.adj@[k]);
                    let dom = self.spec_adj().dom();
                    assert(self.adj@[k].subset_of(dom)) by {
                        assert forall|w: <V as View>::V| #[trigger] self.adj@[k].contains(w)
                            implies dom.contains(w)
                        by {
                            assert(self.spec_adj().index(k).contains(w));
                        };
                    };
                    vstd::set_lib::lemma_len_subset(self.adj@[k], dom);
                    vstd::set_lib::lemma_len_subset(without_v@[k].remove(v_view), without_v@[k]);
                };
                assert(cleaned@.dom().finite()) by { assert(cleaned.spec_orderedtablemtper_wf()); };
                assert(without_v@.dom().finite()) by { assert(without_v.spec_orderedtablemtper_wf()); };
                assert(self.adj@.dom().finite());
                lemma_sum_adj_sizes_monotone(cleaned@, without_v@);
                assert(spec_sum_adj_sizes(cleaned@) <= spec_sum_adj_sizes(without_v@));
                // without_v@ == self.adj@.remove(v@). Prove its sum <= self's sum.
                assert(without_v@ =~= self.adj@.remove(v@));
                if self.adj@.dom().contains(v@) {
                    lemma_sum_adj_remove(self.adj@, v@);
                    // self sum = adj[v@].len + sum(adj.remove(v@)) >= sum(adj.remove(v@))
                    assert(spec_sum_adj_sizes(self.adj@.remove(v@))
                        <= spec_sum_adj_sizes(self.adj@));
                } else {
                    assert(self.adj@.remove(v@) =~= self.adj@);
                }
                // Verus needs substitution: without_v@ =~= self.adj@.remove(v@)
                assert(spec_sum_adj_sizes(without_v@)
                    == spec_sum_adj_sizes(self.adj@.remove(v@)));
                assert(spec_sum_adj_sizes(without_v@) <= spec_sum_adj_sizes(self.adj@));
                assert(self.num_edges as nat == spec_sum_adj_sizes(self.adj@));
                assert(spec_sum_adj_sizes(cleaned@) <= self.num_edges as nat);
                assert(spec_sum_adj_sizes(cleaned@) <= usize::MAX as nat);
            }
            let new_num_edges = count_table_edges(&cleaned);
            let updated = AdjTableGraphMtPer { adj: cleaned, num_edges: new_num_edges };
            proof {
                assert(!updated.spec_adj().dom().contains(v@));
            }
            updated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
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
                    assert(new_adj@[u@] == orig_adj[u@]);
                } else {
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
                assert(new_adj@[u@] == adj_after_u[u@]);
                if u_in_orig {
                    assert(new_adj@[u@] == orig_adj[u@]);
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
                assert(u_neighbors@.finite());
                if u_in_orig {
                    assert(u_neighbors@.len() <= orig_dom_len);
                }
            }
            // Check whether edge already exists before consuming v.
            assert_avltreesetmtper_always_wf(&u_neighbors);
            let had_edge = u_neighbors.find(&v);
            let new_u_neighbors = u_neighbors.insert(v);
            // insert ensures: new_u_neighbors@ == u_neighbors@.insert(v@), wf preserved.
            proof {
                assert(new_adj@.dom().len() <= orig_dom_len + 2);
                assert(new_adj@.dom().len() + 1 < usize::MAX as nat);
            }
            proof {
                assert(self.num_edges as nat == spec_sum_adj_sizes(self.adj@));
                assert((self.num_edges as nat) < (usize::MAX as nat));
            }
            let new_num_edges: usize = if had_edge { self.num_edges } else { self.num_edges + 1 };
            let updated_adj = new_adj.insert_wf(u, new_u_neighbors);
            let updated = AdjTableGraphMtPer {
                adj: updated_adj,
                num_edges: new_num_edges,
            };
            proof {
                reveal(spec_sum_adj_sizes);
                // insert_wf ensures:
                //   updated.adj@.dom() =~= new_adj@.dom().insert(u@) (u@ already in dom)
                //   updated.adj@[u@] == new_u_neighbors@ == u_neighbors@.insert(v@)
                //   forall|k2 != u@| new_adj@.contains_key(k2) ==> updated.adj@[k2] == new_adj@[k2]

                // Postcondition: updated.adj@[u@].contains(v@).
                assert(updated.adj@[u@] =~= u_neighbors@.insert(v@));
                assert(updated.spec_adj()[u@].contains(v@));

                // Prove cached edge count is correct.
                // Step 1: spec_sum_adj_sizes(new_adj@) == self.num_edges.
                // new_adj@ was built from self.adj@ by inserting at most u@→empty, v@→empty.
                // Each such insert adds 0 to the sum because empty set has 0 elements.
                // Prove by decomposing at each inserted key.
                if !u_in_orig {
                    // u@ was inserted with empty set. adj_after_u == self.adj@.insert(u@, empty).
                    // But after v's potential insert, we need to reason about new_adj@.
                    // Since adj_after_u[u@] == empty and new_adj@ preserves u@'s value:
                }
                // Prove spec_sum_adj_sizes(new_adj@) == self.num_edges as nat.
                assert(self.num_edges as nat == spec_sum_adj_sizes(self.adj@));
                let self_sum = spec_sum_adj_sizes(self.adj@);
                assert(self_sum == self.num_edges as nat);
                // After adding u@ (if not present):
                if !u_in_orig {
                    assert(adj_after_u.dom().finite()) by {
                        // adj_after_u == result of insert_wf on self.adj, which is wf.
                        // insert_wf ensures dom =~= self.adj@.dom().insert(u@).
                        assert(adj_after_u.dom() =~= self.adj@.dom().insert(u@));
                    };
                    assert(adj_after_u.dom().contains(u@));
                    lemma_sum_adj_remove(adj_after_u, u@);
                    assert(adj_after_u[u@] =~= Set::<<V as View>::V>::empty());
                    assert(adj_after_u.remove(u@) =~= self.adj@);
                    assert(spec_sum_adj_sizes(adj_after_u) == self_sum);
                } else {
                    assert(adj_after_u =~= self.adj@);
                    assert(adj_after_u.dom().finite());
                }
                // After adding v@ (if not present):
                assert(new_adj@.dom().finite()) by {
                    assert(new_adj.spec_orderedtablemtper_wf());
                };
                let ghost v_in_adj_after_u = adj_after_u.dom().contains(v@);
                if !v_in_adj_after_u {
                    assert(new_adj@.dom().contains(v@));
                    lemma_sum_adj_remove(new_adj@, v@);
                    assert(new_adj@[v@] =~= Set::<<V as View>::V>::empty());
                    assert(new_adj@.remove(v@) =~= adj_after_u);
                    assert(spec_sum_adj_sizes(new_adj@) == spec_sum_adj_sizes(adj_after_u));
                } else {
                    assert(new_adj@ =~= adj_after_u);
                }
                assert(spec_sum_adj_sizes(new_adj@) == self_sum);

                // Step 2: Relate updated.adj@ sum to new_adj@ sum.
                lemma_sum_adj_remove(updated.adj@, u@);
                lemma_sum_adj_remove(new_adj@, u@);
                assert(updated.adj@.remove(u@) =~= new_adj@.remove(u@));
                // spec_sum_adj_sizes(updated.adj@)
                //   = updated.adj@[u@].len() + spec_sum_adj_sizes(new_adj@.remove(u@))
                //   = (u_neighbors@.insert(v@)).len() + (self_sum - u_neighbors@.len())
                // If had_edge (v@ in u_neighbors@): insert is idempotent, len unchanged.
                // If !had_edge: len increases by 1.
                if had_edge {
                    assert(u_neighbors@.contains(v@));
                    assert(u_neighbors@.insert(v@) =~= u_neighbors@);
                    assert(spec_sum_adj_sizes(updated.adj@) == self_sum);
                } else {
                    assert(!u_neighbors@.contains(v@));
                    vstd::set_lib::lemma_len_union(u_neighbors@, Set::<<V as View>::V>::empty().insert(v@));
                    assert(u_neighbors@.insert(v@).len() == u_neighbors@.len() + 1nat);
                    assert(spec_sum_adj_sizes(updated.adj@) == self_sum + 1);
                }

                // Graph closure on updated.
                assert forall|u2: <V as View>::V, w: <V as View>::V|
                    updated.spec_adj().dom().contains(u2)
                    && #[trigger] updated.spec_adj().index(u2).contains(w)
                    implies updated.spec_adj().dom().contains(w)
                by {
                    if u2 == u@ {
                        if w == v@ {
                            assert(new_adj@.dom().contains(v@));
                        } else {
                            assert(u_neighbors@.contains(w));
                            if u_in_orig {
                                assert(orig_adj.index(u@).contains(w));
                                assert(orig_adj.dom().contains(w));
                            }
                        }
                    } else {
                        assert(new_adj@.contains_key(u2));
                        assert(updated.adj@[u2] == new_adj@[u2]);
                        if orig_adj.contains_key(u2) {
                            assert(orig_adj.index(u2).contains(w));
                            assert(orig_adj.dom().contains(w));
                        }
                    }
                };
            }
            updated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + d), Span O(log n + d)
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
                    // Check whether edge exists before deleting.
                    assert_avltreesetmtper_always_wf(&u_neighbors);
                    let had_edge = u_neighbors.find(v);
                    let new_u_neighbors = u_neighbors.delete(v);
                    // delete ensures: new_u_neighbors@ == u_neighbors@.remove(v@), wf preserved.
                    let u_clone = u.clone();
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        crate::vstdplus::feq::feq::lemma_cloned_view_eq::<V>(*u, u_clone);
                    }
                    proof {
                        reveal(spec_sum_adj_sizes);
                        if had_edge {
                            // Edge exists → u_neighbors@ contains v@ → u_neighbors@.len() >= 1.
                            // spec_sum_adj_sizes(self.adj@) >= self.adj@[u@].len() >= 1.
                            lemma_sum_adj_remove(self.adj@, u@);
                            assert(u_neighbors@.contains(v@));
                            assert(u_neighbors@.len() >= 1);
                        }
                    }
                    let new_num_edges: usize = if had_edge { self.num_edges - 1 } else { self.num_edges };
                    let updated_inner = AdjTableGraphMtPer {
                        adj: self.adj.insert_wf(u_clone, new_u_neighbors),
                        num_edges: new_num_edges,
                    };
                    proof {
                        reveal(spec_sum_adj_sizes);
                        // Dom is same as self's dom (u@ already present).
                        assert(self.adj@.dom().insert(u@) =~= self.adj@.dom());

                        // Prove cached edge count.
                        assert(self.num_edges as nat == spec_sum_adj_sizes(self.adj@));
                        lemma_sum_adj_remove(updated_inner.adj@, u@);
                        lemma_sum_adj_remove(self.adj@, u@);
                        assert(updated_inner.adj@.remove(u@) =~= self.adj@.remove(u@));
                        // updated sum = (u_neighbors@.remove(v@)).len() + spec_sum_adj_sizes(self.adj@.remove(u@))
                        // self sum = u_neighbors@.len() + spec_sum_adj_sizes(self.adj@.remove(u@))
                        if had_edge {
                            assert(u_neighbors@.contains(v@));
                            vstd::set_lib::lemma_len_subset(u_neighbors@.remove(v@), u_neighbors@);
                            assert(u_neighbors@.remove(v@).len() == u_neighbors@.len() - 1);
                        } else {
                            assert(!u_neighbors@.contains(v@));
                            assert(u_neighbors@.remove(v@) =~= u_neighbors@);
                        }

                        // Graph closure on updated_inner.
                        assert forall|u2: <V as View>::V, w: <V as View>::V|
                            updated_inner.spec_adj().dom().contains(u2)
                            && #[trigger] updated_inner.spec_adj().index(u2).contains(w)
                            implies updated_inner.spec_adj().dom().contains(w)
                        by {
                            if u2 == u@ {
                                assert(u_neighbors@.contains(w));
                                assert(self.spec_adj().index(u@).contains(w));
                                assert(self.spec_adj().dom().contains(w));
                            } else {
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
                    let cloned = AdjTableGraphMtPer { adj: cloned_adj, num_edges: self.num_edges };
                    proof {
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
                        assert(!cloned.spec_adj().dom().contains(u@));
                    }
                    cloned
                }
            };
            updated
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<V: StTInMtT + Ord + TotalOrder + Clone + 'static> Clone for AdjTableGraphMtPer<V> {
        fn clone(&self) -> Self {
            AdjTableGraphMtPer { adj: self.adj.clone(), num_edges: self.num_edges }
        }
    }

    impl<V: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Debug for AdjTableGraphMtPer<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjTableGraphMtPer(vertices: {}, edges: {})", self.adj.size(), self.num_edges)
        }
    }

    impl<V: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Display for AdjTableGraphMtPer<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdjTableGraphMtPer(vertices: {}, edges: {})", self.adj.size(), self.num_edges)
        }
    }
}
