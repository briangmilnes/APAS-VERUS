// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Single-threaded ephemeral ordered set implementation extending AVLTreeSetStEph.
//!
//! R67: Rewritten from AVLTreeSeqStEph-backed to ParamBST-backed. All ordered operations
//! (first, last, previous, next, split, get_range, rank, select, split_rank) now use tree
//! operations (min_key, max_key, split, expose, size) instead of scanning a flat sequence.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!


//		Section 1. module

pub mod OrderedSetStEph {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger};

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    vstd::laws_cmp::group_laws_cmp,
};

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEph<T: StT + Ord + TotalOrder> {
        pub base_set: AVLTreeSetStEph<T>,
    }

    pub type OrderedSetEph<T> = OrderedSetStEph<T>;

    //		Section 5. view impls


    impl<T: StT + Ord + TotalOrder> View for OrderedSetStEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.base_set@ }
    }

    //		Section 7. proof fns/broadcast groups


    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry<T: StT + Ord + TotalOrder>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    proof fn lemma_cmp_transitivity<T: StT + Ord + TotalOrder>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Less,
            b.cmp_spec(&c) == Less,
        ensures a.cmp_spec(&c) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// Equal congruence: Equal(a,b) implies cmp(a,c) == cmp(b,c).
    proof fn lemma_cmp_equal_congruent<T: StT + Ord + TotalOrder>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    //		Section 8. traits


    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with ephemeral semantics.
    /// Postconditions for ordering operations use cmp_spec (from Ord) rather than TotalOrder::le;
    /// matching ParamBST's ensure style directly.
    pub trait OrderedSetStEphTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetsteph_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - ephemeral semantics
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetsteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
            requires self.spec_orderedsetsteph_wf(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&mut self, x: T)
            requires
                old(self).spec_orderedsetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&mut self, x: &T)
            requires old(self).spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: PredSt<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
            requires
                old(self).spec_orderedsetsteph_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                self@.subset_of(old(self)@),
                self.spec_orderedsetsteph_wf(),
                forall|v: T::V| #[trigger] self@.contains(v)
                    ==> old(self)@.contains(v) && spec_pred(v),
                forall|v: T::V| old(self)@.contains(v) && spec_pred(v)
                    ==> #[trigger] self@.contains(v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn intersection(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.intersect(other@),
                self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn union(&mut self, other: &Self)
            requires
                old(self).spec_orderedsetsteph_wf(),
                other.spec_orderedsetsteph_wf(),
                old(self)@.len() + other@.len() < usize::MAX as nat,
            ensures
                self@ == old(self)@.union(other@),
                self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedsetsteph_wf(), other.spec_orderedsetsteph_wf(),
            ensures
                self@ == old(self)@.difference(other@),
                self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                seq.spec_avltreeseqstper_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqstper_wf(),
                seq@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed.spec_orderedsetsteph_wf();

        // Ordering operations (ADT 43.1) — postconditions in cmp_spec style.
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn first(&self) -> (first: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn last(&self) -> (last: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> v.cmp_spec(k) == Less,
                predecessor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn next(&self, k: &T) -> (successor: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&mut self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.1 == old(self)@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(old(self)@),
                split.2@.subset_of(old(self)@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| #[trigger] old(self)@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn join(&mut self, other: Self)
            requires
                old(self).spec_orderedsetsteph_wf(),
                other.spec_orderedsetsteph_wf(),
                old(self)@.len() + other@.len() < usize::MAX as nat,
            ensures self@ == old(self)@.union(other@), self@.finite(), self.spec_orderedsetsteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetsteph_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn rank(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn select(&self, i: usize) -> (selected: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(old(self)@),
                split.1@.subset_of(old(self)@),
                split.0@.disjoint(split.1@),
                forall|x| #[trigger] old(self)@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);
        /// Iterative alternative to `first`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST min_key traversal
        fn first_iter(&self) -> (first: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// Iterative alternative to `last`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST max_key traversal
        fn last_iter(&self) -> (last: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// Iterative alternative to `previous`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + max_key
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> v.cmp_spec(k) == Less,
                predecessor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// Iterative alternative to `next`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + min_key
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// Iterative alternative to `split`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split
        fn split_iter(&mut self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.1 == old(self)@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(old(self)@),
                split.2@.subset_of(old(self)@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| #[trigger] old(self)@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// Iterative alternative to `get_range`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- two BST splits + conditional inserts
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetsteph_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// Iterative alternative to `rank`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + size
        fn rank_iter(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// Iterative alternative to `split_rank`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- tree_select + BST split
        fn split_rank_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedsetsteph_wf(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                old(self)@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(old(self)@),
                split.1@.subset_of(old(self)@),
                split.0@.disjoint(split.1@),
                forall|x| #[trigger] old(self)@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);
    }

    //		Section 9. impls


    /// Maximum key in a ParamBST via right-spine walk.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST traversal to rightmost node
    fn tree_max_key<T: StT + Ord + TotalOrder>(tree: &ParamBST<T>) -> (maximum: Option<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            tree@.len() == 0 <==> maximum.is_none(),
            maximum.is_some() ==> tree@.contains(maximum.unwrap()@),
            maximum.is_some() ==> forall|t: T| (#[trigger] tree@.contains(t@)) ==>
                t.cmp_spec(&maximum.unwrap()) == Less || maximum.unwrap()@ == t@,
        decreases tree@.len(),
    {
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(left, key, right) => {
                // Veracity: NEEDED proof block
                // Veracity: NEEDED proof block
                proof {
                    // Establish termination: right@.len() < tree@.len().
                    vstd::set_lib::lemma_len_subset(right@, left@.union(right@));
                    // tree@ =~= union.insert(key@), key@ not in union, so |tree@| = |union| + 1.
                    // |right@| <= |union| < |union| + 1 = |tree@|.
                }
                if right.is_empty() {
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED proof block
                    proof {
                    }
                    Some(key)
                } else {
                    // Veracity: NEEDED proof block
                    let max_right = tree_max_key(&right);
                    // Veracity: NEEDED proof block
                    proof {
                        let mr = max_right.unwrap();
                        // mr ∈ right@, expose ensures mr.cmp_spec(&key) == Greater.
                        lemma_cmp_antisymmetry(mr, key);
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|t: T| (#[trigger] tree@.contains(t@))
                            implies t.cmp_spec(&mr) == Less || mr@ == t@ by {
                            if left@.contains(t@) {
                                lemma_cmp_transitivity(t, key, mr);
                            } else if right@.contains(t@) {
                                // Recursive ensures.
                            } else {
                                // t@ == key@. Equal congruence: t.cmp_spec(&mr) == key.cmp_spec(&mr).
                                lemma_cmp_equal_congruent(t, key, mr);
                            }
                        };
                    }
                    max_right
                }
            }
        }
    }

    /// Recursive select: find the i-th element in the BST (0-indexed, in sorted order).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- augmented BST traversal by rank
    fn tree_select<T: StT + Ord + TotalOrder>(tree: &ParamBST<T>, i: usize) -> (selected: Option<T>)
        requires
            tree@.finite(),
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures
            i as nat >= tree@.len() ==> selected.is_none(),
            (i as nat) < tree@.len() ==> selected.is_some(),
            selected.is_some() ==> tree@.contains(selected.unwrap()@),
        decreases tree@.len(),
    {
        match tree.expose() {
            // Veracity: NEEDED proof block
            Exposed::Leaf => None,
            Exposed::Node(left, key, right) => {
                // Veracity: NEEDED proof block
                proof {
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                // Veracity: NEEDED proof block
                let left_sz = left.size();
                if i < left_sz {
                    let result = tree_select(&left, i);
                    // Veracity: NEEDED proof block
                    proof {
                        if result.is_some() {
                        }
                    }
                    result
                } else if i as usize == left_sz {
                    // Veracity: NEEDED proof block
                    Some(key)
                } else {
                    let adjusted = i - left_sz - 1;
                    let result = tree_select(&right, adjusted);
                    // Veracity: NEEDED proof block
                    proof {
                        if result.is_some() {
                        }
                    }
                    result
                }
            }
        }
    }


    impl<T: StT + Ord + TotalOrder> OrderedSetStEphTrait<T> for OrderedSetStEph<T> {
        open spec fn spec_orderedsetsteph_wf(&self) -> bool {
            self.base_set.spec_avltreesetsteph_wf()
            && obeys_feq_full::<T>()
            && vstd::laws_cmp::obeys_cmp_spec::<T>()
            && view_ord_consistent::<T>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
        { self.base_set.size() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
        {
            OrderedSetStEph { base_set: AVLTreeSetStEph::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            OrderedSetStEph { base_set: AVLTreeSetStEph::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST search
        fn find(&self, x: &T) -> (found: bool)
        { self.base_set.find(x) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- treap split + join
        fn insert(&mut self, x: T)
        {
            self.base_set.insert(x);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- treap split + join
        fn delete(&mut self, x: &T)
        {
            self.base_set.delete(x);
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- recursive BST filter + join
        fn filter<F: PredSt<T>>(
            &mut self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        )
        {
            let found = self.base_set.filter(f, Ghost(spec_pred));
            self.base_set = found;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based intersection
        fn intersection(&mut self, other: &Self)
        {
            let found = self.base_set.intersection(&other.base_set);
            self.base_set = found;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based union
        fn union(&mut self, other: &Self)
        {
            let found = self.base_set.union(&other.base_set);
            self.base_set = found;
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based difference
        fn difference(&mut self, other: &Self)
        {
            let found = self.base_set.difference(&other.base_set);
            self.base_set = found;
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + vec copy
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            // Veracity: NEEDED proof block
            let mut elements: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut elements);
            // Veracity: NEEDED proof block
            proof {
            }
            let result = AVLTreeSeqStPerS::from_vec(elements);
            // Veracity: NEEDED proof block
            proof {
                // from_vec ensures: result@ =~= elements@.map_values(|t: T| t@).
                // collect_in_order ensures membership in both directions.
                let ghost elem_views = elements@.map_values(|t: T| t@);

                // Bridge: elem_views[i] == elements@[i]@ for all valid i.

                // Forward: each result element is in self@.

                // Backward: each element of self@ appears in result@.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| self@.contains(v)
                    implies result@.contains(v) by {
                    // collect_in_order: exists|i| elements@[i]@ == v.
                    let i = choose|i: int| 0 <= i < elements@.len() && (#[trigger] elements@[i])@ == v;
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(result@[i] == v);
                };

            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n treap inserts
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as nat == seq.spec_seq().len(),
                    0 <= i <= n,
                    constructed.spec_orderedsetsteph_wf(),
                    constructed@.finite(),
                    constructed@.len() <= i as nat,
                    seq@.len() < usize::MAX as nat,
                decreases n - i,
            {
                let elem = seq.nth(i).clone();
                constructed.insert(elem);
                i = i + 1;
            }
            constructed
        }

        /// First element (minimum) via BST min_key.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST min_key traversal
        fn first_iter(&self) -> (first: Option<T>)
        {
            self.base_set.tree.min_key()
            // min_key ensures match the postcondition directly.
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to first_iter
        fn first(&self) -> (first: Option<T>)
        { self.first_iter() }

        /// Last element (maximum) via tree_max_key.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST max_key traversal
        fn last_iter(&self) -> (last: Option<T>)
        {
            tree_max_key(&self.base_set.tree)
            // tree_max_key ensures match the postcondition directly.
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to last_iter
        fn last(&self) -> (last: Option<T>)
        { self.last_iter() }
// Veracity: NEEDED proof block

        /// Predecessor via split + max_key on left subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + max_key
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            let result = tree_max_key(&left);
            // Veracity: NEEDED proof block
            proof {
                if result.is_some() {
                    let v = result.unwrap();
                    // v is in left@, which has cmp_spec(k) == Less.
                    // v@ != k@ from view_ord_consistent (Less != Equal).
                    // v in self@: left@ ⊂ self@.remove(k@) ⊂ self@.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v@));
                    // Best predecessor: every t in self@ with t < k is in left@.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less
                        implies t.cmp_spec(&v) == Less || v@ == t@ by {
                        // t < k and view_ord_consistent ==> t@ != k@.
                        // t in self@.remove(k@).
                        // self@.remove(k@) =~= left@.union(_right@).
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(left@.union(_right@).contains(t@));
                        // t not in _right@ (right elements have cmp_spec(k) == Greater).
                        if _right@.contains(t@) {
                            // Contradiction: Less != Greater.
                        }
                        // tree_max_key ensures: t.cmp_spec(&v) == Less || v@ == t@.
                    };
                }
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to previous_iter
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
        // Veracity: NEEDED proof block
        { self.previous_iter(k) }

        /// Successor via split + min_key on right subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + min_key
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
        {
            let (_left, _found, right) = self.base_set.tree.split(k);
            let result = right.min_key();
            // Veracity: NEEDED proof block
            proof {
                if result.is_some() {
                    let v = result.unwrap();
                    // v is in right@, which has cmp_spec(k) == Greater.
                    // v in self@: right@ ⊂ self@.remove(k@) ⊂ self@.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v@));
                    // Best successor: every t in self@ with t > k is in right@.
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater
                        implies v.cmp_spec(&t) == Less || v@ == t@ by {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(_left@.union(right@).contains(t@));
                        if _left@.contains(t@) {
                            // Contradiction: Greater != Less.
                        }
                        // min_key ensures: v.cmp_spec(&t) == Less || v@ == t@.
                    };
                }
            }
            result
        }

        fn next(&self, k: &T) -> (successor: Option<T>)
        // Veracity: NEEDED proof block
        { self.next_iter(k) }

        /// Split via BST split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split
        fn split_iter(&mut self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        {
            let ghost old_view = self@;
            let (left_tree, found, right_tree) = self.base_set.tree.split(k);
            // Veracity: NEEDED proof block
            proof {
                // Prove left_tree@ and right_tree@ are subsets of old(self)@.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| left_tree@.contains(v) implies #[trigger] old_view.contains(v) by {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(old_view.remove(k@).contains(v));
                };
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| right_tree@.contains(v) implies #[trigger] old_view.contains(v) by {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(old_view.remove(k@).contains(v));
                };
                vstd::set_lib::lemma_len_subset(left_tree@, old_view);
                vstd::set_lib::lemma_len_subset(right_tree@, old_view);
                // Coverage: every x in old(self)@ is in left or right or equals k@.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|x: T::V| #[trigger] old_view.contains(x)
                    implies left_tree@.contains(x) || right_tree@.contains(x) || x == k@ by {
                    if x != k@ {
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert(left_tree@.union(right_tree@).contains(x));
                    }
                };
            }
            let left = OrderedSetStEph { base_set: AVLTreeSetStEph { tree: left_tree } };
            let right = OrderedSetStEph { base_set: AVLTreeSetStEph { tree: right_tree } };
            *self = Self::empty();
            (left, found, right)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to split_iter
        fn split(&mut self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        { self.split_iter(k) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to union
        // Veracity: NEEDED proof block
        fn join(&mut self, other: Self)
        { self.union(&other); }

        /// Range query via two splits.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- two BST splits + conditional inserts
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
        {
            let (_lt_k1, found_k1, right1) = self.base_set.tree.split(k1);
            let (mid, found_k2, _gt_k2) = right1.split(k2);
            let mut result_tree = mid;
            // Veracity: NEEDED proof block
            proof {
                // mid ⊂ right1 ⊂ self@.remove(k1@) ⊂ self@.
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| right1@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(self@.remove(k1@).contains(v));
                };
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| mid@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(right1@.remove(k2@).contains(v));
                };
                vstd::set_lib::lemma_len_subset(mid@, self@);
            }
            if found_k1 {
                // Veracity: NEEDED proof block
                let k1_clone = k1.clone_plus();
                result_tree.insert(k1_clone);
            }
            // After first insert: result_tree@ ⊂ self@, len bound for second insert.
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            }
            if found_k2 {
                let k2_clone = k2.clone_plus();
                result_tree.insert(k2_clone);
            }
            // Veracity: NEEDED proof block
            proof {
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            // Veracity: NEEDED proof block
            }
            OrderedSetStEph { base_set: AVLTreeSetStEph { tree: result_tree } }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to get_range_iter
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
        { self.get_range_iter(k1, k2) }

        /// Rank via split + size.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + size
        fn rank_iter(&self, k: &T) -> (rank: usize)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert forall|v: T::V| left@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED assert
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v));
                };
                vstd::set_lib::lemma_len_subset(left@, self@);
            }
            left.size()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to rank_iter
        fn rank(&self, k: &T) -> (rank: usize)
        { self.rank_iter(k) }

        /// Select the i-th element using tree_select.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- augmented BST traversal
        fn select(&self, i: usize) -> (selected: Option<T>)
        {
            let sz = self.size();
            if i >= sz {
                None
            } else {
                tree_select(&self.base_set.tree, i)
            }
        }

        /// Split by rank: first i elements go left, rest go right.
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- tree_select + BST split
        fn split_rank_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let sz = self.size();
            if i >= sz {
                // Veracity: NEEDED proof block
                let result = self.clone();
                *self = Self::empty();
                (result, Self::empty())
            } else {
                let pivot = tree_select(&self.base_set.tree, i);
                let pivot_key = pivot.unwrap();
                let (left_tree, _found, right_tree) = self.base_set.tree.split(&pivot_key);
                let mut right_with_pivot = right_tree;
                // Veracity: NEEDED proof block
                proof {
                    // Prove subsets of self@.
                    vstd::set_lib::lemma_len_subset(left_tree@, self@);
                    vstd::set_lib::lemma_len_subset(right_tree@, self@);
                }
                right_with_pivot.insert(pivot_key);
                // Veracity: NEEDED proof block
                proof {
                    // right_with_pivot@ = right_tree@.insert(pivot_key@).
                    // pivot_key@ in self@, right_tree@ ⊂ self@.
                    vstd::set_lib::lemma_len_subset(right_with_pivot@, self@);
                }
                let left = OrderedSetStEph { base_set: AVLTreeSetStEph { tree: left_tree } };
                let right = OrderedSetStEph { base_set: AVLTreeSetStEph { tree: right_with_pivot } };
                *self = Self::empty();
                (left, right)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to split_rank_iter
        fn split_rank(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        { self.split_rank_iter(i) }
    }


    impl<T: StT + Ord + TotalOrder> OrderedSetStEph<T> {
        /// Returns an iterator over the set elements via in-order traversal.
        pub fn iter(&self) -> (it: OrderedSetStEphIter<T>)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant(&it),
        {
            let mut elements: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut elements);
            OrderedSetStEphIter { inner: elements.into_iter() }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- from_vec O(n) + n treap inserts
    pub fn from_sorted_elements<T: StT + Ord + TotalOrder>(elements: Vec<T>) -> (constructed: OrderedSetStEph<T>)
        requires
            elements@.len() < usize::MAX,
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures constructed.spec_orderedsetsteph_wf()
    {
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStEph::from_seq(seq)
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEphIter<T: StT + Ord + TotalOrder> {
        pub inner: IntoIter<T>,
    }

    impl<T: StT + Ord + TotalOrder> View for OrderedSetStEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<T: StT + Ord + TotalOrder>(it: &OrderedSetStEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StT + Ord + TotalOrder> std::iter::Iterator for OrderedSetStEphIter<T> {
        type Item = T;

        fn next(&mut self) -> (next: Option<T>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStEphGhostIterator<T: StT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StT + Ord + TotalOrder> View for OrderedSetStEphGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for OrderedSetStEphIter<T> {
        type GhostIter = OrderedSetStEphGhostIterator<T>;
        open spec fn ghost_iter(&self) -> OrderedSetStEphGhostIterator<T> {
            OrderedSetStEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for OrderedSetStEphGhostIterator<T> {
        type ExecIter = OrderedSetStEphIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedSetStEphIter<T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedSetStEphIter<T>) -> OrderedSetStEphGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StT + Ord + TotalOrder> std::iter::IntoIterator for &'a OrderedSetStEph<T> {
        type Item = T;
        type IntoIter = OrderedSetStEphIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_orderedsetsteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT + Ord + TotalOrder> Clone for OrderedSetStEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrderedSetStEph {
                base_set: self.base_set.clone(),
            }
        }
    }
    } // verus!

    //		Section 13. macros


    /// Macro for creating ephemeral ordered sets from sorted element lists.
    #[macro_export]
    macro_rules! OrderedSetStEphLit {
        () => {
            $crate::Chap43::OrderedSetStEph::OrderedSetStEph::OrderedSetStEph::empty()
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap43::OrderedSetStEph::OrderedSetStEph::from_sorted_elements(vec![$($elem),+])
        };
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder> Default for OrderedSetStEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StT + Ord + TotalOrder> PartialEq for OrderedSetStEph<T> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size() && {
                let seq = self.to_seq();
                for i in 0..seq.length() {
                    if !other.find(seq.nth(i)) {
                        return false;
                    }
                }
                true
            }
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Debug for OrderedSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let mut v: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut v);
            for i in 0..v.len() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", v[i])?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let mut v: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut v);
            for i in 0..v.len() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", v[i])?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord + TotalOrder + fmt::Debug> fmt::Debug for OrderedSetStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStEphIter({:?})", self.inner)
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStEphIter")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Debug for OrderedSetStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStEphGhostIterator")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStEphGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStEphGhostIterator")
        }
    }
}
