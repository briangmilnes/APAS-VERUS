// Standalone test: new-mut-ref with richer matching loop pattern.
//
// Run old: ~/projects/verus/source/target-verus/release/verus --crate-type=lib \
//            experiments/new_mut_ref_test_v3.rs --expand-errors --rlimit 15
//
// Run new: ~/projects/verus/source/target-verus/release/verus --crate-type=lib \
//            experiments/new_mut_ref_test_v3.rs --expand-errors --rlimit 15 -V new-mut-ref
//
// Enriched with: generic type params, trait bounds, view types, more quantifiers,
// and the interaction between dom =~= and roots quantifiers.

use vstd::prelude::*;

verus! {

// Generic bounds mirroring StT + Hash.
pub trait K: Sized + View<V = u64> + Clone + PartialEq {}

pub struct UF<V: K> {
    pub parent: Ghost<Map<u64, V>>,
    pub rank: Ghost<Map<u64, usize>>,
    pub roots: Ghost<Map<u64, u64>>,
    pub elements: Ghost<Seq<V>>,
}

pub struct UFV<V: K> {
    pub parent: Map<u64, V>,
    pub rank: Map<u64, usize>,
    pub roots: Map<u64, u64>,
    pub elements: Seq<V>,
}

impl<V: K> View for UF<V> {
    type V = UFV<V>;
    open spec fn view(&self) -> Self::V {
        UFV {
            parent: self.parent@,
            rank: self.rank@,
            roots: self.roots@,
            elements: self.elements@,
        }
    }
}

// Closed spec_result with generic view types — trigger on new_roots.contains_key(x).
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

// Rich wf with 12 quantified predicates — closer to the real 14.
pub closed spec fn spec_wf<V: K>(uf: &UF<V>) -> bool {
    // roots idempotent
    &&& forall|v: u64| #[trigger] uf@.roots.contains_key(v) ==> {
            uf@.roots.contains_key(uf@.roots[v])
            && uf@.roots[uf@.roots[v]] == uf@.roots[v]
        }
    // parent closed
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            uf@.parent.contains_key(uf@.parent[v]@)
    // roots in dom
    &&& forall|v: u64| #[trigger] uf@.roots.contains_key(v) ==>
            uf@.parent.contains_key(uf@.roots[v])
    // elements forward
    &&& forall|i: int| 0 <= i < uf@.elements.len() as int ==>
            uf@.parent.contains_key(#[trigger] uf@.elements[i]@)
    // elements backward
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            exists|i: int| 0 <= i < uf@.elements.len() as int && #[trigger] uf@.elements[i]@ == v
    // elements distinct (the feq_view_injective interactor)
    &&& forall|i: int, j: int|
            0 <= i < uf@.elements.len() as int &&
            0 <= j < uf@.elements.len() as int &&
            i != j ==>
            #[trigger] uf@.elements[i]@ != #[trigger] uf@.elements[j]@
    // self parent is root
    &&& forall|v: u64| uf@.parent.contains_key(v) && uf@.parent[v]@ == v ==>
            #[trigger] uf@.roots[v] == v
    // parent preserves root
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            uf@.roots[uf@.parent[v]@] == uf@.roots[v]
    // rank increases
    &&& forall|v: u64| uf@.parent.contains_key(v)
            && uf@.parent[v]@ != v ==>
            uf@.rank[v] < #[trigger] uf@.rank[uf@.parent[v]@]
    // rank bounded
    &&& forall|v: u64| #[trigger] uf@.rank.contains_key(v) ==>
            uf@.rank[v] <= uf@.rank[uf@.roots[v]]
    // dom equalities
    &&& uf@.parent.dom() =~= uf@.rank.dom()
    &&& uf@.roots.dom() =~= uf@.parent.dom()
}

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

// find_root — immutable, returns exec u64.
#[verifier::external_body]
fn find_root<V: K>(uf: &UF<V>, v: u64) -> (root: u64)
    requires
        spec_wf(uf),
        uf@.roots.contains_key(v),
    ensures
        root == uf@.roots[v],
        uf@.roots.contains_key(root),
        uf@.roots[root] == root,
{
    unimplemented!()
}

// do_merge — &mut, proves merge_result.
#[verifier::external_body]
fn do_merge<V: K>(uf: &mut UF<V>, root_u: u64, root_v: u64)
    requires
        old(uf)@.roots.contains_key(root_u),
        old(uf)@.roots.contains_key(root_v),
        old(uf)@.roots[root_u] == root_u,
        old(uf)@.roots[root_v] == root_v,
        root_u != root_v,
        spec_wf(old(uf)),
    ensures
        spec_wf(uf),
        uf@.roots.dom() =~= old(uf)@.roots.dom(),
        uf@.elements == old(uf)@.elements,
        uf@.parent.dom() =~= old(uf)@.parent.dom(),
        spec_merge_result(uf@.roots, old(uf)@.roots, root_u, root_v, root_u),
{
    unimplemented!()
}

// THE KEY FUNCTION
fn do_union<V: K>(uf: &mut UF<V>, u: u64, v: u64)
    requires
        spec_wf(old(uf)),
        old(uf)@.roots.contains_key(u),
        old(uf)@.roots.contains_key(v),
    ensures
        spec_wf(uf),
        uf@.roots.dom() =~= old(uf)@.roots.dom(),
        spec_result(uf@.roots, old(uf)@.roots, u, v),
{
    proof { reveal(spec_wf); }
    let root_u = find_root(uf, u);
    let root_v = find_root(uf, v);

    if root_u != root_v {
        do_merge(uf, root_u, root_v);

        proof {
            lemma_bridge(
                uf@.roots, old(uf)@.roots,
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
