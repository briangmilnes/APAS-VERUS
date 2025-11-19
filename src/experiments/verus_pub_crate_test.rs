//! Test if Verus type_invariant works with pub(crate) fields

use vstd::prelude::*;

verus! {

pub struct TestStruct {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

#[verifier::type_invariant]
spec fn test_invariant(s: TestStruct) -> bool {
    s.y == s.x + 1
}

impl TestStruct {
    pub fn new(x: u32) -> (result: Self)
        requires x < u32::MAX,
        ensures result.x == x && result.y == x + 1,
    {
        TestStruct { x, y: x + 1 }
    }
}

pub fn test() {
    let s = TestStruct::new(10);
    assert(s.x == 10);  // Can we access pub(crate) fields from spec?
}

} // verus!

