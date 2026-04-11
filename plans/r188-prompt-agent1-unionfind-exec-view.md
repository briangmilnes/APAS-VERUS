# R188 Prompt — Agent 1: Fix exec-to-view bridges. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `src/lib.rs`, or `scripts/validate.sh`.**

## Read all standards first

Pay close attention to `src/standards/partial_eq_eq_clone_standard.rs`.
It documents the PartialEq/View bridge pattern used across the project.

## Context

R187: 724 verified, 7 errors. You discovered the real blocker:
exec equality (`*a == *b`) does not give view equality (`a@ == b@`).
The array version never hits this because `usize as int` is trivial.

## The exec-to-view bridge

The project has a standard pattern for this. Read these:

1. `src/standards/partial_eq_eq_clone_standard.rs` — the PartialEq pattern
2. `src/vstdplus/feq/feq.rs` — `obeys_feq_full`, `obeys_feq_view_injective`

The key axiom: `obeys_feq_view_injective::<V>()` gives:
```
forall|a: V, b: V| a@ == b@ <==> (a == b)
```

If V implements this (which StT types do), then exec `a == b` gives `a@ == b@`.

### How to use it

Your wf should require `obeys_feq_view_injective::<V>()`. Then at each
exec equality check:

```rust
let parent_val = self.parent.get(&curr);
if *parent_val == curr {
    // curr is a root
    proof {
        assert(obeys_feq_view_injective::<V>());
        // From feq_view_injective: (*parent_val == curr) ==> (parent_val@ == curr@)
        assert(parent_val@ == curr@);
        // From get ensures: parent_val == self.parent@[curr@]
        // Therefore: self.parent@[curr@]@ == curr@
        // Which is: pv(parent, curr@) == curr@ — curr is a spec root
    }
}
```

### HashMapWithViewPlus.get ensures

Check what `get` returns. It should ensure `*result == self@[key@]` or
similar. The bridge is: get gives you the stored V value, feq_view_injective
connects exec equality to view equality, and you're done.

### clone_view for insert

When you do `self.parent.insert(root_v, winner_clone)`, you need:
- `winner_clone@ == root_u@` (the clone preserves view)
- ClonePreservesView gives this via `clone_view()`

You already have ClonePreservesView as a bound. Use `clone_view()` for
every insert value, and assert the view equality immediately after.

## The 7 errors — fix them

Don't analyze, don't restructure, don't discover new problems. FIX THEM.

For each error:
1. Read the error message
2. Identify which exec→view bridge is missing
3. Add the feq_view_injective or clone_view assertion
4. Validate
5. Move to the next error

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Or full validate if isolate doesn't work. Do NOT fix the build system.

## Report

Write `plans/agent1-round188-report.md`.

## RCP

`git add -A && git commit -m "R188 Agent 1: fix exec-to-view bridges in HashMap UnionFind"`, then `git push`.
