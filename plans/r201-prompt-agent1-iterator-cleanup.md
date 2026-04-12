# R201 Prompt — Agent 1: Iterator cleanup + AIR-bug verify + BSTTreap coverage. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`** —
   except the standard `assume(iter_invariant(self))` inside hand-rolled
   iterator `next()` bodies (documented project policy).
6. **NEVER modify `src/` to make a test pass.** Test-side fixes only.
7. **NEVER touch `Example*.rs` or `Problem*.rs` files.**

## Read all standards first.

Especially:
- `src/standards/iterators_standard.rs`
- `src/standards/wrapping_iterators_standard.rs`
- `docs/APAS-VERUSIterators.rs` (6 iteration test patterns)

## Context

Agent2 R199 added 19 iterators but only 4 PTT files. Plus three
loose ends from R199/R200:

1. **AIR-bug verification gap** (R199 Phase 1a not clearly reported):
   `src/Chap18/ArraySeqStPer.rs` previously had its `IntoIterator` impls
   *outside* `verus!` because of a Verus "ill-typed AIR" bug. Agent2
   added an iterator to this file but didn't explicitly report whether
   the IntoIterator is now inside `verus!` or still outside.
2. **PTT coverage gap** (R199 Phase 4): 15 of 19 new iterators
   have RTTs but no dedicated PTT file.
3. **BSTTreapStEph delete coverage gap** (R200 observation): the
   recursive-left-descent branch in `delete_link` at
   `src/Chap39/BSTTreapStEph.rs:1336` has no test reaching it. The
   R200 fix (restoring the recursion) was functionally invisible to
   current tests.

## Goal

Close all three in one cleanup round.

## Plan

### Phase 1: AIR-bug verification on Chap18/ArraySeqStPer

1. Open `src/Chap18/ArraySeqStPer.rs`.
2. Locate the `impl IntoIterator for &'a ArraySeqStPerS<T>` and
   `impl IntoIterator for ArraySeqStPerS<T>` blocks.
3. Determine whether they are **inside** `verus! { ... }` or **outside**
   (before or after the `} // verus!` closing brace).
4. Check whether the historical comment `"// IntoIterator impls moved
   outside verus! — Verus hits ill-typed AIR on
   proj%%core!iter.traits.collect.IntoIterator./Item for ArraySeqStPerS."`
   is still in the file.

**Three outcomes to handle:**

- **Already inside verus! with no legacy comment** → AIR bug was fixed
  upstream. Report "verified clean" and move on.
- **Inside verus! but legacy comment still there** → comment is stale.
  Remove the comment. Run `scripts/validate.sh isolate Chap18`. Report.
- **Outside verus!** → try moving inside. Run
  `scripts/validate.sh isolate Chap18`. If it verifies clean, keep the
  move and remove the legacy comment. If it fails with any AIR/IntoIterator
  error, **revert the move** and update the comment to mention the current
  Verus commit (`ff454ab0f`) against which the bug persists.

**Do not** change iterator behavior — only the block placement.

### Phase 2: BSTTreapStEph delete-recursion coverage

Add a test in `tests/Chap39/TestBSTTreapStEph.rs` (append, don't
replace) that exercises the recursive-left-descent branch in
`BSTTreapStEph::delete_link` at `src/Chap39/BSTTreapStEph.rs:1336`.

**Why this is needed**: R200 restored the line
`rotated.left = Self::delete_link(rotated.left.take(), target);` but no
existing test reaches that code path. The fix is currently cosmetic.

**How to exercise the branch**:
- Read `src/Chap39/BSTTreapStEph.rs` — specifically the `delete_link`
  function — to understand the conditions that steer the delete to go
  through the rotation-then-left-recurse branch. This typically
  requires:
  - Deletion of a value whose target is in the **left** subtree of a
    node whose rotation priority favors the right child (so the node
    rotates left, then the deletion recurses into the former-right's
    now-left subtree).
- Construct a treap whose priorities force this path.
- The test must: (a) build a treap with a specific shape, (b) delete
  an element whose position requires left recursion after rotation,
  (c) assert the resulting treap's `@.dom()` doesn't contain the
  deleted element and does contain everything else.

Name the test `test_delete_requires_left_recursion_after_rotation`
(or similar descriptive name).

If you cannot construct a deterministic treap that reaches this
branch (treap priorities are often hash-based), use a fixed-seed
rand approach that has been independently validated to hit the
branch (record the seed in a code comment).

### Phase 3: PTT backfill for R199 iterators

Agent2 R199 added iterators to 19 files but only 4 PTT files. The
4 covered files (from `rust_verify_test/tests/Chap*/`):

