//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqMtEph. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive with parallel implementations.
//! Use the trait `ArraySeqMtEphTrait` to access these implementations.

pub mod ArraySeqMtEph {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;

    #[cfg(verus_keep_ghost)]
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::Pool;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use {vstd::std_specs::vec::group_vec_axioms, crate::vstdplus::feq::feq::axiom_cloned_implies_eq};
    use vstd::std_specs::clone::*;
    use crate::vstdplus::clone_plus::clone_plus::{ClonePlus, clone_fn, clone_fn2, clone_pred};
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    // Clone spec for ArraySeqMtEphS - defines what cloned() means for this type
    pub assume_specification<T: Clone>
        [ <ArraySeqMtEphS<T> as Clone>::clone ]
        (s: &ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>)
        ensures result.seq@ == s.seq@;

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
            a: &Self, f: F, pool: &Pool
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

        fn filter<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(a: &Self, pred: F, pool: &Pool) -> (result: Self)
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

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T, pool: &Pool) -> T
            where T: 'static
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T, pool: &Pool) -> (result: (Self, T))
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

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>, pool: &Pool) -> (result: Self)
            where T: 'static;
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
            a: &ArraySeqMtEphS<T>, f: F, pool: &Pool
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

                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || -> (r: ArraySeqMtEphS<U>)
                    ensures r.seq@.len() == left_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&left_seq, f1, &pool1)
                };
                let fb = move || -> (r: ArraySeqMtEphS<U>)
                    ensures r.seq@.len() == right_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&right_seq, f2, &pool2)
                };

                let (left, right) = pool.join(fa, fb);

                proof { assert(left.seq@.len() + right.seq@.len() == a.seq@.len() as int); }
                ArraySeqMtEphS::<U>::append(&left, &right)
            }
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>) {
            let pair = ArraySeqMtEphS::<ArraySeqMtEphS<T>>::tabulate(
                &(|i: usize| -> (r: ArraySeqMtEphS<T>)
                    requires i < 2
                    ensures
                        i == 0 ==> r.seq@ == a.seq@,
                        i == 1 ==> r.seq@ == b.seq@,
                {
                    if i == 0 { a.clone_plus() } else { b.clone_plus() }
                }),
                2,
            );
            proof {
                assert(pair.seq@.len() == 2);
                assume(pair.seq@[0].seq@.len() == a.seq@.len());
                assume(pair.seq@[1].seq@.len() == b.seq@.len());
            }
            flatten_seq(&pair)
        }

        fn filter<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: F, pool: &Pool) -> (result: ArraySeqMtEphS<T>)
            where T: 'static
            decreases a.seq@.len()
        {
            if a.seq.len() <= 1 {
                let deflated = ArraySeqMtEphS::<ArraySeqMtEphS<T>>::tabulate(
                    &(|i: usize| -> (r: ArraySeqMtEphS<T>)
                        requires
                            (i as int) < a.spec_len(),
                            pred.requires((&a.nth_spec(i as int),)),
                    {
                        let elem = &a.seq[i];
                        Self::deflate(&pred, elem)
                    }),
                    a.seq.len(),
                );
                proof {
                    assume(forall|i: int| #![auto] 0 <= i < deflated.seq@.len()
                        ==> deflated.seq@[i].seq@.len() <= 1);
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

                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.seq@.len() <= left_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::filter(&left_seq, p1, &pool1)
                };
                let fb = move || -> (r: ArraySeqMtEphS<T>)
                    ensures r.seq@.len() <= right_seq.seq@.len()
                {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::filter(&right_seq, p2, &pool2)
                };

                let (left, right) = pool.join(fa, fb);

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

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T, pool: &Pool) -> T
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

                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left_seq, f1, id1, &pool1);
                let fb = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right_seq, f2, id2, &pool2);

                let (left_result, right_result) = pool.join(fa, fb);

                f(&left_result, &right_result)
            }
        }

        // Algorithm 19.10: Scan using contraction (parallel)
        // Returns (prefix_sums, total) where prefix_sums[i] = f(...f(f(id, a[0]), a[1])..., a[i-1])
        // (exclusive prefix scan)
        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T, pool: &Pool) -> (ArraySeqMtEphS<T>, T)
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
            let (scanned, total) = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::scan(&contracted, f_recurse, id.clone_plus(), pool);
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

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>, pool: &Pool) -> (result: ArraySeqMtEphS<T>)
            where T: 'static
            decreases ss.seq@.len()
        {
            if ss.seq.len() == 0 {
                Self::empty()
            } else if ss.seq.len() == 1 {
                ss.seq[0].clone_plus()
            } else {
                let mid = ss.seq.len() / 2;
                let left_ss = subseq_copy_nested(ss, 0, mid);
                let right_ss = subseq_copy_nested(ss, mid, ss.seq.len() - mid);

                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&left_ss, &pool1);
                let fb = move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&right_ss, &pool2);

                let (left, right) = pool.join(fa, fb);

                assume(left.seq@.len() + right.seq@.len() <= usize::MAX as int);
                Self::append(&left, &right)
            }
        }
    }

    // Helper functions

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

    fn subseq_copy_nested<T: View + Clone + Send + Sync>(
        a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>, start: usize, len: usize
    ) -> (result: ArraySeqMtEphS<ArraySeqMtEphS<T>>)
        requires
            start as int + len as int <= a.seq@.len(),
        ensures
            result.seq@.len() == len,
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
            decreases len - i,
        {
            result.push(a.seq[start + i].clone_plus());
            i += 1;
        }
        ArraySeqMtEphS { seq: result }
    }

    #[verifier::external_body]
    fn flatten_seq<T: View + Clone + Send + Sync>(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (result: ArraySeqMtEphS<T>)
        ensures
            ss.seq@.len() == 2 ==> result.seq@.len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
            (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                ==> result.seq@.len() <= ss.seq@.len(),
    {
        let mut total_len: usize = 0;
        let ss_len = ss.seq.len();
        for i in 0..ss_len {
            total_len = total_len + ss.seq[i].seq.len();
        }
        let mut result: Vec<T> = Vec::with_capacity(total_len);
        for i in 0..ss_len {
            for j in 0..ss.seq[i].seq.len() {
                result.push(ss.seq[i].seq[j].clone_plus());
            }
        }
        ArraySeqMtEphS { seq: result }
    }

    } // verus!

    // Non-verus impl for cargo compatibility
    #[cfg(not(verus_keep_ghost))]
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
}

