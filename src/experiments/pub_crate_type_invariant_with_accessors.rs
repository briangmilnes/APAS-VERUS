// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! Experiment: type_invariant with pub(crate) fields using closed spec fn accessors.
//! Hypothesis: pub(crate) fields make the struct opaque, but closed spec fn accessors
//! can be used in ensures of pub fns, and use_type_invariant recovers the invariant.

pub mod pub_crate_type_invariant_with_accessors {

    use vstd::prelude::*;

    verus! {

    pub struct n_and_n_plus_1 {
        pub(crate) x: u32,
        pub(crate) y: u32,
    }

    impl n_and_n_plus_1 {
        #[verifier::type_invariant]
        spec fn wf(self) -> bool {
            self.x < u32::MAX && self.y == self.x + 1
        }

    }

    pub trait n_and_n_plus_1Trait: Sized {
        spec fn spec_nandnplus1_wf(&self) -> bool;
        spec fn spec_x(&self) -> u32;
        spec fn spec_y(&self) -> u32;

        fn new(x: u32) -> (s: Self)
            requires x < u32::MAX,
            ensures s.spec_nandnplus1_wf(),
                    s.spec_x() == x,
                    s.spec_y() == x + 1;

        fn get_x(&self) -> (v: u32)
            requires self.spec_nandnplus1_wf(),
            ensures v as int == self.spec_x() as int;

        fn get_y(&self) -> (v: u32)
            requires self.spec_nandnplus1_wf(),
            ensures v as int == self.spec_y() as int;

        fn set(&mut self, x: u32)
            requires x < u32::MAX,
            ensures self.spec_nandnplus1_wf(),
                    self.spec_x() == x,
                    self.spec_y() == x + 1;
    }

    impl n_and_n_plus_1Trait for n_and_n_plus_1 {
        open spec fn spec_nandnplus1_wf(&self) -> bool { true }
        closed spec fn spec_x(&self) -> u32 { self.x }
        closed spec fn spec_y(&self) -> u32 { self.y }

        fn new(x: u32) -> (s: Self) {
            n_and_n_plus_1 { x, y: x + 1 }
        }

        fn get_x(&self) -> (v: u32) { self.x }

        fn get_y(&self) -> (v: u32) { self.y }

        fn set(&mut self, x: u32) {
            *self = n_and_n_plus_1 { x, y: x + 1 };
        }
    }

    fn test() {
        let p = n_and_n_plus_1::new(10);
        let x = p.get_x();
        let y = p.get_y();
        assert(y == x + 1);
    }

    // use_type_invariant recovers wf: y == x + 1.
    fn test_use_type_invariant(p: &n_and_n_plus_1) -> (v: u32)
        requires p.spec_x() < u32::MAX,
        ensures v == p.spec_x() + 1,
    {
        proof { use_type_invariant(p); }
        p.y
    }

    } // verus!
}
