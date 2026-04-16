// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: Can type_invariant work with pub(crate) fields?
//! Hypothesis: Verus may now allow type_invariant on structs with pub(crate) fields,
//! using the impl-method form from the guide (bst_map_type_invariant.rs).
//!
//! RESULT: FAILS. pub(crate) fields make the struct opaque outside the crate.
//! Ensures on pub fns cannot reference fields directly:
//! "disallowed: field expression for an opaque datatype".

pub mod pub_crate_type_invariant {

    use vstd::prelude::*;

    verus! {

    pub struct Pair {
        pub(crate) x: u32,
        pub(crate) y: u32,
    }

    impl Pair {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.y == self.x + 1
        }

        pub fn new(x: u32) -> (s: Self)
            requires x < u32::MAX,
            ensures s.x == x,
        {
            Pair { x, y: x + 1 }
        }

        pub fn get_x(&self) -> (v: u32)
            ensures v == self.x,
        {
            self.x
        }

        pub fn get_y(&self) -> (v: u32)
            ensures v == self.y,
        {
            self.y
        }
    }

    fn test_pair() {
        let p = Pair::new(10);
        assert(p.x == 10);
        assert(p.y == 11);
        let x = p.get_x();
        let y = p.get_y();
        assert(y == x + 1);
    }

    fn test_use_type_invariant(p: &Pair) -> (v: u32)
        requires p.x < u32::MAX,
        ensures v == p.x + 1,
    {
        proof { use_type_invariant(p); }
        p.y
    }

    } // verus!
}
