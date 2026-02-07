//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 11 — Parallel Fibonacci with TSM (Tokenized State Machine).
//!
//! Full recursive parallelism with TSM tracking fork-join at each level.
//! No intermediate values stored - pure recomputation.
//!
//! APAS: Work Θ(φⁿ), Span Θ(n) where φ ≈ 1.618

#![cfg(verus_keep_ghost)]

use vstd::prelude::*;
use vstd::thread::*;
use vstd::modes::*;
use verus_state_machines_macros::tokenized_state_machine;
use crate::Concurrency::*;

verus! {

use crate::Chap11::FibonacciStEph::FibonacciStEph::*;

// TSM for tracking one fork-join pair.
// No intermediate values - just completion flags.
tokenized_state_machine!{
    FibForkJoin {
        fields {
            #[sharding(constant)]
            pub n: nat,
            
            #[sharding(variable)]
            pub left_done: bool,
            
            #[sharding(variable)]
            pub right_done: bool,
        }
        
        #[invariant]
        pub fn main_inv(&self) -> bool {
            self.n >= 2
        }
        
        init!{
            initialize(n: nat) {
                require(n >= 2);
                init n = n;
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

/// Parallel Fibonacci with TSM at each recursive fork-join.
///
/// Each call spawns two threads and uses a fresh TSM instance
/// to track their completion. No intermediate values stored.
pub fn fib(n: u64) -> (fibonacci: u64)
    requires n <= 46
    ensures fibonacci == spec_fib(n as nat)
    decreases n
{
    if n <= 1 {
        n
    } else {
        // Create TSM instance for this fork-join
        let tracked (
            Tracked(instance),
            Tracked(left_token),
            Tracked(right_token),
        ) = FibForkJoin::Instance::initialize(n as nat);
        
        let tracked instance1 = instance.clone();
        let tracked instance2 = instance.clone();
        
        // Left branch: fib(n-1)
        let left_handle = spawn(
            (move || -> (out: (u64, Tracked<FibForkJoin::left_done>))
                requires 
                    n >= 2 && n <= 46,
                    left_token.instance_id() == instance1.id(),
                    left_token.value() == false,
                ensures 
                    out.0 == spec_fib((n - 1) as nat),
                    out.1@.instance_id() == instance1.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = left_token;
                let val = fib(n - 1);
                proof { instance1.complete_left(&mut token); }
                (val, Tracked(token))
            })
        );
        
        // Right branch: fib(n-2)
        let right_handle = spawn(
            (move || -> (out: (u64, Tracked<FibForkJoin::right_done>))
                requires 
                    n >= 2 && n <= 46,
                    right_token.instance_id() == instance2.id(),
                    right_token.value() == false,
                ensures 
                    out.0 == spec_fib((n - 2) as nat),
                    out.1@.instance_id() == instance2.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = right_token;
                let val = fib(n - 2);
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
        let tracked left_done = left_out.1.get();
        
        // Join right  
        let right_out = match right_handle.join() {
            Result::Ok(out) => out,
            Result::Err(_) => { assume(false); diverge() }
        };
        let right_val = right_out.0;
        let tracked right_done = right_out.1.get();
        
        // Finalize TSM and prove sum fits
        proof {
            instance.finalize(&left_done, &right_done);
            lemma_fib_sum_fits_u64(n as nat);
        }
        
        left_val + right_val
    }
}

} // verus!
