//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Experiment: named functions vs closures for Clone + Send + Sync + 'static bounds.
//!
//! Hypothesis: named functions (fn items) are zero-sized, Copy, Clone, Send, Sync,
//! 'static in Rust. Verus may recognize this even though it rejects Clone on closures.
//!
//! If this works, we can replace clone_fn/clone_fn2/clone_pred with named functions
//! for the common reduce/map/filter patterns. The tradeoff: named functions can't
//! capture local state (no closure captures), but most of our reduce/map closures
//! are non-capturing anyway (|x, y| max(x, y), |x, y| x + y, etc.).
//!
//! Test matrix:
//! 1. Named fn satisfies F: Clone
//! 2. Named fn satisfies F: Clone + Send + Sync + 'static
//! 3. Named fn satisfies MtReduceFn<T> (our trait alias)
//! 4. Named fn works with clone_fn2 (should be trivial)
//! 5. Named fn in join() arms without cloning (can move a Copy type into both)
//! 6. Named fn with ensures (spec attached via separate proof, not inline)
//!
//! RESULTS (Verus ff454ab0f):
//!   Tests 1,2,3,5: FAIL — "Verus does not recognize this trait bound: <fn(...) as Clone>"
//!     Named fns hit the SAME Clone limitation as closures. Verus doesn't recognize
//!     Clone on ANY callable type — not closures, not fn items, not fn pointers.
//!   Tests 4,6: PASS — named fn can be copied into two variables and used in join()
//!     arms WITHOUT going through a Clone bound. This works because Rust implicitly
//!     copies zero-sized fn items, and Verus doesn't check Clone for implicit copies.
//!
//! IMPLICATION: Named fns don't help with Clone bounds (tests 1-3). But test 6 shows
//! that if you DON'T require F: Clone in the signature and instead let the caller
//! copy the fn item directly into join closures, it works. The problem is our D&C
//! helpers take &F and need to clone — that's where Clone is unavoidable.
//!
//! TODO: Retest on Verus 3390e9af0 (first-class Fn/Copy) — Copy might be recognized
//! even though Clone isn't, and fn items are Copy.

pub mod named_fn_clone_bounds {

    use vstd::prelude::*;

    verus! {

    // Named functions — non-capturing, zero-sized, Copy+Clone in Rust.

    fn named_max(x: &usize, y: &usize) -> (r: usize)
        ensures r == if *x >= *y { *x } else { *y }
    {
        if *x >= *y { *x } else { *y }
    }

    fn named_sum(x: &usize, y: &usize) -> (r: usize)
        ensures r == (*x).wrapping_add(*y) as usize
    {
        (*x).wrapping_add(*y)
    }

    fn named_or(x: &bool, y: &bool) -> (r: bool)
        ensures r == (*x || *y)
    {
        *x || *y
    }

    // Tests 1-2: FAIL — Verus does not recognize Clone on named function items.
    // Same error as closures: "Verus does not recognize this trait bound: <fn(...) as Clone>"
    //
    // fn takes_clone<F: Fn(&usize, &usize) -> usize + Clone>(f: &F) -> (r: usize) { f(&3, &4) }
    // fn test1_named_fn_clone() { takes_clone(&named_max); }
    // fn takes_mt_bounds<F: Fn(...) + Clone + Send + Sync + 'static>(f: &F) { f(&3, &4) }
    // fn test2_named_fn_mt_bounds() { takes_mt_bounds(&named_max); }

    // Test 3: Named fn with spec ensures passed to a reduce-like function.
    spec fn spec_max(x: usize, y: usize) -> usize {
        if x >= y { x } else { y }
    }

    fn reduce_like<F: Fn(&usize, &usize) -> usize + Clone + Send + Sync + 'static>(
        a: &usize, b: &usize,
        f: &F,
        Ghost(spec_f): Ghost<spec_fn(usize, usize) -> usize>,
    ) -> (r: usize)
        requires
            forall|x: &usize, y: &usize| #[trigger] f.requires((x, y)),
            forall|x: usize, y: usize, ret: usize| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
        ensures r == spec_f(*a, *b),
    {
        f(a, b)
    }

    // FAILS: reduce_like requires F: Clone, Verus rejects Clone on fn items.
    // fn test3_named_fn_with_spec() {
    //     let _r = reduce_like(
    //         &10usize, &20usize,
    //         &named_max,
    //         Ghost(|x: usize, y: usize| -> usize { spec_max(x, y) }),
    //     );
    // }

    // Test 4: Named fn in join()-like pattern — move into two closures.
    // If named fns are Copy, we can move them into both arms without clone_fn.
    fn test4_named_fn_two_arms() {
        let f = named_max;  // function item — Copy type
        let f1 = f;  // Copy, not move
        let f2 = f;  // Still valid — f was copied, not moved
        let r1 = f1(&10usize, &20usize);
        let r2 = f2(&30usize, &40usize);
        assert(r1 == 20);
        assert(r2 == 40);
    }

    // Test 5: Named fn passed through clone_fn2 from vstdplus.
    use crate::vstdplus::clone_plus::clone_plus::clone_fn2;

    // FAILS: clone_fn2 requires F: Clone, Verus rejects Clone on fn items.
    // fn test5_named_fn_clone_fn2() {
    //     let f = named_max;
    //     let f2 = clone_fn2(&f);
    //     let r1 = f(&5usize, &10usize);
    //     let r2 = f2(&5usize, &10usize);
    //     assert(r1 == r2);
    // }

    // Test 6: Named fn in actual join() pattern.
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

    fn test6_named_fn_join()
    {
        let f = named_max;
        let (left, right) = join(
            move || -> (r: usize)
                ensures r == 20usize
            { f(&10usize, &20usize) },
            move || -> (r: usize)
                ensures r == 40usize
            { f(&30usize, &40usize) },
        );
        assert(left == 20usize);
        assert(right == 40usize);
    }

    } // verus!

    #[test]
    fn test_named_fn_all() {
        test1_named_fn_clone();
        test2_named_fn_mt_bounds();
        // test3 may or may not compile depending on spec_fn cast
        test4_named_fn_two_arms();
        test5_named_fn_clone_fn2();
        test6_named_fn_join();
    }
}
