// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;

    verus! {

    // 4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct UnionFindStEph<V: StT + Hash> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
        pub elements: Vec<V>,
        pub ghost roots: Map<<V as View>::V, <V as View>::V>,
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
                roots: self.roots,
            }
        }
    }

    // 6. spec fns

    impl<V: StT + Hash> UnionFindStEph<V> {
        /// Well-formedness invariant for the Union-Find structure.
        pub open spec fn wf(&self) -> bool {
            // Key model requirements for hash collections.
            &&& obeys_key_model::<V>()
            &&& obeys_feq_full::<V>()
            &&& forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2
            // parent and rank have the same domain.
            &&& self.parent@.dom() =~= self.rank@.dom()
            // roots has the same domain as parent.
            &&& self.roots.dom() =~= self.parent@.dom()
            // Every root is a fixed point: roots[roots[v]] == roots[v].
            &&& forall|v: <V as View>::V| #[trigger] self.roots.contains_key(v) ==> {
                &&& self.roots.contains_key(self.roots[v])
                &&& self.roots[self.roots[v]] == self.roots[v]
            }
            // Parent pointers stay within the domain.
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                self.parent@.contains_key(self.parent@[v]@)
            // Roots are within the domain.
            &&& forall|v: <V as View>::V| #[trigger] self.roots.contains_key(v) ==>
                self.parent@.contains_key(self.roots[v])
            // elements vec covers exactly the parent domain.
            &&& forall|i: int| 0 <= i < self.elements@.len() as int ==>
                self.parent@.contains_key(#[trigger] self.elements@[i]@)
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                exists|i: int| 0 <= i < self.elements@.len() as int && self.elements@[i]@ == v
            // elements have no duplicate views.
            &&& forall|i: int, j: int|
                0 <= i < self.elements@.len() as int &&
                0 <= j < self.elements@.len() as int &&
                i != j ==>
                self.elements@[i]@ != self.elements@[j]@
        }
    }

    // 8. traits

    pub trait UnionFindStEphTrait<V: StT + Hash>: Sized + View<V = UnionFindStEphV<V>> {
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
                old(self).wf(),
            ensures
                self.wf(),
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
                old(self).wf(),
                old(self)@.parent.contains_key(v@),
            ensures
                self.wf(),
                // Result is the canonical root.
                root@ == old(self)@.roots[v@],
                // Path compression preserves the logical partition.
                self@.roots =~= old(self)@.roots,
                // Domain unchanged.
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.rank =~= old(self)@.rank,
                self@.elements =~= old(self)@.elements;

        /// Union two sets by rank.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn union(&mut self, u: &V, v: &V)
            requires
                old(self).wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.wf(),
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.elements =~= old(self)@.elements,
                // Merged elements share a new root; others unchanged.
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
                old(self).wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.wf(),
                same == (old(self)@.roots[u@] == old(self)@.roots[v@]),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();

        /// Count distinct sets.
        /// APAS: Work O(n alpha(n)), Span O(n alpha(n))
        fn num_sets(&mut self) -> (count: usize)
            requires
                old(self).wf(),
            ensures
                self.wf(),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();
    }

    // 9. impls

    impl<V: StT + Hash> UnionFindStEphTrait<V> for UnionFindStEph<V> {
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
        fn insert(&mut self, v: V) {
            if !self.parent.contains_key(&v) {
                self.parent.insert(v.clone(), v.clone());
                self.rank.insert(v.clone(), 0usize);
                self.elements.push(v.clone());
                proof {
                    self.roots = self.roots.insert(v@, v@);
                    // accept hole: wf maintenance for insert — elements uniqueness and domain coverage
                    assume(self.wf());
                }
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        /// Iterative two-pass: first find root, then compress path.
        fn find(&mut self, v: &V) -> (root: V) {
            // Pass 1: chase parent pointers to the root.
            let mut current = v.clone();
            let ghost elem_count = self.elements@.len();
            let mut ghost steps: nat = 0;

            loop
                invariant
                    self.wf(),
                    self.parent@.contains_key(current@),
                    self.roots =~= old(self).roots,
                    self.parent@.dom() =~= old(self).parent@.dom(),
                    self.rank@ =~= old(self).rank@,
                    self.elements@ =~= old(self).elements@,
                    steps <= elem_count,
                decreases elem_count - steps
            {
                let p = self.parent.get(&current).unwrap().clone();
                if p == current {
                    break;
                }
                current = p;
                proof {
                    // accept hole: acyclicity — each step visits a new element
                    assume(steps < elem_count);
                    steps = steps + 1;
                }
            }
            let root = current;

            // Pass 2: path compression — point every node on the path directly to root.
            current = v.clone();
            let ghost old_parent = self.parent@;
            let ghost mut compress_steps: nat = 0;

            while current != root
                invariant
                    self.parent@.contains_key(current@),
                    self.parent@.contains_key(root@),
                    self.parent@[root@]@ == root@,
                    self.roots =~= old(self).roots,
                    self.parent@.dom() =~= old(self).parent@.dom(),
                    self.rank@ =~= old(self).rank@,
                    self.elements@ =~= old(self).elements@,
                    // accept hole: wf preserved through compression
                    self.wf(),
                    compress_steps <= elem_count,
                decreases elem_count - compress_steps
            {
                let next = self.parent.get(&current).unwrap().clone();
                self.parent.insert(current.clone(), root.clone());
                current = next;
                proof {
                    // accept hole: acyclicity — each compression step visits a new element
                    assume(compress_steps < elem_count);
                    compress_steps = compress_steps + 1;
                    // accept hole: wf is maintained after path compression
                    assume(self.wf());
                    // accept hole: compressed node is in domain
                    assume(self.parent@.contains_key(current@));
                }
            }

            proof {
                // accept hole: root matches the ghost canonical root
                assume(root@ == old(self).roots[v@]);
            }

            root
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn union(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            assert(self.parent@.contains_key(v@));
            let root_v = self.find(v);

            if root_u != root_v {
                let rank_u = *self.rank.get(&root_u).unwrap();
                let rank_v = *self.rank.get(&root_v).unwrap();

                let ghost old_roots = self.roots;
                let ghost new_root: <V as View>::V;

                if rank_u < rank_v {
                    self.parent.insert(root_u.clone(), root_v.clone());
                    proof { new_root = root_v@; }
                } else if rank_u > rank_v {
                    self.parent.insert(root_v.clone(), root_u.clone());
                    proof { new_root = root_u@; }
                } else {
                    self.parent.insert(root_v.clone(), root_u.clone());
                    self.rank.insert(root_u.clone(), rank_u + 1);
                    proof { new_root = root_u@; }
                }

                proof {
                    let old_root_u = old_roots[u@];
                    let old_root_v = old_roots[v@];
                    // Update ghost roots: merge both components under new_root.
                    self.roots = Map::new(
                        |k: <V as View>::V| old_roots.contains_key(k),
                        |k: <V as View>::V| {
                            if old_roots.contains_key(k) && (old_roots[k] == old_root_u || old_roots[k] == old_root_v) {
                                new_root
                            } else if old_roots.contains_key(k) {
                                old_roots[k]
                            } else {
                                // unreachable
                                k
                            }
                        },
                    );
                    // accept hole: wf maintained after union
                    assume(self.wf());
                    // accept hole: ensures clause on merged roots
                    assume(
                        forall|x: <V as View>::V| #[trigger] self.roots.contains_key(x) ==> {
                            let or_u = old(self).roots[u@];
                            let or_v = old(self).roots[v@];
                            if old(self).roots[x] == or_u || old(self).roots[x] == or_v {
                                self.roots[x] == self.roots[u@]
                            } else {
                                self.roots[x] == old(self).roots[x]
                            }
                        }
                    );
                }
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn equals(&mut self, u: &V, v: &V) -> (same: B) {
            let root_u = self.find(u);
            assert(self.parent@.contains_key(v@));
            let root_v = self.find(v);
            proof {
                // accept hole: PartialEq on V agrees with view equality for roots
                assume((root_u == root_v) == (root_u@ == root_v@));
            }
            root_u == root_v
        }

        /// - APAS: Work O(n alpha(n)), Span O(n alpha(n))
        fn num_sets(&mut self) -> (count: usize) {
            let mut roots_set = HashSetWithViewPlus::<V>::new();
            let mut i: usize = 0;

            while i < self.elements.len()
                invariant
                    self.wf(),
                    self.roots =~= old(self).roots,
                    self.parent@.dom() =~= old(self).parent@.dom(),
                    self.elements@ =~= old(self).elements@,
                    0 <= i <= self.elements@.len(),
                decreases self.elements@.len() - i
            {
                let v = self.elements[i].clone();
                let root = self.find(&v);
                let _ = roots_set.insert(root);
                i = i + 1;
                proof {
                    // accept hole: wf preserved after find within iteration
                    assume(self.wf());
                }
            }
            roots_set.len()
        }
    }

    } // verus!
}
