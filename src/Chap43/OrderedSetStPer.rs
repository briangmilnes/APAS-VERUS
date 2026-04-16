// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Single-threaded persistent ordered set implementation extending AVLTreeSetStPer.
//!
//! R82: Rewritten from AVLTreeSeqStPer-backed to ParamBST-backed. All ordered operations
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

pub mod OrderedSetStPer {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_full_trigger, lemma_cloned_view_eq};
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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
    pub struct OrderedSetStPer<T: StT + Ord + TotalOrder> {
        pub base_set: AVLTreeSetStPer<T>,
    }

    pub type OrderedSetPer<T> = OrderedSetStPer<T>;

    //		Section 5. view impls


    impl<T: StT + Ord + TotalOrder> View for OrderedSetStPer<T> {
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


    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with persistent semantics.
    /// Postconditions for ordering operations use cmp_spec (from Ord) rather than TotalOrder::le;
    /// matching ParamBST's ensure style directly.
    pub trait OrderedSetStPerTrait<T: StT + Ord + TotalOrder>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetstper_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - delegated
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn find(&self, x: &T) -> (found: bool)
            requires self.spec_orderedsetstper_wf(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures updated@ == self@.insert(x@), updated.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n)
        fn delete(&self, x: &T) -> (updated: Self)
            requires self.spec_orderedsetstper_wf(),
            ensures updated@ == self@.remove(x@), updated.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_orderedsetstper_wf(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures filtered@.subset_of(self@), filtered.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures common@ == self@.intersect(other@), common.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_orderedsetstper_wf(),
                other.spec_orderedsetstper_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@ == self@.union(other@), combined.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures remaining@ == self@.difference(other@), remaining.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_orderedsetstper_wf(),
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
            ensures constructed.spec_orderedsetstper_wf();

        // Ordering operations (ADT 43.1) — postconditions in cmp_spec style.
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn first(&self) -> (first: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn last(&self) -> (last: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetstper_wf(),
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
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.1 == self@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(self@),
                split.2@.subset_of(self@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn join(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_orderedsetstper_wf(),
                right.spec_orderedsetstper_wf(),
                left@.len() + right@.len() < usize::MAX as nat,
            ensures joined@ == left@.union(right@), joined@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn rank(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn select(&self, i: usize) -> (selected: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(self@),
                split.1@.subset_of(self@),
                split.0@.disjoint(split.1@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);

        // Iterative alternatives (delegate to defaults for BST-backed version).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST min_key
        fn first_iter(&self) -> (first: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST max_key
        fn last_iter(&self) -> (last: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + max_key
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> v.cmp_spec(k) == Less,
                predecessor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + min_key
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split
        fn split_iter(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.1 == self@.contains(k@),
                split.0@.finite(),
                split.2@.finite(),
                split.0@.subset_of(self@),
                split.2@.subset_of(self@),
                split.0@.disjoint(split.2@),
                !split.0@.contains(k@),
                !split.2@.contains(k@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.2@.contains(x) || x == k@;
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- two BST splits + conditional inserts
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + size
        fn rank_iter(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- tree_select + BST split
        fn split_rank_iter(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                split.0@.finite(),
                split.1@.finite(),
                split.0@.subset_of(self@),
                split.1@.subset_of(self@),
                split.0@.disjoint(split.1@),
                forall|x| #[trigger] self@.contains(x) ==> split.0@.contains(x) || split.1@.contains(x);
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
                    vstd::set_lib::lemma_len_subset(right@, left@.union(right@));
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
                        lemma_cmp_antisymmetry(mr, key);
                        // Veracity: NEEDED assert
                        // Veracity: NEEDED assert
                        assert forall|t: T| (#[trigger] tree@.contains(t@))
                            implies t.cmp_spec(&mr) == Less || mr@ == t@ by {
                            if left@.contains(t@) {
                                lemma_cmp_transitivity(t, key, mr);
                            } else if right@.contains(t@) {
                            } else {
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
                    // Veracity: NEEDED assert (speed hint)
                    assert(!right@.contains(key@));
                    // Veracity: NEEDED assert (speed hint)
                    assert(!left@.union(right@).contains(key@));
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                // Veracity: NEEDED proof block
                }
                let left_sz = left.size();
                if i < left_sz {
                    let result = tree_select(&left, i);
                    proof {
                        if result.is_some() {
                            // Veracity: NEEDED assert (speed hint)
                            assert(left@.contains(result.unwrap()@));
                            // Veracity: NEEDED assert (speed hint)
                            assert(tree@.contains(result.unwrap()@));
                        }
                    }
                    result
                // Veracity: NEEDED proof block
                } else if i as usize == left_sz {
                    Some(key)
                } else {
                    let adjusted = i - left_sz - 1;
                    let result = tree_select(&right, adjusted);
                    proof {
                        if result.is_some() {
                            // Veracity: NEEDED assert (speed hint)
                            assert(right@.contains(result.unwrap()@));
                            // Veracity: NEEDED assert (speed hint)
                            assert(tree@.contains(result.unwrap()@));
                        }
                    }
                    result
                }
            }
        }
    }


    impl<T: StT + Ord + TotalOrder> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        open spec fn spec_orderedsetstper_wf(&self) -> bool {
            self.base_set.spec_avltreesetstper_wf()
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
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<T>());
            OrderedSetStPer { base_set: AVLTreeSetStPer::empty() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(x: T) -> (tree: Self)
        {
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<T>());
            OrderedSetStPer { base_set: AVLTreeSetStPer::singleton(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST search
        fn find(&self, x: &T) -> (found: bool)
        { self.base_set.find(x) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- treap split + join
        fn insert(&self, x: T) -> (updated: Self)
        {
            OrderedSetStPer { base_set: self.base_set.insert(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- treap split + join
        fn delete(&self, x: &T) -> (updated: Self)
        {
            OrderedSetStPer { base_set: self.base_set.delete(x) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- recursive BST filter + join
        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            OrderedSetStPer { base_set: self.base_set.filter(f, Ghost(spec_pred)) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based intersection
        fn intersection(&self, other: &Self) -> (common: Self)
        {
            OrderedSetStPer { base_set: self.base_set.intersection(&other.base_set) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based union
        fn union(&self, other: &Self) -> (combined: Self)
        {
            OrderedSetStPer { base_set: self.base_set.union(&other.base_set) }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST split-based difference
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            OrderedSetStPer { base_set: self.base_set.difference(&other.base_set) }
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + vec copy
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            // Veracity: NEEDED proof block
            let mut elements: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut elements);
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(elements@.len() < usize::MAX);
            }
            let result = AVLTreeSeqStPerS::from_vec(elements);
            proof {
                let ghost elem_views = elements@.map_values(|t: T| t@);
                // Veracity: NEEDED assert (speed hint)
                assert(result@ =~= elem_views);

                // Veracity: NEEDED assert (speed hint)
                assert forall|i: int| 0 <= i < elements@.len()
                    implies elem_views[i] == (#[trigger] elements@[i])@ by {};

                // Veracity: NEEDED assert (speed hint)
                assert forall|i: int| 0 <= i < result@.len()
                    implies #[trigger] self@.contains(result@[i]) by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(result@[i] == elem_views[i]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(elem_views[i] == elements@[i]@);
                };

                // Veracity: NEEDED assert (speed hint)
                assert forall|v: T::V| self@.contains(v)
                    implies result@.contains(v) by {
                    let i = choose|i: int| 0 <= i < elements@.len() && (#[trigger] elements@[i])@ == v;
                    // Veracity: NEEDED assert (speed hint)
                    assert(0 <= i < result@.len());
                    // Veracity: NEEDED assert (speed hint)
                    assert(result@[i] == elem_views[i]);
                    // Veracity: NEEDED assert (speed hint)
                    assert(elem_views[i] == elements@[i]@);
                    // Veracity: NEEDED assert (speed hint)
                    assert(result@[i] == v);
                };

                // Veracity: NEEDED assert (speed hint)
                assert(result@.to_set() =~= self@);
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n treap inserts
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            // Veracity: NEEDED assert (speed hint)
            assert(obeys_feq_full_trigger::<T>());
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as nat == seq.spec_seq().len(),
                    0 <= i <= n,
                    // Veracity: NEEDED proof block
                    constructed.spec_orderedsetstper_wf(),
                    constructed@.finite(),
                    constructed@.len() <= i as nat,
                    seq@.len() < usize::MAX as nat,
                decreases n - i,
            {
                let elem_ref = seq.nth(i);
                let elem = elem_ref.clone();
                proof { lemma_cloned_view_eq(*elem_ref, elem); }
                constructed = constructed.insert(elem);
                i = i + 1;
            }
            constructed
        }

        /// First element (minimum) via BST min_key.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST min_key traversal
        fn first_iter(&self) -> (first: Option<T>)
        {
            self.base_set.tree.min_key()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to first_iter
        fn first(&self) -> (first: Option<T>)
        { self.first_iter() }

        /// Last element (maximum) via tree_max_key.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST max_key traversal
        fn last_iter(&self) -> (last: Option<T>)
        {
            tree_max_key(&self.base_set.tree)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to last_iter
        // Veracity: NEEDED proof block
        fn last(&self) -> (last: Option<T>)
        { self.last_iter() }

        /// Predecessor via split + max_key on left subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + max_key
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            let result = tree_max_key(&left);
            proof {
                if result.is_some() {
                    let v = result.unwrap();
// Veracity: UNNEEDED assert                     assert(left@.contains(v@));
// Veracity: UNNEEDED assert                     assert(v.cmp_spec(k) == Less);
                    // Veracity: NEEDED assert (speed hint)
                    assert(v@ != k@);
                    // Veracity: NEEDED assert (speed hint)
                    assert(left@.union(_right@) =~= self@.remove(k@));
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v@));
// Veracity: UNNEEDED assert                     assert(self@.contains(v@));
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less
                        implies t.cmp_spec(&v) == Less || v@ == t@ by {
// Veracity: UNNEEDED assert                         assert(t@ != k@);
                        // Veracity: NEEDED assert (speed hint)
                        assert(self@.remove(k@).contains(t@));
                        // Veracity: NEEDED assert (speed hint)
                        assert(left@.union(_right@).contains(t@));
                        if _right@.contains(t@) {
                            // Veracity: NEEDED assert (speed hint)
                            assert(t.cmp_spec(k) == Greater);
                        }
                        // Veracity: NEEDED assert (speed hint)
                        assert(left@.contains(t@));
                    };
                }
            }
            result
        }

        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to previous_iter
        fn previous(&self, k: &T) -> (predecessor: Option<T>)
        { self.previous_iter(k) }

        /// Successor via split + min_key on right subtree.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + min_key
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
        {
            let (_left, _found, right) = self.base_set.tree.split(k);
            let result = right.min_key();
            proof {
                if result.is_some() {
                    let v = result.unwrap();
// Veracity: UNNEEDED assert                     assert(right@.contains(v@));
                    // Veracity: NEEDED assert (speed hint)
                    assert(v.cmp_spec(k) == Greater);
                    // Veracity: NEEDED assert (speed hint)
                    assert(v@ != k@);
// Veracity: UNNEEDED assert                     assert(_left@.union(right@) =~= self@.remove(k@));
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v@));
                    // Veracity: NEEDED assert (speed hint)
                    assert(self@.contains(v@));
                    // Veracity: NEEDED assert
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater
                        implies v.cmp_spec(&t) == Less || v@ == t@ by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(t@ != k@);
                        // Veracity: NEEDED assert (speed hint)
                        assert(self@.remove(k@).contains(t@));
                        // Veracity: NEEDED assert (speed hint)
                        assert(_left@.union(right@).contains(t@));
                        if _left@.contains(t@) {
                            // Veracity: NEEDED assert (speed hint)
                            assert(t.cmp_spec(k) == Less);
                        }
// Veracity: UNNEEDED assert                         assert(right@.contains(t@));
                    };
                }
            }
            result
        }
// Veracity: NEEDED proof block

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to next_iter
        fn next(&self, k: &T) -> (successor: Option<T>)
        { self.next_iter(k) }

        /// Split via BST split.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split
        fn split_iter(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        {
            let (left_tree, found, right_tree) = self.base_set.tree.split(k);
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(left_tree@.union(right_tree@) =~= self@.remove(k@));
                // Veracity: NEEDED assert
                assert forall|v: T::V| left_tree@.contains(v) implies #[trigger] self@.contains(v) by {
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v));
                };
                // Veracity: NEEDED assert (speed hint)
                assert(left_tree@.subset_of(self@));
                // Veracity: NEEDED assert
                assert forall|v: T::V| right_tree@.contains(v) implies #[trigger] self@.contains(v) by {
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v));
                };
                // Veracity: NEEDED assert (speed hint)
                assert(right_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(left_tree@, self@);
                vstd::set_lib::lemma_len_subset(right_tree@, self@);
                // Veracity: NEEDED assert
                assert forall|x: T::V| #[trigger] self@.contains(x)
                    implies left_tree@.contains(x) || right_tree@.contains(x) || x == k@ by {
                    if x != k@ {
                        // Veracity: NEEDED assert (speed hint)
                        assert(self@.remove(k@).contains(x));
                        // Veracity: NEEDED assert (speed hint)
                        assert(left_tree@.union(right_tree@).contains(x));
                    }
                };
            }
            let left = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: left_tree } };
            let right = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: right_tree } };
            (left, found, right)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to split_iter
        fn split(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        // Veracity: NEEDED proof block
        { self.split_iter(k) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to union
        fn join(left: &Self, right: &Self) -> (joined: Self)
        { left.union(right) }

        /// Range query via two splits.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- two BST splits + conditional inserts
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
        {
            let (_lt_k1, found_k1, right1) = self.base_set.tree.split(k1);
            let (mid, found_k2, _gt_k2) = right1.split(k2);
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(_lt_k1@.union(right1@) =~= self@.remove(k1@));
                // Veracity: NEEDED assert
                assert forall|v: T::V| right1@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED assert
                    assert(self@.remove(k1@).contains(v));
                };
// Veracity: UNNEEDED assert                 assert(mid@.union(_gt_k2@) =~= right1@.remove(k2@));
                // Veracity: NEEDED assert
                // Veracity: NEEDED proof block
                assert forall|v: T::V| mid@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED assert
                    assert(right1@.remove(k2@).contains(v));
// Veracity: UNNEEDED assert                     assert(right1@.contains(v));
                };
// Veracity: UNNEEDED assert                 assert(mid@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(mid@, self@);
            }
            let mut result_tree = mid;
            if found_k1 {
                let k1_clone = k1.clone_plus();
                result_tree.insert(k1_clone);
            }
            proof {
// Veracity: UNNEEDED assert                 assert forall|v: T::V| result_tree@.contains(v) implies self@.contains(v) by {
// Veracity: NEEDED proof block (speed hint)
// Veracity: UNNEEDED assert                     if mid@.contains(v) {
// Veracity: UNNEEDED assert                     } else {
// Veracity: UNNEEDED assert                         // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                         assert(self@.contains(k1@));
// Veracity: UNNEEDED assert                     }
// Veracity: UNNEEDED assert                 };
                // Veracity: NEEDED assert (speed hint)
                assert(result_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            }
            if found_k2 {
                let k2_clone = k2.clone_plus();
                result_tree.insert(k2_clone);
            }
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert forall|v: T::V| result_tree@.contains(v) implies self@.contains(v) by {
                    if mid@.contains(v) {
                    } else if v == k1@ {
                        // Veracity: NEEDED assert (speed hint)
                        assert(self@.contains(k1@));
                    } else {
// Veracity: UNNEEDED assert                         assert(self@.contains(k2@));
                    }
                };
                // Veracity: NEEDED assert (speed hint)
                // Veracity: NEEDED proof block
                assert(result_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            }
            OrderedSetStPer { base_set: AVLTreeSetStPer { tree: result_tree } }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to get_range_iter
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
        { self.get_range_iter(k1, k2) }

        /// Rank via split + size.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- BST split + size
        fn rank_iter(&self, k: &T) -> (rank: usize)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            proof {
                // Veracity: NEEDED assert (speed hint)
                assert(left@.union(_right@) =~= self@.remove(k@));
                // Veracity: NEEDED assert
                assert forall|v: T::V| left@.contains(v) implies self@.contains(v) by {
                    // Veracity: NEEDED assert
                    assert(self@.remove(k@).contains(v));
                };
                // Veracity: NEEDED assert (speed hint)
                assert(left@.subset_of(self@));
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
            // Veracity: NEEDED proof block
            } else {
                tree_select(&self.base_set.tree, i)
            // Veracity: NEEDED proof block
            }
        }

        /// Split by rank: first i elements go left, rest go right.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- tree_select + BST split + insert
        fn split_rank_iter(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let sz = self.size();
            if i >= sz {
                (self.clone(), Self::empty())
            } else {
                let pivot = tree_select(&self.base_set.tree, i);
                // Veracity: NEEDED assert (speed hint)
                proof { assert(pivot.is_some()); }
                let pivot_key = pivot.unwrap();
                let (left_tree, _found, right_tree) = self.base_set.tree.split(&pivot_key);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert (speed hint)
                    assert(left_tree@.union(right_tree@) =~= self@.remove(pivot_key@));
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|v: T::V| left_tree@.contains(v) implies self@.contains(v) by {
                        // Veracity: NEEDED assert (speed hint)
                        assert(self@.remove(pivot_key@).contains(v));
                    };
// Veracity: UNNEEDED assert                     assert forall|v: T::V| right_tree@.contains(v) implies self@.contains(v) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert(self@.remove(pivot_key@).contains(v));
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                     assert(left_tree@.subset_of(self@));
// Veracity: UNNEEDED assert                     assert(right_tree@.subset_of(self@));
                    // Veracity: NEEDED proof block
                    vstd::set_lib::lemma_len_subset(left_tree@, self@);
                    vstd::set_lib::lemma_len_subset(right_tree@, self@);
                }
                let mut right_with_pivot = right_tree;
                right_with_pivot.insert(pivot_key);
                proof {
// Veracity: UNNEEDED assert                     assert forall|v: T::V| right_with_pivot@.contains(v) implies self@.contains(v) by {
// Veracity: UNNEEDED assert                         if right_tree@.contains(v) {
// Veracity: UNNEEDED assert                         } else {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             assert(v == pivot_key@);
// Veracity: UNNEEDED assert                             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                             assert(self@.contains(pivot_key@));
// Veracity: UNNEEDED assert                         }
// Veracity: UNNEEDED assert                     };
// Veracity: UNNEEDED assert                     assert(right_with_pivot@.subset_of(self@));
                    vstd::set_lib::lemma_len_subset(right_with_pivot@, self@);
                }
                let left = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: left_tree } };
                let right = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: right_with_pivot } };
                proof {
                    // Disjointness: left has elements < pivot, right has elements >= pivot.
// Veracity: UNNEEDED assert                     assert(left@.disjoint(right@)) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         assert forall|x: T::V| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             if left@.contains(x) && right@.contains(x) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // x in left_tree@: cmp_spec(pivot) == Less.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // x in right_with_pivot@: either x == pivot@ or x in right_tree@ (Greater).
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // left_tree disjoint right_tree, so x not in both.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // If x == pivot@, but left elements have cmp_spec(pivot) == Less,
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 // so x@ != pivot@ (view_ord_consistent). Contradiction.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 if !right_tree@.contains(x) {
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                     // x == pivot_key@, but left@ elements have cmp(pivot) == Less.
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                     assert(x == pivot_key@);
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                     assert(!left_tree@.contains(pivot_key@));
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                                 }
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                             }
// Veracity: UNNEEDED assert // Veracity: UNNEEDED assert                         };
// Veracity: UNNEEDED assert                     };
                    // Coverage.
                    // Veracity: NEEDED assert (speed hint)
                    assert forall|x: T::V| #[trigger] self@.contains(x)
                        implies left@.contains(x) || right@.contains(x) by {
                        if x != pivot_key@ {
// Veracity: UNNEEDED assert                             assert(self@.remove(pivot_key@).contains(x));
// Veracity: UNNEEDED assert                             assert(left_tree@.union(right_tree@).contains(x));
                            if right_tree@.contains(x) {
                                // Veracity: NEEDED assert (speed hint)
                                assert(right_with_pivot@.contains(x));
                            }
                        } else {
                            // Veracity: NEEDED assert (speed hint)
                            assert(right_with_pivot@.contains(pivot_key@));
                        }
                    };
                }
                (left, right)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) -- delegates to split_rank_iter
        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        { self.split_rank_iter(i) }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- from_vec + n treap inserts
    pub fn from_sorted_elements<T: StT + Ord + TotalOrder>(elements: Vec<T>) -> (constructed: OrderedSetStPer<T>)
        requires
            elements@.len() < usize::MAX,
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures constructed.spec_orderedsetstper_wf()
    {
        // Veracity: NEEDED assert (speed hint)
        assert(obeys_feq_full_trigger::<T>());
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStPer::from_seq(seq)
    }


    impl<T: StT + Ord + TotalOrder> OrderedSetStPer<T> {
        /// Returns an iterator over the set elements via in-order traversal.
        pub fn iter(&self) -> (it: OrderedSetStPerIter<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self@.len(),
                iter_invariant(&it),
        {
            let mut elements: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut elements);
            OrderedSetStPerIter { inner: elements.into_iter() }
        }
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPerIter<T: StT + Ord + TotalOrder> {
        pub inner: IntoIter<T>,
    }

    impl<T: StT + Ord + TotalOrder> View for OrderedSetStPerIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<T: StT + Ord + TotalOrder>(it: &OrderedSetStPerIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StT + Ord + TotalOrder> std::iter::Iterator for OrderedSetStPerIter<T> {
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
    pub struct OrderedSetStPerGhostIterator<T: StT + Ord + TotalOrder> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StT + Ord + TotalOrder> View for OrderedSetStPerGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIteratorNew for OrderedSetStPerIter<T> {
        type GhostIter = OrderedSetStPerGhostIterator<T>;
        open spec fn ghost_iter(&self) -> OrderedSetStPerGhostIterator<T> {
            OrderedSetStPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StT + Ord + TotalOrder> vstd::pervasive::ForLoopGhostIterator for OrderedSetStPerGhostIterator<T> {
        type ExecIter = OrderedSetStPerIter<T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedSetStPerIter<T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedSetStPerIter<T>) -> OrderedSetStPerGhostIterator<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    //		Section 12. derive impls in verus!


    impl<T: StT + Ord + TotalOrder> Clone for OrderedSetStPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrderedSetStPer {
                base_set: self.base_set.clone(),
            }
        }
    }

    impl<T: StT + Ord + TotalOrder> Default for OrderedSetStPer<T> {
        fn default() -> (d: Self)
            ensures d@.finite(), d@.len() == 0
        {
            OrderedSetStPer { base_set: AVLTreeSetStPer { tree: ParamBST::new() } }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord + TotalOrder> PartialEqSpecImpl for OrderedSetStPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord + TotalOrder> Eq for OrderedSetStPer<T> {}

    } // verus!

    //		Section 13. macros


    /// Macro for creating ordered sets from sorted element lists.
    #[macro_export]
    macro_rules! OrderedSetStPerLit {
        () => {
            $crate::Chap43::OrderedSetStPer::OrderedSetStPer::OrderedSetStPer::empty()
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap43::OrderedSetStPer::OrderedSetStPer::from_sorted_elements(vec![$($elem),+])
        };
    }

    //		Section 14. derive impls outside verus!

    impl<T: StT + Ord + TotalOrder> PartialEq for OrderedSetStPer<T> {
        fn eq(&self, other: &Self) -> bool {
            self.base_set == other.base_set
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Debug for OrderedSetStPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStPer<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord + TotalOrder + fmt::Debug> fmt::Debug for OrderedSetStPerIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStPerIter({:?})", self.inner)
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStPerIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStPerIter")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Debug for OrderedSetStPerGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStPerGhostIterator")
        }
    }

    impl<T: StT + Ord + TotalOrder> fmt::Display for OrderedSetStPerGhostIterator<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedSetStPerGhostIterator")
        }
    }
}
