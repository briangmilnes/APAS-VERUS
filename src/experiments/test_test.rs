//! Experiment: Can we get a simple verified function to run with cargo test?
//! 
//! Goal: Figure out which Verus features need conditional compilation for runtime tests.

pub mod test_test {
    use vstd::prelude::*;

    verus! {

    // Simple verified function - no fancy features
    pub fn add_one(x: u32) -> (result: u32)
        requires x < u32::MAX
        ensures result == x + 1
    {
        x + 1
    }

    // Another simple verified function
    pub fn multiply_by_two(x: u32) -> (result: u32)
        requires x <= u32::MAX / 2
        ensures result == x * 2
    {
        x + x
    }

    // Function that returns a tuple
    pub fn swap(a: u32, b: u32) -> (result: (u32, u32))
        ensures result.0 == b && result.1 == a
    {
        (b, a)
    }

    } // verus!
}
