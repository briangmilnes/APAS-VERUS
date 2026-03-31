// Standalone test: Does new-mut-ref encoding avoid the union matching loop?
//
// Run WITHOUT new-mut-ref:
//   ~/projects/verus/source/target-verus/release/verus --crate-type=lib experiments/new_mut_ref_test.rs --expand-errors --rlimit 15
//
// Run WITH new-mut-ref:
//   ~/projects/verus/source/target-verus/release/verus --crate-type=lib experiments/new_mut_ref_test.rs --expand-errors --rlimit 15 -V new-mut-ref
//
// The key question: does the bridge proof in do_union succeed or fail/timeout?

use vstd::prelude::*;

verus! {

// Simplified UF struct with roots, parent, and rank ghost maps.
// Use u64 to approximate the hash/view layer without needing crate deps.
pub struct UF {
    pub parent: Ghost<Map<u64, u64>>,
    pub rank: Ghost<Map<u64, u64>>,
    pub roots: Ghost<Map<u64, u64>>,
    pub elements: Ghost<Seq<u64>>,
}

// Analogous to spec_union_result — closed quantifier over new_roots.
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

// Analogous to spec_roots_changed_by_merge — closed quantifier.
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

// Analogous to spec_unionfindsteph_wf — 10 closed quantified predicates.
pub closed spec fn spec_wf(uf: &UF) -> bool {
    // 1. roots idempotent
    &&& forall|v: u64| #[trigger] uf.roots@.contains_key(v) ==> {
            uf.roots@.contains_key(uf.roots@[v])
            && uf.roots@[uf.roots@[v]] == uf.roots@[v]
        }
    // 2. parent closed
    &&& forall|v: u64| #[trigger] uf.parent@.contains_key(v) ==>
            uf.parent@.contains_key(uf.parent@[v])
    // 3. roots in dom
    &&& forall|v: u64| #[trigger] uf.roots@.contains_key(v) ==>
            uf.parent@.contains_key(uf.roots@[v])
    // 4. elements forward
    &&& forall|i: int| 0 <= i < uf.elements@.len() as int ==>
            uf.parent@.contains_key(#[trigger] uf.elements@[i])
    // 5. elements backward
    &&& forall|v: u64| #[trigger] uf.parent@.contains_key(v) ==>
            exists|i: int| 0 <= i < uf.elements@.len() as int && #[trigger] uf.elements@[i] == v
    // 6. elements distinct
    &&& forall|i: int, j: int|
            0 <= i < uf.elements@.len() as int &&
            0 <= j < uf.elements@.len() as int &&
            i != j ==>
            #[trigger] uf.elements@[i] != #[trigger] uf.elements@[j]
    // 7. self parent is root
    &&& forall|v: u64| uf.parent@.contains_key(v) && uf.parent@[v] == v ==>
            #[trigger] uf.roots@[v] == v
    // 8. parent preserves root
    &&& forall|v: u64| #[trigger] uf.parent@.contains_key(v) ==>
            uf.roots@[uf.parent@[v]] == uf.roots@[v]
    // 9. rank increases
    &&& forall|v: u64| uf.parent@.contains_key(v)
            && uf.parent@[v] != v ==>
            uf.rank@[v] < #[trigger] uf.rank@[uf.parent@[v]]
    // 10. rank bounded
    &&& forall|v: u64| #[trigger] uf.rank@.contains_key(v) ==>
            uf.rank@[v] <= uf.rank@[uf.roots@[v]]
    // dom equalities
    &&& uf.parent@.dom() =~= uf.rank@.dom()
    &&& uf.roots@.dom() =~= uf.parent@.dom()
}

// Bridge lemma — isolated proof context, no &mut.
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

// Analogous to find_root_loop — immutable ref, just returns root.
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

// Analogous to union_merge — modifies &mut UF, proves merge_result.
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
        uf.elements@ == old(uf).elements@,
        uf.parent@.dom() =~= old(uf).parent@.dom(),
        spec_merge_result(uf.roots@, old(uf).roots@, root_u, root_v, root_u),
{
    unimplemented!()
}

// THE KEY FUNCTION: analogous to union.
// Will it verify with the bridge lemma?
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
    proof { reveal(spec_wf); }
    let root_u = find_root(uf, u);
    let root_v = find_root(uf, v);

    if root_u != root_v {
        do_merge(uf, root_u, root_v);

        proof {
            // The matching loop site.
            // lemma_bridge reveals both closed quantifiers.
            // The &mut context has dom =~= from do_merge ensures.
            // In old encoding, this creates the feedback loop.
            lemma_bridge(
                uf.roots@, old(uf).roots@,
                root_u, root_v,
                u, v,
                root_u,
            );
        }
    } else {
        proof {
            reveal(spec_result);
        }
    }
}

} // verus!
