# Verus For-Loop Verification Architecture

## Overview

Verus provides automatic verification of `for` loops through the `ForLoopGhostIterator`
trait (PR #954: https://github.com/verus-lang/verus/pull/954). This document explains
how the trait methods are used during verification and why we want `next()` to establish invariants.

## The ForLoopGhostIterator Trait

```rust
pub trait ForLoopGhostIterator {
    type ExecIter;  // The concrete type of the iterator that holds executable state.
    type Item;      // The type of the item in a collection being iterated over.
    type Decrease;  // In practice: always `int` for well-founded termination.

    // Ghost state matches the ghost spec to executable state.
    spec fn exec_invariant(&self, exec_iter: &Self::ExecIter) -> bool;

    // Progression properties relative to initial state
    spec fn ghost_invariant(&self, init: Option<&Self>) -> bool;

    // Postcondition when iteration completes.
    spec fn ghost_ensures(&self) -> bool;

    // Termination measure: Some(n) decreases to Some(0) when done; None = "no auto measure, user must write decreases"
    spec fn ghost_decrease(&self) -> Option<Self::Decrease>;

    // What next() will return without calling it
    spec fn ghost_peek_next(&self) -> Option<Self::Item>;

    // Compute new ghost state after next()
    spec fn ghost_advance(&self, exec_iter: &Self::ExecIter) -> Self;
}
```

**Note:** The trait evolved from the original PR. Originally `ghost_condition()` was used to check
if iteration should continue; now `ghost_ensures()` specifies the final state postcondition.

## Actual Verus Transformation

Given this source code:
```rust
assert(user_pre());
for x in iter: collection.iter()
    invariant
        user_invariant(iter@),
{
    user_body(x);
}
assert(user_post())
```

**Currently Verus transforms it to code like this in the concrete syntax:***

```rust
{
    assert(user_pre());
    let mut        exec_iter = collection.iter();
    let ghost mut ghost_iter = vstd::pervasive::ForLoopGhostIteratorNew::ghost_iter(&exec_iter);
    let ghost init_ghost_iter = ghost_iter;  // Capture initial ghost state for ghost_invariant
    
    loop
        invariant
            vstd::pervasive::ForLoopGhostIterator::exec_invariant(&ghost_iter, &exec_iter),
            vstd::pervasive::ForLoopGhostIterator::ghost_invariant(&ghost_iter, Some(&init_ghost_iter)),
            { let x = vstd::pervasive::ForLoopGhostIterator::ghost_peek_next(&ghost_iter); user_inv },
        ensures
            vstd::pervasive::ForLoopGhostIterator::ghost_ensures(&ghost_iter),
        decreases
            vstd::pervasive::ForLoopGhostIterator::ghost_decrease(&ghost_iter),
    {
        if let Some(x) = core::iter::Iterator::next(&mut exec_iter) {
            body
        } else {
            break
        }
        proof {
            ghost_iter = vstd::pervasive::ForLoopGhostIterator::ghost_advance(&ghost_iter, &exec_iter);
        }
    }
    assert(user_post())
}

```

**Key observations:**
- User invariants are combined with automatic invariants (`exec_invariant`, `ghost_invariant`)
- User invariant binds `x` from `ghost_peek_next(&y)` 
- `next()` is called on **executable** iterator `VERUS_exec_iter`
- `ghost_advance` is in a **proof block AFTER the body**
- The ghost iterator `y` tracks abstract state; exec iterator `VERUS_exec_iter` does the work
- Termination is automatic via `ghost_decrease()`

**Note:** Historical PR #954 used `ghost_condition()` instead of `ghost_ensures()` - current Verus uses `ghost_ensures()`.

## Conceptual Model: Verification Points

The following shows **logical verification points** as explicit assertions to understand
where invariants are checked. This is not the actual transformation (see above), but a model
for understanding the verification flow:

Given this source code:
```rust
assert(user_pre());
for x in iter: collection.iter()
    invariant
        user_invariant(iter@),
{
    user_body(x);
}
assert(user_post())
```

**Conceptual verification flow:**

```rust
  1| let mut iter = collection.iter();
  2| let ghost init_iter = iter@;
  3| 
  4| assert(iter@.exec_invariant(&iter));           // From iter()'s ensures
  5| assert(iter@.ghost_invariant(Some(&init_iter)));  // From iter()'s ensures
  6| assert(user_invariant(iter@));                 // From iter()'s ensures
  7| 
  8| loop {
  9|     assert(iter@.exec_invariant(&iter));
 10|     assert(iter@.ghost_invariant(Some(&init_iter)));
 11|     assert(user_invariant(iter@));
 12|     
 13|     let ghost peeked = iter@.ghost_peek_next();
 14|     
 15|     if peeked is None {
 16|         assert(iter@.ghost_ensures());
 17|         assert(user_invariant(iter@));
 18|         assert(user_post());
 19|         break;
 20|     }
 21|     
 22|     let x = peeked.unwrap();
 23|     
 24|     let ghost old_measure = iter@.ghost_decrease();
 25|     assert(old_measure is Some);
 26|     let old_measure_value = old_measure.unwrap();
 27|     
 28|     let ghost old_iter = iter@;
 29|     assert(old_iter.exec_invariant(&iter));
 30|     
 31|     let result = iter.next();
 32|     
 33|     assert(iter@.exec_invariant(&iter));          // ← KEY: from next() ensures
 34|     assert(match result {
 35|         Some(ret) => ret@ == old_iter.ghost_peek_next().unwrap(),
 36|         None => false,
 37|     });
 38|     
 39|     let returned_x = result.unwrap();
 40|     assert(returned_x@ == x);
 41|     
 42|     let ghost new_iter = old_iter.ghost_advance(&iter);
 43|     assert(iter@ == new_iter);
 44|     assert(new_iter.exec_invariant(&iter));
 45|     assert(new_iter.ghost_invariant(Some(&init_iter)));
 46|     
 47|     let ghost new_measure = iter@.ghost_decrease();
 48|     assert(new_measure is Some);
 49|     let new_measure_value = new_measure.unwrap();
 50|     assert(new_measure_value < old_measure_value);
 51|     
 52|     user_body(returned_x);
 53|     
 54|     assert(user_invariant(iter@));
 55| }
```

## General Iterator next() Contract

For an iterator to work with Verus for-loop verification, its `next()` method should have:

**Requires:**
```rust
fn next(&mut self) -> (result: Option<Self::Item>)
    requires
        self@.exec_invariant(&self),  // Iterator in valid state
    ensures
        // Ghost state updated correctly
        self@.exec_invariant(&self),  // ← CRITICAL: Partition/structure preserved
        
        // Result matches what ghost_peek_next predicted
        match result {
            Some(x) => old(self)@.ghost_peek_next() == Some(x@),
            None => old(self)@.ghost_peek_next() == None,
        },
        
        // Ghost state transitions correctly
        result is Some ==> self@ == old(self)@.ghost_advance(&old(self)),
        
        // Termination: measure decreased or stayed at 0
        match (old(self)@.ghost_decrease(), self@.ghost_decrease()) {
            (Some(old_m), Some(new_m)) => new_m <= old_m,
            _ => true,
        },
```
