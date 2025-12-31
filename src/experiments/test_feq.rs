// Copyright (c) 2025 Brian G. Milnes
//! Tests for feq - Full Equality specification

pub mod test_feq {
    use vstd::prelude::*;
    use vstd::std_specs::cmp::PartialEqSpec;
    use crate::vstdplus::feq::feq::*;
    use crate::Types::Types::Pair;

    verus! {

    // Test struct with manual Clone, PartialEq, Eq
    #[derive(PartialEq, Eq)]
    pub struct Point {
        pub x: u64,
        pub y: u64,
    }

    impl Clone for Point {
        fn clone(&self) -> (result: Self)
            ensures result == *self
        {
            Point { x: self.x, y: self.y }
        }
    }

    impl View for Point {
        type V = (int, int);
        open spec fn view(&self) -> Self::V {
            (self.x as int, self.y as int)
        }
    }

    // Test enum with manual Clone, PartialEq, Eq
    #[derive(PartialEq, Eq)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    impl Clone for Color {
        fn clone(&self) -> (result: Self)
            ensures result == *self
        {
            match self {
                Color::Red => Color::Red,
                Color::Green => Color::Green,
                Color::Blue => Color::Blue,
            }
        }
    }

    impl View for Color {
        type V = int;
        open spec fn view(&self) -> Self::V {
            match self {
                Color::Red => 0,
                Color::Green => 1,
                Color::Blue => 2,
            }
        }
    }

    // More complex struct: holds a mutable Vec of ints
    #[derive(PartialEq, Eq)]
    pub struct IntTree {
        pub nodes: Vec<i64>,
    }

    impl Clone for IntTree {
        fn clone(&self) -> (result: Self)
            ensures result@ == self@
        {
            IntTree { nodes: self.nodes.clone() }
        }
    }

    impl View for IntTree {
        type V = Seq<int>;
        open spec fn view(&self) -> Self::V {
            self.nodes@.map(|i: int, v: i64| v as int)
        }
    }

    // Trigger specs for per-type broadcast axioms
    pub open spec fn point_feq_trigger() -> bool { true }
    pub open spec fn color_feq_trigger() -> bool { true }
    pub open spec fn inttree_feq_trigger() -> bool { true }
    // Per-type broadcast axioms for obeys_feq_full
    broadcast proof fn axiom_point_feq()
        requires #[trigger] point_feq_trigger()
        ensures obeys_feq_full::<Point>()
    { admit(); }

    broadcast proof fn axiom_color_feq()
        requires #[trigger] color_feq_trigger()
        ensures obeys_feq_full::<Color>()
    { admit(); }

    broadcast proof fn axiom_inttree_feq()
        requires #[trigger] inttree_feq_trigger()
        ensures obeys_feq_full::<IntTree>()
    { admit(); }

    broadcast group group_feq_axioms {
        axiom_point_feq,
        axiom_color_feq,
        axiom_inttree_feq,
    }

    // Tests for generic T - requires obeys_feq_full
    fn test_generic_reflexive<T: Eq + View + Clone + Sized>(x: T)
        requires obeys_feq_full::<T>()
    {
        proof {
            assert(feq_reflexive::<T>());
            assert(x.eq_spec(&x));
        }
    }

    fn test_generic_symmetric<T: Eq + View + Clone + Sized>(x: T, y: T)
        requires obeys_feq_full::<T>()
    {
        proof {
            assert(feq_symmetric::<T>());
            if x.eq_spec(&y) {
                assert(y.eq_spec(&x));
            }
        }
    }

    fn test_generic_transitive<T: Eq + View + Clone + Sized>(x: T, y: T, z: T)
        requires obeys_feq_full::<T>()
    {
        proof {
            assert(feq_transitive::<T>());
            if x.eq_spec(&y) && y.eq_spec(&z) {
                assert(x.eq_spec(&z));
            }
        }
    }

    fn test_generic_view<T: Eq + View + Clone + Sized>(x: T, y: T)
        requires obeys_feq_full::<T>()
    {
        proof {
            if x.eq_spec(&y) {
                assert(x@ == y@);
            }
        }
    }

    fn test_generic_view_injective<T: Eq + View + Clone + Sized>(x: T, y: T)
        requires obeys_feq_full::<T>()
    {
        proof {
            if x@ == y@ {
                assert(x == y);
            }
        }
    }

    // Test: exec == should give us spec == and view ==
    // but can't!. 
    #[verifier::external_body]
    fn test_exec_eq_implies_view_eq<T: Eq + View + Clone + Sized>(x: &T, y: &T)
        requires obeys_feq_full::<T>()
    {
        if *x == *y {
            proof {
                assert(*x == *y);
                assert(x@ == y@);
            }
        }
    }

    fn test_exec_feq_implies_eq_and_view<T: Eq + View + Clone + Sized>(x: &T, y: &T)
        requires obeys_feq_full::<T>()
    {
        if feq(x,y) {
            proof {
                assert(*x == *y);
                assert(x@ == y@);
            }
        }
    }

    fn test_generic_clone<T: Eq + View + Clone + Sized>(x: T)
        requires obeys_feq_full::<T>()
    {
        let x_clone = x.clone();
        proof {
            assert(cloned(x, x_clone));
            assert(x.eq_spec(&x_clone));
        }
    }

    // Test without requires - does not work for generic T
    fn test_generic_no_requires<T: Eq + Sized>(x: T) {
        proof {
            // These would fail without requires obeys_feq_full::<T>()
            // assert(feq_reflexive::<T>());
            // assert(x.eq_spec(&x));
        }
    }

