//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqMtEph. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive with parallel implementations.
//! Use the trait `ArraySeqMtEphTrait` to access these implementations.

pub mod ArraySeqMtEph {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;

    #[cfg(verus_keep_ghost)]
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::join;

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::{SetStEph, SetStEphTrait, valid_key_type};

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::StT;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {vstd::std_specs::vec::group_vec_axioms, crate::vstdplus::feq::feq::axiom_cloned_implies_eq};
    use crate::vstdplus::clone_plus::clone_plus::{ClonePlus, clone_fn, clone_fn2, clone_pred};
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    /// Sum of all inner array lengths (total flattened length)
    pub open spec fn total_len<T>(ss: Seq<ArraySeqMtEphS<T>>) -> int
        decreases ss.len()
    {
        if ss.len() == 0 { 0 }
        else { ss[0].seq@.len() + total_len(ss.skip(1)) }
    }

    /// Lemma: total_len splits at any index
    proof fn lemma_total_len_split<T>(ss: Seq<ArraySeqMtEphS<T>>, mid: int)
        requires 0 <= mid <= ss.len()
        ensures total_len(ss) == total_len(ss.take(mid)) + total_len(ss.skip(mid))
        decreases mid
    {
        if mid == 0 {
            assert(ss.take(0) =~= Seq::empty());
            assert(ss.skip(0) =~= ss);
        } else {
            lemma_total_len_split(ss.skip(1), mid - 1);
            assert(ss.take(mid) =~= Seq::empty().push(ss[0]).add(ss.skip(1).take(mid - 1)));
            assert(ss.skip(1).skip(mid - 1) =~= ss.skip(mid));
            lemma_total_len_take_push(ss.skip(1).take(mid - 1), ss[0]);
        }
    }

    /// Proves that pushing an element and adding a sequence preserves total length.
    proof fn lemma_total_len_take_push<T>(rest: Seq<ArraySeqMtEphS<T>>, first: ArraySeqMtEphS<T>)
        ensures total_len(Seq::empty().push(first).add(rest)) == first.seq@.len() + total_len(rest)
        decreases rest.len()
    {
        let combined = Seq::empty().push(first).add(rest);
        assert(combined[0] == first);
        assert(combined.skip(1) =~= rest);
    }

    /// Lemma: total_len is non-negative
    proof fn lemma_total_len_nonneg<T>(ss: Seq<ArraySeqMtEphS<T>>)
        ensures total_len(ss) >= 0
        decreases ss.len()
    {
        if ss.len() > 0 {
            lemma_total_len_nonneg(ss.skip(1));
        }
    }

