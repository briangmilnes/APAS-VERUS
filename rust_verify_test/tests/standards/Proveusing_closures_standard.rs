//! Proof tests for standards::using_closures_standard.
//!
//! Tests closure spec propagation patterns:
//!   - Pattern A: Let-bind closure with ensures, pass by reference.
//!   - Pattern B: Inline closure with requires+ensures in call.
//!   - Pattern C: Ghost spec_fn companion for filter.

#[macro_use]
#[path = "../common/mod.rs"]
mod common;
use common::*;

// Pattern A: Let-bind a closure with ensures, pass to tabulate.
test_verify_one_file! {
    #[test] closures_let_bind_ensures verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::using_closures_standard::using_closures_standard::*;

        fn test_let_bind() {
            let f = |i: usize| -> (v: u64)
                requires i < 10,
                ensures v == i as u64 * 2,
            { (i * 2) as u64 };

            let s: ExampleS<u64> = ExampleS::tabulate(&f, 5);
            assert(s.spec_len() == 5);
        }
    } => Ok(())
}

// Pattern B: Inline closure with requires+ensures.
test_verify_one_file! {
    #[test] closures_inline_requires_ensures verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::using_closures_standard::using_closures_standard::*;

        fn test_inline() {
            let s: ExampleS<u64> = ExampleS::tabulate(
                &(|i: usize| -> (v: u64)
                    requires i < 100,
                    ensures v == i as u64 + 1,
                { (i + 1) as u64 }),
                10,
            );
            assert(s.spec_len() == 10);
        }
    } => Ok(())
}

// Pattern C: Ghost spec_fn companion for filter.
test_verify_one_file! {
    #[test] closures_ghost_spec_fn verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::using_closures_standard::using_closures_standard::*;

        spec fn spec_is_even(x: u64) -> bool {
            x % 2 == 0
        }

        fn test_ghost_spec_fn() {
            let s: ExampleS<u64> = ExampleS::tabulate(
                &(|i: usize| -> (v: u64)
                    ensures v == i as u64,
                { i as u64 }),
                6,
            );

            let pred = |x: &u64| -> (keep: bool)
                ensures keep == spec_is_even(*x),
            { *x % 2 == 0 };
            let ghost spec_pred: spec_fn(u64) -> bool = |x: u64| spec_is_even(x);

            let evens: ExampleS<u64> = s.filter(&pred, Ghost(spec_pred));
            assert(evens.spec_len() <= 6);
        }
    } => Ok(())
}

// map_apply: closure ensures propagated through map.
test_verify_one_file! {
    #[test] closures_map_apply verus_code! {
        use vstd::prelude::*;
        use apas_verus::standards::using_closures_standard::using_closures_standard::*;

        fn test_map_apply() {
            let s: ExampleS<u64> = ExampleS::tabulate(
                &(|i: usize| -> (v: u64)
                    ensures v == i as u64,
                { i as u64 }),
                3,
            );

            let doubled = s.map_apply(
                &(|x: &u64| -> (r: u64)
                    requires *x < 1000,
                    ensures r == *x * 2,
                { *x * 2 }),
            );
            assert(doubled.spec_len() == 3);
        }
    } => Ok(())
}
