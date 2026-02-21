//! Copyright (C) 2025 Acar, Blelloch and Milnes.
//! Hypothesis: Which Fn types can Verus verify when boxed or stored?
//!
//! Supported: impl Fn, impl FnOnce, impl FnMut, spec_fn (ghost lambda).
//! Not supported: fn(T) -> U (function pointer), Box<dyn Fn> (trait object).

pub mod boxing_fns {

    use vstd::prelude::*;

    verus! {

    // 1. impl Fn — pass-through (no boxing). Verus supports this.
    pub fn apply_impl_fn(f: impl Fn(u8) -> u8, x: u8) -> (res: u8)
        requires
            call_requires(f, (x,)),
        ensures
            call_ensures(f, (x,), res),
    {
        f(x)
    }

    pub fn double(x: u8) -> (res: u8)
        requires 0 <= x < 128,
        ensures res == 2 * x,
    {
        2 * x
    }

    pub fn test_impl_fn() {
        let r = apply_impl_fn(double, 10);
        assert(r == 20);
    }

    // 2. impl Fn with closure — inline closure, no storage.
    pub fn test_impl_fn_closure() {
        let f = |x: u8| -> (res: u8)
            requires 0 <= x < 128,
            ensures res == 2 * x,
        {
            2 * x
        };
        proof { assert(call_requires(f, (10,))); }
        let r = apply_impl_fn(f, 10);
        assert(r == 20);
    }

    // 3. First-class fn item — fn triple passed as impl Fn. (Verus rejects fn(T)->U pointers.)
    pub fn triple(x: u8) -> (res: u8)
        requires 0 <= x < 86,
        ensures res == 3 * x,
    {
        3 * x
    }

    pub fn test_fn_item_as_impl_fn() {
        let r = apply_impl_fn(triple, 5);
        assert(r == 15);
    }

    // 4. spec_fn (ghost) — no exec boxing. spec_fn lives in proof/spec mode only.
    pub open spec fn ghost_double(x: int) -> int {
        2 * x
    }

    pub proof fn test_spec_fn_direct() {
        assert(ghost_double(10) == 20);
        assert(ghost_double(0) == 0);
    }

    pub proof fn test_spec_fn_lambda() {
        let ghost f: spec_fn(int) -> int = |x: int| 2 * x;
        assert(f(10) == 20);
    }

    // 5. impl FnOnce — consumed on call. Single use.
    pub fn apply_fn_once(f: impl FnOnce(u8) -> u8, x: u8) -> (res: u8)
        requires
            call_requires(f, (x,)),
        ensures
            call_ensures(f, (x,), res),
    {
        f(x)
    }

    pub fn test_fn_once() {
        let f = |x: u8| -> (res: u8)
            requires 0 <= x < 128,
            ensures res == 2 * x,
        {
            2 * x
        };
        let r = apply_fn_once(f, 10);
        assert(r == 20);
    }

    // 6. impl FnMut — mutable closure. Verus supports FnMut with call_requires/call_ensures.
    pub fn apply_fn_mut(f: &mut impl FnMut(u8) -> u8, x: u8) -> (res: u8)
        requires
            call_requires(*old(f), (x,)),
        ensures
            call_ensures(*old(f), (x,), res),
    {
        f(x)
    }

    pub fn test_fn_mut() {
        let mut f = |x: u8| -> (res: u8)
            requires 0 <= x < 255,
            ensures res == x + 1,
        {
            x + 1
        };
        let r = apply_fn_mut(&mut f, 10);
        assert(r == 11);
    }

    // 7. Box<dyn Fn> — trait object. RESULT: FAILS — "The verifier does not yet support: dyn"
    // Uncomment to reproduce:
    // pub fn box_dyn_fn(f: Box<dyn Fn(u8) -> u8>) -> (res: u8) { f(10) }

    } // verus!
}