    // Tests for u64 (concrete type) - no requires needed
    fn test_u64_reflexive(x: u64) {
        proof {
            assert(feq_reflexive::<u64>());
            assert(x.eq_spec(&x));
        }
    }

    fn test_u64_symmetric(x: u64, y: u64) {
        proof {
            assert(feq_symmetric::<u64>());
            if x.eq_spec(&y) {
                assert(y.eq_spec(&x));
            }
        }
    }

    fn test_u64_transitive(x: u64, y: u64, z: u64) {
        proof {
            assert(feq_transitive::<u64>());
            if x.eq_spec(&y) && y.eq_spec(&z) {
                assert(x.eq_spec(&z));
            }
        }
    }

    fn test_u64_view(x: u64, y: u64) {
        proof {
            assert(obeys_feq_view::<u64>());
            if x.eq_spec(&y) {
                assert(x@ == y@);
            }
        }
    }

    fn test_u64_view_injective(x: u64, y: u64) {
        proof {
            assert(obeys_feq_view_injective::<u64>());
            if x@ == y@ {
                assert(x == y);
            }
        }
    }

    fn test_u64_clone(x: u64) {
        let x_clone = x.clone();
        proof {
            assert(cloned(x, x_clone));
            assert(obeys_feq_clone::<u64>());
            assert(x.eq_spec(&x_clone));
        }
    }

    // Tests for Point (struct) - uses generic tests with obeys_feq_full requirement
    fn test_point_reflexive(p: Point)
        requires obeys_feq_full::<Point>()
    {
// Veracity: TESTING         test_generic_reflexive(p);
    }

    fn test_point_symmetric(p1: Point, p2: Point)
        requires obeys_feq_full::<Point>()
    {
        test_generic_symmetric(p1, p2);
    }

    fn test_point_clone(p: Point)
        requires obeys_feq_full::<Point>()
    {
        test_generic_clone(p);
    }

    // Tests for Color (enum) - uses generic tests with obeys_feq_full requirement
    fn test_color_reflexive(c: Color)
        requires obeys_feq_full::<Color>()
    {
// Veracity: TESTING         test_generic_reflexive(c);
    }

    fn test_color_symmetric(c1: Color, c2: Color)
        requires obeys_feq_full::<Color>()
    {
        test_generic_symmetric(c1, c2);
    }

    fn test_color_clone(c: Color)
        requires obeys_feq_full::<Color>()
    {
        test_generic_clone(c);
    }

    // Tests using broadcast axioms instead of requires or assume
    fn test_point_with_axiom(p: Point) {
        broadcast use group_feq_axioms;
        proof {
            assert(point_feq_trigger()); // Fire the axiom
            assert(obeys_feq_full::<Point>()); // Now this works
            assert(p.eq_spec(&p));
        }
    }

    fn test_color_with_axiom(c: Color) {
        broadcast use group_feq_axioms;
        proof {
            assert(color_feq_trigger()); // Fire the axiom
            assert(obeys_feq_full::<Color>()); // Now this works
            assert(c.eq_spec(&c));
        }
    }

    // Tests for IntTree (struct with Vec) - uses generic tests with obeys_feq_full requirement
    fn test_inttree_reflexive(t: IntTree)
        requires obeys_feq_full::<IntTree>()
    {
// Veracity: TESTING         test_generic_reflexive(t);
    }

    fn test_inttree_symmetric(t1: IntTree, t2: IntTree)
        requires obeys_feq_full::<IntTree>()
    {
        test_generic_symmetric(t1, t2);
    }

    fn test_inttree_clone(t: IntTree)
        requires obeys_feq_full::<IntTree>()
    {
        test_generic_clone(t);
    }

    fn test_inttree_with_axiom(t: IntTree) {
        broadcast use group_feq_axioms;
        proof {
            assert(inttree_feq_trigger()); // Fire the axiom
            assert(obeys_feq_full::<IntTree>()); // Now this works
            assert(t.eq_spec(&t));
        }
    }

    // Tests for Pair<u64, u64>
    fn test_pair_reflexive(p: Pair<u64, u64>)
        requires obeys_feq_full::<Pair<u64, u64>>()
    {
// Veracity: TESTING         test_generic_reflexive(p);
    }

    fn test_pair_symmetric(p1: Pair<u64, u64>, p2: Pair<u64, u64>)
        requires obeys_feq_full::<Pair<u64, u64>>()
    {
        test_generic_symmetric(p1, p2);
    }

    fn test_pair_view(p1: Pair<u64, u64>, p2: Pair<u64, u64>)
        requires obeys_feq_full::<Pair<u64, u64>>()
    {
        test_generic_view(p1, p2);
    }

    fn test_pair_view_injective(p1: Pair<u64, u64>, p2: Pair<u64, u64>)
        requires obeys_feq_full::<Pair<u64, u64>>()
    {
        test_generic_view_injective(p1, p2);
    }

    fn test_pair_with_axiom(p: Pair<u64, u64>) {
        broadcast use crate::Types::Types::group_Pair_axioms;
        proof {
            assert(crate::Types::Types::Pair_feq_trigger::<u64, u64>()); // Fire the axiom
            assert(obeys_feq_full::<Pair<u64, u64>>()); // Now this works
            assert(p.eq_spec(&p));
        }
    }

    } // verus!
}
