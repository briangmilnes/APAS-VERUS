//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Multi-threaded ephemeral set backed by BSTParaMtEph for parallel set operations.
//!
//! Work/Span Analysis (via delegation to BSTParaMtEph parallel D&C):
//! - union: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - intersection: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - difference: Work O(m·lg(1+n/m)), Span O(lg² n) via PARALLEL divide-and-conquer
//! - filter: Work O(Σ W(f(x))), Span O(n + max S(f(x))) — sequential (spec_fn not Send)

pub mod AVLTreeSetMtEph {

    // Table of Contents
    // 1. module
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    use std::fmt;

    use vstd::prelude::*;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap38::BSTParaMtEph::BSTParaMtEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full_trigger, lemma_cloned_view_eq};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::Types::Types::*;

    verus! {

// 3. broadcast use

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        pub tree: ParamBST<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphIter<T: StTInMtT + Ord + 'static> {
        pub snapshot: Vec<T>,
        pub pos: usize,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct AVLTreeSetMtEphGhostIter<T: StTInMtT + Ord + 'static> {
        pub pos: int,
        pub elements: Seq<T::V>,
    }

    // 5. view impls

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEph<T> {
        type V = Set<<T as View>::V>;
        open spec fn view(&self) -> Set<<T as View>::V> { self.tree@ }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphIter<T> {
        type V = (int, Seq<T::V>);
        open spec fn view(&self) -> (int, Seq<T::V>) {
            (self.pos as int, self.snapshot@.map_values(|t: T| t@))
        }
    }

    impl<T: StTInMtT + Ord + 'static> View for AVLTreeSetMtEphGhostIter<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> { self.elements.take(self.pos) }
    }

    // 6. spec fns

    pub open spec fn avltreesetmteph_iter_invariant<T: StTInMtT + Ord + 'static>(it: &AVLTreeSetMtEphIter<T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 8. traits

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static>: Sized + View<V = Set<<T as View>::V>> {
        /// Well-formedness: backing BST is well-formed.
        spec fn spec_avltreesetmteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> (count: usize)
            requires self.spec_avltreesetmteph_wf(),
            ensures count == self@.len(), self@.finite();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential in-order traversal
        /// - claude-4-sonet: Work Θ(n), Span Θ(n)
        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures
                self@.finite(),
                seq@.to_set() =~= self@,
                seq.spec_avltreeseqsteph_wf(),
                forall|i: int| 0 <= i < seq@.len() ==> #[trigger] self@.contains(seq@[i]);
        /// - APAS Cost Spec 41.4: Work 1, Span 1
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> (empty: Self)
            ensures
                empty@ == Set::<<T as View>::V>::empty(),
                empty.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> (tree: Self)
            ensures
                tree@ == Set::<<T as View>::V>::empty().insert(x@),
                tree.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch41 Ex 41.3): Work O(n lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n lg n) — DIFFERS: sequential loop of inserts
        /// - claude-4-sonet: Work Θ(n log n), Span Θ(log n), Parallelism Θ(n)
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
            requires
                seq.spec_avltreeseqsteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                constructed@ =~= seq@.to_set(),
                constructed.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u + Σ W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(Σ W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ W(f(x))), Span O(n + max S(f(x))) — DIFFERS: sequential filter (spec_fn not Send)
        /// - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                forall|t: &T| #[trigger] f.requires((t,)),
                forall|x: T, keep: bool|
                    f.ensures((&x,), keep) ==> keep == spec_pred(x@),
            ensures
                filtered@.subset_of(self@),
                filtered.spec_avltreesetmteph_wf(),
                forall|v: T::V| #[trigger] filtered@.contains(v)
                    ==> self@.contains(v) && spec_pred(v),
                forall|v: T::V| self@.contains(v) && spec_pred(v)
                    ==> #[trigger] filtered@.contains(v);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn intersection(&self, other: &Self) -> (common: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                common@ == self@.intersect(other@),
                common.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                remaining@ == self@.difference(other@),
                remaining.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(m * lg(1+n/m)), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m·lg(1+n/m)), Span O(lg² n) — matches APAS; parallel D&C via BSTParaMtEph
        /// - claude-4-sonet: Work Θ(m + n), Span Θ(log(m + n)), Parallelism Θ((m+n)/log(m+n))
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_avltreesetmteph_wf(),
                other.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                self@.len() + other@.len() < usize::MAX as nat,
            ensures
                combined@ == self@.union(other@),
                combined.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn find(&self, x: &T) -> (found: bool)
            requires
                self.spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures found == self@.contains(x@);
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn delete(&mut self, x: &T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
            ensures
                self@ == old(self)@.remove(x@),
                self.spec_avltreesetmteph_wf();
        /// - Alg Analysis: APAS (Ch41 CS 41.3): Work O(u), Span O(1)
        /// - Alg Analysis: APAS (Ch41 CS 41.4): Work O(lg |a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) — matches APAS CS 41.4
        /// - claude-4-sonet: Work Θ(log n), Span Θ(log n), Parallelism Θ(1)
        fn insert(&mut self, x: T)
            requires
                old(self).spec_avltreesetmteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<T>(),
                view_ord_consistent::<T>(),
                old(self)@.len() + 1 < usize::MAX as nat,
            ensures
                self@ == old(self)@.insert(x@),
                self.spec_avltreesetmteph_wf();
        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it);
    }

    // 9. impls

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        open spec fn spec_avltreesetmteph_wf(&self) -> bool {
            self.tree@.finite()
        }

        fn size(&self) -> (count: usize)
        {
            self.tree.size()
        }

        fn to_seq(&self) -> (seq: AVLTreeSeqStEphS<T>)
        {
            proof { assert(obeys_feq_full_trigger::<T>()); }
            let mut out: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut out);
            let ghost out_seq = out@;
            proof { assume(out@.len() < usize::MAX); }
            let seq = AVLTreeSeqStEphS::from_vec(out);
            proof {
                // from_vec: seq@ =~= out_seq.map_values(|t: T| t@), so seq@[i] == out_seq[i]@.
                // collect_in_order (empty start): out_seq[i]@ in self.tree@ for all i,
                // and every v in self.tree@ has a witness j with out_seq[j]@ == v.
                assert forall|i: int| 0 <= i < seq@.len() implies
                    #[trigger] self@.contains(seq@[i])
                by {
                    assert(seq@[i] == out_seq[i]@);
                };
                assert forall|v: T::V|
                    #[trigger] seq@.to_set().contains(v) <==> self@.contains(v)
                by {
                    if seq@.to_set().contains(v) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                        assert(seq@[j] == out_seq[j]@);
                    }
                    if self@.contains(v) {
                        let j = choose|j: int| 0 <= j < out_seq.len() && #[trigger] out_seq[j]@ == v;
                        assert(seq@[j] == out_seq[j]@);
                    }
                };
                assert(seq@.to_set() =~= self@);
                vstd::seq_lib::seq_to_set_is_finite(seq@);
            }
            seq
        }

        fn empty() -> (empty: Self)
        {
            AVLTreeSetMtEph { tree: ParamBST::new() }
        }

        fn singleton(x: T) -> (tree: Self)
        {
            AVLTreeSetMtEph { tree: ParamBST::singleton(x) }
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> (constructed: Self)
        {
            proof { assert(obeys_feq_full_trigger::<T>()); }
            let mut tree = ParamBST::new();
            let n = seq.length();
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == seq@.len(),
                    tree@.finite(),
                    tree@.len() <= i as nat,
                    vstd::laws_cmp::obeys_cmp_spec::<T>(),
                    view_ord_consistent::<T>(),
                    seq.spec_avltreeseqsteph_wf(),
                    forall|j: int| 0 <= j < i as int ==> #[trigger] tree@.contains(seq@[j]),
                    forall|v: T::V| tree@.contains(v) ==>
                        exists|j: int| 0 <= j < i as int && #[trigger] seq@[j] == v,
                decreases n - i,
            {
                let ghost old_tree = tree@;
                let elem_ref = seq.nth(i);
                let elem = elem_ref.clone_plus();
                proof {
                    lemma_cloned_view_eq::<T>(*elem_ref, elem);
                    // tree@.len() <= i < n <= usize::MAX, so tree@.len() < usize::MAX.
                    assert(tree@.len() <= i as nat);
                }
                tree.insert(elem);
                proof {
                    assert forall|j: int| 0 <= j < (i + 1) as int implies
                        #[trigger] tree@.contains(seq@[j]) by {
                        if j < i as int {
                            assert(old_tree.contains(seq@[j]));
                        } else {
                            assert(j == i as int);
                        }
                    };
                    assert forall|v: T::V| tree@.contains(v) implies
                        exists|j: int| 0 <= j < (i + 1) as int && #[trigger] seq@[j] == v by {
                        if v == elem@ {
                            assert(seq@[i as int] == v);
                        } else {
                            assert(old_tree.contains(v));
                        }
                    };
                }
                i = i + 1;
            }
            proof {
                assert forall|v: T::V| #[trigger] tree@.contains(v) <==> seq@.to_set().contains(v) by {
                    if tree@.contains(v) {
                        let j = choose|j: int| 0 <= j < n as int && seq@[j] == v;
                        assert(seq@.to_set().contains(v));
                    }
                    if seq@.to_set().contains(v) {
                        let j = choose|j: int| 0 <= j < seq@.len() && seq@[j] == v;
                        assert(tree@.contains(seq@[j]));
                    }
                };
                assert(tree@ =~= seq@.to_set());
            }
            AVLTreeSetMtEph { tree }
        }

        fn filter<F: Pred<T> + Clone>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(T::V) -> bool>,
        ) -> (filtered: Self)
        {
            let filtered_tree = self.tree.filter(f, Ghost(spec_pred));
            AVLTreeSetMtEph { tree: filtered_tree }
        }

        fn intersection(&self, other: &Self) -> (common: Self)
        {
            let common_tree = self.tree.intersect(&other.tree);
            AVLTreeSetMtEph { tree: common_tree }
        }

        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let remaining_tree = self.tree.difference(&other.tree);
            AVLTreeSetMtEph { tree: remaining_tree }
        }

        fn union(&self, other: &Self) -> (combined: Self)
        {
            let combined_tree = self.tree.union(&other.tree);
            AVLTreeSetMtEph { tree: combined_tree }
        }

        fn find(&self, x: &T) -> (found: bool)
        {
            let result = self.tree.find(x);
            result.is_some()
        }

        fn delete(&mut self, x: &T)
        {
            proof {
                assert(obeys_feq_full_trigger::<T>());
                assume(self.tree@.len() < usize::MAX as nat);
            }
            self.tree.delete(x);
        }

        fn insert(&mut self, x: T)
        {
            proof { assert(obeys_feq_full_trigger::<T>()); }
            self.tree.insert(x);
        }

        fn iter(&self) -> (it: AVLTreeSetMtEphIter<T>)
        {
            let mut vals: Vec<T> = Vec::new();
            self.tree.collect_in_order(&mut vals);
            AVLTreeSetMtEphIter { snapshot: vals, pos: 0 }
        }
    }

    impl<T: StTInMtT + Ord + 'static> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self { Self::empty() }
    }

    impl<T: StTInMtT + Ord + 'static> std::iter::Iterator for AVLTreeSetMtEphIter<T> {
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
                        &&& element@ == old_seq[old_index]
                    },
                }
            })
        {
            if self.pos >= self.snapshot.len() {
                None
            } else {
                let item = self.snapshot[self.pos].clone();
                self.pos = self.pos + 1;
                proof { assume(item@ == old(self)@.1[old(self)@.0]); }  // accept hole: Clone preserves value
                Some(item)
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIteratorNew for AVLTreeSetMtEphIter<T> {
        type GhostIter = AVLTreeSetMtEphGhostIter<T>;
        open spec fn ghost_iter(&self) -> AVLTreeSetMtEphGhostIter<T> {
            AVLTreeSetMtEphGhostIter { pos: self@.0, elements: self@.1 }
        }
    }

    impl<T: StTInMtT + Ord + 'static> vstd::pervasive::ForLoopGhostIterator for AVLTreeSetMtEphGhostIter<T> {
        type ExecIter = AVLTreeSetMtEphIter<T>;
        type Item = T::V;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &AVLTreeSetMtEphIter<T>) -> bool {
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

        open spec fn ghost_peek_next(&self) -> Option<T::V> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &AVLTreeSetMtEphIter<T>) -> AVLTreeSetMtEphGhostIter<T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: StTInMtT + Ord + 'static> std::iter::IntoIterator for &'a AVLTreeSetMtEph<T> {
        type Item = T;
        type IntoIter = AVLTreeSetMtEphIter<T>;
        fn into_iter(self) -> (it: AVLTreeSetMtEphIter<T>)
            requires self.spec_avltreesetmteph_wf(),
            ensures it@.0 == 0, avltreesetmteph_iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<T: StTInMtT + Ord + 'static> Clone for AVLTreeSetMtEph<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            AVLTreeSetMtEph { tree: self.tree.clone() }
        }
    }

    } // verus!

    // Ghost fields are zero-sized; ParamBST is Send/Sync via BSTParaMtEph.
    unsafe impl<T: StTInMtT + Ord + 'static> Send for AVLTreeSetMtEph<T> {}
    unsafe impl<T: StTInMtT + Ord + 'static> Sync for AVLTreeSetMtEph<T> {}

    // 12. macros

    #[macro_export]
    macro_rules! AVLTreeSetMtEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    // 13. derive impls outside verus!

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter(pos={})", self.pos)
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEphIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEphGhostIter<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "AVLTreeSetMtEphGhostIter")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Debug for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord + 'static> fmt::Display for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }
}
