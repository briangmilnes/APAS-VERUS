//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Primitive tree sequence implementation for Chapter 23. Verusified.
//!
//! A single-threaded persistent tree sequence backed by Vec.
//! Sequences may be exposed as `Zero`, `One`, or `Two` parts, and the
//! corresponding `join` operation reassembles a sequence.
//! The balancing here chooses a midpoint split.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module




pub mod PrimTreeSeqStPer {

    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::vec::*,
        vstd::std_specs::cmp::PartialEqSpecImpl,
    };
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use vstd::multiset::Multiset;
    #[cfg(verus_keep_ghost)]
    use {
        crate::vstdplus::feq::feq::*,
        crate::vstdplus::multiset::multiset::spec_filter_len,
    };


    //		3. broadcast use

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    //		4. type definitions

    /// Primitive tree sequence stored as a persistent Vec-backed collection.
    #[verifier::reject_recursive_types(T)]
    pub struct PrimTreeSeqStS<T> {
        pub seq: Vec<T>,
    }

    /// Exposed tree structure: Zero (empty), One (single element), or Two (split halves).
    #[verifier::reject_recursive_types(T)]
    pub enum PrimTreeSeqStTree<T> {
        Zero,
        One(T),
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),
    }

    /// Ghost view of the exposed tree structure.
    #[verifier::reject_recursive_types(T)]
    pub ghost enum PrimTreeSeqStTreeView<T> {
        Zero,
        One(T),
        Two(Seq<T>, Seq<T>),
    }


    //		5. view impls

    //		5. view impls

    impl<T> View for PrimTreeSeqStS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    impl<T> View for PrimTreeSeqStTree<T> {
        type V = PrimTreeSeqStTreeView<T>;

        open spec fn view(&self) -> PrimTreeSeqStTreeView<T> {
            match self {
                PrimTreeSeqStTree::Zero => PrimTreeSeqStTreeView::Zero,
                PrimTreeSeqStTree::One(v) => PrimTreeSeqStTreeView::One(*v),
                PrimTreeSeqStTree::Two(l, r) => PrimTreeSeqStTreeView::Two(l@, r@),
            }
        }
    }


    //		6. spec fns

    pub open spec fn prim_tree_seq_iter_invariant<'a, T>(it: &PrimTreeSeqStIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    //		8. traits

    //		8. traits

    pub trait PrimTreeSeqStTrait<T>: Sized + View<V = Seq<T>> {
        spec fn spec_len(&self) -> nat;
        spec fn spec_index(&self, i: int) -> T;

        /// Creates an empty sequence.
        /// - APAS: Algorithm 23.3. Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// Builds a sequence containing a single element.
        /// - APAS: Algorithm 23.3. Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn singleton(value: T) -> (single: Self)
            ensures
                single.spec_len() == 1,
                single.spec_index(0) == value;

        /// Constructs a sequence from the provided vector.
        /// - APAS: N/A — not in prose, Vec-specific constructor.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing Vec.
        fn from_vec(vec: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == vec@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < vec@.len() ==> seq.spec_index(i) == vec@[i];

        /// Returns the number of elements in the sequence.
        /// - APAS: Cost Spec 23.2. Work Θ(1), Span Θ(1).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn length(&self) -> (len: usize)
            ensures len == self.spec_len();

        /// Algorithm 23.3 (nth). Return a reference to the element at `index`.
        /// - APAS: Algorithm 23.3. Work Θ(log n), Span Θ(log n) — tree-based recursive.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec-backed direct index.
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// Exposes the internal structure as Zero, One, or Two parts.
        /// - APAS: Cost Spec 23.2. Work Θ(1), Span Θ(1) — tree-based, just look at root.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — Vec-backed, clones elements into two halves.
        fn expose(&self) -> (tree: PrimTreeSeqStTree<T>)
            where T: Clone + Eq
            requires
                self.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                self.spec_len() == 0 <==> tree@ is Zero,
                self.spec_len() == 1 <==> {
                    &&& tree@ is One
                    &&& tree@->One_0 == self@[0]
                },
                self.spec_len() >= 2 <==> {
                    &&& tree@ is Two
                    &&& tree@->Two_0.len() >= 1
                    &&& tree@->Two_1.len() >= 1
                    &&& tree@->Two_0.len() + tree@->Two_1.len() == self.spec_len()
                    &&& tree@->Two_0 + tree@->Two_1 =~= self@
                };

        /// Reassembles a primitive tree sequence from an exposed tree.
        /// - APAS: Cost Spec 23.2. Work Θ(1 + |r(L) − r(R)|), Span Θ(1 + |r(L) − r(R)|) for Two; Θ(1) for Zero/One.
        /// - Claude-Opus-4.6: Work Θ(|L| + |R|), Span Θ(|L| + |R|) for Two; Θ(1) for Zero/One — Vec append.
        fn join(tree: PrimTreeSeqStTree<T>) -> (joined: Self)
            ensures
                tree@ is Zero ==> joined@ =~= Seq::<T>::empty(),
                tree@ is One ==> joined@ =~= seq![tree@->One_0],
                tree@ is Two ==> joined@ =~= tree@->Two_0 + tree@->Two_1;

        /// Definition 18.13 (append). Concatenate two sequences.
        /// - APAS: Algorithm 23.3. append = join(Two(a,b)). Work Θ(1 + |r(a) − r(b)|), Span same.
        /// - Claude-Opus-4.6: Work Θ(|a| + |b|), Span Θ(|a| + |b|) — sequential clone loops.
        fn append(a: &Self, b: &Self) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() + b.spec_len() <= usize::MAX as nat,
            ensures
                appended.spec_len() == a.spec_len() + b.spec_len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.spec_len() ==> appended.spec_index(i) == a.spec_index(i),
                forall|i: int| #![trigger b.spec_index(i)] 0 <= i < b.spec_len() ==> appended.spec_index(a.spec_len() as int + i) == b.spec_index(i);

        /// Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - APAS: Algorithm 23.3. subseq = take(drop(S,a), n). Tree cost depends on drop+take.
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(length) — sequential clone loop.
        fn subseq(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// Definition 18.16 (update). Return a copy with the element at `index` replaced.
        /// - APAS: Algorithm 23.3. Tree-based Work Θ(log² n), Span Θ(log² n).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(|a|) — copies entire Vec.
        fn update(a: &Self, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.spec_len(),
            ensures
                updated.spec_len() == a.spec_len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.spec_len() && i != index as int ==> updated.spec_index(i) == a.spec_index(i);

        /// Algorithm 23.3 (map). Transform each element via `f`.
        /// - APAS: Algorithm 23.3. Work Θ(n), Span Θ(log n) — parallel tree-based.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop.
        fn map<U: Clone, F: Fn(&T) -> U>(a: &PrimTreeSeqStS<T>, f: &F) -> (mapped: PrimTreeSeqStS<U>)
            requires
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.spec_index(i),)),
            ensures
                mapped.spec_len() == a.spec_len(),
                forall|i: int| #![trigger mapped.spec_index(i)] 0 <= i < a.spec_len() ==> f.ensures((&a.spec_index(i),), mapped.spec_index(i));

        /// Algorithm 23.3 (tabulate). Build a sequence by applying `f` to each index.
        /// - APAS: Algorithm 23.3. Work Θ(n), Span Θ(log n) — parallel tree-based.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop.
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: Self)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.spec_len() == length,
                forall|i: int| #![trigger tab_seq.spec_index(i)] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.spec_index(i));

        /// Algorithm 23.3 (filter). Keep elements satisfying the predicate.
        /// - APAS: Algorithm 23.3. Work Θ(n), Span Θ(log² n) — parallel tree-based with rebalancing.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop.
        fn filter<F: Fn(&T) -> bool>(a: &PrimTreeSeqStS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: PrimTreeSeqStS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(i),)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_len() <= a.spec_len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_pred),
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.spec_len(), |i: int| a.spec_index(i)).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// Algorithm 23.3 (drop). Drop the first `n` elements.
        /// - APAS: Algorithm 23.3. Tree-based Work Θ(log² n), Span Θ(log² n).
        /// - Claude-Opus-4.6: Work Θ(|a| − n), Span Θ(|a| − n) — delegates to subseq.
        fn drop(&self, n: usize) -> (dropped: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                n <= self.spec_len(),
                self.spec_len() <= usize::MAX,
            ensures
                dropped.spec_len() == self.spec_len() - n,
                forall|i: int| #![trigger dropped.spec_index(i)] 0 <= i < dropped.spec_len() ==> dropped.spec_index(i) == self.spec_index(n as int + i);

        /// Algorithm 23.3 (flatten). Concatenate a sequence of sequences.
        /// - APAS: Algorithm 23.3. flatten = reduce append empty. Tree-based cost depends on reduce+append.
        /// - Claude-Opus-4.6: Work Θ(Σ|a_i|), Span Θ(Σ|a_i|) — nested sequential loops.
        fn flatten(a: &PrimTreeSeqStS<PrimTreeSeqStS<T>>) -> (flattened: PrimTreeSeqStS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: PrimTreeSeqStS<T>| inner.seq@).flatten();

        /// Borrows the inner slice.
        /// - APAS: N/A — utility method, not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn as_slice(&self) -> (result: &[T])
            ensures result@ =~= self@;

        /// Unwraps into the inner Vec.
        /// - APAS: N/A — utility method, not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn into_vec(self) -> (result: Vec<T>)
            ensures result@ =~= self@;
    }


    //		9. impls

    //		6. spec fns

    impl<T> PrimTreeSeqStS<T> {
        pub open spec fn spec_len(&self) -> nat {
            self.seq@.len() as nat
        }

        pub open spec fn spec_index(&self, i: int) -> T
            recommends 0 <= i < self.spec_len(),
        {
            self.seq@[i]
        }

        /// Returns a borrow iterator over the sequence elements.
        /// - APAS: N/A — Verus-specific iterator scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps slice::Iter.
        pub fn iter(&self) -> (it: PrimTreeSeqStIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                prim_tree_seq_iter_invariant(&it),
        {
            PrimTreeSeqStIter { inner: self.seq.iter() }
        }
    }

    //		9. impls

    impl<T> PrimTreeSeqStTrait<T> for PrimTreeSeqStS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len() as nat
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq@[i]
        }

        fn empty() -> (empty_seq: Self)
        {
            PrimTreeSeqStS { seq: Vec::new() }
        }

        fn singleton(value: T) -> (single: Self)
        {
            let mut v = Vec::new();
            v.push(value);
            PrimTreeSeqStS { seq: v }
        }

        fn from_vec(vec: Vec<T>) -> (seq: Self)
        {
            PrimTreeSeqStS { seq: vec }
        }

        fn length(&self) -> (len: usize)
        {
            self.seq.len()
        }

        fn nth(&self, index: usize) -> (nth_elem: &T) {
            &self.seq[index]
        }

        fn expose(&self) -> (tree: PrimTreeSeqStTree<T>)
            where T: Clone + Eq
        {
            let len = self.seq.len();
            if len == 0 {
                PrimTreeSeqStTree::Zero
            } else if len == 1 {
                let elem = self.seq[0].clone();
                proof {
                    assert(cloned(self.seq@[0], elem));
                    axiom_cloned_implies_eq_owned(self.seq@[0], elem);
                }
                PrimTreeSeqStTree::One(elem)
            } else {
                let mid = len / 2;
                let mut left_vec: Vec<T> = Vec::new();
                let mut right_vec: Vec<T> = Vec::new();
                let mut i: usize = 0;
                while i < mid
                    invariant
                        len == self.seq@.len(),
                        0 < mid <= len,
                        mid == len / 2,
                        0 <= i <= mid,
                        left_vec@.len() == i,
                        obeys_feq_clone::<T>(),
                        forall|j: int| #![trigger left_vec@[j]] 0 <= j < left_vec@.len() ==> left_vec@[j] == self.seq@[j],
                    decreases mid - i,
                {
                    left_vec.push(self.seq[i].clone());
                    proof {
                        let ghost last = left_vec@[left_vec@.len() - 1 as int];
                        assert(cloned(self.seq[i as int], last));
                        axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                    }
                    i = i + 1;
                }
                while i < len
                    invariant
                        len == self.seq@.len(),
                        mid <= i <= len,
                        mid == len / 2,
                        left_vec@.len() == mid,
                        right_vec@.len() == i - mid,
                        obeys_feq_clone::<T>(),
                        forall|j: int| #![trigger left_vec@[j]] 0 <= j < left_vec@.len() ==> left_vec@[j] == self.seq@[j],
                        forall|j: int| #![trigger right_vec@[j]] 0 <= j < right_vec@.len() ==> right_vec@[j] == self.seq@[mid as int + j],
                    decreases len - i,
                {
                    right_vec.push(self.seq[i].clone());
                    proof {
                        let ghost last = right_vec@[right_vec@.len() - 1 as int];
                        assert(cloned(self.seq[i as int], last));
                        axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                    }
                    i = i + 1;
                }
                assert(left_vec@ + right_vec@ =~= self.seq@);
                PrimTreeSeqStTree::Two(
                    PrimTreeSeqStS { seq: left_vec },
                    PrimTreeSeqStS { seq: right_vec },
                )
            }
        }

        fn join(tree: PrimTreeSeqStTree<T>) -> (joined: PrimTreeSeqStS<T>)
            ensures
                tree@ is Zero ==> joined@ =~= Seq::<T>::empty(),
                tree@ is One ==> joined@ =~= seq![tree@->One_0],
                tree@ is Two ==> joined@ =~= tree@->Two_0 + tree@->Two_1,
        {
            match tree {
                PrimTreeSeqStTree::Zero => Self::empty(),
                PrimTreeSeqStTree::One(value) => Self::singleton(value),
                PrimTreeSeqStTree::Two(left, right) => {
                    let mut combined = left.seq;
                    let mut right_vec = right.seq;
                    combined.append(&mut right_vec);
                    PrimTreeSeqStS { seq: combined }
                }
            }
        }

        fn append(a: &Self, b: &Self) -> (appended: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let a_len = a.seq.len();
            let b_len = b.seq.len();
            let mut seq: Vec<T> = Vec::with_capacity(a_len + b_len);
            let mut i: usize = 0;
            while i < a_len
                invariant
                    i <= a_len,
                    a_len == a.seq@.len(),
                    seq@.len() == i as int,
                    obeys_feq_clone::<T>(),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < i ==> seq@[k] == a.seq@[k],
                decreases a_len - i,
            {
                seq.push(a.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(a.seq[i as int], last));
                    axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                }
                i += 1;
            }
            let mut j: usize = 0;
            while j < b_len
                invariant
                    j <= b_len,
                    b_len == b.seq@.len(),
                    a_len == a.seq@.len(),
                    seq@.len() == a_len + j,
                    obeys_feq_clone::<T>(),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < a_len ==> seq@[k] == a.seq@[k],
                    forall|k: int| #![trigger b.seq@[k]] 0 <= k < j ==> seq@[a_len as int + k] == b.seq@[k],
                decreases b_len - j,
            {
                seq.push(b.seq[j].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(b.seq[j as int], last));
                    axiom_cloned_implies_eq_owned(b.seq[j as int], last);
                }
                j += 1;
            }
            PrimTreeSeqStS { seq }
        }

        fn subseq(&self, start: usize, length: usize) -> (subseq: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let end = start + length;
            let mut seq: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = start;
            while i < end
                invariant
                    start <= i <= end,
                    end == start + length,
                    end <= self.seq@.len(),
                    seq@.len() == (i - start) as int,
                    obeys_feq_clone::<T>(),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> seq@[j] == self.seq@[(start + j) as int],
                decreases end - i,
            {
                seq.push(self.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(self.seq[i as int], last));
                    axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                }
                i += 1;
            }
            PrimTreeSeqStS { seq }
        }

        fn update(a: &Self, index: usize, item: T) -> (updated: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let len = a.seq.len();
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    obeys_feq_clone::<T>(),
                    index < len,
                    forall|k: int| #![trigger seq@[k]] 0 <= k < i && k != index as int ==> seq@[k] == a.seq@[k],
                    i > index ==> seq@[index as int] == item,
                decreases len - i,
            {
                if i == index {
                    seq.push(item.clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(item, last));
                        axiom_cloned_implies_eq_owned(item, last);
                    }
                } else {
                    seq.push(a.seq[i].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(a.seq[i as int], last));
                        axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                    }
                }
                i += 1;
            }
            PrimTreeSeqStS { seq }
        }

        fn map<U: Clone, F: Fn(&T) -> U>(a: &PrimTreeSeqStS<T>, f: &F) -> (mapped: PrimTreeSeqStS<U>)
        {
            let len = a.seq.len();
            let mut seq: Vec<U> = Vec::with_capacity(len);
            let mut i: usize = 0;
            proof {
                assert forall|j: int| 0 <= j < a.seq@.len() implies #[trigger] f.requires((&a.seq@[j],)) by {
                    assert(a.seq@[j] == a.spec_index(j));
                }
            }
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    forall|j: int| 0 <= j < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[j],)),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==> f.ensures((&a.seq@[j],), seq@[j]),
                decreases len - i,
            {
                seq.push(f(&a.seq[i]));
                i += 1;
            }
            PrimTreeSeqStS { seq }
        }

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: PrimTreeSeqStS<T>)
        {
            let mut seq = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    seq@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==> f.ensures((j as usize,), seq@[j]),
                decreases length - i,
            {
                seq.push(f(i));
                i += 1;
            }
            PrimTreeSeqStS { seq }
        }

        fn filter<F: Fn(&T) -> bool>(a: &PrimTreeSeqStS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            proof {
                assert forall|j: int| 0 <= j < a.seq@.len() implies #[trigger] pred.requires((&a.seq@[j],)) by {
                    assert(a.seq@[j] == a.spec_index(j));
                }
                broadcast use vstd::seq_lib::group_to_multiset_ensures;
                assert(a.seq@.subrange(0, 0int) =~= Seq::<T>::empty());
            }
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() <= i,
                    obeys_feq_clone::<T>(),
                    forall|j: int| 0 <= j < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[j],)),
                    forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> pred.ensures((&seq@[j],), true),
                    seq@.len() == spec_filter_len(a.seq@.subrange(0, i as int), spec_pred),
                    seq@.to_multiset() =~= a.seq@.subrange(0, i as int).to_multiset().filter(spec_pred),
                decreases len - i,
            {
                proof {
                    broadcast use vstd::seq_lib::group_to_multiset_ensures;
                }
                assert(a.seq@.subrange(0, i as int + 1) =~= a.seq@.subrange(0, i as int).push(a.seq@[i as int]));
                assert(a.seq@.subrange(0, i as int + 1).drop_last() =~= a.seq@.subrange(0, i as int));
                if pred(&a.seq[i]) {
                    let elem = a.seq[i].clone();
                    proof {
                        axiom_cloned_implies_eq_owned(a.seq[i as int], elem);
                    }
                    seq.push(elem);
                }
                i += 1;
            }
            let filtered = PrimTreeSeqStS { seq };
            proof {
                assert(a.seq@.subrange(0, a.seq@.len() as int) =~= a.seq@);
                assert(a.seq@ =~= Seq::new(a.spec_len(), |i: int| a.spec_index(i)));
                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
            }
            filtered
        }

        fn drop(&self, n: usize) -> (dropped: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let len = self.seq.len();
            let remaining = len - n;
            self.subseq(n, remaining)
        }

        fn flatten(a: &PrimTreeSeqStS<PrimTreeSeqStS<T>>) -> (flattened: PrimTreeSeqStS<T>)
            where T: Clone + Eq
        {
            let outer_len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < outer_len
                invariant
                    i <= outer_len,
                    outer_len == a.seq@.len(),
                    obeys_feq_clone::<T>(),
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: PrimTreeSeqStS<T>| inner.seq@).flatten(),
                decreases outer_len - i,
            {
                let inner = &a.seq[i];
                let inner_len = inner.seq.len();
                let mut j: usize = 0;
                while j < inner_len
                    invariant
                        j <= inner_len,
                        inner_len == inner.seq@.len(),
                        i < outer_len,
                        outer_len == a.seq@.len(),
                        obeys_feq_clone::<T>(),
                        seq@ =~= a.seq@.take(i as int).map_values(|inner: PrimTreeSeqStS<T>| inner.seq@).flatten()
                            + inner.seq@.take(j as int),
                    decreases inner_len - j,
                {
                    seq.push(inner.seq[j].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(inner.seq[j as int], last));
                        axiom_cloned_implies_eq_owned(inner.seq[j as int], last);
                        assert(inner.seq@.take(j as int + 1) =~= inner.seq@.take(j as int).push(inner.seq@[j as int]));
                    }
                    j += 1;
                }
                proof {
                    assert(inner.seq@.take(inner_len as int) =~= inner.seq@);
                    let ghost prefix = a.seq@.take(i as int).map_values(|inner: PrimTreeSeqStS<T>| inner.seq@);
                    assert(a.seq@.take(i as int + 1).map_values(|inner: PrimTreeSeqStS<T>| inner.seq@)
                        =~= prefix.push(a.seq@[i as int].seq@));
                    prefix.lemma_flatten_push(a.seq@[i as int].seq@);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(outer_len as int) =~= a.seq@);
            }
            PrimTreeSeqStS { seq }
        }

        fn as_slice(&self) -> (result: &[T]) { &self.seq }

        fn into_vec(self) -> (result: Vec<T>) { self.seq }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for PrimTreeSeqStS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq + View> PartialEqSpecImpl for PrimTreeSeqStTree<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    //		10. iterators

    /// Borrow iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct PrimTreeSeqStIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for PrimTreeSeqStIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct PrimTreeSeqStGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for PrimTreeSeqStGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<'a, T> std::iter::Iterator for PrimTreeSeqStIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
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

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for PrimTreeSeqStIter<'a, T> {
        type GhostIter = PrimTreeSeqStGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> PrimTreeSeqStGhostIterator<'a, T> {
            PrimTreeSeqStGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for PrimTreeSeqStGhostIterator<'a, T> {
        type ExecIter = PrimTreeSeqStIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &PrimTreeSeqStIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &PrimTreeSeqStIter<'a, T>) -> PrimTreeSeqStGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a PrimTreeSeqStS<T> {
        type Item = &'a T;
        type IntoIter = PrimTreeSeqStIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                prim_tree_seq_iter_invariant(&it),
        {
            PrimTreeSeqStIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for PrimTreeSeqStS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            self.seq.into_iter()
        }
    }


    //		11. derive impls in verus!

    //		11. derive impls in verus!

    impl<T: Clone> Clone for PrimTreeSeqStS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = PrimTreeSeqStS { seq: self.seq.clone() };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    impl<T: PartialEq + View> PartialEq for PrimTreeSeqStS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = self.seq == other.seq;
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    impl<T: Eq + View> Eq for PrimTreeSeqStS<T> {}

    impl<T: Clone> Clone for PrimTreeSeqStTree<T> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            let cloned = match self {
                PrimTreeSeqStTree::Zero => PrimTreeSeqStTree::Zero,
                PrimTreeSeqStTree::One(v) => PrimTreeSeqStTree::One(v.clone()),
                PrimTreeSeqStTree::Two(l, r) => PrimTreeSeqStTree::Two(l.clone(), r.clone()),
            };
            proof { assume(cloned@ == self@); }
            cloned
        }
    }

    impl<T: PartialEq + View> PartialEq for PrimTreeSeqStTree<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            match (self, other) {
                (PrimTreeSeqStTree::Zero, PrimTreeSeqStTree::Zero) => {
                    assert(self@ == other@);
                    true
                },
                (PrimTreeSeqStTree::One(a), PrimTreeSeqStTree::One(b)) => {
                    let r = *a == *b;
                    proof { assume(r == (self@ == other@)); }
                    r
                },
                (PrimTreeSeqStTree::Two(l1, r1), PrimTreeSeqStTree::Two(l2, r2)) => {
                    let r = *l1 == *l2 && *r1 == *r2;
                    proof { assume(r == (self@ == other@)); }
                    r
                },
                _ => {
                    false
                },
            }
        }
    }

    impl<T: Eq + View> Eq for PrimTreeSeqStTree<T> {}

    } // verus!


    //		13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for PrimTreeSeqStS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PrimTreeSeqStS {{ data: {:?} }}", self.seq)
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for PrimTreeSeqStTree<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                PrimTreeSeqStTree::Zero => write!(f, "Zero"),
                PrimTreeSeqStTree::One(v) => write!(f, "One({v:?})"),
                PrimTreeSeqStTree::Two(l, r) => write!(f, "Two({l:?}, {r:?})"),
            }
        }
    }

} // mod