- `ProveParamBSTStEph.rs` (Chap38 → `BSTParaStEph`)
- `ProveBSTTreapMtEph.rs` (Chap39 → `BSTTreapMtEph`)
- `ProveAVLTreeSetStEph.rs` (Chap41 → `AVLTreeSetStEph`)
- `ProveAVLTreeSetStPer.rs` (Chap41 → `AVLTreeSetStPer`)

**Target list for this round — 15 R199 files needing PTT coverage:**

| # | Chap | File |
|---|---|---|
| 1 | 18 | ArraySeq (not ArraySeqStEph — the shared Chap18 module) |
| 2 | 18 | ArraySeqStPer |
| 3 | 18 | LinkedListStEph |
| 4 | 18 | LinkedListStPer |
| 5 | 37 | BSTSplayStEph |
| 6 | 39 | BSTTreapStEph |
| 7 | 39 | BSTSetTreapMtEph |
| 8 | 39 | BSTParaTreapMtEph |
| 9 | 40 | BSTSizeStEph |
| 10 | 40 | BSTKeyValueStEph |
| 11 | 40 | BSTReducedStEph |
| 12 | 41 | ArraySetStEph |
| 13 | 41 | AVLTreeSetMtPer |
| 14 | 42 | TableStEph |
| 15 | 42 | TableStPer |

**First**: verify the list. Each entry should have a 10-component
iterator in `src/Chap*/<File>.rs` with no matching
`rust_verify_test/tests/Chap*/Prove<Name>.rs`. If any entry already
has a PTT file, skip it and note in report.

**For each target file**:
- Create `rust_verify_test/tests/ChapNN/Prove<File>.rs` modeled on
  `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs` (the canonical
  pattern).
- Cover the 6 patterns (or as many as apply):
  1. loop-borrow-iter — `while let Some(x) = iter.next()` with `.iter()`
  2. loop-borrow-into — `while let Some(x) = iter.next()` with `(&foo).into_iter()`
  3. for-borrow-iter — `for x in foo.iter()`
  4. for-borrow-into — `for x in &foo`
  5. loop-consume — `while let Some(x) = iter.next()` with `foo.into_iter()`
  6. for-consume — `for x in foo`

Skip patterns 5+6 if the file doesn't implement `IntoIterator for Self`.
Use `// SKIPPED: <reason>` comment lines in the file to document.

- Register each new PTT in `rust_verify_test/Cargo.toml`.

**Write all PTTs first, run PTT once at the end** (per R199 prompt
— PTT compile overhead is ~4 min per run).

**If a specific iterator's consume pattern blows up PTT with
`BSTTreapMtEphLit!`-style issues** (HashMap-backed literals aren't
Verus-visible in PTT): construct instances via `new()` + `insert()`
instead, same as Chap39's pattern.

### Phase 4: Validation

```bash
scripts/validate.sh isolate Chap18   # Phase 1
scripts/validate.sh isolate Chap39   # Phase 2 + iterator PTTs
scripts/validate.sh                  # full
scripts/rtt.sh
scripts/ptt.sh                       # ONCE, at end
```

All must be clean. Expected deltas:
- validate: +0 (no new src spec fns — iterators already in R199)
- rtt: +~15–30 (the new BSTTreap test + any iterator RTTs added
  incidentally)
- ptt: **+~30–60** (15 new PTT files × 4–6 patterns each)

## Out of scope

- New iterators (agent2's R201 scope).
- Modifying any of R199's iterator implementations.
- Any `src/` edits except Phase 1's block-move (if the AIR bug is
  fixed) or removing a stale comment.
- PTTs for the 4 iterators that already have PTT files
  (ProveParamBSTStEph / ProveBSTTreapMtEph / ProveAVLTreeSetStEph /
  ProveAVLTreeSetStPer).

## Report

Write `plans/agent1-round201-report.md` with:

- **Phase 1 outcome**: AIR bug state (fixed / still broken / stale
  comment removed). Include the exact location (file:line) where
  the IntoIterator impl now lives.
- **Phase 2**: the new delete test, the specific treap shape it
  constructs, and confirmation that it exercises the R200-restored
  recursion branch (coverage check or manually traced).
- **Phase 3**: per-target PTT status table (file, patterns covered,
  patterns skipped with reason).
- **Phase 4**: validate / rtt / ptt numbers vs R200 baseline
  (5728 / 4208 / 237).
- Any veracity-surfaced issues (unlikely, but note if any).

## RCP

```
git add -A
git commit -m "R201 Agent 1: iterator PTT backfill (15 files) + AIR-bug verify + BSTTreap delete test"
git push
```
