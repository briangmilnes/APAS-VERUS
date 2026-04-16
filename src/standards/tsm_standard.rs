// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

//! TSM Standard: writing a custom tokenized_state_machine! for fork-join tracking.
//!
//! Use a TSM when the protocol is beyond what RwLock can express. RwLock gives
//! mutual exclusion with an invariant; a TSM tracks arbitrary protocol state
//! across threads using ghost tokens.
//!
//! This standard shows:
//! - Defining a tokenized_state_machine! with fields, invariant, init, transitions, property.
//! - Sharding modes: `constant` (immutable), `variable` (mutable).
//! - Client code: initialize returns (instance, tokens), move tokens to threads,
//!   call transitions in proof blocks, join, call property to finalize.
//! - Inductive proofs: each init/transition must preserve the invariant.
//!
//! Advanced sharding modes (not shown here):
//! - `option`: single-element token (present or absent).
//! - `multiset`: multi-element token set (e.g., reader tracking in RwLock).
//! - `storage_option`: exclusive ownership with withdraw/deposit.
//!
//! When to use TSM vs RwLock:
//! - RwLock: shared mutable state with an invariant (most concurrent data structures).
//! - TSM: protocols beyond mutual exclusion — fork-join tracking, multi-phase
//!   coordination, producer-consumer sequencing.
//!
//! References:
//! - src/Chap11/FibonacciMtPerTSM.rs (recursive fork-join with TSM)
//! - vstd::rwlock (RwLock is itself a TSM internally)

#![cfg(verus_keep_ghost)]

pub mod tsm_standard {

    use vstd::prelude::*;
    use vstd::thread::*;
    use vstd::modes::*;
    use verus_state_machines_macros::tokenized_state_machine;
    use crate::Concurrency::diverge;

    verus! {

    use crate::vstdplus::accept::accept;

    // 4. type definitions

    // A minimal TSM tracking fork-join completion of two workers.
    // - `target`: constant field (set at init, never changes).
    // - `left_done`, `right_done`: variable fields (updated by transitions).
    // - `finalize`: property that asserts both workers completed.
    tokenized_state_machine!{
        ForkJoinTracker {
            fields {
                #[sharding(constant)]
                pub target: nat,

                #[sharding(variable)]
                pub left_done: bool,

                #[sharding(variable)]
                pub right_done: bool,
            }

            #[invariant]
            pub fn main_inv(&self) -> bool {
                self.target > 0
            }

            init!{
                initialize(target: nat) {
                    require(target > 0);
                    init target = target;
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

            /// Property: assert both done without changing state.
            /// Unlike transition!, property! has no update — it only asserts.
            property!{
                finalize() {
                    require(pre.left_done);
                    require(pre.right_done);
                }
            }

            // Each init/transition must preserve the invariant.
            // Trivial here because the invariant only constrains the constant field.
            #[inductive(initialize)]
            fn initialize_inductive(post: Self, target: nat) { }

            #[inductive(complete_left)]
            fn complete_left_inductive(pre: Self, post: Self) { }

            #[inductive(complete_right)]
            fn complete_right_inductive(pre: Self, post: Self) { }
        }
    }

    // 9. impls

    /// Spawn two threads, each returning one input value. TSM tracks completion.
    /// The computation is trivial — the point is the token protocol.
    pub fn parallel_add(a: u64, b: u64) -> (sum: u64)
        requires a as nat + b as nat <= u64::MAX as nat,
        ensures sum == a + b,
    {
        // 1. Initialize: returns instance + one token per variable field.
        let tracked (
            Tracked(instance),
            Tracked(left_token),
            Tracked(right_token),
        ) = ForkJoinTracker::Instance::initialize(1);

        // Clone instance for each thread (instance is ghost, cloneable).
        let tracked instance1 = instance.clone();
        let tracked instance2 = instance.clone();

        // 2. Left thread: returns a, completes left_done transition.
        let left_handle = spawn(
            (move || -> (out: (u64, Tracked<ForkJoinTracker::left_done>))
                requires
                    left_token.instance_id() == instance1.id(),
                    left_token.value() == false,
                ensures
                    out.0 == a,
                    out.1@.instance_id() == instance1.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = left_token;
                proof { instance1.complete_left(&mut token); }
                (a, Tracked(token))
            })
        );

        // 3. Right thread: returns b, completes right_done transition.
        let right_handle = spawn(
            (move || -> (out: (u64, Tracked<ForkJoinTracker::right_done>))
                requires
                    right_token.instance_id() == instance2.id(),
                    right_token.value() == false,
                ensures
                    out.0 == b,
                    out.1@.instance_id() == instance2.id(),
                    out.1@.value() == true,
            {
                let tracked mut token = right_token;
                proof { instance2.complete_right(&mut token); }
                (b, Tracked(token))
            })
        );

        // 4. Join both threads, recover tokens.
        let left_out = match left_handle.join() {
            Result::Ok(out) => out,
            Result::Err(_) => { proof { accept(false); }; diverge() }
        };
        let left_val = left_out.0;
        let tracked left_done = left_out.1.get();

        let right_out = match right_handle.join() {
            Result::Ok(out) => out,
            Result::Err(_) => { proof { accept(false); }; diverge() }
        };
        let right_val = right_out.0;
        let tracked right_done = right_out.1.get();

        // 5. Finalize: both tokens prove completion.
        proof {
            instance.finalize(&left_done, &right_done);
        }

        left_val + right_val
    }

    } // verus!
} // pub mod tsm_standard
