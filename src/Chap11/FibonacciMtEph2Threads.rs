//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci using Tokenized State Machine.
//!
//! Two threads compute fib(n-1) and fib(n-2), TSM proves correctness.
//! Pattern from: https://verus-lang.github.io/verus/state_machines/examples/src-counting-to-2.html
//!
//! NOTE: This module is Verus-only due to TSM machinery.

#![cfg(verus_keep_ghost)]

use vstd::prelude::*;
use vstd::thread::*;
use std::sync::Arc;
use vstd::atomic_ghost::*;
use vstd::modes::*;
use verus_state_machines_macros::tokenized_state_machine;
use crate::Concurrency::diverge;

verus! {

use crate::Chap11::FibonacciStEph::FibonacciStEph::{spec_fib, fib as seq_fib, lemma_fib_sum_fits_u64};

// TSM for tracking two parallel Fibonacci subtasks
tokenized_state_machine!{
    FibPair {
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
// Veracity: UNNEEDED assert                 assert(pre.expected_left + pre.expected_right == spec_fib(pre.n));
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

/// Two-threaded Fibonacci using TSM tokens to prove correctness.
pub fn fib_2threads(n: u64) -> (result: u64)
    requires n >= 2 && n <= 46
    ensures result == spec_fib(n as nat)
{
    // Initialize the TSM protocol - get instance and tokens
    let tracked (
        Tracked(instance),
        Tracked(left_done_token),
        Tracked(right_done_token),
    ) = FibPair::Instance::initialize(n as nat);
    
    let tracked instance1 = instance.clone();
    let tracked instance2 = instance.clone();
    
    // Thread 1: compute fib(n-1)
    let join_handle1 = spawn(
        (move || -> (out: (u64, Tracked<FibPair::left_done>))
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
            let val = seq_fib(n - 1);
            proof { instance1.complete_left(&mut token); }
            (val, Tracked(token))
        })
    );
    
    // Thread 2: compute fib(n-2)
    let join_handle2 = spawn(
        (move || -> (out: (u64, Tracked<FibPair::right_done>))
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
            let val = seq_fib(n - 2);
            proof { instance2.complete_right(&mut token); }
            (val, Tracked(token))
        })
    );
    
    // Join thread 1
    let left_out = match join_handle1.join() {
        Result::Ok(out) => out,
        Result::Err(_) => { assume(false); diverge() }
    };
    let left_val = left_out.0;
    let tracked left_token = left_out.1.get();
    
    // Join thread 2
    let right_out = match join_handle2.join() {
        Result::Ok(out) => out,
        Result::Err(_) => { assume(false); diverge() }
    };
    let right_val = right_out.0;
    let tracked right_token = right_out.1.get();
    
    // Use finalize property to prove left + right == fib(n)
    proof {
        instance.finalize(&left_token, &right_token);
        lemma_fib_sum_fits_u64(n as nat);
    }
    
    left_val + right_val
}

} // verus!
