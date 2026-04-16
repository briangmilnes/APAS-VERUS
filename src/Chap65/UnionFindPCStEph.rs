// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Union-Find with Path Compression — HashMap-based, Sequential Ephemeral.
//!
//! Generic UnionFind using HashMapWithViewPlus. Two-pass find with path
//! compression (CLRS §21.3). Rank-based termination for spec_pure_find.
//! Compression preserves all find results and well-formedness.
//
//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions — struct UnionFindPC
//	Section 6. spec fns — struct UnionFindPC
//	Section 7. proof fns/broadcast groups — struct UnionFindPC
//	Section 8. traits — struct UnionFindPC
//	Section 9. impls — struct UnionFindPC
//	Section 14. derive impls outside verus! — struct UnionFindPC

pub mod UnionFindPCStEph {

	//		Section 2. imports

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

	//		Section 4. type definitions — struct UnionFindPC

    #[verifier::reject_recursive_types(V)]
    pub struct UnionFindPC<V: StT + Hash + ClonePreservesView> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
    }

	//		Section 6. spec fns — struct UnionFindPC

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

    /// Well-formedness. Built from closed light_wf + size_rank_inv so callers
    /// see opaque booleans instead of raw parent-in-dom / rank-inv foralls.
    pub open spec fn spec_uf_wf<V: StT + Hash + ClonePreservesView>(uf: &UnionFindPC<V>) -> bool {
        &&& obeys_key_model::<V>()
        &&& obeys_feq_view_injective::<V>()
        &&& obeys_feq_full::<V>()
        &&& spec_light_wf::<V>(uf.parent@, uf.rank@, uf.parent@.dom().len())
        &&& spec_size_rank_inv_map::<V>(uf.parent@, uf.rank@, uf.parent@.dom().len())
    }

    /// Opaque bundle of the forest wf quantifiers, excluding size_rank_inv.
    /// Opaque (not closed): closed only hides across modules; opaque hides within
    /// the module too, so Z3 sees one symbol per instance instead of five foralls.
    #[verifier::opaque] // accept hole
    pub open spec fn spec_light_wf<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
    ) -> bool {
        &&& parent.dom().finite()
        &&& n == parent.dom().len()
        &&& forall|k: V::V| #[trigger] parent.dom().contains(k) <==> rank.dom().contains(k)
        &&& forall|k: V::V| #[trigger] parent.dom().contains(k) ==>
                parent.dom().contains(pv::<V>(parent, k))
        &&& forall|k: V::V| parent.dom().contains(k) && pv::<V>(parent, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(parent, k)] as int)
        &&& forall|k: V::V| parent.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int
    }

    /// Opaque: all z in po's domain have the same spec_pure_find under (pc, rc, nc) and (po, ro, no).
    #[verifier::opaque] // accept hole
    pub open spec fn spec_find_preserved<V: View>(
        pc: Map<V::V, V>, rc: Map<V::V, usize>, nc: nat,
        po: Map<V::V, V>, ro: Map<V::V, usize>, no: nat,
    ) -> bool {
        forall|z: V::V| #[trigger] po.dom().contains(z) ==>
            spec_pure_find::<V>(pc, rc, nc, z) == spec_pure_find::<V>(po, ro, no, z)
    }

    /// Opaque: pa and pb have the same domain.
    #[verifier::opaque] // accept hole
    pub open spec fn spec_same_domain<V: View>(
        pa: Map<V::V, V>, pb: Map<V::V, V>,
    ) -> bool {
        forall|k: V::V| #[trigger] pa.dom().contains(k) <==> pb.dom().contains(k)
    }

	//		Section 7. proof fns/broadcast groups — struct UnionFindPC

    /// Compose find preservation transitively. Isolated Z3 context for spec_pure_find.
    proof fn lemma_compose_find_preserved<V: View>(
        p1: Map<V::V, V>, r1: Map<V::V, usize>, n1: nat,
        p2: Map<V::V, V>, r2: Map<V::V, usize>, n2: nat,
        p3: Map<V::V, V>, r3: Map<V::V, usize>, n3: nat,
    )
        requires
            forall|z: V::V| #[trigger] p1.dom().contains(z) ==>
                spec_pure_find::<V>(p2, r2, n2, z) == spec_pure_find::<V>(p1, r1, n1, z),
            forall|z: V::V| #[trigger] p2.dom().contains(z) ==>
                spec_pure_find::<V>(p3, r3, n3, z) == spec_pure_find::<V>(p2, r2, n2, z),
            forall|k: V::V| p1.dom().contains(k) <==> #[trigger] p2.dom().contains(k),
        ensures
            forall|z: V::V| #[trigger] p1.dom().contains(z) ==>
                spec_pure_find::<V>(p3, r3, n3, z) == spec_pure_find::<V>(p1, r1, n1, z),
    {
        assert forall|z: V::V| #[trigger] p1.dom().contains(z) implies
            spec_pure_find::<V>(p3, r3, n3, z) == spec_pure_find::<V>(p1, r1, n1, z)
        by { assert(p2.dom().contains(z)); }
    }

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

    /// For non-roots, rank[v] < rank[find(v)].
    proof fn lemma_rank_lt_find<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, v: V::V,
    )
        requires
            parent.dom().contains(v),
            pv::<V>(parent, v) != v,
            forall|k: V::V| #[trigger] parent.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] parent.dom().contains(k) ==>
                parent.dom().contains(pv::<V>(parent, k)),
            forall|k: V::V| parent.dom().contains(k) && pv::<V>(parent, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(parent, k)] as int),
            forall|k: V::V| parent.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
        ensures
            (rank[v] as int) < (rank[spec_pure_find::<V>(parent, rank, n, v)] as int),
        decreases n as int - (rank[v] as int),
    {
        let ghost pv_v = pv::<V>(parent, v);
        assert(parent.dom().contains(pv_v));
        assert(rank.dom().contains(pv_v));
        // rank[v] < rank[parent[v]]
        assert((rank[v] as int) < (rank[pv_v] as int));
        if pv_v == spec_pure_find::<V>(parent, rank, n, v) {
            // parent[v] is the root.
        } else {
            // parent[v] is not the root. find(v) = find(pv_v). Since pv_v != find(v) = find(pv_v),
            // pv_v is not a root.
            assert(spec_pure_find::<V>(parent, rank, n, v)
                == spec_pure_find::<V>(parent, rank, n, pv_v));
            // If pv_v were a root (pv(parent, pv_v) == pv_v), then find(pv_v) = pv_v.
            // But pv_v != find(v) = find(pv_v). Contradiction.
            if pv::<V>(parent, pv_v) == pv_v {
                assert(spec_pure_find::<V>(parent, rank, n, pv_v) == pv_v);
                assert(false);
            }
            lemma_rank_lt_find::<V>(parent, rank, n, pv_v);
        }
    }

    /// Compressing a single node v preserves find for all z.
    /// pn is po with parent[v] set to root (= find(v)).
    #[verifier::rlimit(20)]
    proof fn lemma_compress_preserves_find<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>, n: nat,
        v: V::V, root: V::V, z: V::V,
    )
        requires
            // po is a valid forest.
            po.dom().finite(),
            po.dom().contains(v), po.dom().contains(z),
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) ==>
                po.dom().contains(pv::<V>(po, k)),
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(po, k)] as int),
            forall|k: V::V| po.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
            root == spec_pure_find::<V>(po, rank, n, v),
            spec_is_root_map::<V>(po, root),
            n == po.dom().len(),
            // pn characterization.
            pv::<V>(pn, v) == root,
            forall|k: V::V| k != v && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            pn.dom().contains(z), rank.dom().contains(z),
            // pn wf (needed for recursive call and ensures evaluation).
            forall|k: V::V| #[trigger] pn.dom().contains(k) ==>
                pn.dom().contains(pv::<V>(pn, k)),
            forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(pn, k)] as int),
        ensures
            spec_pure_find::<V>(pn, rank, n, z) == spec_pure_find::<V>(po, rank, n, z),
        decreases n as int - (rank[z] as int),
    {
        if pv::<V>(po, z) == z {
            // z is a root in po.
            if z == v {
                // v is a root. root = find(v) = v. pv(pn, v) = root = v. Still a root.
                assert(root == v);
                assert(pv::<V>(pn, v) == v);
            } else {
                assert(pv::<V>(pn, z) == z);
            }
        } else {
            // z is NOT a root in po.
            if z == v {
                assert(root != v) by {
                    lemma_find_is_root::<V>(po, rank, n, v);
                }
                assert(pv::<V>(pn, root) == pv::<V>(po, root)) by {
                    if root == v { assert(false); }
                }
                assert(pv::<V>(pn, root) == root);
                assert(pn.dom().contains(root));
                assert(rank.dom().contains(root));
                assert(spec_pure_find::<V>(pn, rank, n, root) == root);
            } else {
                // z != v, z is not a root. pv(pn, z) = pv(po, z).
                let ghost pz = pv::<V>(po, z);
                assert(po.dom().contains(pz));
                assert(pn.dom().contains(pz));
                assert(rank.dom().contains(pz));
                assert(pv::<V>(pn, z) == pz);
                lemma_compress_preserves_find::<V>(po, pn, rank, n, v, root, pz);
            }
        }
    }

    /// Wrapper: compression preserves find for ALL z at once.
    proof fn lemma_compress_preserves_find_all<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>, n: nat,
        v: V::V, root: V::V,
    )
        requires
            po.dom().finite(),
            po.dom().contains(v),
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) ==>
                po.dom().contains(pv::<V>(po, k)),
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(po, k)] as int),
            forall|k: V::V| po.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
            root == spec_pure_find::<V>(po, rank, n, v),
            spec_is_root_map::<V>(po, root),
            n == po.dom().len(),
            // pn characterization.
            pv::<V>(pn, v) == root,
            forall|k: V::V| k != v && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            // pn wf.
            forall|k: V::V| #[trigger] pn.dom().contains(k) ==>
                pn.dom().contains(pv::<V>(pn, k)),
            forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(pn, k)] as int),
        ensures
            forall|z: V::V| #[trigger] po.dom().contains(z) ==>
                spec_pure_find::<V>(pn, rank, n, z) == spec_pure_find::<V>(po, rank, n, z),
    {
        assert forall|z: V::V| #[trigger] po.dom().contains(z) implies
            spec_pure_find::<V>(pn, rank, n, z) == spec_pure_find::<V>(po, rank, n, z)
        by {
            assert(pn.dom().contains(z));
            assert(rank.dom().contains(z));
            lemma_compress_preserves_find::<V>(po, pn, rank, n, v, root, z);
        }
    }

    /// Micro-lemma 1: domain finiteness, length, rank-domain, root validity.
    /// No parent-in-domain or rank-invariant quantifiers.
    proof fn lemma_compress_basic<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>, n: nat,
        curr: V::V, root: V::V,
    )
        requires
            po.dom().finite(),
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> rank.dom().contains(k),
            n == po.dom().len(),
            po.dom().contains(root), spec_is_root_map::<V>(po, root), root != curr,
            pv::<V>(pn, curr) == root,
            forall|k: V::V| k != curr && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            forall|k: V::V| po.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
        ensures
            pn.dom().finite(),
            n == pn.dom().len(),
            forall|k: V::V| #[trigger] pn.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| pn.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
            spec_is_root_map::<V>(pn, root),
    {
        assert forall|k: V::V| #[trigger] pn.dom().contains(k) <==> rank.dom().contains(k) by {}
        assert(pn.dom().subset_of(po.dom())) by {
            assert forall|k: V::V| pn.dom().contains(k) implies #[trigger] po.dom().contains(k) by {}
        }
        assert(po.dom().subset_of(pn.dom())) by {
            assert forall|k: V::V| po.dom().contains(k) implies #[trigger] pn.dom().contains(k) by {}
        }
        lemma_len_subset::<V::V>(pn.dom(), po.dom());
        lemma_len_subset::<V::V>(po.dom(), pn.dom());
        assert(spec_is_root_map::<V>(pn, root));
    }

    /// Micro-lemma 2: parent-in-domain only. No rank quantifiers.
    proof fn lemma_compress_parent_in_dom<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        curr: V::V, root: V::V,
    )
        requires
            forall|k: V::V| #[trigger] po.dom().contains(k) ==>
                po.dom().contains(pv::<V>(po, k)),
            po.dom().contains(root),
            pv::<V>(pn, curr) == root,
            forall|k: V::V| k != curr && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
        ensures
            forall|k: V::V| #[trigger] pn.dom().contains(k) ==>
                pn.dom().contains(pv::<V>(pn, k)),
    {
        assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
            pn.dom().contains(pv::<V>(pn, k))
        by {}
    }

    /// Micro-lemma 3: rank invariant only. No parent-in-domain quantifiers.
    proof fn lemma_compress_rank_inv<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>,
        curr: V::V, root: V::V,
    )
        requires
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(po, k)] as int),
            (rank[curr] as int) < (rank[root] as int),
            pv::<V>(pn, curr) == root,
            forall|k: V::V| k != curr && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
        ensures
            forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(pn, k)] as int),
    {
        assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
            (#[trigger] rank[k] as int) < (rank[pv::<V>(pn, k)] as int)
        by {}
    }

    /// Single compression iteration, stated in closed preds. Reveals spec_light_wf,
    /// spec_find_preserved, spec_same_domain in an isolated Z3 context so the caller
    /// (find()'s loop body) never exposes their raw foralls — avoids parent-in-dom
    /// self-chaining matching loop.
    #[verifier::rlimit(200)]
    proof fn lemma_compress_iter<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>,
        orig_parent: Map<V::V, V>, orig_rank: Map<V::V, usize>, orig_n: nat,
        root: V::V, curr: V::V, next: V::V,
    )
        requires
            spec_light_wf::<V>(po, rank, orig_n),
            spec_same_domain::<V>(po, orig_parent),
            spec_find_preserved::<V>(po, rank, orig_n, orig_parent, orig_rank, orig_n),
            po.dom().contains(curr), po.dom().contains(root),
            spec_is_root_map::<V>(po, root),
            root == spec_pure_find::<V>(po, rank, orig_n, curr),
            pv::<V>(po, curr) == next,
            pv::<V>(po, curr) != curr,
            orig_n == po.dom().len(),
            // pn characterization (from HashMap::insert).
            pv::<V>(pn, curr) == root,
            forall|k: V::V| k != curr && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
        ensures
            spec_light_wf::<V>(pn, rank, orig_n),
            spec_same_domain::<V>(pn, orig_parent),
            spec_find_preserved::<V>(pn, rank, orig_n, orig_parent, orig_rank, orig_n),
            spec_is_root_map::<V>(pn, root),
            pn.dom().contains(root), pn.dom().contains(next),
            spec_pure_find::<V>(pn, rank, orig_n, next) == root,
            orig_n == pn.dom().len(),
            (rank[next] as int) > (rank[curr] as int),
    {
        reveal(spec_light_wf);
        reveal(spec_find_preserved);
        reveal(spec_same_domain);
        let n = orig_n;
        lemma_rank_lt_find::<V>(po, rank, n, curr);
        lemma_compress_basic::<V>(po, pn, rank, n, curr, root);
        lemma_compress_parent_in_dom::<V>(po, pn, curr, root);
        lemma_compress_rank_inv::<V>(po, pn, rank, curr, root);
        lemma_compress_step_find::<V>(po, pn, rank, n, curr, root);
        lemma_compose_find_preserved::<V>(
            orig_parent, orig_rank, orig_n,
            po, rank, n,
            pn, rank, n,
        );
        if root == curr {
            lemma_find_is_root::<V>(po, rank, n, curr);
            assert(false);
        }
        assert(pv::<V>(pn, root) == pv::<V>(po, root));
        assert(pv::<V>(pn, root) == root);
        // find(po, next) == find(po, curr) == root (unfold spec_pure_find at curr).
        assert(spec_pure_find::<V>(po, rank, n, curr) == spec_pure_find::<V>(po, rank, n, next));
    }

    /// Final-state wf reconstruction. At loop exit (curr == root), build
    /// size_rank_inv(self.parent@, ...) and spec_uf_wf(self) from orig's invariants
    /// via find preservation and same domain. Reveals happen inside the lemma.
    #[verifier::rlimit(80)]
    proof fn lemma_build_final_wf<V: StT + Hash + ClonePreservesView>(
        uf: &UnionFindPC<V>,
        orig_parent: Map<V::V, V>, orig_rank: Map<V::V, usize>, orig_n: nat,
    )
        requires
            obeys_key_model::<V>(),
            obeys_feq_view_injective::<V>(),
            obeys_feq_full::<V>(),
            spec_light_wf::<V>(uf.parent@, uf.rank@, uf.parent@.dom().len()),
            spec_light_wf::<V>(orig_parent, orig_rank, orig_n),
            spec_size_rank_inv_map::<V>(orig_parent, orig_rank, orig_n),
            spec_same_domain::<V>(uf.parent@, orig_parent),
            spec_find_preserved::<V>(
                uf.parent@, uf.rank@, uf.parent@.dom().len(),
                orig_parent, orig_rank, orig_n),
            uf.rank@ == orig_rank,
            uf.parent@.dom().len() == orig_n,
        ensures
            spec_size_rank_inv_map::<V>(uf.parent@, uf.rank@, uf.parent@.dom().len()),
    {
        reveal(spec_light_wf);
        reveal(spec_find_preserved);
        reveal(spec_same_domain);
        assert(spec_size_rank_inv_map::<V>(uf.parent@, uf.rank@, orig_n)) by {
            assert forall|r: V::V| uf.parent@.dom().contains(r)
                && pv::<V>(uf.parent@, r) == r implies
                spec_subtree::<V>(uf.parent@, uf.rank@, orig_n, r).finite()
                && spec_subtree::<V>(uf.parent@, uf.rank@, orig_n, r).len()
                    >= (#[trigger] uf.rank@[r] as nat) + 1
            by {
                assert(orig_parent.dom().contains(r));
                lemma_find_is_root::<V>(orig_parent, orig_rank, orig_n, r);
                assert(pv::<V>(orig_parent, r) == r);
                let st_new = spec_subtree::<V>(uf.parent@, uf.rank@, orig_n, r);
                let st_old = spec_subtree::<V>(orig_parent, orig_rank, orig_n, r);
                assert(st_new.subset_of(st_old)) by {
                    assert forall|k: V::V| st_new.contains(k) implies
                        #[trigger] st_old.contains(k)
                    by { if orig_parent.dom().contains(k) { assert(uf.parent@.dom().contains(k)); } }
                }
                assert(st_old.subset_of(st_new)) by {
                    assert forall|k: V::V| st_old.contains(k) implies
                        #[trigger] st_new.contains(k)
                    by { if orig_parent.dom().contains(k) { assert(uf.parent@.dom().contains(k)); } }
                }
                assert(st_old.finite());
                lemma_len_subset::<V::V>(st_new, st_old);
                lemma_len_subset::<V::V>(st_old, st_new);
            }
        }
    }

    /// rank[k] < n for k in light_wf'd map. Wrapper around reveal.
    proof fn lemma_rank_lt_n_from_light_wf<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, k: V::V,
    )
        requires spec_light_wf::<V>(parent, rank, n), parent.dom().contains(k),
        ensures (rank[k] as int) < n as int,
    {
        reveal(spec_light_wf);
    }

    /// pv(parent, curr) == curr ==> find(parent, curr) == curr. Wrapper around reveal
    /// so callers use closed light_wf instead of exposing raw foralls.
    proof fn lemma_root_find_self<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, curr: V::V,
    )
        requires spec_light_wf::<V>(parent, rank, n), parent.dom().contains(curr),
            pv::<V>(parent, curr) == curr,
        ensures spec_pure_find::<V>(parent, rank, n, curr) == curr,
    {
        reveal(spec_light_wf);
    }

    /// Non-root neighbor: next := pv(parent, curr) lies in domain with higher rank.
    proof fn lemma_non_root_next<V: View>(
        parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat, curr: V::V,
    )
        requires spec_light_wf::<V>(parent, rank, n), parent.dom().contains(curr),
            pv::<V>(parent, curr) != curr,
        ensures
            parent.dom().contains(pv::<V>(parent, curr)),
            rank.dom().contains(pv::<V>(parent, curr)),
            (rank[pv::<V>(parent, curr)] as int) > (rank[curr] as int),
    {
        reveal(spec_light_wf);
    }

    /// Compression step part 2: proves find preservation (NO rank re-derivation).
    /// Isolated from rank quantifiers to prevent matching loop.
    proof fn lemma_compress_step_find<V: View>(
        po: Map<V::V, V>, pn: Map<V::V, V>,
        rank: Map<V::V, usize>, n: nat,
        curr: V::V, root: V::V,
    )
        requires
            // po wf.
            po.dom().finite(),
            po.dom().contains(curr), po.dom().contains(root),
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> rank.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) ==>
                po.dom().contains(pv::<V>(po, k)),
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(po, k)] as int),
            forall|k: V::V| po.dom().contains(k) ==>
                (#[trigger] rank[k] as int) < n as int,
            root == spec_pure_find::<V>(po, rank, n, curr),
            spec_is_root_map::<V>(po, root),
            n == po.dom().len(),
            // pn characterization.
            pv::<V>(pn, curr) == root,
            forall|k: V::V| k != curr && po.dom().contains(k) ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            // pn wf (from step_wf).
            forall|k: V::V| #[trigger] pn.dom().contains(k) ==>
                pn.dom().contains(pv::<V>(pn, k)),
            forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k ==>
                (#[trigger] rank[k] as int) < (rank[pv::<V>(pn, k)] as int),
        ensures
            forall|z: V::V| #[trigger] po.dom().contains(z) ==>
                spec_pure_find::<V>(pn, rank, n, z) == spec_pure_find::<V>(po, rank, n, z),
    {
        lemma_compress_preserves_find_all::<V>(po, pn, rank, n, curr, root);
    }

    // BYPASSED: postcondition evaluation can't unfold spec_subtree through
    // spec_pure_find's decreases_when guard. Proof inlined at call site instead.
    // proof fn lemma_find_preserved_size_rank_inv<V: View>(
    //     po: Map<V::V, V>, pn: Map<V::V, V>,
    //     rank: Map<V::V, usize>, n: nat,
    // )
    //     requires spec_size_rank_inv_map::<V>(po, rank, n), ...
    //     ensures spec_size_rank_inv_map::<V>(pn, rank, n),

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
            forall|k: V::V| po.dom().contains(k) && k != ra ==> #[trigger] pv::<V>(pn, k) == pv::<V>(po, k),
            forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) <==> ro.dom().contains(k),
            forall|k: V::V| #[trigger] po.dom().contains(k) ==> po.dom().contains(pv::<V>(po, k)),
            forall|k: V::V| po.dom().contains(k) && pv::<V>(po, k) != k ==>
                (#[trigger] ro[k] as int) < (ro[pv::<V>(po, k)] as int),
            forall|k: V::V| po.dom().contains(k) ==> (#[trigger] ro[k] as int) < n as int,
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
                assert(pn.dom().contains(rb));
                assert(rn.dom().contains(rb));
                assert(pv::<V>(pn, rb) == rb);
                assert(spec_pure_find::<V>(pn, rn, n, rb) == rb);
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
            forall|j: V::V| #[trigger] po.dom().contains(j) <==> ro.dom().contains(j),
            forall|j: V::V| #[trigger] po.dom().contains(j) ==> po.dom().contains(pv::<V>(po, j)),
            forall|j: V::V| po.dom().contains(j) && pv::<V>(po, j) != j ==>
                (#[trigger] ro[j] as int) < (ro[pv::<V>(po, j)] as int),
            forall|j: V::V| po.dom().contains(j) ==> (#[trigger] ro[j] as int) < n_old as int,
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
        assert(k != v);
        assert(pv::<V>(pn, k) == pv::<V>(po, k));
        if pv::<V>(po, k) != k {
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
        assert(su.finite() && su.len() >= r as nat + 1);
        assert(sv.finite() && sv.len() >= r as nat + 1);
        assert(su.disjoint(sv)) by {
            assert forall|k: V::V| !(su.contains(k) && sv.contains(k)) by {}
        }
        assert(su.subset_of(parent.dom())) by {
            assert forall|k: V::V| su.contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        assert(sv.subset_of(parent.dom())) by {
            assert forall|k: V::V| sv.contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        assert((su + sv).subset_of(parent.dom())) by {
            assert forall|k: V::V| (su + sv).contains(k) implies #[trigger] parent.dom().contains(k) by {}
        }
        lemma_set_disjoint_lens(su, sv);
        assert((su + sv).len() == su.len() + sv.len());
        assert((su + sv).finite()) by {
            lemma_len_subset::<V::V>(su + sv, parent.dom());
        }
        lemma_len_subset::<V::V>(su + sv, parent.dom());
        assert((su + sv).len() <= n);
    }

	//		Section 8. traits — struct UnionFindPC

    pub trait UnionFindPCStEphTrait<V: StT + Hash + ClonePreservesView>: Sized {
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
            ensures
                self.spec_wf(),
                self.spec_contains(v@),
                forall|z: V::V| z != v@ ==>
                    (#[trigger] self.spec_contains(z) <==> old(self).spec_contains(z)),
                self.spec_n() == old(self).spec_n() + 1;

        fn find_root(&self, v: &V) -> (root: V)
            requires self.spec_wf(), self.spec_contains(v@),
            ensures self.spec_contains(root@), root@ == self.spec_find(v@),
                self.spec_is_root(root@);

        fn find(&mut self, v: &V) -> (root: V)
            requires old(self).spec_wf(), old(self).spec_contains(v@),
            ensures
                self.spec_wf(),
                self.spec_contains(root@),
                root@ == old(self).spec_find(v@),
                self.spec_is_root(root@),
                forall|z: V::V| old(self).spec_contains(z) ==>
                    self.spec_find(z) == old(self).spec_find(z),
                forall|z: V::V| old(self).spec_contains(z) <==> self.spec_contains(z),
                self.spec_n() == old(self).spec_n();

        fn union(&mut self, u: &V, v: &V)
            requires old(self).spec_wf(), old(self).spec_contains(u@), old(self).spec_contains(v@),
            ensures
                self.spec_wf(),
                self.spec_same_set(u@, v@),
                forall|z: V::V| old(self).spec_contains(z) <==> self.spec_contains(z),
                self.spec_n() == old(self).spec_n();

        fn equals(&mut self, u: &V, v: &V) -> (eq: bool)
            requires old(self).spec_wf(), old(self).spec_contains(u@), old(self).spec_contains(v@),
            ensures
                eq == old(self).spec_same_set(u@, v@),
                self.spec_wf(),
                forall|z: V::V| old(self).spec_contains(z) <==> self.spec_contains(z),
                self.spec_n() == old(self).spec_n();

        fn size(&self) -> (n: usize) requires self.spec_wf(), ensures n as nat == self.spec_n();
    }

	//		Section 9. impls — struct UnionFindPC

    impl<V: StT + Hash + ClonePreservesView> UnionFindPCStEphTrait<V> for UnionFindPC<V> {
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
            let uf = UnionFindPC { parent: HashMapWithViewPlus::new(), rank: HashMapWithViewPlus::new() };
            proof { reveal(spec_light_wf); }
            uf
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
                reveal(spec_light_wf);
                let pn = self.parent@;
                let rn = self.rank@;
                let n_new = self.spec_n();
                assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                    pn.dom().contains(pv::<V>(pn, k))
                by {
                    if k == vv { assert(pv::<V>(pn, k) == vv); }
                    else { assert(po.dom().contains(k)); assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                }
                assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                    (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                by {
                    if k == vv { assert(pv::<V>(pn, vv) == vv); }
                    else {
                        assert(pv::<V>(pn, k) == pv::<V>(po, k));
                        assert(rn[k] == ro[k]);
                        let ghost pk = pv::<V>(po, k);
                        if pk == vv { assert(po.dom().contains(pk)); assert(false); }
                        else { assert(rn[pk] == ro[pk]); }
                    }
                }
                assert forall|k: V::V| pn.dom().contains(k) implies
                    (#[trigger] rn[k] as int) < (n_new as int)
                by {
                    if k == vv { assert(rn[vv] == 0usize); assert(n_new >= 1); }
                    else { assert((ro[k] as int) < (n_old as int)); assert(rn[k] == ro[k]); }
                }
                assert(spec_size_rank_inv_map::<V>(pn, rn, n_new)) by {
                    assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                        spec_subtree::<V>(pn, rn, n_new, r).finite()
                        && spec_subtree::<V>(pn, rn, n_new, r).len() >= (#[trigger] rn[r] as nat) + 1
                    by {
                        if r == vv {
                            let st = spec_subtree::<V>(pn, rn, n_new, vv);
                            assert(pn.dom().contains(vv));
                            assert(pv::<V>(pn, vv) == vv);
                            assert(spec_pure_find::<V>(pn, rn, n_new, vv) == vv);
                            assert(st.contains(vv));
                            assert(st.subset_of(pn.dom())) by {
                                assert forall|k: V::V| st.contains(k) implies
                                    #[trigger] pn.dom().contains(k) by {}
                            }
                            lemma_len_subset::<V::V>(st, pn.dom());
                            assert(rn[vv] == 0usize);
                        } else {
                            let st_new = spec_subtree::<V>(pn, rn, n_new, r);
                            let st_old = spec_subtree::<V>(po, ro, n_old, r);
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
                                }
                            }
                            assert(rn[r] == ro[r]);
                        }
                    }
                }
            }
        }

        fn find_root(&self, v: &V) -> (root: V) {
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
                                reveal(spec_light_wf);
                                assert(parent_val@ == curr@);
                                assert(pv::<V>(self.parent@, curr@) == curr@);
                                lemma_find_in_dom::<V>(self.parent@, self.rank@, self.spec_n(), v@);
                                lemma_find_is_root::<V>(self.parent@, self.rank@, self.spec_n(), v@);
                            }
                            return curr;
                        }
                        proof {
                            reveal(spec_light_wf);
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
                        proof { reveal(spec_light_wf); assert(false); }
                        return curr;
                    },
                }
            }
            proof {
                reveal(spec_light_wf);
                assert((self.rank@[curr@] as int) >= n as int);
                assert((self.rank@[curr@] as int) < (self.spec_n() as int));
                assert(false);
            }
            curr
        }

        fn find(&mut self, v: &V) -> (root: V) {
            // Capture ghost state BEFORE find_root so we can bridge to old(self) directly.
            let ghost orig_parent = self.parent@;
            let ghost orig_rank = self.rank@;
            let ghost orig_n = self.spec_n();
            let root = self.find_root(v);
            let n = self.parent.len();
            let mut curr = v.clone_view();
            let mut steps: usize = 0;
            proof {
                reveal(spec_light_wf);
                reveal(spec_same_domain);
                reveal(spec_find_preserved);
            }
            while steps < n
                invariant
                    obeys_key_model::<V>(),
                    obeys_feq_view_injective::<V>(),
                    obeys_feq_full::<V>(),
                    // Bridge orig ghost values to old(self) — find_root takes &self.
                    orig_parent == old(self).parent@,
                    orig_rank == old(self).rank@,
                    orig_n == old(self).spec_n(),
                    // Closed predicates — Z3 sees opaque booleans, not raw quantifiers.
                    spec_light_wf::<V>(self.parent@, self.rank@, self.parent@.dom().len()),
                    spec_light_wf::<V>(orig_parent, orig_rank, orig_n),
                    spec_size_rank_inv_map::<V>(orig_parent, orig_rank, orig_n),
                    spec_same_domain::<V>(self.parent@, orig_parent),
                    spec_find_preserved::<V>(
                        self.parent@, self.rank@, self.parent@.dom().len(),
                        orig_parent, orig_rank, orig_n),
                    // Small open facts.
                    self.rank@ == orig_rank,
                    self.parent@.dom().len() == orig_n,
                    n == orig_n,
                    spec_is_root_map::<V>(self.parent@, root@),
                    root@ == spec_pure_find::<V>(orig_parent, orig_rank, orig_n, v@),
                    self.parent@.dom().contains(root@),
                    self.parent@.dom().contains(curr@),
                    spec_pure_find::<V>(self.parent@, self.rank@, self.parent@.dom().len(), curr@) == root@,
                    steps <= n,
                    (self.rank@[curr@] as int) >= steps as int,
                decreases n - steps,
            {
                let same = feq(&curr, &root);
                if same {
                    proof {
                        lemma_build_final_wf::<V>(self, orig_parent, orig_rank, orig_n);
                        reveal(spec_find_preserved);
                        reveal(spec_same_domain);
                        // Bridge ghost orig to old(self): find_root takes &self, so no mutation.
                        assert(orig_parent == old(self).parent@);
                        assert(orig_rank == old(self).rank@);
                        assert(orig_n == old(self).spec_n());
                        assert(root@ == old(self).spec_find(v@));
                        assert(self.spec_n() == old(self).spec_n());
                        assert forall|z: V::V| old(self).spec_contains(z) implies
                            self.spec_find(z) == old(self).spec_find(z) by {}
                        assert forall|z: V::V| old(self).spec_contains(z) <==> self.spec_contains(z) by {}
                    }
                    return root;
                }
                let next_opt = self.parent.get(&curr);
                match next_opt {
                    Some(next_val) => {
                        let next = next_val.clone_view();
                        proof {
                            // Rule out curr being a root (otherwise curr == root).
                            if pv::<V>(self.parent@, curr@) == curr@ {
                                lemma_root_find_self::<V>(
                                    self.parent@, self.rank@, self.parent@.dom().len(), curr@);
                                assert(curr@ == root@);
                                assert(false);
                            }
                            assert(pv::<V>(self.parent@, curr@) == next@);
                            lemma_non_root_next::<V>(
                                self.parent@, self.rank@, self.parent@.dom().len(), curr@);
                        }
                        let ghost po_step = self.parent@;
                        self.parent.insert(curr.clone_view(), root.clone_view());
                        proof {
                            lemma_compress_iter::<V>(
                                po_step, self.parent@, self.rank@,
                                orig_parent, orig_rank, orig_n,
                                root@, curr@, next@,
                            );
                        }
                        curr = next;
                        steps = steps + 1;
                    },
                    None => {
                        proof { assert(false); }
                        return root;
                    },
                }
            }
            proof {
                lemma_rank_lt_n_from_light_wf::<V>(
                    self.parent@, self.rank@, self.parent@.dom().len(), curr@);
                assert((self.rank@[curr@] as int) >= n as int);
                assert(false);
            }
            root
        }

        #[verifier::rlimit(40)]
        fn union(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            // After find(u): domain preserved, find results preserved, wf holds.
            let root_v = self.find(v);
            let same = feq(&root_u, &root_v);
            if same { return; }
            let rank_u = *self.rank.get(&root_u).unwrap_or(&0);
            let rank_v = *self.rank.get(&root_v).unwrap_or(&0);
            proof {
                reveal(spec_light_wf);
                // After both finds, self.spec_find preserves all original results.
                // root_u@ = find(u) and root_v@ = find(v) are roots.
                // Prove root_u is still a root: find(root_u) = root_u in current state.
                // Then lemma_find_is_root gives spec_is_root_map(parent, find(root_u)) = spec_is_root(root_u).
                assert(self.parent@.dom().contains(root_u@));
                assert(self.parent@.dom().contains(root_v@));
                lemma_find_is_root::<V>(self.parent@, self.rank@, self.spec_n(), root_u@);
                lemma_find_is_root::<V>(self.parent@, self.rank@, self.spec_n(), root_v@);
                // find(root_u@) in current state = root_u@ (since find preserved, root_u was a root).
                // spec_is_root(find(root_u@)) from lemma. find(root_u@) = root_u@.
                assert(self.spec_is_root(root_u@));
                assert(self.spec_is_root(root_v@));
                assert(pv::<V>(self.parent@, root_u@) == root_u@);
                assert(pv::<V>(self.parent@, root_v@) == root_v@);
            }
            let ghost po = self.parent@;
            let ghost ro = self.rank@;
            let ghost n = self.spec_n();
            proof {
                reveal(spec_light_wf);
                assert(ro.dom().contains(root_u@));
                assert(ro.dom().contains(root_v@));
                assert(rank_u == ro[root_u@]);
                assert(rank_v == ro[root_v@]);
            }
            if rank_u < rank_v {
                self.parent.insert(root_u.clone_view(), root_v);
                proof {
                    reveal(spec_light_wf);
                    let pn = self.parent@;
                    let rn = self.rank@;
                    assert(rn == ro);
                    assert(pv::<V>(pn, root_u@) == root_v@);
                    assert forall|k: V::V| po.dom().contains(k) <==> #[trigger] pn.dom().contains(k) by {}
                    assert forall|k: V::V| #[trigger] pn.dom().contains(k) implies
                        pn.dom().contains(pv::<V>(pn, k))
                    by {
                        if k == root_u@ { assert(pv::<V>(pn, k) == root_v@); }
                        else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    assert forall|k: V::V| pn.dom().contains(k) && pv::<V>(pn, k) != k implies
                        (#[trigger] rn[k] as int) < (rn[pv::<V>(pn, k)] as int)
                    by {
                        if k == root_u@ {} else { assert(pv::<V>(pn, k) == pv::<V>(po, k)); }
                    }
                    assert(spec_size_rank_inv_map::<V>(pn, rn, n)) by {
                        assert forall|r: V::V| pn.dom().contains(r) && pv::<V>(pn, r) == r implies
                            spec_subtree::<V>(pn, rn, n, r).finite()
                            && spec_subtree::<V>(pn, rn, n, r).len() >= (#[trigger] rn[r] as nat) + 1
                        by {
                            assert(r != root_u@);
                            let st_new = spec_subtree::<V>(pn, rn, n, r);
                            if r == root_v@ {
                                let st_old_u = spec_subtree::<V>(po, ro, n, root_u@);
                                let st_old_v = spec_subtree::<V>(po, ro, n, root_v@);
                                assert(st_new =~= st_old_u + st_old_v) by {
                                    assert forall|k: V::V|
                                        (#[trigger] st_new.contains(k)) == (st_old_u + st_old_v).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k));
                                            assert(rn.dom().contains(k));
                                            assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, k);
                                        }
                                    }
                                }
                                assert(st_old_u.finite()); assert(st_old_v.finite());
                                assert(st_old_u.disjoint(st_old_v)) by {
                                    assert forall|k: V::V| !(st_old_u.contains(k) && st_old_v.contains(k)) by {}
                                }
                                lemma_set_disjoint_lens(st_old_u, st_old_v);
                                assert(rn.dom().contains(root_v@)); assert(ro.dom().contains(root_v@));
                                assert(rn[root_v@] == ro[root_v@]);
                            } else {
                                let st_old = spec_subtree::<V>(po, ro, n, r);
                                assert(st_new =~= st_old) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == st_old.contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k)); assert(rn.dom().contains(k)); assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r)); assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    assert(po.dom().contains(u@)); assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@)); assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@)); assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@)); assert(ro.dom().contains(v@));
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_u@, root_v@, v@);
                }
            } else if rank_u > rank_v {
                self.parent.insert(root_v.clone_view(), root_u);
                proof {
                    reveal(spec_light_wf);
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
                                        (#[trigger] st_new.contains(k)) == (st_old_v + st_old_u).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k)); assert(rn.dom().contains(k)); assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(st_old_v.finite()); assert(st_old_u.finite());
                                assert(st_old_v.disjoint(st_old_u)) by {
                                    assert forall|k: V::V| !(st_old_v.contains(k) && st_old_u.contains(k)) by {}
                                }
                                lemma_set_disjoint_lens(st_old_v, st_old_u);
                                assert(rn.dom().contains(root_u@)); assert(ro.dom().contains(root_u@));
                                assert(rn[root_u@] == ro[root_u@]);
                            } else {
                                let st_old = spec_subtree::<V>(po, ro, n, r);
                                assert(st_new =~= st_old) by {
                                    assert forall|k: V::V|
                                        st_new.contains(k) == st_old.contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k)); assert(rn.dom().contains(k)); assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r)); assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    assert(po.dom().contains(u@)); assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@)); assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@)); assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@)); assert(ro.dom().contains(v@));
                    assert(pv::<V>(pn, root_u@) == pv::<V>(po, root_u@));
                    assert(pv::<V>(pn, root_u@) == root_u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, v@);
                }
            } else {
                let root_u2 = root_u.clone_view();
                self.parent.insert(root_v.clone_view(), root_u.clone_view());
                let n_len = self.parent.len();
                proof {
                    reveal(spec_light_wf);
                    assert(po.dom().finite());
                    assert(po.dom().contains(root_u@)); assert(po.dom().contains(root_v@));
                    assert(ro.dom().contains(root_u@)); assert(ro.dom().contains(root_v@));
                    lemma_rank_lt_n_minus_1::<V>(po, ro, n, root_u@, root_v@);
                    assert((ro[root_u@] as int) + 1 < (n as int));
                    assert((rank_u as int) + 1 < (n as int));
                    assert(rank_u < n_len);
                }
                self.rank.insert(root_u2, rank_u + 1);
                proof {
                    reveal(spec_light_wf);
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
                    assert forall|k: V::V| pn.dom().contains(k) implies
                        (#[trigger] rn[k] as int) < (n as int)
                    by {
                        assert(rn.dom().contains(k));
                        if k == root_u@ { assert(rn[root_u@] == rank_u + 1); }
                        else { assert(ro.dom().contains(k)); assert(rn[k] == ro[k]); }
                    }
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
                                        (#[trigger] st_new.contains(k)) == (st_old_v + st_old_u).contains(k)
                                    by {
                                        if po.dom().contains(k) {
                                            assert(pn.dom().contains(k)); assert(rn.dom().contains(k)); assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(st_old_v.finite()); assert(st_old_u.finite());
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
                                            assert(pn.dom().contains(k)); assert(rn.dom().contains(k)); assert(ro.dom().contains(k));
                                            lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, k);
                                        }
                                    }
                                }
                                assert(rn.dom().contains(r)); assert(ro.dom().contains(r));
                                assert(rn[r] == ro[r]);
                            }
                        }
                    }
                    assert(po.dom().contains(u@)); assert(po.dom().contains(v@));
                    assert(pn.dom().contains(u@)); assert(pn.dom().contains(v@));
                    assert(rn.dom().contains(u@)); assert(rn.dom().contains(v@));
                    assert(ro.dom().contains(u@)); assert(ro.dom().contains(v@));
                    assert(pv::<V>(pn, root_u@) == pv::<V>(po, root_u@));
                    assert(pv::<V>(pn, root_u@) == root_u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, u@);
                    lemma_find_after_link::<V>(po, ro, pn, rn, n, root_v@, root_u@, v@);
                }
            }
        }

        fn equals(&mut self, u: &V, v: &V) -> (eq: bool) {
            let root_u = self.find(u);
            let root_v = self.find(v);
            // root_u@ == old(pre-find-u).spec_find(u@) == old(self).spec_find(u@)
            // root_v@ == old(pre-find-v).spec_find(v@)
            //   == old(pre-find-v) preserved by find(u), so == old(self).spec_find(v@).
            // feq(root_u, root_v) == (root_u@ == root_v@)
            //   == (old(self).spec_find(u@) == old(self).spec_find(v@))
            //   == old(self).spec_same_set(u@, v@). ✓
            feq(&root_u, &root_v)
        }

        fn size(&self) -> (n: usize) { self.parent.len() }
    }

    } // verus!

	//		Section 14. derive impls outside verus! — struct UnionFindPC

    impl<V: StT + Hash + ClonePreservesView> Debug for UnionFindPC<V> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "UnionFindPC(n={})", self.parent.len())
        }
    }

    impl<V: StT + Hash + ClonePreservesView> Display for UnionFindPC<V> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "UnionFindPC(n={})", self.parent.len())
        }
    }

} // mod
