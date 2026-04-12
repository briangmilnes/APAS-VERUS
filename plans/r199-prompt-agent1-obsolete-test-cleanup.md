# R199 Prompt — Agent 1: Obsolete test cleanup + deadlock bug triage. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER modify `src/` to make a test pass.** The one exception in
   this round is #5 below (`OrderedSetMtEph::filter` deadlock), and
   only if the deadlock fix is genuinely minimal (uncommenting a
   `release_write` call or equivalent). Anything larger: STOP and
   report.

## Read all standards first.

## Context

Agent1 R198 audited 57 commented `[[test]]` entries and reactivated 32
of them (+273 live tests). 14 entries remain commented in four
categories that are *genuinely* blocked, not stale:

1. **`Weighed*` tests (8 entries, Chap06)** — import `apas_ai::...`
   (nonexistent crate) + target `Weighed*` and `*MtEph*` types that
   don't exist. Real src has `Weighted*StEph*` only.
2. **`TestBSTMtEph` (1 entry, Chap37)** — source
   `src/Chap37/BSTMtEph.rs` doesn't exist (chapter has
   BSTAVL/BBAlpha/Plain/RB/Splay MtEph variants, no plain BSTMtEph).
3. **`TestTSPApproxStEph` (1 entry, Chap64)** —
   `tests/Chap64/TestTSPApproxStEph.rs:11` genuinely uses
   `use ordered_float::OrderedFloat;`. The crate was removed; need
   migration.
4. **F64Dist tests (4 entries)** — `AllPairsResult*F64` /
   `SSSPResult*F64` — each uses `fn dist(v: f64) -> F64Dist { F64Dist { val: v } }`.
   `F64Dist` was renamed to `WrappedF64` in `src/vstdplus/float.rs:261`.
   Same shape (`{ val: f64 }`), same API — test-side rename only.

Plus R198 found a **real bug in `src/`**:

5. **Deadlock in `src/Chap43/OrderedSetMtEph.rs::filter`** — acquires
   write lock, `release_write` is commented out, next op deadlocks
   (60s SIGTERM). R198 removed 3 tests to sidestep.

## Goal

Clean up the obsolete test files (categories 1 & 2), migrate
TestTSPApproxStEph off `ordered_float` (category 3), fix the
OrderedSetMtEph deadlock (item 5), restore the 3 removed tests. Leave
F64Dist (category 4) — its blocker is a missing src type, not a test
problem.

## Plan

### Step 1: Investigate the `ordered_float` migration path

Read `src/vstdplus/float.rs` (contains `FloatTotalOrder`). That's the
replacement abstraction. Then read an in-tree usage — e.g.,
`src/Chap57/DijkstraStEphF64.rs` or any other file using
`FloatTotalOrder` with `f64`.

Then look at `tests/Chap64/TestTSPApproxStEph.rs` and the underlying
`src/Chap64/TSPApproxStEph.rs`. Two questions:

- Does `TSPApproxStEph` itself take `OrderedFloat<f64>` or does it
  take `f64` directly?
- If the source takes `OrderedFloat<f64>`, the migration is a src-side
  change (out of scope) — STOP on this item, leave the test
  commented.
- If the source takes `f64` or some `FloatTotalOrder`-bounded type,
  migrate the test to construct its inputs with that type instead of
  `OrderedFloat<f64>`.

### Step 2: Migrate or leave TestTSPApproxStEph

**If migrating is a test-only change** (src/ takes a generic bound
that `f64` already satisfies): rewrite the test to use plain `f64` or
`FloatTotalOrder`-wrapped types, uncomment in `Cargo.toml`, run
`cargo test --test TestTSPApproxStEph`.

**If migrating requires src/ changes**: STOP. Update the Cargo.toml
comment from `# uses ordered_float (removed crate)` to
`# needs TSPApproxStEph to migrate from OrderedFloat<f64> to FloatTotalOrder`.
Report it as a blocker.

### Step 3: Delete the `Weighed*` orphan tests

These 8 test files import `apas_ai` (nonexistent) and target types
that never existed (`Weighed*MtEph*`, spelled without the "t").
There's no salvage path — they reference a parallel universe of the
codebase that never shipped.

Action:
- Delete all 8 `tests/Chap06/TestWeighed*.rs` files.
- Delete the corresponding 8 commented `[[test]]` blocks in
  `Cargo.toml`.

**Do not** attempt to rewrite them to target the real
`Weighted*StEph*` types — those already have tests
(`TestWeightedDirGraphStEphF64.rs` etc.) that are active and
passing. The Weighed* files are redundant even conceptually.

### Step 4: Delete TestBSTMtEph

`tests/Chap37/TestBSTMtEph.rs` imports BSTAVLMtEph, BSTBBAlphaMtEph,
etc. — but there's no `src/Chap37/BSTMtEph.rs` to test.

Check: does the test add any coverage beyond what the existing per-
variant BST*MtEph tests already cover? Read it end to end.

