// Experiment: Does exec_spec_verified! work with generic types?
//
// CONTEXT: Natalie Neamtu's exec_spec_verified! macro auto-generates
// exec code from spec functions with verified equivalence proofs.
// It supports user-defined structs/enums but the docs say nothing
// about generics. Our codebase uses StT-bounded generics everywhere.
//
// TEST 1: Basic non-generic struct + forall. RESULT: PASSES.
// TEST 2: Generic struct with trait bound. RESULT: FAILS — "generics not supported".
// TEST 3: Forall over Seq<u64> (no overflow). RESULT: PASSES.
// TEST 4: Enum with match (no overflow). RESULT: PASSES.
//
// CONCLUSION: exec_spec_verified! works for concrete (non-generic) types.
// It cannot handle our StT-bounded generic types, which rules out direct
// use on APAS-VERUS data structure modules. Could be useful for concrete
// utility specs (graph algorithm helpers, cost specs with fixed types).
// Triggers are auto-inserted by the macro — do not add #[trigger] manually.
// Arithmetic overflow must be guarded with recommends.

pub mod exec_spec_verified_test {

    // Test 1: Non-generic struct + forall — the documented happy path.
    pub mod test1_basic {
        use vstd::contrib::exec_spec::*;
        use vstd::prelude::*;

        verus! {

        exec_spec_verified! {

        struct Point {
            x: i64,
            y: i64,
        }

        spec fn on_line(points: Seq<Point>) -> bool {
            forall |i: usize| 0 <= i < points.len()
                ==> points[i as int].y == points[i as int].x
        }

        } // exec_spec_verified!

        } // verus!
    }

    // Test 2: Generic struct — does the macro handle type parameters?
    // RESULT: FAILS — "generics not supported"
    // pub mod test2_generic_struct {
    //     use vstd::contrib::exec_spec::*;
    //     use vstd::prelude::*;
    //     verus! {
    //     exec_spec_verified! {
    //     struct Wrapper<T> { value: T, count: u64 }
    //     spec fn get_count<T>(w: Wrapper<T>) -> u64 { w.count }
    //     } // exec_spec_verified!
    //     } // verus!
    // }

    // Test 3: Forall over Seq<u64> — no arithmetic, clean verification.
    pub mod test3_seq_forall {
        use vstd::contrib::exec_spec::*;
        use vstd::prelude::*;

        verus! {

        exec_spec_verified! {

        spec fn all_positive(s: Seq<i64>) -> bool {
            forall |i: usize| 0 <= i < s.len() ==> s[i as int] > 0i64
        }

        spec fn all_below(s: Seq<u64>, bound: u64) -> bool {
            forall |i: usize| 0 <= i < s.len() ==> s[i as int] < bound
        }

        } // exec_spec_verified!

        } // verus!
    }

    // Test 4: Enum with match — no arithmetic overflow.
    pub mod test4_enum {
        use vstd::contrib::exec_spec::*;
        use vstd::prelude::*;

        verus! {

        exec_spec_verified! {

        enum Color {
            Red,
            Green,
            Blue,
            Custom { r: u8, g: u8, b: u8 },
        }

        spec fn is_primary(c: Color) -> bool {
            match c {
                Color::Red => true,
                Color::Green => true,
                Color::Blue => true,
                Color::Custom { r, g, b } => false,
            }
        }

        spec fn channel_sum(c: Color) -> u64 {
            match c {
                Color::Red => 255u64,
                Color::Green => 255u64,
                Color::Blue => 255u64,
                Color::Custom { r, g, b } => (r as u64 + g as u64 + b as u64) as u64,
            }
        }

        } // exec_spec_verified!

        } // verus!
    }
}
