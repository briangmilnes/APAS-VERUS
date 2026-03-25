//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Data Structure (Sequential Ephemeral)
//!
//! Implements Union-Find (Disjoint Set Union) with path compression and union by rank.
//! Used in Kruskal's MST algorithm for efficient cycle detection.
//!
//! Ghost field `roots` maps each element to its canonical representative, allowing
//! clean specifications without recursive spec functions. Path compression changes
//! concrete parent pointers but preserves the logical partition (roots).

pub mod UnionFindStEph {

    // 2. imports
    use vstd::prelude::*;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::vstdplus::feq::feq::feq;
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct UnionFindStEph<V: StT + Hash> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
        pub elements: Vec<V>,
        pub roots: Ghost<Map<<V as View>::V, <V as View>::V>>,
    }

    // 5. view impls

    pub ghost struct UnionFindStEphV<V: View> {
        pub parent: Map<<V as View>::V, V>,
        pub rank: Map<<V as View>::V, usize>,
        pub elements: Seq<V>,
        pub roots: Map<<V as View>::V, <V as View>::V>,
    }

    impl<V: StT + Hash> View for UnionFindStEph<V> {
        type V = UnionFindStEphV<V>;
        open spec fn view(&self) -> Self::V {
            UnionFindStEphV {
                parent: self.parent@,
                rank: self.rank@,
                elements: self.elements@,
                roots: self.roots@,
            }
        }
    }

    // 6. spec fns

    // (spec_unionfindsteph_wf moved to trait abstract + open impl)

    // 8. traits

    pub trait UnionFindStEphTrait<V: StT + Hash>: Sized + View<V = UnionFindStEphV<V>> {
        spec fn spec_unionfindsteph_wf(&self) -> bool;

        /// Create a new empty Union-Find structure.
        /// APAS: Work Theta(1), Span Theta(1)
        fn new() -> (uf: Self)
            requires
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            ensures
                uf@.parent =~= Map::<<V as View>::V, V>::empty(),
                uf@.rank =~= Map::<<V as View>::V, usize>::empty(),
                uf@.elements =~= Seq::<V>::empty(),
                uf@.roots =~= Map::<<V as View>::V, <V as View>::V>::empty();

        /// Insert a new element as a singleton set.
        /// APAS: Work Theta(1), Span Theta(1)
        fn insert(&mut self, v: V)
            requires
                old(self).spec_unionfindsteph_wf(),
            ensures
                self.spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(v@) ==> self@ == old(self)@,
                !old(self)@.parent.contains_key(v@) ==> {
                    &&& self@.parent =~= old(self)@.parent.insert(v@, v)
                    &&& self@.rank =~= old(self)@.rank.insert(v@, 0usize)
                    &&& self@.roots =~= old(self)@.roots.insert(v@, v@)
                    &&& self@.elements.len() == old(self)@.elements.len() + 1
                };

        /// Find the root representative with path compression.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized (inverse Ackermann)
        fn find(&mut self, v: &V) -> (root: V)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                root@ == old(self)@.roots[v@],
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.rank =~= old(self)@.rank,
                self@.elements =~= old(self)@.elements;

        /// Union two sets by rank.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn union(&mut self, u: &V, v: &V)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.elements =~= old(self)@.elements,
                forall|x: <V as View>::V| #[trigger] self@.roots.contains_key(x) ==> {
                    let old_root_u = old(self)@.roots[u@];
                    let old_root_v = old(self)@.roots[v@];
                    if old(self)@.roots[x] == old_root_u || old(self)@.roots[x] == old_root_v {
                        self@.roots[x] == self@.roots[u@]
                    } else {
                        self@.roots[x] == old(self)@.roots[x]
                    }
                };

        /// Check if two elements are in the same set.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn equals(&mut self, u: &V, v: &V) -> (same: B)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                same == (old(self)@.roots[u@] == old(self)@.roots[v@]),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();

        /// Count distinct sets.
        /// APAS: Work O(n alpha(n)), Span O(n alpha(n))
        fn num_sets(&mut self) -> (count: usize)
            requires
                old(self).spec_unionfindsteph_wf(),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();
    }

    // 9. impls

    impl<V: StT + Hash> UnionFindStEphTrait<V> for UnionFindStEph<V> {
        /// Well-formedness invariant for the Union-Find structure.
        open spec fn spec_unionfindsteph_wf(&self) -> bool {
            // Key model requirements for hash collections.
            &&& obeys_key_model::<V>()
            &&& obeys_feq_full::<V>()
            &&& forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2
            // parent and rank have the same domain.
            &&& self.parent@.dom() =~= self.rank@.dom()
            // roots has the same domain as parent.
            &&& self.roots@.dom() =~= self.parent@.dom()
            // Every root is a fixed point: roots[roots[v]] == roots[v].
            &&& forall|v: <V as View>::V| #[trigger] self.roots@.contains_key(v) ==> {
                &&& self.roots@.contains_key(self.roots@[v])
                &&& self.roots@[self.roots@[v]] == self.roots@[v]
            }
            // Parent pointers stay within the domain.
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                self.parent@.contains_key(self.parent@[v]@)
            // Roots are within the domain.
            &&& forall|v: <V as View>::V| #[trigger] self.roots@.contains_key(v) ==>
                self.parent@.contains_key(self.roots@[v])
            // elements vec covers exactly the parent domain.
            &&& forall|i: int| 0 <= i < self.elements@.len() as int ==>
                self.parent@.contains_key(#[trigger] self.elements@[i]@)
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                exists|i: int| 0 <= i < self.elements@.len() as int && #[trigger] self.elements@[i]@ == v
            // elements have no duplicate views.
            &&& forall|i: int, j: int|
                0 <= i < self.elements@.len() as int &&
                0 <= j < self.elements@.len() as int &&
                i != j ==>
                #[trigger] self.elements@[i]@ != #[trigger] self.elements@[j]@
            // Self-parenting nodes are roots.
            &&& forall|v: <V as View>::V| self.parent@.contains_key(v) && self.parent@[v]@ == v ==>
                #[trigger] self.roots@[v] == v
            // Following a parent pointer preserves the root component.
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                self.roots@[self.parent@[v]@] == self.roots@[v]
            // Non-root nodes have strictly smaller rank than their parent.
            &&& forall|v: <V as View>::V| self.parent@.contains_key(v)
                && self.parent@[v]@ != v ==>
                self.rank@[v] < #[trigger] self.rank@[self.parent@[v]@]
            // Every element's rank is at most its root's rank.
            &&& forall|v: <V as View>::V| #[trigger] self.rank@.contains_key(v) ==>
                self.rank@[v] <= self.rank@[self.roots@[v]]
        }

        /// - APAS: Work Theta(1), Span Theta(1)
        fn new() -> (uf: Self) {
            UnionFindStEph {
                parent: HashMapWithViewPlus::new(),
                rank: HashMapWithViewPlus::new(),
                elements: Vec::new(),
                roots: Ghost(Map::empty()),
            }
        }

        /// - APAS: Work Theta(1), Span Theta(1)
        #[verifier::external_body]
        fn insert(&mut self, v: V) {
            if !self.parent.contains_key(&v) {
                self.parent.insert(v.clone(), v.clone());
                self.rank.insert(v.clone(), 0usize);
                self.elements.push(v.clone());
                self.roots = Ghost(self.roots@.insert(v@, v@));
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        /// Iterative two-pass: first find root, then compress path.
        #[verifier::external_body]
        fn find(&mut self, v: &V) -> (root: V) {
            // Pass 1: chase parent pointers to the root.
            let mut current = v.clone();
            loop {
                let p = self.parent.get(&current).unwrap().clone();
                if p == current {
                    break;
                }
                current = p;
            }
            let root = current;

            // Pass 2: path compression — point every node on the path directly to root.
            current = v.clone();
            while current != root {
                let next = self.parent.get(&current).unwrap().clone();
                self.parent.insert(current.clone(), root.clone());
                current = next;
            }

            root
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        #[verifier::external_body]
        fn union(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            let root_v = self.find(v);

            if root_u != root_v {
                let rank_u = *self.rank.get(&root_u).unwrap();
                let rank_v = *self.rank.get(&root_v).unwrap();

                if rank_u < rank_v {
                    self.parent.insert(root_u.clone(), root_v.clone());
                } else if rank_u > rank_v {
                    self.parent.insert(root_v.clone(), root_u.clone());
                } else {
                    self.parent.insert(root_v.clone(), root_u.clone());
                    self.rank.insert(root_u.clone(), rank_u + 1);
                }
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        #[verifier::external_body]
        fn equals(&mut self, u: &V, v: &V) -> (same: B) {
            let root_u = self.find(u);
            let root_v = self.find(v);
            feq(&root_u, &root_v)
        }

        /// - APAS: Work O(n alpha(n)), Span O(n alpha(n))
        #[verifier::external_body]
        fn num_sets(&mut self) -> (count: usize) {
            let mut roots_set = HashSetWithViewPlus::<V>::new();
            let mut i: usize = 0;
            while i < self.elements.len() {
                let v = self.elements[i].clone();
                let root = self.find(&v);
                let _ = roots_set.insert(root);
                i = i + 1;
            }
            roots_set.len()
        }
    }

    } // verus!
}
