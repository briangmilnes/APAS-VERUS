//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Union-Find (Disjoint Set Union) — HashMap-based, Sequential Ephemeral.
//!
//! Generic UnionFind using HashMapWithViewPlus. Real proofs: spec_pure_find
//! returns canonical root, union merges sets. Rank-based termination for
//! spec_pure_find with decreases_when guard.

pub mod UnionFindStEph {

    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;

    use vstd::prelude::*;
    use vstd::set_lib::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

    verus! {

    #[verifier::reject_recursive_types(V)]
    pub struct UnionFind<V: StT + Hash + ClonePreservesView> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
    }

    // 6. spec fns

    /// Parent view: parent[k]@.
    pub open spec fn pv<V: View>(parent: Map<V::V, V>, k: V::V) -> V::V { parent[k]@ }

    /// Recursive root-chasing with rank-based termination.
    pub open spec fn spec_pure_find<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, v: V::V,
    ) -> V::V
        decreases n as int - (rank[v] as int),
    {
        decreases_when(
            parent.dom().contains(v)
            && rank.dom().contains(v)
            && (forall|k: V::V| #[trigger] parent.dom().contains(k) <==> rank.dom().contains(k))
            && (forall|k: V::V| #[trigger] parent.dom().contains(k) ==>
                    parent.dom().contains(pv::<V>(parent, k)))
            && (forall|k: V::V| parent.dom().contains(k) && pv::<V>(parent, k) != k ==>
                    (#[trigger] rank[k] as int) < (rank[pv::<V>(parent, k)] as int))
            && (forall|k: V::V| parent.dom().contains(k) ==>
                    (#[trigger] rank[k] as int) < n as int)
        );
        if !parent.dom().contains(v) { v }
        else if pv::<V>(parent, v) == v { v }
        else { spec_pure_find::<V>(parent, rank, n, pv::<V>(parent, v)) }
    }

    pub open spec fn spec_is_root_map<V: View>(parent: Map<V::V, V>, v: V::V) -> bool {
        parent.dom().contains(v) && pv::<V>(parent, v) == v
    }

    /// Subtree: set of elements whose root is `root`.
    pub open spec fn spec_subtree<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, root: V::V,
    ) -> Set<V::V> {
        parent.dom().filter(|k: V::V| spec_pure_find::<V>(parent, rank, n, k) == root)
    }

    /// Size-rank invariant: each root's subtree has >= rank + 1 elements.
    pub open spec fn spec_size_rank_inv_map<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
    ) -> bool {
        forall|r: V::V| parent.dom().contains(r) && pv::<V>(parent, r) == r ==>
            spec_subtree::<V>(parent, rank, n, r).finite()
            && spec_subtree::<V>(parent, rank, n, r).len() >= (#[trigger] rank[r] as nat) + 1
    }

    /// Well-formedness.
    pub open spec fn spec_uf_wf<V: StT + Hash + ClonePreservesView>(uf: &UnionFind<V>) -> bool {
        &&& obeys_key_model::<V>()
        &&& obeys_feq_view_injective::<V>()
        &&& obeys_feq_full::<V>()
        &&& uf.parent@.dom().finite()
        &&& forall|k: V::V| #[trigger] uf.parent@.dom().contains(k) <==> uf.rank@.dom().contains(k)
        &&& forall|k: V::V| #[trigger] uf.parent@.dom().contains(k) ==>
                uf.parent@.dom().contains(pv::<V>(uf.parent@, k))
        // Rank invariant.
        &&& forall|k: V::V| uf.parent@.dom().contains(k) && pv::<V>(uf.parent@, k) != k ==>
                (#[trigger] uf.rank@[k] as int) < (uf.rank@[pv::<V>(uf.parent@, k)] as int)
        // Rank bounded by n.
        &&& forall|k: V::V| uf.parent@.dom().contains(k) ==>
                (#[trigger] uf.rank@[k] as int) < (uf.parent@.dom().len() as int)
        // Size-rank invariant.
        &&& spec_size_rank_inv_map::<V>(uf.parent@, uf.rank@, uf.parent@.dom().len())
    }

    // 7. proof fns

    /// spec_pure_find result is in the domain.
    proof fn lemma_find_in_dom<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, v: V::V,
    )
        requires
            parent.dom().contains(v),
            forall|k: V::V| #[trigger] parent.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] parent.dom().contains(k) ==>
                parent.dom().contains(pv::<V>(parent, k)),
            forall|k: V::V| parent.dom().contains(k) && pv::<V>(parent, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(parent, k)] as int),
            forall|k: V::V| parent.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
        ensures parent.dom().contains(spec_pure_find::<V>(parent, rank, n, v)),
        decreases n as int - (rank[v] as int),
    {
        if pv::<V>(parent, v) != v {
            let ghost pv_v = pv::<V>(parent, v);
            assert(parent.dom().contains(pv_v));
            assert(rank.dom().contains(pv_v));
            lemma_find_in_dom::<V>(parent, rank, n, pv_v);
        }
    }

    /// spec_pure_find returns a root.
    proof fn lemma_find_is_root<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, v: V::V,
    )
        requires
            parent.dom().contains(v),
            forall|k: V::V| #[trigger] parent.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] parent.dom().contains(k) ==>
                parent.dom().contains(pv::<V>(parent, k)),
            forall|k: V::V| parent.dom().contains(k) && pv::<V>(parent, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(parent, k)] as int),
            forall|k: V::V| parent.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
        ensures spec_is_root_map::<V>(parent, spec_pure_find::<V>(parent, rank, n, v)),
        decreases n as int - (rank[v] as int),
    {
        if pv::<V>(parent, v) != v {
            let ghost pv_v = pv::<V>(parent, v);
            assert(parent.dom().contains(pv_v));
            assert(rank.dom().contains(pv_v));
            lemma_find_is_root::<V>(parent, rank, n, pv_v);
        }
    }

    /// Trichotomy after linking root_a under root_b.
    #[verifier::rlimit(20)]
    proof fn lemma_find_after_link<V: View>(
        po: Map<V::V, V>, ro: Map<V::V, usize>,
        pn: Map<V::V, V>, rn: Map<V::V, usize>,
        n: nat, ra: V::V, rb: V::V, z: V::V,
    )
        requires
            po.dom().contains(z), po.dom().contains(ra), po.dom().contains(rb),
            pn.dom().contains(z), rn.dom().contains(z), ro.dom().contains(z),
            ra != rb,
            pv::<V>(po, ra) == ra, pv::<V>(po, rb) == rb,
            pv::<V>(pn, ra) == rb, pv::<V>(pn, rb) == rb,
            forall|k: V::V| po.dom().contains(k) && k != ra ==> pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            // Old invariants.
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> ro.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) ==> po.dom().contains(pv::<V>(po, k)),
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] ro[k] as int) < (ro[pv::<V>(po, k)] as int),
            forall|k: V::V| po.dom().contains(k) ==> (#[trigger] ro[k] as int) < n as int,
            // New invariants.
            forall|k: V::V| #[trigger] pn.dom().contains(k) <==> rn.dom().contains(k),
            forall|k: V::V| #[trigger] pn.dom().contains(k) ==> pn.dom().contains(pv::<V>(pn, k)),
            forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k ==>
                (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int),
            forall|k: V::V| pn.dom().contains(k) ==> (#[trigger] rn[k] as int) < n as int,
        ensures ({
            let fo = spec_pure_find::<V>(po, ro, n, z);
            let fn_ = spec_pure_find::<V>(pn, rn, n, z);
            if fo == ra { fn_ == rb }
            else if fo == rb { fn_ == rb }
            else { fn_ == fo }
        }),
        decreases n as int - (ro[z] as int),
    {
        assert(ro.dom().contains(z));
        if pv::<V>(po, z) == z {
            if z == ra {
                // ra is a root in old. In new: pv(pn, ra) = rb.
                // rb is a root in new: pv(pn, rb) = rb.
                // Need: find(pn, rn, n, ra) = rb.
                assert(pn.dom().contains(rb));
                assert(rn.dom().contains(rb));
                assert(pv::<V>(pn, rb) == rb);
                // Help Z3 unfold find at rb: since pv(pn, rb) == rb, find returns rb.
                // (Asserting domain membership triggers the decreases_when guard.)
                assert(spec_pure_find::<V>(pn, rn, n, rb) == rb);
                // find(ra): pv(pn, ra) = rb != ra, recurse on rb.
                assert(pn.dom().contains(ra));
                assert(rn.dom().contains(ra));
                assert(pv::<V>(pn, ra) == rb);
            } else if z == rb {
                assert(pn.dom().contains(rb));
                assert(rn.dom().contains(rb));
                assert(pv::<V>(pn, rb) == rb);
            } else {
                assert(pn.dom().contains(z));
                assert(pv::<V>(pn, z) == z);
            }
        } else {
            assert(z != ra);
            let ghost pz = pv::<V>(po, z);
            assert(po.dom().contains(pz));
            assert(pn.dom().contains(pz));
            assert(ro.dom().contains(pz));
            assert(rn.dom().contains(pz));
            assert(pv::<V>(pn, z) == pz);
            lemma_find_after_link::<V>(po, ro, pn, rn, n, ra, rb, pz);
        }
    }

    /// After insert of new element v, find is unchanged for old elements.
    proof fn lemma_find_insert_unchanged<V: View>(
        po: Map<V::V, V>, ro: Map<V::V, usize>, n_old: nat,
        pn: Map<V::V, V>, rn: Map<V::V, usize>, n_new: nat,
        v: V::V, k: V::V,
    )
        requires
            po.dom().contains(k), !po.dom().contains(v),
            n_new == n_old + 1,
            pn == po.insert(v, pn[v]), rn == ro.insert(v, 0),
            pv::<V>(pn, v) == v,
            // Old invariants.
            forall|j: V::V| #[trigger] po.dom().contains(j) <==> ro.dom().contains(j),
            forall|j: V::V| #[trigger] po.dom().contains(j) ==> po.dom().contains(pv::<V>(po, j)),
            forall|j: V::V| po.dom().contains(j) && pv::<V>(po, j) != j ==>
                (#[trigger] ro[j] as int) < (ro[pv::<V>(po, j)] as int),
            forall|j: V::V| po.dom().contains(j) ==> (#[trigger] ro[j] as int) < n_old as int,
            // New invariants.
            forall|j: V::V| #[trigger] pn.dom().contains(j) <==> rn.dom().contains(j),
            forall|j: V::V| #[trigger] pn.dom().contains(j) ==> pn.dom().contains(pv::<V>(pn, j)),
            forall|j: V::V| pn.dom().contains(j) && pv::<V>(pn, j) != j ==>
                (#[trigger] rn[j] as int) < (rn[pv::<V>(pn, j)] as int),
            forall|j: V::V| pn.dom().contains(j) ==> (#[trigger] rn[j] as int) < n_new as int,
        ensures
            spec_pure_find::<V>(pn, rn, n_new, k) == spec_pure_find::<V>(po, ro, n_old, k),
        decreases n_old as int - (ro[k] as int),
    {
        assert(ro.dom().contains(k));
        assert(pn.dom().contains(k));
        assert(rn.dom().contains(k));
        // For old k: pn[k] = po[k], rn[k] = ro[k] (since k != v).
        assert(k != v);
        assert(pv::<V>(pn, k) == pv::<V>(po, k));
        if pv::<V>(po, k) == k {
            // Root in old state. pv(pn, k) = pv(po, k) = k. Root in new state too.
            // find(pn, rn, n_new, k) = k = find(po, ro, n_old, k).
        } else {
            let ghost pk = pv::<V>(po, k);
            assert(po.dom().contains(pk));
            assert(ro.dom().contains(pk));
            lemma_find_insert_unchanged::<V>(po, ro, n_old, pn, rn, n_new, v, pk);
        }
    }

    /// Two distinct roots of equal rank: rank + 1 < n.
    proof fn lemma_rank_lt_n_minus_1<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
        ru: V::V, rv: V::V,
    )
        requires
            parent.dom().finite(),
            parent.dom().contains(ru), parent.dom().contains(rv),
            ru != rv,
            pv::<V>(parent, ru) == ru, pv::<V>(parent, rv) == rv,
            rank[ru] == rank[rv],
            n == parent.dom().len(),
            spec_size_rank_inv_map::<V>(parent, rank, n),
        ensures (rank[ru] as int) + 1 < n as int,
    {
        let r = rank[ru];
        let su = spec_subtree::<V>(parent, rank, n, ru);
        let sv = spec_subtree::<V>(parent, rank, n, rv);
        // From size_rank_inv: su and sv are finite with len >= r + 1.
        assert(su.finite() && su.len() >= r as nat + 1);
        assert(sv.finite() && sv.len() >= r as nat + 1);
        // su and sv are disjoint: any k has a unique find result.
        assert(su.disjoint(sv)) by {
            assert forall|k: V::V| !(su.contains(k) && sv.contains(k)) by {
                // su.contains(k) => find(k) == ru
                // sv.contains(k) => find(k) == rv
                // But find(k) can't be both ru and rv (ru != rv).
            }
        }
        // su and sv are subsets of parent.dom().
        assert(su.subset_of(parent.dom())) by {
            assert forall|k: V::V| su.contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        assert(sv.subset_of(parent.dom())) by {
            assert forall|k: V::V| sv.contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        // (su + sv) is a subset of dom.
        assert((su + sv).subset_of(parent.dom())) by {
            assert forall|k: V::V| (su + sv).contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        // |su + sv| = |su| + |sv| (disjoint).
        lemma_set_disjoint_lens(su, sv);
        assert((su + sv).len() == su.len() + sv.len());
        // |su + sv| <= |dom| = n.
        assert((su + sv).finite()) by {
            lemma_len_subset::<V::V>(su + sv, parent.dom());
        }
        lemma_len_subset::<V::V>(su + sv, parent.dom());
        assert((su + sv).len() <= n);
        // 2*(r + 1) <= su.len() + sv.len() = |su + sv| <= n.
        // So r + 1 <= n/2. Since n >= 2*(r+1) >= 2, r + 1 < n.
    }

    // 8. traits

    pub trait UnionFindStEphTrait<V: StT + Hash + ClonePreservesView>: Sized {
        spec fn spec_wf(&self) -> bool;
        spec fn spec_contains(&self, v: V::V) -> bool;
        spec fn spec_find(&self, v: V::V) -> V::V;
        spec fn spec_same_set(&self, u: V::V, v: V::V) -> bool;
        spec fn spec_is_root(&self, v: V::V) -> bool;
        spec fn spec_n(&self) -> nat;

        fn new() -> (uf: Self)
            requires obeys_key_model::<V>(), obeys_feq_view_injective::<V>(), obeys_feq_full::<V>(),
            ensures uf.spec_wf(), uf.spec_n() == 0;

        fn insert(&mut self, v: V)
            requires old(self).spec_wf(), !old(self).spec_contains(v@),
            ensures self.spec_wf(), self.spec_contains(v@);

        fn find(&self, v: &V) -> (root: V)
            requires self.spec_wf(), self.spec_contains(v@),
            ensures self.spec_contains(root@), root@ == self.spec_find(v@),
                self.spec_is_root(root@);

        fn union_sets(&mut self, u: &V, v: &V)
            requires old(self).spec_wf(), old(self).spec_contains(u@), old(self).spec_contains(v@),
            ensures self.spec_wf(), self.spec_same_set(u@, v@);

        fn equals(&self, u: &V, v: &V) -> (eq: bool)
            requires self.spec_wf(), self.spec_contains(u@), self.spec_contains(v@),
            ensures eq == self.spec_same_set(u@, v@);

        fn size(&self) -> (n: usize) requires self.spec_wf(), ensures n as nat == self.spec_n();
    }

    // 9. impls

    impl<V: StT + Hash + ClonePreservesView> UnionFindStEphTrait<V> for UnionFind<V> {
        open spec fn spec_wf(&self) -> bool { spec_uf_wf(self) }
        open spec fn spec_contains(&self, v: V::V) -> bool { self.parent@.dom().contains(v) }
        open spec fn spec_n(&self) -> nat { self.parent@.dom().len() }
        open spec fn spec_find(&self, v: V::V) -> V::V {
            spec_pure_find::<V>(self.parent@, self.rank@, self.spec_n(), v)
        }
        open spec fn spec_same_set(&self, u: V::V, v: V::V) -> bool {
            self.spec_find(u) == self.spec_find(v)
        }
        open spec fn spec_is_root(&self, v: V::V) -> bool {
            spec_is_root_map::<V>(self.parent@, v)
        }

        fn new() -> (uf: Self) {
            UnionFind { parent: HashMapWithViewPlus::new(), rank: HashMapWithViewPlus::new() }
        }

        fn insert(&mut self, v: V) {
            let ghost vv = v@;
            let ghost po = self.parent@;
            let ghost ro = self.rank@;
            let ghost n_old = self.spec_n();
            let v2 = v.clone_view();
            self.parent.insert(v.clone_view(), v);
            self.rank.insert(v2, 0usize);
            proof {
                let pn = self.parent@;
                let rn = self.rank@;
                let n_new = self.spec_n();
                // Parent-in-domain for new state.
                assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                    pn.dom().contains(pv::<V>(pn, k))
                by {
                    if k == vv { assert(pv::<V>(pn, k) == vv); }
                    else {
                        assert(po.dom().contains(k));
                        assert(pv::<V>(pn, k) == pv::<V>(po, k));
                    }
                }
                // Rank invariant for new state.
                assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                    (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                by {
                    if k == vv {
                        assert(pv::<V>(pn, vv) == vv); // root, so antecedent is false
                    } else {
                        assert(pv::<V>(pn, k) == pv::<V>(po, k));
                        assert(rn[k] == ro[k]);
                        let ghost pk = pv::<V>(po, k);
                        if pk == vv {
                            // parent of k is vv in old state? No — vv not in old domain.
                            assert(po.dom().contains(pk));
                            assert(false); // vv not in old domain.
                        } else {
                            assert(rn[pk] == ro[pk]);
                        }
                    }
                }
                // Rank bounded for new state.
                assert forall|k: V::V| pn.dom().contains(k) implies
                    (#[trigger] rn[k] as int) < (n_new as int)
                by {
                    if k == vv {
                        assert(rn[vv] == 0usize);
                        assert(n_new >= 1); // at least vv is in domain
                    } else {
                        assert((ro[k] as int) < (n_old as int));
                        assert(rn[k] == ro[k]);
                    }
                }
                // Size-rank invariant for new state.
                assert(spec_size_rank_inv_map::<V>(pn, rn, n_new)) by {
                    assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                        spec_subtree::<V>(pn, rn, n_new, r).finite()
                        && spec_subtree::<V>(pn, rn, n_new, r).len() >= (#[trigger] rn[r] as nat) + 1
                    by {
                        if r == vv {
                            // New element: root with rank 0. Subtree = {vv}.
                            let st = spec_subtree::<V>(pn, rn, n_new, vv);
                            // vv is in subtree: find(pn, rn, n_new, vv) = vv.
                            assert(pn.dom().contains(vv));
                            assert(pv::<V>(pn, vv) == vv);
                            assert(spec_pure_find::<V>(pn, rn, n_new, vv) == vv);
                            assert(st.contains(vv));
                            // st is subset of pn.dom() which is finite.
                            assert(st.subset_of(pn.dom())) by {
                                assert forall|k: V::V| st.contains(k) implies
                                    #[trigger] pn.dom().contains(k) by {}
                            }
                            lemma_len_subset::<V::V>(st, pn.dom());
                            // st has at least vv, so len >= 1 = rank + 1.
                            assert(rn[vv] == 0usize);
                        } else {
                            // Old root r. Subtree in new state = subtree in old state.
                            let st_new = spec_subtree::<V>(pn, rn, n_new, r);
                            let st_old = spec_subtree::<V>(po, ro, n_old, r);
                            // Show st_new =~= st_old.
                            assert(st_new =~= st_old) by {
                                assert forall|k: V::V|
                                    st_new.contains(k) == st_old.contains(k)
                                by {
                                    if po.dom().contains(k) {
                                        assert(k != vv);
                                        lemma_find_insert_unchanged::<V>(
                                            po, ro, n_old, pn, rn, n_new, vv, k,
                                        );
                                    }
                                    // k == vv: find(pn, rn, n_new, vv) = vv != r (vv not in old dom, r is old root).
                                    // k not in old dom and k != vv: not in new dom either.
                                }
                            }
                            // Old size_rank_inv: st_old.finite() && st_old.len() >= ro[r] + 1.
                            assert(rn[r] == ro[r]);
                        }
                    }
                }
            }
        }

        fn find(&self, v: &V) -> (root: V) {
            let mut curr = v.clone_view();
            let n = self.parent.len();
            let mut steps: usize = 0;
            while steps < n
                invariant
                    self.spec_wf(),
                    self.parent@.dom().contains(curr@),
                    self.parent@.dom().contains(v@),
                    spec_pure_find::<V>(self.parent@, self.rank@, self.spec_n(), curr@)
                        == spec_pure_find::<V>(self.parent@, self.rank@, self.spec_n(), v@),
                    steps <= n, n == self.spec_n(),
                    (self.rank@[curr@] as int) >= steps as int,
                decreases n - steps,
            {
                let p = self.parent.get(&curr);
                match p {
                    Some(parent_val) => {
                        let is_root = feq(parent_val, &curr);
                        if is_root {
                            proof {
                                assert(parent_val@ == curr@);
                                assert(pv::<V>(self.parent@, curr@) == curr@);
                                lemma_find_in_dom::<V>(self.parent@, self.rank@, self.spec_n(), v@);
                                lemma_find_is_root::<V>(self.parent@, self.rank@, self.spec_n(), v@);
                            }
                            return curr;
                        }
                        // Not root: pv(parent, curr@) = parent_val@ != curr@.
                        // rank[parent_val@] > rank[curr@] >= steps.
                        // After update: rank[new_curr@] >= steps + 1.
                        proof {
                            assert(parent_val@ != curr@);
                            assert(pv::<V>(self.parent@, curr@) == parent_val@);
                            assert(self.parent@.dom().contains(parent_val@));
                            assert(self.rank@.dom().contains(parent_val@));
                            assert((self.rank@[parent_val@] as int) > (self.rank@[curr@] as int));
                        }
                        curr = parent_val.clone_view();
                        steps = steps + 1;
                    },
                    None => {
                        proof { assert(false); } // unreachable: curr in domain
                        return curr;
                    },
                }
            }
            // Unreachable: rank[curr] >= steps == n, but rank_bounded says rank[curr] < n.
            proof {
                assert((self.rank@[curr@] as int) >= n as int);
                assert((self.rank@[curr@] as int) < (self.spec_n() as int));
                assert(false);
            }
            curr
        }

        #[verifier::rlimit(30)]
        fn union_sets(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            let root_v = self.find(v);
            let same = feq(&root_u, &root_v);
            if same { return; }
            let rank_u = *self.rank.get(&root_u).unwrap_or(&0);
            let rank_v = *self.rank.get(&root_v).unwrap_or(&0);
            // Root facts from find() postcondition.
            proof {
                assert(self.spec_is_root(root_u@));
                assert(self.spec_is_root(root_v@));
                assert(pv::<V>(self.parent@, root_u@) == root_u@);
                assert(pv::<V>(self.parent@, root_v@) == root_v@);
            }
            let ghost po = self.parent@;
            let ghost ro = self.rank@;
            let ghost n = self.spec_n();
            proof {
                // Connect exec rank reads to ghost map.
                assert(ro.dom().contains(root_u@));
                assert(ro.dom().contains(root_v@));
                assert(rank_u == ro[root_u@]);
                assert(rank_v == ro[root_v@]);
            }
            if rank_u < rank_v {
                // Link root_u under root_v.
                self.parent.insert(root_u.clone_view(), root_v);
                proof {
                    let pn = self.parent@;
                    let rn = self.rank@;
                    assert(rn == ro);
                    assert(pv::<V>(pn, root_u@) == root_v@);
                    // Domain unchanged (insert on existing key).
                    assert forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k) by {}
                    // Parent-in-domain.
                    assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                        pn.dom().contains(pv::<V>(pn, k))
                    by {
                        if k == root_u@ { assert(pv::<V>(pn, k) == root_v@); }
                        else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    // Rank invariant.
                    assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                        (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                    by {
                        if k == root_u@ {} else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    // Rank bounded (unchanged).
                    // Size-rank invariant.
                    assert(spec_size_rank_inv_map::<V>(pn, rn, n)) by {
                        assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                            spec_subtree::<V>(pn, rn, n, r).finite()
                            && spec_subtree::<V>(pn, rn, n, r).len() >= (#[trigger] rn[r] as nat) + 1
                        by {
                            assert(r != root_u@); // root_u is no longer a root
                            let st_new = spec_subtree::<V>(pn, rn, n, r);
                            if r == root_v@ {
                                // Winner: subtree = old subtree(root_u) + old subtree(root_v).
                                let st_old_u = spec_subtree::<V>(po, ro, n, root_u@);
                                let st_old_v = spec_subtree::<V>(po, ro, n, root_v@);
                                assert(st_new =~= st_old_u + st_old_v) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == (st_old_u + st_old_v).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, k);
                                        }
                                    }
                                }
                                // Both old subtrees are finite (from old size_rank_inv).
                                assert(st_old_u.finite());
                                assert(st_old_v.finite());
                                // Disjoint: find maps to unique root.
                                assert(st_old_u.disjoint(st_old_v)) by {
                                    assert forall|k: V::V| !(st_old_u.contains(k) && st_old_v.contains(k)) by {}
                                }
                                lemma_set_disjoint_lens(st_old_u, st_old_v);
                                assert(st_new.len() == st_old_u.len() + st_old_v.len());
                                assert(rn.dom().contains(root_v@));
                                assert(ro.dom().contains(root_v@));
                                assert(rn[root_v@] == ro[root_v@]);
                            } else {
                                // Other root: subtree unchanged.
                                let st_old = spec_subtree::<V>(po, ro, n, r);
                                assert(st_new =~= st_old) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == st_old.contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r));
                                assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    // Trichotomy for u and v.
                    assert(po.dom().contains(u@));
                    assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@));
                    assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@));
                    assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@));
                    assert(ro.dom().contains(v@));
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, v@);
                }
            } else if rank_u > rank_v {
                // Link root_v under root_u.
                self.parent.insert(root_v.clone_view(), root_u);
                proof {
                    let pn = self.parent@;
                    let rn = self.rank@;
                    assert(rn == ro);
                    assert(pv::<V>(pn, root_v@) == root_u@);
                    assert forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k) by {}
                    assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                        pn.dom().contains(pv::<V>(pn, k))
                    by {
                        if k == root_v@ { assert(pv::<V>(pn, k) == root_u@); }
                        else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                        (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                    by {
                        if k == root_v@ {} else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    // Size-rank invariant.
                    assert(spec_size_rank_inv_map::<V>(pn, rn, n)) by {
                        assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                            spec_subtree::<V>(pn, rn, n, r).finite()
                            && spec_subtree::<V>(pn, rn, n, r).len() >= (#[trigger] rn[r] as nat) + 1
                        by {
                            assert(r != root_v@);
                            let st_new = spec_subtree::<V>(pn, rn, n, r);
                            if r == root_u@ {
                                let st_old_v = spec_subtree::<V>(po, ro, n, root_v@);
                                let st_old_u = spec_subtree::<V>(po, ro, n, root_u@);
                                assert(st_new =~= st_old_v + st_old_u) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == (st_old_v + st_old_u).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(st_old_v.finite());
                                assert(st_old_u.finite());
                                assert(st_old_v.disjoint(st_old_u)) by {
                                    assert forall|k: V::V| !(st_old_v.contains(k) && st_old_u.contains(k)) by {}
                                }
                                lemma_set_disjoint_lens(st_old_v, st_old_u);
                                assert(rn.dom().contains(root_u@));
                                assert(ro.dom().contains(root_u@));
                                assert(rn[root_u@] == ro[root_u@]);
                            } else {
                                let st_old = spec_subtree::<V>(po, ro, n, r);
                                assert(st_new =~= st_old) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == st_old.contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r));
                                assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    assert(po.dom().contains(u@));
                    assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@));
                    assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@));
                    assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@));
                    assert(ro.dom().contains(v@));
                    assert(pv::<V>(pn, root_u@) == pv::<V>(po, root_u@));
                    assert(pv::<V>(pn, root_u@) == root_u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, v@);
                }
            } else {
                // Equal rank. Link root_v under root_u, increment root_u's rank.
                let root_u2 = root_u.clone_view();
                self.parent.insert(root_v.clone_view(), root_u.clone_view());
                // Prove rank_u + 1 < n using size-rank invariant.
                let n_len = self.parent.len();
                proof {
                    assert(po.dom().finite());
                    assert(po.dom().contains(root_u@));
                    assert(po.dom().contains(root_v@));
                    assert(ro.dom().contains(root_u@));
                    assert(ro.dom().contains(root_v@));
                    lemma_rank_lt_n_minus_1::<V>(po, ro, n, root_u@, root_v@);
                    assert((ro[root_u@] as int) + 1 < (n as int));
                    assert((rank_u as int) + 1 < (n as int));
                    assert(rank_u < n_len);
                }
                self.rank.insert(root_u2, rank_u + 1);
                proof {
                    let pn = self.parent@;
                    let rn = self.rank@;
                    assert(pv::<V>(pn, root_v@) == root_u@);
                    assert forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k) by {}
                    assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                        pn.dom().contains(pv::<V>(pn, k))
                    by {
                        if k == root_v@ { assert(pv::<V>(pn, k) == root_u@); }
                        else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    // Domain equality for new rank map (needed early for Map lookups).
                    assert forall|k: V::V| #[trigger] pn.dom().contains(k) <==> rn.dom().contains(k) by {
                        assert(po.dom().contains(k) <==> ro.dom().contains(k));
                    }
                    assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                        (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                    by {
                        assert(rn.dom().contains(k));
                        if k == root_v@ {
                            assert(pv::<V>(pn, k) == root_u@);
                            assert(rn.dom().contains(root_u@));
                        } else {
                            assert(pv::<V>(pn, k) == pv::<V>(po, k));
                            assert(rn.dom().contains(pv::<V>(pn, k)));
                        }
                    }
                    // Rank bounded: rn[root_u@] = rank_u + 1 < n.
                    assert forall|k: V::V| pn.dom().contains(k) implies
                        (#[trigger] rn[k] as int) < (n as int)
                    by {
                        assert(rn.dom().contains(k));
                        if k == root_u@ {
                            assert(rn[root_u@] == rank_u + 1);
                        } else {
                            assert(ro.dom().contains(k));
                            assert(rn[k] == ro[k]);
                        }
                    }
                    // Size-rank invariant.
                    assert(spec_size_rank_inv_map::<V>(pn, rn, n)) by {
                        assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                            spec_subtree::<V>(pn, rn, n, r).finite()
                            && spec_subtree::<V>(pn, rn, n, r).len() >= (#[trigger] rn[r] as nat) + 1
                        by {
                            assert(r != root_v@);
                            let st_new = spec_subtree::<V>(pn, rn, n, r);
                            if r == root_u@ {
                                let st_old_v = spec_subtree::<V>(po, ro, n, root_v@);
                                let st_old_u = spec_subtree::<V>(po, ro, n, root_u@);
                                assert(st_new =~= st_old_v + st_old_u) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == (st_old_v + st_old_u).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(st_old_v.finite());
                                assert(st_old_u.finite());
                                assert(st_old_v.disjoint(st_old_u)) by {
                                    assert forall|k: V::V| !(st_old_v.contains(k) && st_old_u.contains(k)) by {}
                                }
                                lemma_set_disjoint_lens(st_old_v, st_old_u);
                                assert(rn.dom().contains(root_u@));
                                assert(rn[root_u@] == rank_u + 1);
                            } else {
                                let st_old = spec_subtree::<V>(po, ro, n, r);
                                assert(st_new =~= st_old) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == st_old.contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r));
                                assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    assert(po.dom().contains(u@));
                    assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@));
                    assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@));
                    assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@));
                    assert(ro.dom().contains(v@));
                    assert(pv::<V>(pn, root_u@) == pv::<V>(po, root_u@));
                    assert(pv::<V>(pn, root_u@) == root_u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, v@);
                }
            }
        }

        fn equals(&self, u: &V, v: &V) -> (eq: bool) {
            let root_u = self.find(u);
            let root_v = self.find(v);
            feq(&root_u, &root_v)
        }

        fn size(&self) -> (n: usize) { self.parent.len() }
    }

    } // verus!

    impl<V: StT + Hash + ClonePreservesView> Debug for UnionFind<V> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "UnionFind(n={})", self.parent.len())
        }
    }

    impl<V: StT + Hash + ClonePreservesView> Display for UnionFind<V> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "UnionFind(n={})", self.parent.len())
        }
    }

} // mod
