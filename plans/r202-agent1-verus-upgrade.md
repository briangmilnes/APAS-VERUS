# Agent1 R202 — Verus Upgrade + Close 4 Send/Sync Holes

## Mission

Upgrade Verus from `ff454ab0f` (0.2026.03.24) to `release/rolling/0.2026.04.10.fc697a7`
(latest April 10 rolling release), then close the 4 remaining proof holes in Chap41 that
the upgrade unlocks.

You are agent1. Work only in your own worktree. Do not touch other agents' worktrees.
Do not spawn subagents. Do all work sequentially. Run validate, rtt, ptt sequentially —
never in parallel.

---

## Background

The 4 remaining proof holes in the codebase are:

```
src/Chap41/AVLTreeSetMtEph.rs:681  unsafe impl Send for AVLTreeSetMtEph
src/Chap41/AVLTreeSetMtEph.rs:682  unsafe impl Sync for AVLTreeSetMtEph
src/Chap41/AVLTreeSetMtPer.rs:739  unsafe impl Send for AVLTreeSetMtPer
src/Chap41/AVLTreeSetMtPer.rs:740  unsafe impl Sync for AVLTreeSetMtPer
```

These exist because `Ghost<T>` did not implement `Send` or `Sync` in vstd. The structs
contain ghost fields, so Rust could not auto-derive Send/Sync, and we had to use
`unsafe impl` as a workaround.

Verus PR #2287 (commit `16fa185f3`, merged 2026-04-02) adds safe impls to vstd:

```rust
unsafe impl<A> Send for Ghost<A> {}
unsafe impl<A> Sync for Ghost<A> {}
```

Once Verus is upgraded, Ghost is Send+Sync, so our types' ghost fields no longer block
auto-derivation. The `unsafe impl` blocks become redundant and can be removed. Verus will
likely emit a warning or error about conflicting impls — the fix is simply to delete the
four `unsafe impl` lines (two in each file).

---

## The 77 New Commits

The upgrade spans 77 commits. Most are verita CI tooling and new-mut-ref fixes. The ones
most likely to affect APAS-VERUS:

- `16fa185f3` Ghost Send/Sync (the target fix)
- `07044c3ce` polymorphic SMT functions for decreases — could affect termination proofs
- `3390e9af0` Copy/Fn/FnMut/FnOnce as first-class traits — could affect closure specs
- `c1c43e849` adds `requires` to `is_disjoint` — may require call-site updates
- `ac8c94e28` PartialEq/Eq/PartialOrd/Ord specs for shared references — probably additive
- `e28bf825e` for-loop desugaring fix — could affect for-loop proofs
- `3ce5c2a76` complex return value patterns — additive, unlikely to break

The previous upgrade (to ff454ab0f, 32 commits) required 3 conjunction flakiness fixes
in Chap35/Chap36. Expect similar minor breakage here. Read errors carefully; do not
revert proof work.

---

## Steps

### Step 1 — Upgrade Verus

```bash
cd ~/projects/verus
git fetch origin
git checkout release/rolling/0.2026.04.10.fc697a7
cd source
vargo build --release
```

Confirm the build succeeds before proceeding.

### Step 2 — Full validate

```bash
cd ~/projects/APAS-VERUS
scripts/validate.sh
```

Read the full output. Fix all errors and trigger warnings before proceeding. Common
patterns from prior upgrades:
- Conjunction flakiness: use incremental ghost conjunction + explicit equivalence assert
  (see Z3 Conjunction Flakiness Workaround in memory)
- Trigger warnings: add explicit `#[trigger]` annotations
- `is_disjoint` call sites may need updated requires if the new vstd signature adds one

Do NOT revert proof work. Fix forward.

### Step 3 — Remove the 4 unsafe impl holes

In `src/Chap41/AVLTreeSetMtEph.rs`, lines 681-682:
```rust
// DELETE these two lines:
unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Send for AVLTreeSetMtEph<T> {}
unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Sync for AVLTreeSetMtEph<T> {}
```
Also delete the comment above them (line 680): `// Ghost fields are zero-sized; ...`

In `src/Chap41/AVLTreeSetMtPer.rs`, lines 739-740:
```rust
// DELETE these two lines:
unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Send for AVLTreeSetMtPer<T> {}
unsafe impl<T: StTInMtT + Ord + TotalOrder + 'static> Sync for AVLTreeSetMtPer<T> {}
```
Also delete the comment above them (line 738): `// Ghost fields are zero-sized; ...`

If Rust says Send/Sync cannot be auto-derived after deletion (because the struct still
has a field that is not Send/Sync), check what field blocks it and investigate. Do not
re-add unsafe impl without reporting to the user. The Ghost Send/Sync fix should be
sufficient — if it is not, read the Verus error carefully and report what field is
blocking derivation.

### Step 4 — Isolate validate Chap41

```bash
scripts/validate.sh isolate Chap41
```

Confirm Chap41 verifies clean with the unsafe impls removed.

### Step 5 — Full validate

```bash
scripts/validate.sh
```

Confirm the whole codebase is clean.

### Step 6 — RTT

```bash
scripts/rtt.sh
```

### Step 7 — Holes report

```bash
scripts/holes.sh src/Chap41/
```

Confirm AVLTreeSetMtEph and AVLTreeSetMtPer show 0 holes.

### Step 8 — Update memory

Update `~/projects/APAS-VERUS/plans/` and memory to reflect:
- Verus version upgraded to `release/rolling/0.2026.04.10.fc697a7`
- 4 unsafe impl holes closed
- New verified count and hole count

### Step 9 — Commit and push

```bash
git add -A
git commit -m "R202 Verus upgrade to 0.2026.04.10 + close 4 Send/Sync holes in Chap41"
git push origin agent1/<topic>
```

---

## Success Criteria

- Verus built from `release/rolling/0.2026.04.10.fc697a7`
- `scripts/validate.sh` clean (same or higher verified count)
- `scripts/rtt.sh` clean
- 0 holes in Chap41 (AVLTreeSetMtEph and AVLTreeSetMtPer)
- Total holes: 0 (down from 4)
- No unsafe impl Send/Sync remaining in the codebase

## Report

Write your report to `plans/agent1-r202-report.md`. Include:
- Verus version before/after
- Verified count before/after
- Holes before/after
- Any breakage found and how it was fixed
- RTT count