- If it's purely a catch-all with no unique cases → **delete** the
  test file and the `Cargo.toml` entry.
- If it has distinctive tests (e.g., cross-variant comparisons) →
  leave it. Update the comment from `# module BSTMtEph does not
  exist` to something clearer like
  `# needs cross-variant BST test harness, deferred`.

Report whichever happened.

### Step 5: Fix the `OrderedSetMtEph::filter` deadlock

Open `src/Chap43/OrderedSetMtEph.rs`. Find the `filter` method.
Look for:
- A `release_write()` call that's commented out.
- A mismatched `acquire_write` without a matching release.

Context from R198 report:
> OrderedSetMtEph::filter acquires write lock but release_write is
> commented out in src/Chap43/OrderedSetMtEph.rs — lock never
> released, next operation deadlocks (SIGTERM after 60s).

The fix is one of:
- **Uncomment** the `release_write` call if it's just commented out
  (verify the code path was complete when commented).
- **Add** the missing `release_write` call if the code is structurally
  incomplete.

Run `scripts/validate.sh isolate Chap43` after the fix. If the
verification breaks — which it might, because the lock invariant
analysis is delicate — **STOP and report**. Do not try to re-prove
anything. Restore the commented-out state (leaving the deadlock) and
flag it for a separate proof round.

If verification passes, proceed.

### Step 6: Restore the 3 removed tests in `TestOrderedSetMtEph.rs`

R198 removed `test_filter`, `test_large_dataset_performance`,
`test_parallel_operations`. Those removals are visible in git history:

```bash
git log --all --diff-filter=D --source -- tests/Chap43/TestOrderedSetMtEph.rs
```

OR inspect the R198 commit (`5cf0371cf` — "R198 Agent 1: Cargo.toml
test-entry audit ..."):

```bash
git show 5cf0371cf -- tests/Chap43/TestOrderedSetMtEph.rs
```

Put the three tests back. They should now pass with the deadlock
fixed.

### Step 6.5: F64Dist → WrappedF64 rename (4 test files)

Tests in `tests/Chap56/` reference `F64Dist`, which was renamed to
`WrappedF64` at `src/vstdplus/float.rs:261`. Same shape and API.
Files to fix:

- `tests/Chap56/TestAllPairsResultStEphF64.rs`
- `tests/Chap56/TestAllPairsResultStPerF64.rs`
- `tests/Chap56/TestSSSPResultStEphF64.rs`
- `tests/Chap56/TestSSSPResultStPerF64.rs`

In each, rename `F64Dist` → `WrappedF64` (both in the helper
`fn dist(v: f64) -> F64Dist { F64Dist { val: v } }` and anywhere
else the identifier appears — grep to confirm, only 1 occurrence per
file per earlier audit).

Then uncomment the 4 corresponding `[[test]]` blocks in `Cargo.toml`
(currently labeled `# references non-existent F64Dist type`) and run:

```bash
cargo test --test TestAllPairsResultStEphF64
cargo test --test TestAllPairsResultStPerF64
cargo test --test TestSSSPResultStEphF64
cargo test --test TestSSSPResultStPerF64
```

All must pass. If they don't, the failure is a new bug report — do
not edit `src/` to force them.

### Step 7: Validate the whole thing

```bash
scripts/validate.sh
scripts/rtt.sh
scripts/ptt.sh
```

All must be clean. RTT count should be **higher** than 4,123 (gained
3 restored tests, lost 8 Weighed* + potentially 1 BSTMtEph = net ≈
−6 or better depending on Step 4 choice; plus any new tests revealed
by OrderedSetMtEph::filter working properly).

## Out of scope
- Writing NEW iterators, moving iterators into `verus!`, or touching
  PTTs — that's agent2's R199 iterator round.
- Agent3's Chap52 EdgeSetGraphMtEph work (already in agent3 branch,
  merging separately).
- Touching `src/` outside of `OrderedSetMtEph::filter`. The filter
  fix is the only sanctioned src/ edit this round.
- Adding new [[test]] entries, new iterators, or new algorithms.

## Report

Write `plans/agent1-round199-report.md` with:

- Step 1-2: TSPApprox migration outcome (migrated/blocked + reason).
- Step 3: list of deleted Weighed* files (should be 8 src + 8 Cargo
  entries).
- Step 4: deletion decision for TestBSTMtEph (delete/leave + reason).
- Step 5: OrderedSetMtEph deadlock fix — what was wrong, what
  changed, one-line diff summary.
- Step 6: 3 restored tests in TestOrderedSetMtEph.rs — run result.
- Step 6.5: F64Dist → WrappedF64 rename in 4 test files, 4 Cargo
  entries reactivated, test run results.
- Step 7: validate/rtt/ptt final numbers.
- Any new **bugs found** (prominent section).

## RCP

```
git add -A
git commit -m "R199 Agent 1: delete obsolete Weighed*/BSTMtEph tests; fix OrderedSetMtEph deadlock; restore 3 tests"
git push
```