    /// Lemma: If inner lengths match, total_len matches
    proof fn lemma_total_len_copy_eq<T>(copy: Seq<ArraySeqMtEphS<T>>, orig: Seq<ArraySeqMtEphS<T>>)
        requires
            copy.len() == orig.len(),
            forall|i: int| 0 <= i < copy.len() ==> #[trigger] copy[i].seq@.len() == orig[i].seq@.len(),
        ensures
            total_len(copy) == total_len(orig)
        decreases copy.len()
    {
        if copy.len() > 0 {
            assert(copy[0].seq@.len() == orig[0].seq@.len());
            assert(copy.skip(1).len() == orig.skip(1).len());
            assert forall|i: int| 0 <= i < copy.skip(1).len() implies #[trigger] copy.skip(1)[i].seq@.len() == orig.skip(1)[i].seq@.len() by {
                assert(copy.skip(1)[i] == copy[i + 1]);
                assert(orig.skip(1)[i] == orig[i + 1]);
            }
            lemma_total_len_copy_eq(copy.skip(1), orig.skip(1));
        }
    }

    /// Lemma: total_len of subrange
    proof fn lemma_total_len_subrange<T>(ss: Seq<ArraySeqMtEphS<T>>, start: int, end: int)
        requires
            0 <= start <= end <= ss.len(),
        ensures
            total_len(ss.subrange(start, end)) == total_len(ss.skip(start).take(end - start))
    {
        assert(ss.subrange(start, end) =~= ss.skip(start).take(end - start));
    }

    // Chapter 19 trait - provides parallel algorithmic implementations
    pub trait ArraySeqMtEphTrait<T: View + Clone + Send + Sync + Eq>: Sized {
        spec fn spec_len(&self) -> nat;

        fn empty() -> (result: Self)
            ensures result.spec_len() == 0;

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_len() == 1;

        fn tabulate<F: Fn(usize) -> T + Send + Sync>(f: &F, length: usize) -> (result: Self)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                result.spec_len() == length;

        fn map<U: View + Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &Self, f: F
        ) -> (result: ArraySeqMtEphS<U>)
            where Self: Sized, T: 'static + Eq
            requires
                a.spec_len() <= usize::MAX as int,
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.nth_spec(i),)),
            ensures
                result.seq@.len() == a.spec_len();

        spec fn nth_spec(&self, i: int) -> T;

        fn nth(&self, i: usize) -> (result: &T)
            requires (i as int) < self.spec_len()
            ensures *result == self.nth_spec(i as int);

        fn length(&self) -> (result: usize)
            ensures result == self.spec_len();

        fn append(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as nat
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn filter<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> (result: Self)
            where T: 'static + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.nth_spec(i),)),
            ensures
                result.spec_len() <= a.spec_len();

        fn update(a: &Self, index: usize, item: T) -> (result: Self)
            requires (index as int) < a.spec_len()
            ensures result.spec_len() == a.spec_len();

        fn is_empty(a: &Self) -> (result: bool)
            ensures result == (a.spec_len() == 0);

        fn is_singleton(a: &Self) -> (result: bool)
            ensures result == (a.spec_len() == 1);

        fn iterate<A: Clone, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, x: A) -> A
            requires
                forall|acc: &A, elem: &T| #[trigger] f.requires((acc, elem));

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> T
            where T: 'static
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> (result: (Self, T))
            where T: 'static
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            ensures
                result.0.spec_len() == a.spec_len();

        fn select(a: &Self, b: &Self, index: usize) -> (result: Option<T>)
            ensures (index as int) < a.spec_len() + b.spec_len() ==> result.is_some();

        fn append_select(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as nat
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn deflate<F: Fn(&T) -> bool + Send + Sync>(f: &F, x: &T) -> (result: Self)
            requires f.requires((x,))
            ensures result.spec_len() <= 1;

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (result: Self)
            where T: 'static
            requires
                total_len(ss.seq@) <= usize::MAX as int,
                obeys_feq_clone::<ArraySeqMtEphS<T>>(),
            ensures result.spec_len() == total_len(ss.seq@) as nat;

        fn from_set(set: &SetStEph<T>) -> (result: Self)
            where T: StT + Hash
            requires valid_key_type::<T>()
            ensures result.spec_len() == set@.len();
    }

    impl<T: View + Clone + Send + Sync + Eq> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len() as nat
        }

        fn empty() -> (result: ArraySeqMtEphS<T>) {
            ArraySeqMtEphS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (result: ArraySeqMtEphS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqMtEphS { seq }
        }

        fn tabulate<F: Fn(usize) -> T + Send + Sync>(f: &F, length: usize) -> (result: ArraySeqMtEphS<T>)
        {
            let mut seq = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    seq@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                decreases length - i,
            {
                seq.push(f(i));
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        open spec fn nth_spec(&self, i: int) -> T {
            self.seq@[i]
        }

        fn nth(&self, i: usize) -> (result: &T) {
            &self.seq[i]
        }

        fn length(&self) -> (result: usize) {
            self.seq.len()
        }

        fn map<U: View + Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>, f: F
        ) -> (result: ArraySeqMtEphS<U>)
            where T: 'static
            decreases a.seq@.len()
        {
            if a.seq.len() <= 1 {
                ArraySeqMtEphS::<U>::tabulate(
                    &(|i: usize| -> (r: U)
                        requires
                            (i as int) < a.spec_len(),
                            f.requires((&a.nth_spec(i as int),)),
                    {
                        let elem = &a.seq[i];
                        f(elem)
                    }),
                    a.seq.len(),
                )
            } else {
                let mid = a.seq.len() / 2;
                let left_seq = subseq_copy(a, 0, mid);
                let right_seq = subseq_copy(a, mid, a.seq.len() - mid);

                let f1 = clone_fn(&f);
                let f2 = f;
                proof {
                    // subseq_copy(a, 0, mid) ensures: forall|j| 0 <= j < mid ==> cloned(a.seq@[0 + j], left_seq.seq@[j])
                    // axiom_cloned_implies_eq: cloned(*x, y) && obeys_feq_clone::<T>() ==> *x == y
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        // Step 1: Get cloned from subseq_copy
                        assert(cloned(a.seq@[0 as int + i], left_seq.seq@[i]));  // from subseq_copy ensures
                        assert(cloned(a.seq@[i], left_seq.seq@[i]));
                        // Step 2: cloned + obeys_feq_clone ==> equality (via broadcast axiom)
                        // axiom_cloned_implies_eq should fire here
                        assert(a.seq@[i] == left_seq.seq@[i]);
                        // Step 3: Original requires
                        assert(i < a.seq@.len() as int);  // since i < mid < a.len()
                        assert(f.requires((&a.nth_spec(i),)));  // from map's requires
                        assert(a.nth_spec(i) == a.seq@[i]);
                        assert(f.requires((&a.seq@[i],)));
                        // Step 4: f1 == f from clone_fn
                        assert(f1.requires((&a.seq@[i],)));  // from clone_fn ensures
                        // Step 5: Substitute equal elements
                        assert(f1.requires((&left_seq.seq@[i],)));
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        let orig_i = mid as int + i;
                        assert(cloned(a.seq@[mid as int + i], right_seq.seq@[i]));  // from subseq_copy ensures
                        assert(a.seq@[orig_i] == right_seq.seq@[i]);  // from axiom_cloned_implies_eq
                        assert(orig_i < a.seq@.len() as int);
                        // map's requires uses nth_spec which equals seq@[i]
                        assert(a.nth_spec(orig_i) == a.seq@[orig_i]);
                        assert(f.requires((&a.nth_spec(orig_i),)));  // from map's requires
                        assert(f.requires((&a.seq@[orig_i],)));
                        // f2 == f (f was moved, not cloned)
                    }
                }

                let fa = move || -> (r: ArraySeqMtEphS<U>)
                    ensures r.seq@.len() == left_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&left_seq, f1)
                };
                let fb = move || -> (r: ArraySeqMtEphS<U>)
                    ensures r.seq@.len() == right_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&right_seq, f2)
                };

                let (left, right) = join(fa, fb);

                proof { assert(left.seq@.len() + right.seq@.len() == a.seq@.len() as int); }
                ArraySeqMtEphS::<U>::append(&left, &right)
            }
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>) {
            // Clone inner Vecs (vstd has clone spec for Vec)
            let a_clone = ArraySeqMtEphS { seq: a.seq.clone() };
            let b_clone = ArraySeqMtEphS { seq: b.seq.clone() };
            let mut pair_vec: Vec<ArraySeqMtEphS<T>> = Vec::with_capacity(2);
            pair_vec.push(a_clone);
            pair_vec.push(b_clone);
            let pair = ArraySeqMtEphS::<ArraySeqMtEphS<T>> { seq: pair_vec };
            proof {
                assert(pair.spec_len() == 2);
                assert(pair.seq@[0].seq@.len() == a.seq@.len());
                assert(pair.seq@[1].seq@.len() == b.seq@.len());
                // Prove flatten_seq precondition
                assert(sum_lens_seq(pair.seq@, 0) == 0);
                assert(sum_lens_seq(pair.seq@, 1) == a.seq@.len());
                assert(sum_lens_seq(pair.seq@, 2) == a.seq@.len() + b.seq@.len());
            }
            flatten_seq(&pair)
        }

        fn filter<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: F) -> (result: ArraySeqMtEphS<T>)
            where T: 'static
            decreases a.seq@.len()
        {
            if a.seq.len() <= 1 {
                // Build deflated array manually to avoid opaque f.ensures
                let n = a.seq.len();
                let mut deflated_vec: Vec<ArraySeqMtEphS<T>> = Vec::with_capacity(n);
                proof {
                    assert forall|j: usize| j < n implies #[trigger] pred.requires((&a.seq@[j as int],)) by {
                        assert(a.nth_spec(j as int) == a.seq@[j as int]);
                    }
                }
                let mut i: usize = 0;
                while i < n
                    invariant
                        i <= n,
                        n == a.seq@.len(),
                        deflated_vec@.len() == i as int,
                        forall|j: int| 0 <= j < i as int ==> #[trigger] deflated_vec@[j].seq@.len() <= 1,
                        forall|j: usize| j < n ==> #[trigger] pred.requires((&a.seq@[j as int],)),
                    decreases n - i,
                {
                    let elem = &a.seq[i];
                    let deflated_elem = Self::deflate(&pred, elem);
                    proof {
                        assert(deflated_elem.seq@.len() <= 1);
                    }
                    deflated_vec.push(deflated_elem);
                    i += 1;
                }
                let deflated = ArraySeqMtEphS::<ArraySeqMtEphS<T>> { seq: deflated_vec };
                proof {
                    // Prove flatten_seq precondition: sum_lens_seq <= n <= usize::MAX
                    lemma_sum_lens_seq_bounded(deflated.seq@, n as int);
                }
                flatten_seq(&deflated)
            } else {
                let mid = a.seq.len() / 2;
                let left_seq = subseq_copy(a, 0, mid);
                let right_seq = subseq_copy(a, mid, a.seq.len() - mid);

                let p1 = clone_pred(&pred);
                let p2 = pred;
                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] p1.requires((&left_seq.seq@[i],)) by {
                        assert(cloned(a.seq@[0 as int + i], left_seq.seq@[i]));
                        assert(cloned(a.seq@[i], left_seq.seq@[i]));
                        assert(a.seq@[i] == left_seq.seq@[i]);
                        assert(i < a.seq@.len() as int);
                        assert(a.nth_spec(i) == a.seq@[i]);
                        assert(pred.requires((&a.nth_spec(i),)));
                        assert(pred.requires((&a.seq@[i],)));
                        assert(p1.requires((&a.seq@[i],)));
                        assert(p1.requires((&left_seq.seq@[i],)));
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] p2.requires((&right_seq.seq@[i],)) by {
                        let orig_i = mid as int + i;
                        assert(cloned(a.seq@[mid as int + i], right_seq.seq@[i]));
                        assert(a.seq@[orig_i] == right_seq.seq@[i]);
                        assert(orig_i < a.seq@.len() as int);
                        assert(a.nth_spec(orig_i) == a.seq@[orig_i]);
                        assert(pred.requires((&a.nth_spec(orig_i),)));
                        assert(pred.requires((&a.seq@[orig_i],)));
                        // p2 is pred moved, not cloned
                    }
                }

                let fa = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.seq@.len() <= left_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::filter(&left_seq, p1)
                };
                let fb = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.seq@.len() <= right_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::filter(&right_seq, p2)
                };

                let (left, right) = join(fa, fb);

                proof { assert(left.seq@.len() + right.seq@.len() <= a.seq@.len() as int); }
                Self::append(&left, &right)
            }
        }

        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> (result: ArraySeqMtEphS<T>) {
            Self::tabulate(
                &(|j: usize| -> (r: T)
                    requires j < a.seq@.len()
                {
                    if j == index { item.clone_plus() } else { a.seq[j].clone_plus() }
                }),
                a.seq.len(),
            )
        }

        fn is_empty(a: &ArraySeqMtEphS<T>) -> (result: bool) {
            a.seq.len() == 0
        }

        fn is_singleton(a: &ArraySeqMtEphS<T>) -> (result: bool) {
            a.seq.len() == 1
        }

        fn iterate<A: Clone, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            let mut i: usize = 0;
            while i < a.seq.len()
                invariant
                    i <= a.seq@.len(),
                    forall|acc: &A, elem: &T| #[trigger] f.requires((acc, elem)),
                decreases a.seq@.len() - i,
            {
                acc = f(&acc, &a.seq[i]);
                i += 1;
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
            where T: 'static
            decreases a.seq@.len()
        {
            if a.seq.len() == 0 {
                id
            } else if a.seq.len() == 1 {
                a.seq[0].clone_plus()
            } else {
                let mid = a.seq.len() / 2;
                let left_seq = subseq_copy(a, 0, mid);
                let right_seq = subseq_copy(a, mid, a.seq.len() - mid);

                let f1 = clone_fn2(&f);
                let f2 = clone_fn2(&f);
                proof {
                    assert(forall|x: &T, y: &T| #[trigger] f1.requires((x, y)));
                    assert(forall|x: &T, y: &T| #[trigger] f2.requires((x, y)));
                }

                let id1 = id.clone_plus();
                let id2 = id;

                let fa = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left_seq, f1, id1);
                let fb = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right_seq, f2, id2);

                let (left_result, right_result) = join(fa, fb);

                f(&left_result, &right_result)
            }
        }

        // Algorithm 19.10: Scan using contraction (parallel)
        // Returns (prefix_sums, total) where prefix_sums[i] = f(...f(f(id, a[0]), a[1])..., a[i-1])
        // (exclusive prefix scan)
        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> (ArraySeqMtEphS<T>, T)
            where T: 'static
            decreases a.seq@.len()
        {
            let n = a.seq.len();
            if n == 0 {
                return (Self::empty(), id);
            }
            if n == 1 {
                return (Self::singleton(id), a.seq[0].clone_plus());
            }

            // Contract: combine pairs a' = [f(a[0],a[1]), f(a[2],a[3]), ...]
            let contracted_len = n / 2 + n % 2;  // ceil(n/2) without overflow
            proof {
                // Prove contracted_len bounds
                assert(contracted_len == (n + 1) / 2);
                assert(contracted_len >= 1);  // since n >= 2
                assert(contracted_len < n);   // since n >= 2
            }
            let f_contract = clone_fn2(&f);
            proof { assert(forall|x: &T, y: &T| #[trigger] f_contract.requires((x, y))); }
            let contracted = Self::tabulate(
                &(|i: usize| -> (r: T)
                    requires
                        i < contracted_len,
                        contracted_len == (n + 1) / 2,
                        n >= 2,
                {
                    proof {
                        // Prove 2*i < n from i < ceil(n/2) = (n+1)/2
                        // Case 1: n even => contracted_len = n/2, i < n/2 => 2i < n
                        // Case 2: n odd => contracted_len = (n+1)/2, i <= n/2 => 2i <= n
                        //         but max i = (n-1)/2 so 2i = n-1 < n
                        assert(i < (n + 1) / 2);
                        assert(2 * i + 2 <= n + 1);  // 2*(i+1) <= n+1
                        assert(2 * i <= n - 1);  // since 2*i + 2 <= n + 1 means 2*i <= n - 1
                        assert(2 * i < n);
                    }
                    if 2 * i + 1 < n {
                        f_contract(&a.seq[2 * i], &a.seq[2 * i + 1])
                    } else {
                        a.seq[2 * i].clone_plus()
                    }
                }),
                contracted_len,
            );

            // Recursive scan on contracted
            let f_recurse = clone_fn2(&f);
            proof { assert(forall|x: &T, y: &T| #[trigger] f_recurse.requires((x, y))); }
            let (scanned, total) = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::scan(&contracted, f_recurse, id.clone_plus());
            proof {
                // scanned length equals contracted length (from scan's ensures on tabulate result)
                assert(scanned.seq@.len() == contracted_len);
            }

            // Expand: even indices get scanned[i/2], odd indices get f(scanned[i/2], a[i-1])
            let f_expand = clone_fn2(&f);
            proof { assert(forall|x: &T, y: &T| #[trigger] f_expand.requires((x, y))); }
            let expanded = Self::tabulate(
                &(|i: usize| -> (r: T)
                    requires
                        i < n,
                        n >= 2,
                        scanned.seq@.len() == (n + 1) / 2,
                {
                    proof {
                        // Prove i/2 < scanned.seq@.len() = (n+1)/2
                        assert(i / 2 <= (n - 1) / 2);
                        assert((n - 1) / 2 < (n + 1) / 2);
                    }
                    if i % 2 == 0 {
                        scanned.seq[i / 2].clone_plus()
                    } else {
                        // i is odd, so i >= 1
                        proof { assert(i >= 1); }
                        f_expand(&scanned.seq[i / 2], &a.seq[i - 1])
                    }
                }),
                n,
            );

            (expanded, total)
        }

        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: usize) -> (result: Option<T>) {
            if index < a.seq.len() {
                Some(a.seq[index].clone_plus())
            } else {
                let offset = index - a.seq.len();
                if offset < b.seq.len() {
                    Some(b.seq[offset].clone_plus())
                } else {
                    None
                }
            }
        }

        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>) {
            Self::tabulate(
                &(|i: usize| -> (r: T)
                    requires i < a.seq@.len() + b.seq@.len()
                {
                    Self::select(a, b, i).unwrap()
                }),
                a.seq.len() + b.seq.len(),
            )
        }

        fn deflate<F: Fn(&T) -> bool + Send + Sync>(f: &F, x: &T) -> (result: ArraySeqMtEphS<T>) {
            if f(x) {
                Self::singleton(x.clone_plus())
            } else {
                Self::empty()
            }
        }

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (result: ArraySeqMtEphS<T>)
            where T: 'static
            decreases ss.seq@.len()
        {
            if ss.seq.len() == 0 {
                proof {
                    // total_len of empty seq is 0, empty() returns len 0
                    assert(total_len(ss.seq@) == 0);
                }
                Self::empty()
            } else if ss.seq.len() == 1 {
                proof {
                    // total_len([x]) == x.len() + total_len([]) == x.len() + 0 == x.len()
                    assert(ss.seq@.skip(1) =~= Seq::<ArraySeqMtEphS<T>>::empty());
                    assert(total_len(ss.seq@) == ss.seq@[0].seq@.len() + total_len(ss.seq@.skip(1)));
                    assert(total_len(ss.seq@) == ss.seq@[0].seq@.len());
                    // clone_plus preserves length (from VstdPlus)
                }
                let result = ss.seq[0].clone_plus();
                proof {
                    // clone_plus ensures cloned(ss.seq@[0], result)
                    // axiom_cloned_implies_eq: cloned + obeys_feq_clone ==> equality
                    assert(cloned(ss.seq@[0], result));
                    // obeys_feq_clone from precondition, axiom fires: ss.seq@[0] == result
                    assert(ss.seq@[0] == result);
                    assert(ss.seq@[0].seq@ =~= result.seq@);
                }
                result
            } else {
                let mid = ss.seq.len() / 2;
                let ss_len = ss.seq.len();
                let left_ss = subseq_copy_nested(ss, 0, mid);
                let right_ss = subseq_copy_nested(ss, mid, ss_len - mid);

                proof {
                    // Show total_len splits properly
                    lemma_total_len_split(ss.seq@, mid as int);
                    // ss.take(mid) corresponds to left_ss, ss.skip(mid) corresponds to right_ss

                    // Prove left_ss has same total_len as ss.take(mid)
                    assert forall|i: int| 0 <= i < mid implies #[trigger] left_ss.seq@[i].seq@.len() == ss.seq@.take(mid as int)[i].seq@.len() by {
                        assert(ss.seq@.take(mid as int)[i] == ss.seq@[i]);
                    }
                    lemma_total_len_copy_eq(left_ss.seq@, ss.seq@.take(mid as int));

                    // Prove right_ss has same total_len as ss.skip(mid)
                    assert forall|i: int| 0 <= i < (ss_len - mid) implies #[trigger] right_ss.seq@[i].seq@.len() == ss.seq@.skip(mid as int)[i].seq@.len() by {
                        assert(ss.seq@.skip(mid as int)[i] == ss.seq@[mid as int + i]);
                    }
                    lemma_total_len_copy_eq(right_ss.seq@, ss.seq@.skip(mid as int));

                    // Now: total_len(left_ss) + total_len(right_ss) == total_len(ss) <= usize::MAX
                    // So both halves satisfy the precondition
                    lemma_total_len_nonneg(left_ss.seq@);
                    lemma_total_len_nonneg(right_ss.seq@);
                }

                // Capture total_lens before closures move the values
                let ghost left_total = total_len(left_ss.seq@);
                let ghost right_total = total_len(right_ss.seq@);

                let fa = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.spec_len() == left_total as nat
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&left_ss)
                };
                let fb = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.spec_len() == right_total as nat
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&right_ss)
                };

                let (left, right) = join(fa, fb);

                proof {
                    // join ensures: fa.ensures((), left) && fb.ensures((), right)
                    // So: left.spec_len() == left_total@ and right.spec_len() == right_total@
                    lemma_total_len_split(ss.seq@, mid as int);
                    
                    // Reconnect the proof for left_ss
                    assert forall|i: int| 0 <= i < mid implies #[trigger] left_ss.seq@[i].seq@.len() == ss.seq@.take(mid as int)[i].seq@.len() by {
                        assert(ss.seq@.take(mid as int)[i] == ss.seq@[i]);
                    }
                    lemma_total_len_copy_eq(left_ss.seq@, ss.seq@.take(mid as int));
                    
                    // Reconnect the proof for right_ss
                    assert forall|i: int| 0 <= i < (ss_len - mid) implies #[trigger] right_ss.seq@[i].seq@.len() == ss.seq@.skip(mid as int)[i].seq@.len() by {
                        assert(ss.seq@.skip(mid as int)[i] == ss.seq@[mid as int + i]);
                    }
                    lemma_total_len_copy_eq(right_ss.seq@, ss.seq@.skip(mid as int));

                    // Chain: left.len() + right.len() == total_len(ss) <= usize::MAX
                    assert(left_total == total_len(ss.seq@.take(mid as int)));
                    assert(right_total == total_len(ss.seq@.skip(mid as int)));
                    // So: left.spec_len() + right.spec_len() == total_len(ss.seq@)
                    assert(left.spec_len() as int + right.spec_len() as int == total_len(ss.seq@));
                }
                let result = Self::append(&left, &right);
                proof {
                    // append ensures: result.spec_len() == left.spec_len() + right.spec_len()
                    // We proved: left.spec_len() + right.spec_len() == total_len(ss.seq@)
                    // Therefore: result.spec_len() == total_len(ss.seq@)
                    assert(result.spec_len() == total_len(ss.seq@) as nat);
                }
                result
            }
        }

        fn from_set(set: &SetStEph<T>) -> (result: ArraySeqMtEphS<T>)
            where T: StT + Hash
        {
            let seq = set.to_seq();
            proof {
                // to_seq ensures: seq@.no_duplicates() && bijection with set@
                // Use unique_seq_to_set: no_duplicates ==> seq.len() == seq.to_set().len()
                let mapped = seq@.map(|_i: int, t: T| t@);
                mapped.unique_seq_to_set();
                // mapped.to_set() == set@ from the bijection ensures
                assert(mapped.to_set() =~= set@);
                assert(seq@.len() == set@.len());
            }
            ArraySeqMtEphS { seq }
        }
    }

    // Sequence operations used by parallel algorithms.

    fn subseq_copy<T: View + Clone>(a: &ArraySeqMtEphS<T>, start: usize, len: usize) -> (result: ArraySeqMtEphS<T>)
        requires
            start as int + len as int <= a.seq@.len(),
        ensures
            result.seq@.len() == len,
            forall|j: int| 0 <= j < len ==> cloned(#[trigger] a.seq@[start as int + j], result.seq@[j]),
    {
        let a_len = a.seq.len();
        let mut result: Vec<T> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                result@.len() == i as int,
                start as int + len as int <= a_len as int,
                a_len as int == a.seq@.len(),
                forall|j: int| 0 <= j < i ==> cloned(#[trigger] a.seq@[start as int + j], result@[j]),
            decreases len - i,
        {
            let elem = a.seq[start + i].clone_plus();
            result.push(elem);
            i += 1;
        }
        ArraySeqMtEphS { seq: result }
    }

    fn subseq_copy_nested<T: View + Clone + Send + Sync + Eq>(
        a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>, start: usize, len: usize
    ) -> (result: ArraySeqMtEphS<ArraySeqMtEphS<T>>)
        requires
            start as int + len as int <= a.seq@.len(),
            obeys_feq_clone::<ArraySeqMtEphS<T>>(),
        ensures
            result.seq@.len() == len,
            // Each cloned element has same inner length as original
            forall|i: int| 0 <= i < len ==> #[trigger] result.seq@[i].seq@.len() == a.seq@[start as int + i].seq@.len(),
    {
        let a_len = a.seq.len();
        let mut result: Vec<ArraySeqMtEphS<T>> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                result@.len() == i as int,
                start as int + len as int <= a_len as int,
                a_len as int == a.seq@.len(),
                obeys_feq_clone::<ArraySeqMtEphS<T>>(),
                forall|j: int| 0 <= j < i ==> #[trigger] result@[j].seq@.len() == a.seq@[start as int + j].seq@.len(),
            decreases len - i,
        {
            let elem_clone = a.seq[start + i].clone_plus();
            // clone_plus ensures cloned(a.seq[start+i], elem_clone)
            // axiom_cloned_implies_eq + obeys_feq_clone ==> equality
            // Therefore: elem_clone.seq@.len() == a.seq@[start+i].seq@.len()
            result.push(elem_clone);
            i += 1;
        }
        ArraySeqMtEphS { seq: result }
    }

    // Spec function for flatten_seq bounds (sum of inner lengths)
    spec fn sum_lens_seq<T>(ss: Seq<ArraySeqMtEphS<T>>, n: int) -> int
        decreases n
    {
        if n <= 0 { 0 }
        else { sum_lens_seq(ss, n - 1) + ss[n - 1].seq@.len() as int }
    }

    // Lemma: if all inner lengths <= 1, then sum_lens_seq(n) <= n
    proof fn lemma_sum_lens_seq_bounded<T>(ss: Seq<ArraySeqMtEphS<T>>, n: int)
        requires
            0 <= n <= ss.len(),
            forall|i: int| #![auto] 0 <= i < ss.len() ==> ss[i].seq@.len() <= 1,
        ensures
            sum_lens_seq(ss, n) <= n,
        decreases n,
    {
        if n > 0 {
            lemma_sum_lens_seq_bounded(ss, n - 1);
        }
    }

    // Lemma: sum_lens_seq is monotonically increasing
    proof fn lemma_sum_lens_seq_monotonic<T>(ss: Seq<ArraySeqMtEphS<T>>, a: int, b: int)
        requires
            0 <= a <= b <= ss.len(),
        ensures
            sum_lens_seq(ss, a) <= sum_lens_seq(ss, b),
        decreases b - a,
    {
        if a < b {
            lemma_sum_lens_seq_monotonic(ss, a, b - 1);
        }
    }

    fn flatten_seq<T: View + Clone + Send + Sync>(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (result: ArraySeqMtEphS<T>)
        requires
            sum_lens_seq(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
        ensures
            ss.seq@.len() == 2 ==> result.seq@.len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
            (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                ==> result.seq@.len() <= ss.seq@.len(),
    {
        // First pass: compute total length
        let ss_len = ss.seq.len();
        let mut total_len: usize = 0;
        let mut i: usize = 0;
        proof {
            lemma_sum_lens_seq_monotonic(ss.seq@, 0, ss.seq@.len() as int);
        }
        while i < ss_len
            invariant
                i <= ss_len,
                ss_len == ss.seq@.len(),
                total_len as int == sum_lens_seq(ss.seq@, i as int),
                sum_lens_seq(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
                sum_lens_seq(ss.seq@, i as int) <= sum_lens_seq(ss.seq@, ss.seq@.len() as int),
            decreases ss_len - i,
        {
            proof {
                lemma_sum_lens_seq_monotonic(ss.seq@, (i + 1) as int, ss.seq@.len() as int);
            }
            total_len = total_len + ss.seq[i].seq.len();
            i = i + 1;
        }

        // Second pass: copy all elements
        let mut result: Vec<T> = Vec::with_capacity(total_len);
        let mut j: usize = 0;
        while j < ss_len
            invariant
                j <= ss_len,
                ss_len == ss.seq@.len(),
                result@.len() == sum_lens_seq(ss.seq@, j as int),
            decreases ss_len - j,
        {
            let inner = &ss.seq[j];
            let inner_len = inner.seq.len();
            let mut k: usize = 0;
            while k < inner_len
                invariant
                    k <= inner_len,
                    inner_len == inner.seq@.len(),
                    j < ss_len,
                    ss_len == ss.seq@.len(),
                    result@.len() == sum_lens_seq(ss.seq@, j as int) + k as int,
                decreases inner_len - k,
            {
                result.push(inner.seq[k].clone_plus());
                k = k + 1;
            }
            j = j + 1;
        }

        proof {
            if ss.seq@.len() == 2 {
                assert(sum_lens_seq(ss.seq@, 2) == sum_lens_seq(ss.seq@, 1) + ss.seq@[1].seq@.len() as int);
                assert(sum_lens_seq(ss.seq@, 1) == sum_lens_seq(ss.seq@, 0) + ss.seq@[0].seq@.len() as int);
                assert(sum_lens_seq(ss.seq@, 0) == 0int);
            }
            if forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1 {
                lemma_sum_lens_seq_bounded(ss.seq@, ss.seq@.len() as int);
            }
        }

        ArraySeqMtEphS { seq: result }
    }

    } // verus!

    // Non-verus impl for cargo compatibility
    #[cfg(not(verus_keep_ghost))]
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
}
