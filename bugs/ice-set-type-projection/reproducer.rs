// Verus ICE: assert forall over Map<V::V, Set<V::V>> crashes sst_to_air
//
// Verus revision: c78aa4958372cfa69e6cb38fd31997c881473271
// Date: 2026-03-28
//
// Testing progressively richer type bounds to find ICE trigger.

//  Table of Contents
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups

use vstd::prelude::*;
use std::hash::Hash;

verus! {

    //		Section 4. type definitions


// Graph struct wrapping a concrete table-like field.
pub struct Graph<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized> {
    pub adj_view: Map<V::V, Set<V::V>>,
}

    //		Section 5. view impls


impl<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized> View for Graph<V> {
    type V = Map<V::V, Set<V::V>>;
    open spec fn view(&self) -> Map<V::V, Set<V::V>> { self.adj_view }
}

    //		Section 6. spec fns


pub open spec fn graph_closure<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    g: &Graph<V>
) -> bool {
    forall|u: V::V, v: V::V|
        g@.dom().contains(u)
        && #[trigger] g@[u].contains(v)
        ==> g@.dom().contains(v)
}

pub open spec fn stored_value_wf<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    g: &Graph<V>
) -> bool {
    forall|u: V::V| g@.dom().contains(u) ==>
        #[trigger] g@[u].finite()
}

    //		Section 7. proof fns/broadcast groups


// Test 1: Simple graph closure proof — does this crash?
proof fn test1_closure<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    g: &Graph<V>,
    u: V::V,
    v: V::V,
)
    requires
        graph_closure(g),
        g@.dom().contains(u),
        g@[u].contains(v),
    ensures
        g@.dom().contains(v),
{}

// Test 2: assert forall inside proof fn
proof fn test2_assert_forall<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    g: &Graph<V>,
)
    requires graph_closure(g),
{
    assert forall|u: V::V, v: V::V|
        g@.dom().contains(u)
        && #[trigger] g@[u].contains(v)
    implies g@.dom().contains(v)
    by {}
}

// Test 3: Prove closure after map insert (the actual ICE pattern)
proof fn test3_insert_closure<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    old_map: Map<V::V, Set<V::V>>,
    key: V::V,
)
    requires
        old_map.dom().finite(),
        forall|u: V::V, v: V::V|
            old_map.dom().contains(u)
            && #[trigger] old_map[u].contains(v)
            ==> old_map.dom().contains(v),
    ensures
    {
        let new_map = old_map.insert(key, Set::empty());
        assert forall|u: V::V, v: V::V|
            new_map.dom().contains(u)
            && #[trigger] new_map[u].contains(v)
        implies new_map.dom().contains(v)
        by {
            if u == key {
                // empty set has no members, contradiction
            } else {
                assert(old_map.dom().contains(u));
                assert(old_map[u] == new_map[u]);
                assert(old_map[u].contains(v));
            }
        }
    }

// Test 4: Combine closure + stored_value_wf (full wf pattern)
proof fn test4_combined_wf<V: View + Clone + Eq + PartialEq + Ord + Hash + Sized>(
    g: &Graph<V>,
)
    requires
        graph_closure(g),
        stored_value_wf(g),
        g@.dom().finite(),
{
    // Prove neighbor sets are subsets of domain
    assert forall|u: V::V| g@.dom().contains(u) implies
        #[trigger] g@[u].subset_of(g@.dom())
    by {
        assert forall|v: V::V| (#[trigger] g@[u].contains(v)) implies
            g@.dom().contains(v)
        by {
            // From graph_closure
        }
    }
}

} // verus!

fn main() {}
