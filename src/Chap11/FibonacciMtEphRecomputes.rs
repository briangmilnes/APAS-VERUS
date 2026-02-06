//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci with TSM at every recursive level.
//!
//! Each fib(n) spawns two threads and uses a TSM to track completion.
//! Exponential work, maximum parallelism, TSM per fork-join.

#![cfg(verus_keep_ghost)]

use vstd::prelude::*;
use vstd::thread::*;
use vstd::modes::*;
use verus_state_machines_macros::tokenized_state_machine;
use crate::Concurrency::*;

verus! {

use crate::Chap11::FibonacciStEph::FibonacciStEph::*;

// TSM for tracking one fork-join pair at each recursive level
tokenized_state_machine!{
    FibFork {
        fields {
            #[sharding(constant)]
            pub n: nat,
            
            #[sharding(constant)]
            pub expected_left: nat,
            
            #[sharding(constant)]
            pub expected_right: nat,
            
            #[sharding(variable)]
            pub left_done: bool,
            
            #[sharding(variable)]
            pub right_done: bool,
        }
        
        #[invariant]
        pub fn main_inv(&self) -> bool {
            self.expected_left == spec_fib((self.n - 1) as nat)
            && self.expected_right == spec_fib((self.n - 2) as nat)
            && self.n >= 2
        }
        
        init!{
            initialize(n: nat) {
                require(n >= 2);
                init n = n;
                init expected_left = spec_fib((n - 1) as nat);
                init expected_right = spec_fib((n - 2) as nat);
                init left_done = false;
                init right_done = false;
            }
        }
        
        transition!{
            complete_left() {
                require(!pre.left_done);
                update left_done = true;
            }
        }
        
        transition!{
            complete_right() {
                require(!pre.right_done);
                update right_done = true;
            }
        }
        
        property!{
            finalize() {
                require(pre.left_done);
                require(pre.right_done);
            }
        }
        
        #[inductive(initialize)]
        fn initialize_inductive(post: Self, n: nat) { }
        
        #[inductive(complete_left)]
        fn complete_left_inductive(pre: Self, post: Self) { }
        
        #[inductive(complete_right)]
        fn complete_right_inductive(pre: Self, post: Self) { }
    }
}

/// Fully parallel Fibonacci with TSM at each fork-join level.
pub fn fib_recomputes(n: u64) -> (result: u64)
    requires n <= 46
    ensures result == spec_fib(n as nat)
    decreases n
{
    if n <= 1 {
        n
    } else {
        // Create a TSM instance for THIS level's fork-join
        let tracked (
            Tracked(instance),
            Tracked(left_done_token),
            Tracked(right_done_token),
        ) = FibFork::Instance::initialize(n as nat);
        
        let tracked instance1 = instance.clone();
        let tracked instance2 = instance.clone();
        
        // Spawn left thread for fib(n-1)
        let left_handle = spawn(
            (move || -> (out: (u64, Tracked<FibFork::left_done>))
                requires 
                    n >= 2 && n <= 46,
                    left_done_token.instance_id() == instance1.id(),
                    left_done_token.value() == false,
                ensures 
                    out.0 == spec_fib((n - 1) as nat),
                    out.1@.instance_id() == instance1.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = left_done_token;
                let val = fib_recomputes(n - 1);  // Recursive!
                proof { instance1.complete_left(&mut token); }
                (val, Tracked(token))
            })
        );
        
        // Spawn right thread for fib(n-2)
        let right_handle = spawn(
            (move || -> (out: (u64, Tracked<FibFork::right_done>))
                requires 
                    n >= 2 && n <= 46,
                    right_done_token.instance_id() == instance2.id(),
                    right_done_token.value() == false,
                ensures 
                    out.0 == spec_fib((n - 2) as nat),
                    out.1@.instance_id() == instance2.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = right_done_token;
                let val = fib_recomputes(n - 2);  // Recursive!
                proof { instance2.complete_right(&mut token); }
                (val, Tracked(token))
            })
        );
        
        // Join left
        let left_out = match left_handle.join() {
            Result::Ok(out) => out,
            Result::Err(_) => { assume(false); diverge() }
        };
        let left_val = left_out.0;
        let tracked left_token = left_out.1.get();
        
        // Join right
        let right_out = match right_handle.join() {
            Result::Ok(out) => out,
            Result::Err(_) => { assume(false); diverge() }
        };
        let right_val = right_out.0;
        let tracked right_token = right_out.1.get();
        
        // Use finalize to prove left + right == fib(n)
        proof {
            instance.finalize(&left_token, &right_token);
            lemma_fib_sum_fits_u64(n as nat);
        }
        
        left_val + right_val
    }
}

} // verus!
