// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: test Verus first-class Fn/Copy/Clone trait support.
//!
//! Verus commit 3390e9af0 (2026-03-28) treats Copy, FnOnce, FnMut, Fn, and Tuple
//! as first-class traits. This experiment tests whether named closures now satisfy
//! Clone + Send + Sync + 'static bounds without external_body bridges.
//!
//! RESULT: FAILS on Verus 0.2026.03.28.3390e9a. All 5 tests fail with:
//!   "Verus does not recognize this trait bound: <{closure} as std::clone::Clone>"
//! The first-class Fn change treats Fn/FnOnce/FnMut/Copy as first-class but NOT Clone.
//! Non-capturing closures are Copy (and thus Clone) in Rust, but Verus doesn't bridge
//! that inference. The external_body bridges or clone_fn workaround remain necessary.
//!
//! Tests:
//! 1. Named closure satisfies F: Clone
//! 2. Named closure satisfies F: Clone + Send + Sync + 'static
//! 3. Named closure with ensures passed to function requiring MtReduceFn
//! 4. clone_fn preserves ensures on named closure
//! 5. Non-capturing closure passed to reduce-like function with Ghost spec_fn

pub mod closure_clone_first_class {

    use vstd::prelude::*;

    verus! {

    // Test 1: basic Clone on a closure.
    fn takes_cloneable_fn<F: Fn(&usize, &usize) -> usize + Clone>(f: &F) -> (r: usize)
    {
        let _f2 = f.clone();
        f(&1usize, &2usize)
    }

    fn test1_basic_clone() {
        let f = |x: &usize, y: &usize| -> (r: usize) { if *x >= *y { *x } else { *y } };
        let _r = takes_cloneable_fn(&f);
    }

    // Test 2: Clone + Send + Sync + 'static (the full Mt bound).
    fn takes_mt_fn<F: Fn(&usize, &usize) -> usize + Clone + Send + Sync + 'static>(f: &F) -> (r: usize)
    {
        f(&3usize, &4usize)
    }

    fn test2_mt_bounds() {
        let f = |x: &usize, y: &usize| -> (r: usize) { if *x >= *y { *x } else { *y } };
        let _r = takes_mt_fn(&f);
    }

    // Test 3: Named closure with ensures + Ghost spec_fn.
    spec fn spec_max(x: usize, y: usize) -> usize {
        if x >= y { x } else { y }
    }

    fn takes_reduce_like<F: Fn(&usize, &usize) -> usize + Clone + Send + Sync + 'static>(
        f: &F,
        Ghost(spec_f): Ghost<spec_fn(usize, usize) -> usize>,
    ) -> (r: usize)
        requires
            forall|x: &usize, y: &usize| #[trigger] f.requires((x, y)),
            forall|x: usize, y: usize, ret: usize| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
        ensures r == spec_f(3, 4),
    {
        let r = f(&3usize, &4usize);
        r
    }

    fn test3_named_closure_with_ensures() {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_max(*x, *y)
        { if *x >= *y { *x } else { *y } };

        let _r = takes_reduce_like(&f, Ghost(|x: usize, y: usize| -> usize { spec_max(x, y) }));
    }

    // Test 4: clone_fn from vstdplus preserves ensures.
    use crate::vstdplus::clone_plus::clone_plus::clone_fn2;

    fn test4_clone_fn_preserves_ensures() {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_max(*x, *y)
        { if *x >= *y { *x } else { *y } };

        let f2 = clone_fn2(&f);
        let r1 = f(&10usize, &20usize);
        let r2 = f2(&10usize, &20usize);
        assert(r1 == r2);
    }

    // Test 5: the actual Chap26 pattern — non-capturing closure to reduce-like
    // with all the bounds, no external_body.
    fn reduce_like<T, F: Fn(&T, &T) -> T + Clone + Send + Sync + 'static>(
        vals: &Vec<T>,
        f: &F,
        Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
        id: T,
    ) -> (r: T)
        requires
            forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
    {
        if vals.len() == 0 {
            id
        } else {
            let mut acc = id;
            let mut i: usize = 0;
            while i < vals.len()
                invariant
                    i <= vals@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases vals@.len() - i,
            {
                acc = f(&acc, &vals[i]);
                i += 1;
            }
            acc
        }
    }

    fn test5_full_pattern() {
        let f = |x: &usize, y: &usize| -> (r: usize)
            ensures r == spec_max(*x, *y)
        { if *x >= *y { *x } else { *y } };

        let mut v = Vec::new();
        v.push(10usize);
        v.push(30usize);
        v.push(20usize);
        let _r = reduce_like(&v, &f, Ghost(|x: usize, y: usize| -> usize { spec_max(x, y) }), 0usize);
    }

    } // verus!

    #[test]
    fn test_all_closure_clone_patterns() {
        // Runtime sanity check — if it compiles and verifies, these just confirm runtime behavior.
        test1_basic_clone();
        test2_mt_bounds();
        test3_named_closure_with_ensures();
        test4_clone_fn_preserves_ensures();
        test5_full_pattern();
    }
}
