# Agent 4 — Round 8: Chap38 + Chap39 + Chap41 StEph/StPer

## Mission

Prove augmented BSTs (Chap38), treaps (Chap39), and the single-threaded set variants
in Chap41. Three independent attack fronts.

**Your success metric is holes eliminated.** Chap38 has 20 external_body holes with
clean deps — there is no blocker. Chap39 has 37 external_body with internal-only deps.
Chap41 StEph/StPer has 32 assumes that are mostly closure-requires and set-operation
correctness. Read the standards, use the patterns, prove.

## Your Files (ONLY touch these)

Chap38 (2 files, 20 holes — all external_body):
1. `src/Chap38/BSTParaMtEph.rs` — 19 ext_body
2. `src/Chap38/BSTParaStEph.rs` — 1 ext_body

Chap39 (4 files, 37 holes — all external_body):
3. `src/Chap39/BSTParaTreapMtEph.rs` — 18 ext_body
4. `src/Chap39/BSTSetTreapMtEph.rs` — 10 ext_body
5. `src/Chap39/BSTTreapMtEph.rs` — 9 ext_body
6. `src/Chap39/BSTTreapStEph.rs` — 0 holes (reference for StEph patterns)

Chap41 StEph/StPer only (3 files, 32 holes — mostly assumes):
7. `src/Chap41/ArraySetStEph.rs` — 3 assume (feq workaround)
8. `src/Chap41/AVLTreeSetStEph.rs` — 17 assume
9. `src/Chap41/AVLTreeSetStPer.rs` — 12 assume

**DO NOT touch Chap41 MtEph/MtPer files (AVLTreeSetMtEph, AVLTreeSetMtPer,
ArraySetEnumMtEph, Example41_3). Those depend on Chap37 work by Agents 1 and 2.**

## Strategy

### Chap38: Start Here (Quick Win)
- **BSTParaStEph.rs** has 1 ext_body. Start here — prove it, close the StEph file.
- **BSTParaMtEph.rs** has 19 ext_body. These are parallel BST operations (split, join,
  union, intersection, difference). The algorithms are proven in StEph; the MtEph
  version adds threading. Use the Arc<RwLock> standard pattern.

### Chap39: Treaps
- All 37 holes are external_body (algorithmic stubs).
- **BSTTreapStEph.rs** is clean — read it for the sequential algorithms and specs.
- BSTTreapMtEph.rs wraps it with threading. Similar to BST*MtEph pattern.
- BSTParaTreapMtEph.rs adds parallel treap operations (split/join with fork-join).
- BSTSetTreapMtEph.rs is a set wrapper over BSTTreapMtEph.
- Treaps use random priorities — the randomness itself doesn't need proving.
  You just need to prove the BST property is maintained and size/view invariants hold.

### Chap41 StEph/StPer: Closure Requires and Set Operations
- **ArraySetStEph.rs** (3 assumes): These are `obeys_feq_full` assumes in clone/eq.
  Read `src/standards/partial_eq_eq_clone_standard.rs` — these may be the standard
  eq/clone workaround pattern (acceptable holes).
- **AVLTreeSetStEph.rs** (17 assumes): These are in filter, intersect, union, difference.
  Most are `assume(f.requires(...))` — closure requires that should be lifted into the
  function's own `requires` clause. Read `src/standards/using_closures_standard.rs`.
  The fix: add `f.requires((x,))` to the function's requires, remove the assume.
  Then update callers to provide the closure spec.
- **AVLTreeSetStPer.rs** (12 assumes): Same pattern as StEph. Lift closure requires.

**WARNING about Chap41 closure fixes**: When you add requires to trait functions, you
MUST update ALL callers. In Round 7, an agent added requires without updating callers
and caused a 9-verification regression. `scripts/validate.sh` will catch this — run it
after every change.

## Standards to Read First

1. `src/standards/using_closures_standard.rs` — closure requires/ensures pattern
2. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone workaround
3. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — MtEph pattern
4. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridges

## Validation

```bash
scripts/validate.sh          # must show 0 errors — run after EVERY change
scripts/holes.sh src/Chap38/ # track Chap38 progress
scripts/holes.sh src/Chap39/ # track Chap39 progress
scripts/holes.sh src/Chap41/ # track Chap41 progress
```

## Target

**Chap38**: Close BSTParaStEph (1 → 0). BSTParaMtEph: 19 → ≤ 10.
**Chap39**: 37 → ≤ 25. At least BSTTreapMtEph and BSTSetTreapMtEph improved.
**Chap41**: ArraySetStEph stays at 3 (eq/clone pattern). AVLTreeSetStEph 17 → ≤ 8.
AVLTreeSetStPer 12 → ≤ 6.

## When Done

Push to `agent4/ready`. Write `plans/agent4-round8-report.md` with:
- Holes before/after per file (table)
- Chapters closed
- Verification counts
- Techniques used
- Remaining holes with what blocks them
- Commit hash
