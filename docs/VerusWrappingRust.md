# Verus Wrapping of Rust Standard Library Types

This document analyzes how `vstd` wraps Rust standard library types using `#[verifier::external_type_specification]` and `assume_specification`, with particular focus on iterator invariants and preconditions.

## Key Finding: Iterators Have NO `requires` Clauses

**All iterator `next()` methods in vstd have ZERO `requires` clauses.** They assume internal invariants (like `pos <= len`) are maintained, but never require them as preconditions.

## Wrapped Types and Their Specifications

| Type | Method | Data Invariant (assumed) | `requires` Clause | `ensures` Clause |
|------|--------|-------------------------|------------------|------------------|
| **`Iter<'a, T>` (slice)** | `next()` | View: `(int, Seq<T>)` with uninterpreted view | **NONE** | Returns element at index, advances index, or returns None when `index >= seq.len()` |
| | Ghost invariant | `0 <= self.pos <= self.elements.len()` (in `ghost_invariant`) | | |
| | **File** | `vstd/std_specs/slice.rs:62-82` | | |
| **`IntoIter<T, A>` (Vec)** | `next()` | View: `(int, Seq<T>)` with uninterpreted view | **NONE** | Same as slice::Iter |
| | Ghost invariant | `0 <= self.pos <= self.elements.len()` | | |
| | **File** | `vstd/std_specs/vec.rs:311-331` | | |
| **`Iter<'a, T>` (VecDeque)** | `next()` | View: `(int, Seq<T>)` with uninterpreted view | **NONE** | Same as slice::Iter |
| | Ghost invariant | `0 <= self.pos <= self.elements.len()` | | |
| | **File** | `vstd/std_specs/vecdeque.rs:263-283` | | |
| **`Values<'a, K, V>` (HashMap)** | `next()` | View: `(int, Seq<Value>)` with uninterpreted view | **NONE** | Same pattern as Iter |
| | Ghost invariant | `0 <= self.pos <= self.values.len()` | | |
| | **File** | `vstd/std_specs/hash.rs:394-414` | | |
| **`Vec<T, A>`** | `push()` | None | **NONE** | `vec@ == old(vec)@.push(value)` |
| | `pop()` | None | **NONE** | Returns last element if non-empty |
| | `swap_remove()` | None | **`i < old(vec).len()`** | Returns element at i, last element moves to i |
| | `insert()` | None | **`i <= old(vec).len()`** | Inserts at index i |
| | `remove()` | None | **`i < old(vec).len()`** | Removes at index i |
| | `split_off()` | None | **`at <= old(vec)@.len()`** | Splits at index |
| | **File** | `vstd/std_specs/vec.rs:69-429` | | |
| **`Range<Idx>`** | *for loops* | None | N/A | Used in for loops, no explicit next() spec |
| | **File** | `vstd/std_specs/range.rs:8-60` | | |

## Pattern Analysis

### External Type Specifications

Vstd uses `#[verifier::external_type_specification]` to wrap Rust standard library types:

```rust
#[verifier::external_type_specification]
#[verifier::external_body]
#[verifier::accept_recursive_types(T)]
pub struct ExIter<'a, T: 'a>(Iter<'a, T>);

impl<T> View for Iter<'_, T> {
    type V = (int, Seq<T>);
    uninterp spec fn view(&self) -> (int, Seq<T>);  // Uninterpreted!
}
```

The `uninterp` (uninterpreted) view means Verus doesn't know the internal representation—it just trusts the contract.

### Iterator `next()` Pattern

**ALL iterator `next()` methods follow this pattern:**

```rust
pub assume_specification<'a, T>[ Iter::<'a, T>::next ](elements: &mut Iter<'a, T>) -> (r: Option<&'a T>)
    // NO requires clause!
    ensures
        ({
            let (old_index, old_seq) = old(elements)@;
            match r {
                None => {
                    &&& elements@ == old(elements)@
                    &&& old_index >= old_seq.len()  // Note: >= not ==
                },
                Some(element) => {
                    let (new_index, new_seq) = elements@;
                    &&& 0 <= old_index < old_seq.len()
                    &&& new_seq == old_seq
                    &&& new_index == old_index + 1
                    &&& element == old_seq[old_index]
                },
            }
        }),
;
```

### Ghost Iterator Invariants

The invariant `0 <= pos <= len` appears in `ForLoopGhostIterator::ghost_invariant`:

```rust
open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
    init matches Some(init) ==> {
        &&& init.pos == 0
        &&& init.elements == self.elements
        &&& 0 <= self.pos <= self.elements.len()  // HERE!
    }
}
```

**But this is NOT a precondition to `next()`**—it's part of the ghost state verification for `for` loops.

### Vec Mutating Methods

In contrast, Vec methods that need bounds checking **DO have `requires` clauses**:

```rust
pub assume_specification<T, A: Allocator>[ Vec::<T, A>::swap_remove ](
    vec: &mut Vec<T, A>,
    i: usize,
) -> (element: T)
    requires
        i < old(vec).len(),  // Explicit precondition!
    ensures
        // ...
```

## Implications for Custom Iterators

When implementing a custom iterator in Verus:

1. **You cannot add `requires` to trait methods** (Verus limitation)
2. **Vstd's approach**: Trust that external types maintain their invariants
3. **Your approach**: Use `assume` in the trait implementation to stand in for the missing `requires` clause
4. **Confidence building**: Implement a parallel `assumption_free_next` with `requires` to prove the logic is sound

### Example from `simple_seq_iter.rs`

```rust
// The iterator invariant that should be maintained but we can't add to next's requires.
pub open spec fn iter_invariant<V>(it: &SimpleSeqIter<V>) -> bool { 
    it.pos <= it.vec@.len() 
}

// Trait implementation (can't have requires)
impl<V: Clone> Iterator for SimpleSeqIter<V> {
    fn next(&mut self) -> (result: Option<V>)
        ensures /* ... */
    {
        if self.pos < self.vec.len() {
            // ...
        } else {
            proof {
                assume(self.pos <= self.vec.len());  // Stand-in for missing requires
            }
            None
        }
    }
}

// Confidence-building version (WITH requires, proves without assumes)
fn assumption_free_next<V: Clone>(it: &mut SimpleSeqIter<V>) -> (result: Option<V>)
    requires iter_invariant(&old(it)),  // Can add requires here!
    ensures iter_invariant(it),
{
    if it.pos < it.vec.len() {
        // ...
    } else {
        None  // Proves without assume!
    }
}
```

## Conclusion

**The asymmetry:**
- **External Rust types**: `assume_specification` with no requires → trusted, no proof obligation
- **Custom Verus types**: Must prove everything, but can't add requires to trait methods → stuck with `assume`

Your `assume(self.pos <= self.vec.len())` is doing the same thing vstd does with external types—**assuming the invariant holds** because you can't express it as a precondition due to trait limitations.

The `assumption_free_next` pattern provides evidence that the logic would be sound if Verus allowed `requires` on trait method implementations.

