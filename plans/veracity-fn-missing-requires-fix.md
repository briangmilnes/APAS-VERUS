# Plan: Fix veracity-review-proof-holes fn_missing_requires False Positives

## Problem

`veracity-review-proof-holes` flags `fn_missing_requires` on any exec function that has
`ensures` but no `requires`. This creates false positives for functions that genuinely
have no precondition:

- `total_order_le` — calls `TotalOrder::cmp` which has no requires.
- `clone_vec_pairs` / `clone_linked_list_entry` — loop bounded by input length, handles all inputs.
- `new_node` — constructors that build a value from arguments.
- `push_left_iter` — called from `Iterator::next` which can't have requires.
- `from_sorted_iter` — takes an arbitrary iterator, no precondition on it.

These are counted as chapter-level errors and inflate hole counts, causing wasted
agent effort investigating non-issues.

## Proposed Fix

Add a `#[verifier::no_requires]` annotation (or project-level equivalent) that
explicitly marks a function as intentionally having no requires. Then:

### Option A: Source-level annotation (preferred)

Add a comment-based annotation that veracity recognizes:

```rust
// veracity: no_requires — TotalOrder::cmp has no precondition
fn total_order_le(a: &T, b: &T) -> (result: bool)
    ensures result == (a@ <= b@),
{ ... }
```

veracity parses `// veracity: no_requires` on the line immediately before the `fn`
declaration and suppresses the `fn_missing_requires` warning for that function.

### Option B: Configuration file

Add a veracity config file (e.g., `.veracity.toml`) listing functions to exclude:

```toml
[suppress]
fn_missing_requires = [
    "total_order_le",
    "clone_vec_pairs",
    "clone_linked_list_entry",
    "new_node",
    "push_left_iter",
    "from_sorted_iter",
]
```

### Option C: Heuristic suppression

Suppress `fn_missing_requires` when:
- The function's ensures clause uses only the function's parameters (no `self` state).
- The function is a constructor (returns `Self`).
- The function is called from a trait method that has no requires (e.g., `Iterator::next`).

This is fragile and hard to get right. Not recommended.

## Recommendation

**Option A** is simplest and most explicit. One comment per function, placed by the
developer who understands why no requires is needed. The annotation is self-documenting
and version-controlled alongside the code.

## Implementation Steps

1. In veracity's proof-holes scanner, when processing an exec function:
   - Check if the previous non-blank line contains `// veracity: no_requires`.
   - If so, skip emitting `fn_missing_requires` for that function.
   - Optionally emit an `info:` line instead: `info: fn_no_requires_annotated`.

2. Apply `// veracity: no_requires` annotations to the ~10-15 functions across APAS-VERUS
   that are genuine false positives.

3. Verify that the remaining `fn_missing_requires` warnings are all real (functions that
   need a precondition added).

## Files to Modify

In veracity (separate repo `~/projects/veracity/`):
- The proof-holes review module that emits `fn_missing_requires` warnings.
- Look for the pattern-matching code that checks `requires` clause presence.
- Add the comment-annotation check before emitting the warning.

## Testing

- Run `veracity-review-proof-holes src/Chap45/LeftistHeapPQ.rs` before and after.
  Before: `fn_missing_requires` on `total_order_le`.
  After: `info: fn_no_requires_annotated` or silent.
- Run on full project: confirm warning count drops by ~10-15 (the false positives)
  and no true positives are suppressed.
