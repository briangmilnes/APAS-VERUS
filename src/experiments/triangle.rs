// Copyright (c) 2025 Brian G. Milnes
use vstd::prelude::*;

verus! {

spec fn triangle(n: nat) -> nat
    decreases n,
{
    if n == 0 {
        0
    } else {
        n + triangle((n - 1) as nat)
    }
}

proof fn triangle_is_monotonic(i: nat, n: nat)
    requires
        i <= n,
    ensures
        triangle(i) <= triangle(n),
    decreases n - i,
{
    if i < n {
        triangle_is_monotonic(i, (n - 1) as nat);
    }
}

fn loop_triangle(n: u32) -> (sum: u32)
    requires
        triangle(n as nat) < 0x1_0000_0000,
    ensures
        sum == triangle(n as nat),
{
    let mut sum: u32 = 0;
    let mut idx: u32 = 0;
    while idx < n
        invariant
            idx <= n,
            sum == triangle(idx as nat),
            triangle(n as nat) < 0x1_0000_0000,
        decreases n - idx,
    {
        idx = idx + 1;
        assert(sum + idx < 0x1_0000_0000) by {
            triangle_is_monotonic(idx as nat, n as nat);
        }
        sum = sum + idx;
    }
    sum
}

fn loop_triangle_with_vec(nums: &Vec<u32>) -> (sum: u32)
    requires
        nums.len() > 0,
        triangle(nums[0] as nat) < 0x1_0000_0000,
    ensures
        sum == triangle(nums[0] as nat),
{
    let n = nums[0];
    let cloned_nums = nums.clone();
    
    let mut sum: u32 = 0;
    let mut idx: u32 = 0;
    while idx < n
        invariant
            idx <= n,
            sum == triangle(idx as nat),
            triangle(n as nat) < 0x1_0000_0000,
        decreases n - idx,
    {
        idx = idx + 1;
        assert(sum + idx < 0x1_0000_0000) by {
            triangle_is_monotonic(idx as nat, n as nat);
        }
        sum = sum + idx;
    }
    sum
}

}
