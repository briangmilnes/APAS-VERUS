//! Experiment: Does new-mut-ref encoding avoid matching loops?
//!
//! Reproduces the union proof matching loop pattern:
//! - Struct with Map field (roots)
//! - Closed quantifier over the map (spec_result)
//! - &mut function that modifies the struct and needs to prove the quantifier
//! - dom =~= assertion that triggers the loop in old encoding
//!
//! RESULT: (pending)

pub mod new_mut_ref_matching_loop {

    use vstd::prelude::*;

    verus! {

    // Simplified UnionFind-like struct with roots map.
    pub struct UF {
        pub roots: Ghost<Map<u64, u64>>,
        pub data: Ghost<Map<u64, u64>>,
        pub marker: u64,  // exec-visible field for branching
    }

    // Closed quantifier — analogous to spec_union_result.
    pub closed spec fn spec_result(
        new_roots: Map<u64, u64>,
        old_roots: Map<u64, u64>,
        u: u64,
        v: u64,
    ) -> bool {
        forall|x: u64| #[trigger] new_roots.contains_key(x) ==> {
            let old_root_u = old_roots[u];
            let old_root_v = old_roots[v];
            if old_roots[x] == old_root_u || old_roots[x] == old_root_v {
                new_roots[x] == new_roots[u]
            } else {
                new_roots[x] == old_roots[x]
            }
        }
    }

    // Closed quantifier — analogous to spec_roots_changed_by_merge.
    pub closed spec fn spec_merge_result(
        new_roots: Map<u64, u64>,
        old_roots: Map<u64, u64>,
        root_u: u64,
        root_v: u64,
        winner: u64,
    ) -> bool {
        forall|x: u64| old_roots.contains_key(x) ==> (
            #[trigger] new_roots[x] == (
                if old_roots[x] == root_u || old_roots[x] == root_v {
                    winner
                } else {
                    old_roots[x]
                }
            )
        )
    }

    // Closed wf — analogous to spec_unionfindsteph_wf.
    pub closed spec fn spec_wf(uf: &UF) -> bool {
        &&& forall|v: u64| #[trigger] uf.roots@.contains_key(v) ==> {
                uf.roots@.contains_key(uf.roots@[v])
                && uf.roots@[uf.roots@[v]] == uf.roots@[v]
            }
        &&& forall|v: u64| #[trigger] uf.data@.contains_key(v) ==>
                uf.roots@.contains_key(v)
        &&& uf.roots@.dom() =~= uf.data@.dom()
    }

    // Bridge lemma — isolated proof context (no &mut overhead).
    proof fn lemma_bridge(
        new_roots: Map<u64, u64>,
        old_roots: Map<u64, u64>,
        root_u: u64,
        root_v: u64,
        u: u64,
        v: u64,
        winner: u64,
    )
        requires
            spec_merge_result(new_roots, old_roots, root_u, root_v, winner),
            new_roots.dom() =~= old_roots.dom(),
            root_u == old_roots[u],
            root_v == old_roots[v],
            old_roots.contains_key(u),
        ensures
            spec_result(new_roots, old_roots, u, v),
    {
        reveal(spec_merge_result);
        reveal(spec_result);
        assert forall|x: u64| #[trigger] new_roots.contains_key(x) implies ({
            let old_root_u = old_roots[u];
            let old_root_v = old_roots[v];
            if old_roots[x] == old_root_u || old_roots[x] == old_root_v {
                new_roots[x] == new_roots[u]
            } else {
                new_roots[x] == old_roots[x]
            }
        }) by {
            assert(old_roots.contains_key(x));
        }
    }

    // Sub-function: analogous to find (returns an exec value).
    #[verifier::external_body]
    fn find_root(uf: &UF, v: u64) -> (root: u64)
        requires
            spec_wf(uf),
            uf.roots@.contains_key(v),
        ensures
            root == uf.roots@[v],
            uf.roots@.contains_key(root),
            uf.roots@[root] == root,
    {
        unimplemented!()
    }

    // Sub-function: analogous to union_merge.
    #[verifier::external_body]
    fn do_merge(uf: &mut UF, root_u: u64, root_v: u64)
        requires
            old(uf).roots@.contains_key(root_u),
            old(uf).roots@.contains_key(root_v),
            old(uf).roots@[root_u] == root_u,
            old(uf).roots@[root_v] == root_v,
            root_u != root_v,
            spec_wf(old(uf)),
        ensures
            spec_wf(uf),
            uf.roots@.dom() =~= old(uf).roots@.dom(),
            uf.data@ == old(uf).data@,
            spec_merge_result(uf.roots@, old(uf).roots@, root_u, root_v, root_u),
    {
        unimplemented!()
    }

    // THE KEY FUNCTION: analogous to union.
    // This is where the matching loop occurs in the old encoding.
    fn do_union(uf: &mut UF, u: u64, v: u64)
        requires
            spec_wf(old(uf)),
            old(uf).roots@.contains_key(u),
            old(uf).roots@.contains_key(v),
        ensures
            spec_wf(uf),
            uf.roots@.dom() =~= old(uf).roots@.dom(),
            spec_result(uf.roots@, old(uf).roots@, u, v),
    {
        let root_u = find_root(uf, u);
        let root_v = find_root(uf, v);
        let ghost root_u_view = root_u as u64;
        let ghost root_v_view = root_v as u64;

        if root_u != root_v {
            do_merge(uf, root_u, root_v);

            proof {
                // This is the matching loop site in old encoding.
                // Z3 has: old(uf), mid-state after find, final uf state.
                // Plus dom =~= from do_merge ensures.
                // Plus we reveal both closed quantifiers via lemma_bridge.
                lemma_bridge(
                    uf.roots@, old(uf).roots@,
                    root_u_view, root_v_view,
                    u, v,
                    root_u_view,
                );
            }
        } else {
            proof {
                reveal(spec_result);
                assert forall|x: u64| #[trigger] uf.roots@.contains_key(x) implies ({
                    let old_root_u = old(uf).roots@[u];
                    let old_root_v = old(uf).roots@[v];
                    if old(uf).roots@[x] == old_root_u || old(uf).roots@[x] == old_root_v {
                        uf.roots@[x] == uf.roots@[u]
                    } else {
                        uf.roots@[x] == old(uf).roots@[x]
                    }
                }) by {
                    // roots unchanged — find doesn't modify roots.
                }
            }
        }
    }

    } // verus!

} // mod
