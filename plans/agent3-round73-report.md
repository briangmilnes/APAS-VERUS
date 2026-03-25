# R73 Agent 3 Report: Fix 4 Broken Mt Modules

## Task

Fix 4 Mt modules hidden behind `all_chapters` cfg that never cargo-compiled.
Uncomment in lib.rs, fix compilation errors, fix verification errors, pass RTT.

## Results

- Verification: **4543 verified, 0 errors**
- RTT: **2528 passed, 0 failed**
- Trigger warnings: **0** (fixed 6 auto-trigger notes in min_link/max_link)
- 95 harmless warnings (all `verus-related attribute has no effect` inside external_body `splay`)

## Files Fixed

| # | Chap | File | Lines | Status | Actionable Holes | Notes |
|---|------|------|-------|--------|-----------------|-------|
| 1 | 37 | BSTSplayMtEph.rs | ~1880 | Compiles + verifies | 8 holes + 2 warn | 7 external_body + 1 assume |
| 2 | 37 | BSTSetSplayMtEph.rs | ~540 | Compiles + verifies | 13 holes | 13 external_body (most blocked by iter/clone) |
| 3 | 37 | AVLTreeSeqMtPer.rs | ~953 | Compiles + verifies | 0 holes | 3 eq/clone workaround warnings only |
| 4 | 41 | AVLTreeSetMtPer.rs | ~520 | Compiles + verifies | 5 holes | 5 algorithmic assumes (cmp_spec/view_ord/size) |

All holes are pre-existing in the original code except the 5 assumes in AVLTreeSetMtPer.rs
which follow the standard RWLOCK_GHOST Layer 2 pattern (obeys_cmp_spec, view_ord_consistent,
size bounds).

## Techniques Used

1. **Lifetime cascading**: All 4 files needed `+ 'static` bounds on T (and F for closures)
   because Mt modules use Arc/RwLock/thread spawning. Required ~15 iterative fixes across
   trait declarations, impl blocks, free functions, Default, Display, Iterator, IntoIterator.

2. **Exec-mode borrow binding**: `handle.borrow()` cannot be called inside `proof { }` blocks.
   Fix: bind in exec mode (`let st: &Type = handle.borrow();`), then reference in proof block.

3. **PartialOrd external_body**: vstd requires `obeys_partial_cmp_spec()` postcondition.
   Added `#[verifier::external_body]` to `partial_cmp` (delegates to already-external `cmp`).

4. **cfg-gated spec import**: `view_ord_consistent` is spec-only (inside `verus!`), not
   available during cargo test. Gated with `#[cfg(verus_keep_ghost)]`.

5. **Explicit triggers**: Added `#[trigger] link_contains(...)` to 6 quantifiers in
   min_link/max_link to eliminate auto-trigger notes.

## Chapter Hole Counts (Post-R73)

| # | Chap | Actionable Holes | Warnings | Structural (info) |
|---|------|-----------------|----------|-------------------|
| 1 | 37 | 25 | 18 | 33 |
| 2 | 41 | 5 | 17 | 54 |

## Remaining Work in These Files

- **BSTSplayMtEph.rs**: The `splay` function is `external_body` (root cause). Removing it
  would eliminate 95 warnings and unblock 5 downstream external_body holes. The `splay`
  function has full proof code inside but Verus cannot verify it (complex tree rotation
  proofs). Two `fn_missing_requires` warnings on `size_link` and `update`.
- **BSTSetSplayMtEph.rs**: 13 external_body holes, most blocked by missing iter/into_iter/
  clone verification on the underlying BSTSplayMtEph. 2 root causes: `rebuild_from_vec`
  and `iter`.
- **AVLTreeSetMtPer.rs**: 5 assumes follow standard RWLOCK_GHOST pattern. The
  obeys_cmp_spec/view_ord_consistent assumes match AVLTreeSetMtEph's pattern exactly.
