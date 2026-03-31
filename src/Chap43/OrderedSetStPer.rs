//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded persistent ordered set implementation extending AVLTreeSetStPer.
//!
//! R82: Rewritten from AVLTreeSeqStPer-backed to ParamBST-backed. All ordered operations
//! (first, last, previous, next, split, get_range, rank, select, split_rank) now use tree
//! operations (min_key, max_key, split, expose, size) instead of scanning a flat sequence.

pub mod OrderedSetStPer {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

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

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
    vstd::laws_cmp::group_laws_cmp,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPer<T: StT + Ord> {
        pub base_set: AVLTreeSetStPer<T>,
    }

    pub type OrderedSetPer<T> = OrderedSetStPer<T>;

    // 5. view impls

    impl<T: StT + Ord> View for OrderedSetStPer<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.base_set@ }
    }

    // 6. spec fns

    // 7. proof fns

    /// cmp_spec antisymmetry: Greater(a,b) implies Less(b,a).
    proof fn lemma_cmp_antisymmetry<T: StT + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            a.cmp_spec(&b) == Greater,
        ensures b.cmp_spec(&a) == Less,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// cmp_spec transitivity: Less(a,b) and Less(b,c) implies Less(a,c).
    proof fn lemma_cmp_transitivity<T: StT + Ord>(a: T, b: T, c: T)
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
    proof fn lemma_cmp_equal_congruent<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
        assert(a@ == b@);
    }

    /// Maximum key in a ParamBST via right-spine walk.
    fn tree_max_key<T: StT + Ord>(tree: &ParamBST<T>) -> (maximum: Option<T>)
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
                proof {
                    assert(!left@.contains(key@));
                    assert(!right@.contains(key@));
                    assert(!left@.union(right@).contains(key@));
                    vstd::set_lib::lemma_len_subset(right@, left@.union(right@));
                }
                if right.is_empty() {
                    proof {
                        assert forall|t: T| (#[trigger] tree@.contains(t@))
                            implies t.cmp_spec(&key) == Less || key@ == t@ by {
                            if left@.contains(t@) {
                            } else {
                            }
                        };
                    }
                    Some(key)
                } else {
                    let max_right = tree_max_key(&right);
                    proof {
                        let mr = max_right.unwrap();
                        assert(right@.contains(mr@));
                        assert(tree@.contains(mr@));
                        lemma_cmp_antisymmetry(mr, key);
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
    fn tree_select<T: StT + Ord>(tree: &ParamBST<T>, i: usize) -> (selected: Option<T>)
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
            Exposed::Leaf => None,
            Exposed::Node(left, key, right) => {
                proof {
                    assert(!left@.contains(key@));
                    assert(!right@.contains(key@));
                    assert(!left@.union(right@).contains(key@));
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                }
                let left_sz = left.size();
                if i < left_sz {
                    let result = tree_select(&left, i);
                    proof {
                        if result.is_some() {
                            assert(left@.contains(result.unwrap()@));
                            assert(tree@.contains(result.unwrap()@));
                        }
                    }
                    result
                } else if i as usize == left_sz {
                    Some(key)
                } else {
                    let adjusted = i - left_sz - 1;
                    let result = tree_select(&right, adjusted);
                    proof {
                        if result.is_some() {
                            assert(right@.contains(result.unwrap()@));
                            assert(tree@.contains(result.unwrap()@));
                        }
                    }
                    result
                }
            }
        }
    }

    // 8. traits

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with persistent semantics.
    /// Postconditions for ordering operations use cmp_spec (from Ord) rather than TotalOrder::le;
    /// matching ParamBST's ensure style directly.
    pub trait OrderedSetStPerTrait<T: StT + Ord>: Sized + View<V = Set<<T as View>::V>> {
        spec fn spec_orderedsetstper_wf(&self) -> bool;

        // Base set operations (ADT 41.1) - delegated
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn empty() -> (empty: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures empty@ == Set::<<T as View>::V>::empty(), empty.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(x: T) -> (tree: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures tree@ == Set::<<T as View>::V>::empty().insert(x@), tree.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn find(&self, x: &T) -> (found: bool)
            requires self.spec_orderedsetstper_wf(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn insert(&self, x: T) -> (updated: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures updated@ == self@.insert(x@), updated.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn delete(&self, x: &T) -> (updated: Self)
            requires self.spec_orderedsetstper_wf(),
            ensures updated@ == self@.remove(x@), updated.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn intersection(&self, other: &Self) -> (common: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures common@ == self@.intersect(other@), common.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_orderedsetstper_wf(),
                other.spec_orderedsetstper_wf(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures combined@ == self@.union(other@), combined.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(m log(n/m + 1))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires self.spec_orderedsetstper_wf(), other.spec_orderedsetstper_wf(),
            ensures remaining@ == self@.difference(other@), remaining.spec_orderedsetstper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                seq.spec_avltreeseqstper_wf(),
                seq@.to_set() =~= self@,
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqstper_wf(),
                seq@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures constructed.spec_orderedsetstper_wf();

        // Ordering operations (ADT 43.1) — postconditions in cmp_spec style.
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn first(&self) -> (first: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn last(&self) -> (last: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn join(left: &Self, right: &Self) -> (joined: Self)
            requires
                left.spec_orderedsetstper_wf(),
                right.spec_orderedsetstper_wf(),
                left@.len() + right@.len() < usize::MAX as nat,
            ensures joined@ == left@.union(right@), joined@.finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn rank(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
        fn select(&self, i: usize) -> (selected: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                i >= self@.len() ==> selected matches None,
                selected matches Some(v) ==> self@.contains(v@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(lg n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS
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
        fn first_iter(&self) -> (first: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> first matches None,
                first matches Some(v) ==> self@.contains(v@),
                first matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
        fn last_iter(&self) -> (last: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                self@.len() == 0 <==> last matches None,
                last matches Some(v) ==> self@.contains(v@),
                last matches Some(v) ==> forall|t: T| #[trigger] self@.contains(t@) ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                predecessor matches Some(v) ==> self@.contains(v@),
                predecessor matches Some(v) ==> v.cmp_spec(k) == Less,
                predecessor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less ==>
                    t.cmp_spec(&v) == Less || v@ == t@;
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                successor matches Some(v) ==> self@.contains(v@),
                successor matches Some(v) ==> v.cmp_spec(k) == Greater,
                successor matches Some(v) ==> forall|t: T|
                    #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater ==>
                    v.cmp_spec(&t) == Less || v@ == t@;
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
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
            requires
                self.spec_orderedsetstper_wf(),
                self@.len() + 1 < usize::MAX as nat,
            ensures
                self@.finite(),
                range@.finite(),
                range@.subset_of(self@);
        fn rank_iter(&self, k: &T) -> (rank: usize)
            requires self.spec_orderedsetstper_wf(),
            ensures
                self@.finite(),
                rank <= self@.len();
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

    // 9. impls

    impl<T: StT + Ord> OrderedSetStPerTrait<T> for OrderedSetStPer<T> {
        open spec fn spec_orderedsetstper_wf(&self) -> bool {
            self.base_set.spec_avltreesetstper_wf()
            && obeys_feq_full::<T>()
            && vstd::laws_cmp::obeys_cmp_spec::<T>()
            && view_ord_consistent::<T>()
        }

        fn size(&self) -> (count: usize)
        { self.base_set.size() }

        fn empty() -> (empty: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            OrderedSetStPer { base_set: AVLTreeSetStPer::empty() }
        }

        fn singleton(x: T) -> (tree: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            OrderedSetStPer { base_set: AVLTreeSetStPer::singleton(x) }
        }

        fn find(&self, x: &T) -> (found: bool)
        { self.base_set.find(x) }

        fn insert(&self, x: T) -> (updated: Self)
        {
            OrderedSetStPer { base_set: self.base_set.insert(x) }
        }

        fn delete(&self, x: &T) -> (updated: Self)
        {
            OrderedSetStPer { base_set: self.base_set.delete(x) }
        }

        fn filter<F: PredSt<T>>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            OrderedSetStPer { base_set: self.base_set.filter(f, Ghost(spec_pred)) }
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            OrderedSetStPer { base_set: self.base_set.intersection(&other.base_set) }
        }

        fn union(&self, other: &Self) -> (combined: Self)
        {
            OrderedSetStPer { base_set: self.base_set.union(&other.base_set) }
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            OrderedSetStPer { base_set: self.base_set.difference(&other.base_set) }
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStPerS<T>)
        {
            let mut elements: Vec<T> = Vec::new();
            self.base_set.tree.collect_in_order(&mut elements);
            proof {
                assert(elements@.len() < usize::MAX);
            }
            let result = AVLTreeSeqStPerS::from_vec(elements);
            proof {
                let ghost elem_views = elements@.map_values(|t: T| t@);
                assert(result@ =~= elem_views);

                assert forall|i: int| 0 <= i < elements@.len()
                    implies elem_views[i] == (#[trigger] elements@[i])@ by {};

                assert forall|i: int| 0 <= i < result@.len()
                    implies #[trigger] self@.contains(result@[i]) by {
                    assert(result@[i] == elem_views[i]);
                    assert(elem_views[i] == elements@[i]@);
                };

                assert forall|v: T::V| self@.contains(v)
                    implies result@.contains(v) by {
                    let i = choose|i: int| 0 <= i < elements@.len() && (#[trigger] elements@[i])@ == v;
                    assert(0 <= i < result@.len());
                    assert(result@[i] == elem_views[i]);
                    assert(elem_views[i] == elements@[i]@);
                    assert(result@[i] == v);
                };

                assert(result@.to_set() =~= self@);
            }
            result
        }

        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> (constructed: Self)
        {
            assert(obeys_feq_full_trigger::<T>());
            let mut constructed = Self::empty();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    seq.spec_avltreeseqstper_wf(),
                    n as nat == seq.spec_seq().len(),
                    0 <= i <= n,
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
        fn first_iter(&self) -> (first: Option<T>)
        {
            self.base_set.tree.min_key()
        }

        fn first(&self) -> (first: Option<T>)
        { self.first_iter() }

        /// Last element (maximum) via tree_max_key.
        fn last_iter(&self) -> (last: Option<T>)
        {
            tree_max_key(&self.base_set.tree)
        }

        fn last(&self) -> (last: Option<T>)
        { self.last_iter() }

        /// Predecessor via split + max_key on left subtree.
        fn previous_iter(&self, k: &T) -> (predecessor: Option<T>)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            let result = tree_max_key(&left);
            proof {
                if result.is_some() {
                    let v = result.unwrap();
                    assert(left@.contains(v@));
                    assert(v.cmp_spec(k) == Less);
                    assert(v@ != k@);
                    assert(left@.union(_right@) =~= self@.remove(k@));
                    assert(self@.remove(k@).contains(v@));
                    assert(self@.contains(v@));
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Less
                        implies t.cmp_spec(&v) == Less || v@ == t@ by {
                        assert(t@ != k@);
                        assert(self@.remove(k@).contains(t@));
                        assert(left@.union(_right@).contains(t@));
                        if _right@.contains(t@) {
                            assert(t.cmp_spec(k) == Greater);
                        }
                        assert(left@.contains(t@));
                    };
                }
            }
            result
        }

        fn previous(&self, k: &T) -> (predecessor: Option<T>)
        { self.previous_iter(k) }

        /// Successor via split + min_key on right subtree.
        fn next_iter(&self, k: &T) -> (successor: Option<T>)
        {
            let (_left, _found, right) = self.base_set.tree.split(k);
            let result = right.min_key();
            proof {
                if result.is_some() {
                    let v = result.unwrap();
                    assert(right@.contains(v@));
                    assert(v.cmp_spec(k) == Greater);
                    assert(v@ != k@);
                    assert(_left@.union(right@) =~= self@.remove(k@));
                    assert(self@.remove(k@).contains(v@));
                    assert(self@.contains(v@));
                    assert forall|t: T| #[trigger] self@.contains(t@) && t.cmp_spec(k) == Greater
                        implies v.cmp_spec(&t) == Less || v@ == t@ by {
                        assert(t@ != k@);
                        assert(self@.remove(k@).contains(t@));
                        assert(_left@.union(right@).contains(t@));
                        if _left@.contains(t@) {
                            assert(t.cmp_spec(k) == Less);
                        }
                        assert(right@.contains(t@));
                    };
                }
            }
            result
        }

        fn next(&self, k: &T) -> (successor: Option<T>)
        { self.next_iter(k) }

        /// Split via BST split.
        fn split_iter(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        {
            let (left_tree, found, right_tree) = self.base_set.tree.split(k);
            proof {
                assert(left_tree@.union(right_tree@) =~= self@.remove(k@));
                assert forall|v: T::V| left_tree@.contains(v) implies #[trigger] self@.contains(v) by {
                    assert(self@.remove(k@).contains(v));
                };
                assert(left_tree@.subset_of(self@));
                assert forall|v: T::V| right_tree@.contains(v) implies #[trigger] self@.contains(v) by {
                    assert(self@.remove(k@).contains(v));
                };
                assert(right_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(left_tree@, self@);
                vstd::set_lib::lemma_len_subset(right_tree@, self@);
                assert forall|x: T::V| #[trigger] self@.contains(x)
                    implies left_tree@.contains(x) || right_tree@.contains(x) || x == k@ by {
                    if x != k@ {
                        assert(self@.remove(k@).contains(x));
                        assert(left_tree@.union(right_tree@).contains(x));
                    }
                };
            }
            let left = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: left_tree } };
            let right = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: right_tree } };
            (left, found, right)
        }

        fn split(&self, k: &T) -> (split: (Self, bool, Self))
            where Self: Sized
        { self.split_iter(k) }

        fn join(left: &Self, right: &Self) -> (joined: Self)
        { left.union(right) }

        /// Range query via two splits.
        fn get_range_iter(&self, k1: &T, k2: &T) -> (range: Self)
        {
            let (_lt_k1, found_k1, right1) = self.base_set.tree.split(k1);
            let (mid, found_k2, _gt_k2) = right1.split(k2);
            proof {
                assert(_lt_k1@.union(right1@) =~= self@.remove(k1@));
                assert forall|v: T::V| right1@.contains(v) implies self@.contains(v) by {
                    assert(self@.remove(k1@).contains(v));
                };
                assert(mid@.union(_gt_k2@) =~= right1@.remove(k2@));
                assert forall|v: T::V| mid@.contains(v) implies self@.contains(v) by {
                    assert(right1@.remove(k2@).contains(v));
                    assert(right1@.contains(v));
                };
                assert(mid@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(mid@, self@);
            }
            let mut result_tree = mid;
            if found_k1 {
                let k1_clone = k1.clone_plus();
                result_tree.insert(k1_clone);
            }
            proof {
                assert forall|v: T::V| result_tree@.contains(v) implies self@.contains(v) by {
                    if mid@.contains(v) {
                    } else {
                        assert(self@.contains(k1@));
                    }
                };
                assert(result_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            }
            if found_k2 {
                let k2_clone = k2.clone_plus();
                result_tree.insert(k2_clone);
            }
            proof {
                assert forall|v: T::V| result_tree@.contains(v) implies self@.contains(v) by {
                    if mid@.contains(v) {
                    } else if v == k1@ {
                        assert(self@.contains(k1@));
                    } else {
                        assert(self@.contains(k2@));
                    }
                };
                assert(result_tree@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(result_tree@, self@);
            }
            OrderedSetStPer { base_set: AVLTreeSetStPer { tree: result_tree } }
        }

        fn get_range(&self, k1: &T, k2: &T) -> (range: Self)
        { self.get_range_iter(k1, k2) }

        /// Rank via split + size.
        fn rank_iter(&self, k: &T) -> (rank: usize)
        {
            let (left, _found, _right) = self.base_set.tree.split(k);
            proof {
                assert(left@.union(_right@) =~= self@.remove(k@));
                assert forall|v: T::V| left@.contains(v) implies self@.contains(v) by {
                    assert(self@.remove(k@).contains(v));
                };
                assert(left@.subset_of(self@));
                vstd::set_lib::lemma_len_subset(left@, self@);
            }
            left.size()
        }

        fn rank(&self, k: &T) -> (rank: usize)
        { self.rank_iter(k) }

        /// Select the i-th element using tree_select.
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
        fn split_rank_iter(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let sz = self.size();
            if i >= sz {
                (self.clone(), Self::empty())
            } else {
                let pivot = tree_select(&self.base_set.tree, i);
                proof { assert(pivot.is_some()); }
                let pivot_key = pivot.unwrap();
                let (left_tree, _found, right_tree) = self.base_set.tree.split(&pivot_key);
                proof {
                    assert(left_tree@.union(right_tree@) =~= self@.remove(pivot_key@));
                    assert forall|v: T::V| left_tree@.contains(v) implies self@.contains(v) by {
                        assert(self@.remove(pivot_key@).contains(v));
                    };
                    assert forall|v: T::V| right_tree@.contains(v) implies self@.contains(v) by {
                        assert(self@.remove(pivot_key@).contains(v));
                    };
                    assert(left_tree@.subset_of(self@));
                    assert(right_tree@.subset_of(self@));
                    vstd::set_lib::lemma_len_subset(left_tree@, self@);
                    vstd::set_lib::lemma_len_subset(right_tree@, self@);
                }
                let mut right_with_pivot = right_tree;
                right_with_pivot.insert(pivot_key);
                proof {
                    assert forall|v: T::V| right_with_pivot@.contains(v) implies self@.contains(v) by {
                        if right_tree@.contains(v) {
                        } else {
                            assert(v == pivot_key@);
                            assert(self@.contains(pivot_key@));
                        }
                    };
                    assert(right_with_pivot@.subset_of(self@));
                    vstd::set_lib::lemma_len_subset(right_with_pivot@, self@);
                }
                let left = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: left_tree } };
                let right = OrderedSetStPer { base_set: AVLTreeSetStPer { tree: right_with_pivot } };
                proof {
                    // Disjointness: left has elements < pivot, right has elements >= pivot.
                    assert(left@.disjoint(right@)) by {
                        assert forall|x: T::V| !(#[trigger] left@.contains(x) && right@.contains(x)) by {
                            if left@.contains(x) && right@.contains(x) {
                                // x in left_tree@: cmp_spec(pivot) == Less.
                                // x in right_with_pivot@: either x == pivot@ or x in right_tree@ (Greater).
                                // left_tree disjoint right_tree, so x not in both.
                                // If x == pivot@, but left elements have cmp_spec(pivot) == Less,
                                // so x@ != pivot@ (view_ord_consistent). Contradiction.
                                if !right_tree@.contains(x) {
                                    // x == pivot_key@, but left@ elements have cmp(pivot) == Less.
                                    assert(x == pivot_key@);
                                    assert(!left_tree@.contains(pivot_key@));
                                }
                            }
                        };
                    };
                    // Coverage.
                    assert forall|x: T::V| #[trigger] self@.contains(x)
                        implies left@.contains(x) || right@.contains(x) by {
                        if x != pivot_key@ {
                            assert(self@.remove(pivot_key@).contains(x));
                            assert(left_tree@.union(right_tree@).contains(x));
                            if right_tree@.contains(x) {
                                assert(right_with_pivot@.contains(x));
                            }
                        } else {
                            assert(right_with_pivot@.contains(pivot_key@));
                        }
                    };
                }
                (left, right)
            }
        }

        fn split_rank(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        { self.split_rank_iter(i) }
    }

    pub fn from_sorted_elements<T: StT + Ord>(elements: Vec<T>) -> (constructed: OrderedSetStPer<T>)
        requires
            elements@.len() < usize::MAX,
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
        ensures constructed.spec_orderedsetstper_wf()
    {
        assert(obeys_feq_full_trigger::<T>());
        let seq = AVLTreeSeqStPerS::from_vec(elements);
        OrderedSetStPer::from_seq(seq)
    }

    // 10. iterators

    impl<T: StT + Ord> OrderedSetStPer<T> {
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

    #[verifier::reject_recursive_types(T)]
    pub struct OrderedSetStPerIter<T: StT + Ord> {
        pub inner: IntoIter<T>,
    }

    impl<T: StT + Ord> View for OrderedSetStPerIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<T: StT + Ord>(it: &OrderedSetStPerIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<T: StT + Ord> std::iter::Iterator for OrderedSetStPerIter<T> {
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
    pub struct OrderedSetStPerGhostIterator<T: StT + Ord> {
        pub pos: int,
        pub elements: Seq<T>,
    }

    impl<T: StT + Ord> View for OrderedSetStPerGhostIterator<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedSetStPerIter<T> {
        type GhostIter = OrderedSetStPerGhostIterator<T>;
        open spec fn ghost_iter(&self) -> OrderedSetStPerGhostIterator<T> {
            OrderedSetStPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedSetStPerGhostIterator<T> {
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

    // 11. derive impls in verus!

    impl<T: StT + Ord> Clone for OrderedSetStPer<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrderedSetStPer {
                base_set: self.base_set.clone(),
            }
        }
    }

    impl<T: StT + Ord> Default for OrderedSetStPer<T> {
        fn default() -> (d: Self)
            ensures d@.finite(), d@.len() == 0
        {
            OrderedSetStPer { base_set: AVLTreeSetStPer { tree: ParamBST::new() } }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: StT + Ord> PartialEqSpecImpl for OrderedSetStPer<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: StT + Ord> Eq for OrderedSetStPer<T> {}

    } // verus!

    // 12. macros

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

    // 13. derive impls outside verus!

    impl<T: StT + Ord> PartialEq for OrderedSetStPer<T> {
        fn eq(&self, other: &Self) -> bool {
            self.base_set == other.base_set
        }
    }

    impl<T: StT + Ord> fmt::Debug for OrderedSetStPer<T> {
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

    impl<T: StT + Ord> fmt::Display for OrderedSetStPer<T> {
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
}
