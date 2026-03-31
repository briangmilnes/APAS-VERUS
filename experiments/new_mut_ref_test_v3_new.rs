// Same as v3 but with new-mut-ref encoding.
// Run: ~/projects/verus/source/target-verus/release/verus --crate-type=lib \
//        experiments/new_mut_ref_test_v3_new.rs --expand-errors --rlimit 15 -V new-mut-ref

use vstd::prelude::*;

verus! {

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

pub closed spec fn spec_wf<V: K>(uf: &UF<V>) -> bool {
    &&& forall|v: u64| #[trigger] uf@.roots.contains_key(v) ==> {
            uf@.roots.contains_key(uf@.roots[v])
            && uf@.roots[uf@.roots[v]] == uf@.roots[v]
        }
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            uf@.parent.contains_key(uf@.parent[v]@)
    &&& forall|v: u64| #[trigger] uf@.roots.contains_key(v) ==>
            uf@.parent.contains_key(uf@.roots[v])
    &&& forall|i: int| 0 <= i < uf@.elements.len() as int ==>
            uf@.parent.contains_key(#[trigger] uf@.elements[i]@)
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            exists|i: int| 0 <= i < uf@.elements.len() as int && #[trigger] uf@.elements[i]@ == v
    &&& forall|i: int, j: int|
            0 <= i < uf@.elements.len() as int &&
            0 <= j < uf@.elements.len() as int &&
            i != j ==>
            #[trigger] uf@.elements[i]@ != #[trigger] uf@.elements[j]@
    &&& forall|v: u64| uf@.parent.contains_key(v) && uf@.parent[v]@ == v ==>
            #[trigger] uf@.roots[v] == v
    &&& forall|v: u64| #[trigger] uf@.parent.contains_key(v) ==>
            uf@.roots[uf@.parent[v]@] == uf@.roots[v]
    &&& forall|v: u64| uf@.parent.contains_key(v)
            && uf@.parent[v]@ != v ==>
            uf@.rank[v] < #[trigger] uf@.rank[uf@.parent[v]@]
    &&& forall|v: u64| #[trigger] uf@.rank.contains_key(v) ==>
            uf@.rank[v] <= uf@.rank[uf@.roots[v]]
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

#[verifier::external_body]
fn do_merge<V: K>(uf: &mut UF<V>, root_u: u64, root_v: u64)
    requires
        old(uf)@.roots.contains_key(root_u),
        old(uf)@.roots.contains_key(root_v),
        old(uf)@.roots[root_u] == root_u,
        old(uf)@.roots[root_v] == root_v,
        root_u != root_v,
        spec_wf(&*old(uf)),
    ensures
        spec_wf(&*final(uf)),
        final(uf)@.roots.dom() =~= old(uf)@.roots.dom(),
        final(uf)@.elements == old(uf)@.elements,
        final(uf)@.parent.dom() =~= old(uf)@.parent.dom(),
        spec_merge_result(final(uf)@.roots, old(uf)@.roots, root_u, root_v, root_u),
{
    unimplemented!()
}

fn do_union<V: K>(uf: &mut UF<V>, u: u64, v: u64)
    requires
        spec_wf(&*old(uf)),
        old(uf)@.roots.contains_key(u),
        old(uf)@.roots.contains_key(v),
    ensures
        spec_wf(&*final(uf)),
        final(uf)@.roots.dom() =~= old(uf)@.roots.dom(),
        spec_result(final(uf)@.roots, old(uf)@.roots, u, v),
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
