# Veracity Bug: minimize-proofs marks ensures clauses as UNNEEDED proof block

## Bug

`veracity-minimize-proofs` incorrectly identifies `ensures` clauses as "proof blocks"
and comments them out with `// Veracity: UNNEEDED proof block`.

## Example

In `src/Chap18/ArraySeqMtPer.rs`, the consuming `into_iter` had:

```rust
fn into_iter(self) -> (it: Self::IntoIter)
    ensures
        it@.0 == 0,
        it@.1 == self.seq@,    // <-- this line was marked UNNEEDED
{
    self.seq.into_iter()
}
```

Veracity commented out `it@.1 == self.seq@` as `// Veracity: UNNEEDED proof block`.

The function body verifies without this ensures clause — of course it does, a weaker
postcondition is always easier to prove. But callers depend on this ensures to prove
that the iterator's elements match the original sequence. Two PTT tests failed:
`arrayseqmtper_for_consume` and `arrayseqmtper_loop_consume`.

## Root Cause

The proof block detection heuristic is matching ensures clause lines. An ensures
clause is NOT a proof block — it's part of the function's contract. Removing it
weakens the spec, which is a regression even if the function body still verifies.

## Impact

This is a spec-gutting bug. The CLAUDE.md rule "DO NOT WEAKEN ensures TO MAKE PROOFS
EASIER" exists precisely because of this pattern. Veracity is doing mechanically what
agents are told never to do manually.

## Scope

Unknown. Need to audit all `// Veracity: UNNEEDED proof block` markers across agent
worktrees to check if any are actually ensures clauses, requires clauses, or other
non-proof-block constructs.

## Fix

The proof block detector must not match lines that are part of `ensures` or `requires`
blocks. A line is part of an ensures block if it's between `ensures` and the opening
`{` of the function body (accounting for nesting and `by` blocks).

## Bug 2: NEEDED proof block markers shift code, corrupting exec logic

### Bug

When `veracity-minimize-proofs` inserts `// Veracity: NEEDED proof block` markers,
it can place them on the wrong line — between a `let ghost` binding and the `proof {}`
block that uses it, or between an exec statement and its proof block. This shifts
code relative to proof blocks, corrupting the exec logic.

### Example

In `src/Chap37/BSTRBMtEph.rs`, the Red-Black tree `rotate_left` function had:

```rust
// Before (correct):
let ghost old_x_right = x.right;
proof {
    reveal_with_fuel(spec_is_bst_link, 2);

// After (corrupted by marker insertion):
// Veracity: NEEDED proof block
let ghost old_x_right = x.right;
proof {
    reveal_with_fuel(spec_is_bst_link, 2);
```

The marker shifted `let ghost old_x_right` down relative to the proof block, and
similar shifts happened in `rotate_right`. The code still verified (the ghost
bindings don't affect exec), but the RB tree's runtime balancing behavior changed,
causing `test_rb_balancing` to fail with `assert!(height <= 6)`.

### Root Cause

The marker insertion code does not account for the relationship between ghost
bindings and their proof blocks. It inserts markers at proof block boundaries
without checking whether intervening lines are part of the same logical unit.

### Impact

Runtime correctness corruption. This is worse than the ensures-gutting bug because:
1. Verus verification passes (ghost code doesn't affect exec)
2. RTTs catch it only if the specific behavior is tested
3. The corruption is subtle — moved ghost bindings, shifted proof blocks

### Scope

Any file with interleaved `let ghost` bindings and `proof {}` blocks. Common in
BST, AVL, RB tree modules (Chap37-41).

## Bug 3: #[cfg(verus_keep_ghost)] treated as proof block

The `#[cfg(verus_keep_ghost)]` attribute on `PartialEqSpecImpl` impls is a structural
gate, not a proof block. Commenting it out exposes the impl to the non-Verus compiler
(cargo test), which cannot find the `PartialEqSpecImpl` trait. Found in:
- `src/Chap18/ArraySeqStPer.rs:1024`
- `src/Chap37/AVLTreeSeq.rs:1339`

## Workaround

Manual review of all UNNEEDED markers before merging agent results.
Revert any file where RTT fails after merge.
