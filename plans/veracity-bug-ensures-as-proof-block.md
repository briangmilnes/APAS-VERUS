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

## Workaround

Manual review of all UNNEEDED markers before merging agent results.
