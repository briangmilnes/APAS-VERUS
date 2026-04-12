# Veracity Bug: "UNNEEDED proof block" pass mislabels executable code

## Summary

The veracity "UNNEEDED proof block" pass incorrectly labels executable-mode statements as
removable proof scaffolding. The pattern is applied to statements that are structurally
adjacent to proof blocks or assert statements, but are themselves executable code that must
remain for correctness and resource safety.

## Observed Effect

Two classes of bugs have been confirmed:

1. **Silent resource leaks (lock-release removed):** `release_read()` and `release_write()`
   calls were commented out. The paired `acquire_read()` / `acquire_write()` remained. This
   causes RwLock handles to leak, leading to deadlock under write contention (any subsequent
   writer blocks forever waiting for a reader that never releases).

2. **Silent correctness bugs (recursive algorithm step removed):** A recursive descent call
   `rotated.left = Self::delete_link(rotated.left.take(), target)` was commented out in the
   treap delete implementation. Deleting a key that requires descending into the left subtree
   would silently succeed without removing the target node, leaving the treap in an
   inconsistent state.

## Root Cause Hypothesis

The pass appears to classify code as "UNNEEDED" based on structural proximity to `proof { }`
blocks or `assert(...)` statements rather than on whether the code is in exec-mode or
proof-mode. Statements that appear between proof blocks, or that follow a block of
`// Veracity: NEEDED` annotations, are incorrectly labeled as proof scaffolding.

The pass does not appear to distinguish:
- `.release_read()` / `.release_write()` calls (exec-mode, balance acquire/release contract)
- `Self::delete_link(...)` recursive calls (exec-mode, algorithmic step)
- `proof { ... }` blocks (proof-mode, safe to remove if truly unneeded)
- `assert(...)` statements (proof-mode, safe to remove if truly unneeded)

## Evidence: Lines Fixed in R200

| # | Chap | File | Line | Code Uncommented | Category |
|---|------|------|------|------------------|----------|
| 1 | 43 | OrderedSetMtEph.rs | 417 | `other_read.release_read();` (in `intersection`) | Lock release |
| 2 | 43 | OrderedSetMtEph.rs | 434 | `other_read.release_read();` (in `union`) | Lock release |
| 3 | 43 | OrderedTableMtPer.rs | 617 | `read_handle.release_read();` (in `last_key`) | Lock release |
| 4 | 43 | OrderedTableMtPer.rs | 697 | `read_handle.release_read();` (in `get_key_range`) | Lock release |
| 5 | 39 | BSTTreapStEph.rs | 1336 | `rotated.left = Self::delete_link(rotated.left.take(), target);` | Algorithmic step |

Note: for `get_key_range` (line 697), three additional lines (698-700) were also labeled
`UNNEEDED proof block` — those lines contain a `proof { ... }` block and were correctly
left commented. Only the exec-mode `release_read()` call at line 697 was uncommented.

A prior round (R199) fixed the same pattern in `OrderedSetMtEph.rs`'s `filter` method,
where `write_handle.release_write(locked_val)` was mislabeled and removed, causing a
deadlock under writer contention that was caught by the `test_deadlock_prevention_*` test.

## Broader Audit Result (R200)

Grep for all remaining `// Veracity: UNNEEDED proof block` occurrences in `src/` surfaced
approximately 60 additional candidates. After reading context for each, all remaining
candidates were classified as:

- **Proof-mode** (assert, proof block body, lemma call, reveal, forall/exists terms): leave
  commented.
- **Comments** (lines whose content is itself a comment): leave commented.
- **Spec expressions** (ensures/requires/invariant clause fragments): leave commented.
- **Closing braces of proof blocks**: leave commented.

No additional exec-mode mislabels were found beyond the 5 fixed in this round.

## Recommendation for Veracity

The "UNNEEDED proof block" pass should distinguish exec-mode statements from proof-mode
ones before labeling code as removable. Specifically:

- Statements of the form `expr.method()` where `method` is not a known proof lemma (e.g.,
  `release_read`, `release_write`, `take`, `push`, `insert`) should NOT be labeled UNNEEDED
  unless they are inside a `proof { }` block.
- Assignment statements (`lhs = rhs`) to non-ghost lvalues should NOT be labeled UNNEEDED.
- Recursive calls (`Self::fn(...)`) should NOT be labeled UNNEEDED.

A safe conservative rule: only label a line UNNEEDED if it is (a) inside a `proof { }`
block or (b) an `assert(...)` statement. Anything else requires manual confirmation.
