# R194 Prompt — Agent 1: Use broadcast proof fn to prove find() compression. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, or `accept`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `scripts/validate.sh`.**
8. **DO NOT REBASE. Build on your branch.**
9. **NEVER modify `UnionFindArrayStEph.rs` or `UnionFindStEph.rs`.** Only `UnionFindPCStEph.rs`.

## Read all standards first

## Context

R193: 718 verified, 1 error. The compression while loop in find() times
out because `assert forall|z| ... by { lemma(z); }` puts the per-element
lemma's quantifiers into the Z3 context, causing matching loops.

The F* version solves this with `FStar.Classical.forall_intro` — a clean
context barrier that lifts a per-element proof to a universal without Z3
seeing the proof internals.

## The Verus equivalent: broadcast proof fn

Verus has `broadcast proof fn` — this is exactly `forall_intro`. It proves
a lemma for a fresh variable, then makes the universal available via
`broadcast use`. Z3 never sees the proof body.

### How it works

```rust
// Step 1: write the broadcast proof fn
broadcast proof fn broadcast_compress_find_preserved<V: Bounds>(
    parent_old: Map<V::V, V>, parent_new: Map<V::V, V>,
    rank: Map<V::V, int>, n: nat, compressed_node: V::V,
    z: V::V,  // the universally quantified variable
)
    requires
        // wf for parent_old, characterization of parent_new, etc.
    ensures
        spec_pure_find(parent_new, rank, n, z)
            == spec_pure_find(parent_old, rank, n, z),
{
    // Call the per-element lemma — this runs in its OWN Z3 context
    lemma_compress_preserves_find(parent_old, parent_new, rank, n, compressed_node, z);
}

// Step 2: in the compression loop body, use it
proof {
    lemma_compress_step_wf(...);  // proves new parent map is wf
    broadcast use broadcast_compress_find_preserved;
    // Z3 now has: forall|z| find(new) == find(old)
    // WITHOUT seeing the per-element proof internals
}
```

The key: `broadcast proof fn` proves its ensures for a fresh `z` in a
separate Z3 query. Then `broadcast use` adds `forall|z| ensures(z)` as
an axiom. Z3 never sees `lemma_compress_preserves_find`'s internals —
no quantifier leakage, no matching loops.

### Important: broadcast proof fn restrictions

1. Cannot be recursive (yours isn't — it wraps a non-broadcast lemma)
2. Parameters become the universally quantified variables
3. The `requires` become the trigger guard (when the forall fires)
4. Must be declared at module level, not inside a function

### Read the Verus test

`~/projects/verus/source/rust_verify_test/tests/broadcast_forall.rs`

Study how broadcast proof fns work. The pattern is straightforward.

## What to do

1. Remove `#[verifier::external_body]` from find().
2. Write `broadcast proof fn broadcast_compress_find_preserved` that wraps
   `lemma_compress_preserves_find`.
3. In the compression loop body, replace the `assert forall ... by { ... }`
   with `broadcast use broadcast_compress_find_preserved`.
4. Validate.

If the broadcast approach doesn't work for some reason (Verus limitation
on broadcast with generic types, etc.), try:
- Making `lemma_compress_preserves_find`'s ensures as minimal as possible
- Using `assert ... by { ... }` with ONLY the conclusion, no intermediate facts
- Splitting the loop body further

## Also: rename union_sets to union

The trait function should be `fn union` not `fn union_sets`. Kruskal
calls `uf.union(&u, &v)`.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: **0 errors, 0 external_body.**

## Report

Write `plans/agent1-round194-report.md`.

## RCP

`git add -A && git commit -m "R194 Agent 1: broadcast proof fn — find() compression proved"`, then `git push`.
