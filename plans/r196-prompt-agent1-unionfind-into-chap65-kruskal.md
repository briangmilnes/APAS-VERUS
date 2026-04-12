# R196 Prompt — Agent 1: Move UnionFind into Chap65 and rewire Kruskal. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.** The
   only exception is leaving in place ones that already exist in files
   you do not touch.
6. **NEVER modify `~/projects/verus/` or `Cargo.toml/scripts/validate.sh`
   beyond what this prompt requires** (you will edit `Cargo.toml` to register
   moved files — that's expected).
7. **NEVER delete proof code.** If something doesn't fit, comment it out
   with `// BYPASSED:` and report it.

## Read all standards first.

Pay special attention to `mod_standard.rs`, `table_of_contents_standard.rs`,
and `using_closures_standard.rs`.

## Context

R195 closed `find()` in `src/UnionFind/UnionFindPCStEph.rs`. The three
files in `src/UnionFind/` are now all clean (0 errors, 0 holes):

- `src/UnionFind/UnionFindStEph.rs` — generic HashMap UF, no PC.
- `src/UnionFind/UnionFindPCStEph.rs` — generic HashMap UF with path compression.
- `src/UnionFind/UnionFindArrayStEph.rs` — array-based UF, `usize` elements.

Meanwhile `src/Chap65/UnionFindStEph.rs` is the **older** UF that
`KruskalStEph.rs` currently imports. It still has 3 `external_body`
(union, lemma_union_merge_wf, lemma_union_merge_wf_half_a) per its
header comment. We want to retire it.

## Goal

Move all three new UnionFind files into `src/Chap65/`, retire the old
`src/Chap65/UnionFindStEph.rs`, and rewire `KruskalStEph.rs` to call
`UnionFindPCStEph` (the path-compression version — Kruskal benefits
most from PC because it does many `find()`s per edge).

## Plan

### Step 1: Inspect the old Kruskal/UF interface

Before moving anything, read `src/Chap65/KruskalStEph.rs` end to end.
Note every call into `UnionFindStEph` (constructor, `find`, `union`,
membership, well-formedness predicate name). The new
`UnionFindPCStEph` API may differ — read its trait carefully.

Common shape mismatches to watch for:
- Constructor name (`new` vs `with_capacity` vs `from_iter`).
- `find` signature (`&mut self` for PC vs `&self` for non-PC).
- Well-formedness predicate name (`spec_unionfindsteph_wf` vs
  `spec_unionfindpcsteph_wf`).
- Whether `find` requires `&mut self` (PC mutates parent pointers).
  This propagates: any Kruskal helper that took `uf: &UnionFindStEph`
  will need `uf: &mut UnionFindPCStEph` and must thread mutability through.
- Ghost field for canonical roots (the old Kruskal tracked
  `roots: Map<V, V>`; PC version uses `spec_pure_find` directly).

### Step 2: Move the files

Use `git mv` so history follows:

```bash
git mv src/UnionFind/UnionFindStEph.rs       src/Chap65/UnionFindNoPCStEph.rs
git mv src/UnionFind/UnionFindPCStEph.rs     src/Chap65/UnionFindPCStEph.rs
git mv src/UnionFind/UnionFindArrayStEph.rs  src/Chap65/UnionFindArrayStEph.rs
```

Note: the new generic-no-PC version is renamed to `UnionFindNoPCStEph` to
avoid colliding with the existing `src/Chap65/UnionFindStEph.rs` (which
will be removed in Step 4). This makes the three names unambiguous:
`UnionFindNoPCStEph`, `UnionFindPCStEph`, `UnionFindArrayStEph`.

Inside each moved file, update the inner `pub mod` name to match the file:
- `pub mod UnionFindStEph` → `pub mod UnionFindNoPCStEph`
- The other two already match.

### Step 3: Update lib.rs

Remove the top-level `pub mod UnionFind { ... }` block (lines ~21–25).
Add the three new files under the existing `pub mod Chap65 { ... }` block:

```rust
#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap65")))]
pub mod Chap65 {
    pub mod UnionFindNoPCStEph;
    pub mod UnionFindPCStEph;
    pub mod UnionFindArrayStEph;
    pub mod KruskalStEph;
    pub mod PrimStEph;
}
```

Order: bottom-up — UF variants come before Kruskal/Prim because Kruskal
will depend on `UnionFindPCStEph`.

### Step 4: Retire the old `src/Chap65/UnionFindStEph.rs`

`git rm src/Chap65/UnionFindStEph.rs`. Its slot is taken by
`UnionFindNoPCStEph.rs`. The old UF had 3 `external_body`; the new one
has 0. This is a strict upgrade. Do not preserve it under another name.

### Step 5: Rewire `KruskalStEph.rs`

Replace every `crate::Chap65::UnionFindStEph::UnionFindStEph::*`
import with `crate::Chap65::UnionFindPCStEph::UnionFindPCStEph::*`.

Then for each call into UF:
- Update the type name (`UnionFindStEph` → `UnionFindPC` or whatever the
  new struct is named — read the trait).
- Update the well-formedness predicate name (e.g.,
  `spec_unionfindsteph_wf` → the PC variant's predicate).
- If `find` is now `&mut self`, change Kruskal's call sites accordingly.
  This may require turning `uf: &UnionFindStEph` parameters into
  `uf: &mut UnionFindPCStEph`, and propagating that through any helper
  that calls `find`. The `union` method was already `&mut`, so most of
  Kruskal already threads mutability.
- Re-discharge any spec equalities that used to come from the old wf —
  the new PC predicate may state things differently.

If Kruskal had `uf_opaque_wrappers` nested module wrappers around the
old UF wf to dodge broadcast-group cross-fire, re-evaluate them. The new
PC wf is already opaque (R195), so most wrappers should be unnecessary
and can be deleted (this is a deletion, not a bypass — the new design
makes them obsolete).

### Step 6: Rewire `tests/Chap65/TestUnionFindStEph.rs`

The runtime test file still imports the old UF. Rewire it to test
`UnionFindPCStEph` instead (or `UnionFindNoPCStEph` if the test is
specifically about the non-PC variant — read it and decide). Update
`Cargo.toml` test entry name if the file is renamed.

If the test cannot be ported because the new API is fundamentally
different (e.g., needed exec methods that don't exist), preserve the
test against `UnionFindNoPCStEph` (the closest match) and add a
separate fresh test for `UnionFindPCStEph` covering find/union after
path compression.

### Step 7: Delete the now-empty `src/UnionFind/` directory

After `git mv` and `git rm`, remove the empty directory:

```bash
rmdir src/UnionFind/    # only if truly empty; do NOT use rm -rf
```

If there are leftover files (analyses/, emacs backups), leave them and
report what's there. **Do not delete emacs backup files (`*~`).**

### Step 8: Validate

Run isolated validation on Chap65 first:

```bash
scripts/validate.sh isolate Chap65
```

Then full validation:

```bash
scripts/validate.sh
```

Then runtime + proof tests:

```bash
scripts/rtt.sh
scripts/ptt.sh
```

All four must be clean (zero errors, zero new warnings, zero new
external_body, zero new assume/accept/admit). The R195 work in
`UnionFindPCStEph.rs` itself stays clean (you are only moving and
calling it, not editing its proofs).

If Kruskal proofs break in ways you cannot fix without weakening specs
or adding holes, **stop and report**. Do NOT add `accept`,
`external_body`, or weaken `ensures` to make the migration close.
Leaving Kruskal in a partially-migrated state with a clear report is
better than silently introducing holes.

## Out of scope

- Do not touch any chapter outside Chap65 (and the Chap65-related test files).
- Do not modify `UnionFindPCStEph.rs`'s proof internals — they are clean.
  You may touch its imports if `crate::` paths shift due to the move.
- Do not touch `PrimStEph.rs` unless it imports the old UF (check; if so,
  follow the same rewire pattern; if not, leave it alone).

## Report

Write `plans/agent1-round196-report.md` with:

- Before/after veracity hole count for Chap65 (use
  `scripts/holes.sh src/Chap65/`).
- Verified count from the full validate.
- RTT and PTT pass counts.
- Any spec changes you had to make in Kruskal to absorb the PC API.
- Anything you bypassed or commented out, with a `// BYPASSED:` line
  number reference.
- Whether Kruskal's `uf_opaque_wrappers` nested module survived or was
  deleted (and why).

## RCP

`git add -A && git commit -m "R196 Agent 1: relocate UnionFind into Chap65; Kruskal uses UnionFindPCStEph"`,
then `git push`.
