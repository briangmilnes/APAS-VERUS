# Agent 3 — Round 9: Revert Accepts + Prove Chap38/39/41 StEph/StPer

## Mission

A previous agent violated project rules by converting assume() to accept() across
6 files. Your first job is to revert ALL those accepts back to their original form
(assume or external_body). Then do real proof work on the reverted holes.

**BACKGROUND**: accept() is an APAS proof function that replaces assume for
intentionally accepted holes. Converting assumes to accepts without explicit user
approval is forbidden (see CLAUDE.md). The previous agent did this to inflate hole
reduction numbers. You must undo it.

## Your Files (ONLY touch these)

Revert accepts then prove — Chap38:
1. `src/Chap38/BSTParaStEph.rs` — 20 accepts to revert (was 1 ext_body before)
2. `src/Chap38/BSTParaMtEph.rs` — 3 accepts to revert

Revert accepts then prove — Chap39:
3. `src/Chap39/BSTTreapMtEph.rs` — 14 accepts to revert
4. `src/Chap39/BSTSetTreapMtEph.rs` — 13 accepts to revert

Revert accepts then prove — Chap41:
5. `src/Chap41/AVLTreeSetStEph.rs` — 11 accepts to revert (were assumes)
6. `src/Chap41/AVLTreeSetStPer.rs` — 10 accepts to revert (were assumes)
7. `src/Chap41/ArraySetStEph.rs` — 3 assumes (may have accepts too)
8. `src/Chap41/ArraySetEnumMtEph.rs` — 1 ext_body
9. `src/Chap41/Example41_3.rs` — 4 ext_body

**DO NOT touch Chap41 MtEph/MtPer files (AVLTreeSetMtEph, AVLTreeSetMtPer).
Those are Agent 4's.**

## Step 1: Revert All Accepts

For each file with accepts added by the previous agent:

```bash
git diff HEAD~1..HEAD -- src/Chap38/BSTParaStEph.rs
```

This shows what the previous agent changed. For every `accept(...)` that replaced
an `assume(...)`, change it back to `assume(...)`. For every `accept(...)` that replaced
`external_body`, restore the `#[verifier::external_body]` annotation.

**Validate after each revert** to make sure you haven't broken anything:
```bash
scripts/validate.sh
```

The reverts should leave the verification count unchanged (accepts and assumes have
the same verification effect — the difference is semantic, not mechanical).

## Step 2: Real Proof Work

After reverting, you'll have the actual hole landscape:

### Chap38
- BSTParaStEph.rs: Should be back to ~1 ext_body. Prove it.
- BSTParaMtEph.rs: ~17 ext_body. These are parallel BST operations (split, join,
  union, intersection, difference). Use the Arc<RwLock> standard pattern.

### Chap39
- BSTTreapMtEph.rs: ext_body stubs for treap operations behind Arc<RwLock>.
  BSTTreapStEph.rs is clean — read it for the sequential algorithms and specs.
- BSTSetTreapMtEph.rs: thin set wrapper over BSTTreapMtEph.

### Chap41 StEph/StPer
- AVLTreeSetStEph.rs: ~17 assumes (mostly closure requires).
  Read `src/standards/using_closures_standard.rs` — lift `f.requires((x,))` into
  the function's own requires clause, then remove the assume.
- AVLTreeSetStPer.rs: ~12 assumes (same pattern).
- ArraySetStEph.rs: 3 assumes (feq workaround — may be standard eq/clone pattern).

**WARNING**: When you add requires to trait functions, you MUST update ALL callers.
Run `scripts/validate.sh` after every change. A previous agent caused a 9-verification
regression by adding requires without updating callers.

## Standards to Read First

1. `src/standards/using_closures_standard.rs` — closure requires/ensures pattern
2. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone workaround + feq propagation
3. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — MtEph pattern
4. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridges

## CRITICAL RULES

- Do NOT use accept() anywhere. Do NOT convert assumes to accepts.
- If you can't prove something, leave the assume and explain what blocks it.
- Run `scripts/validate.sh` after EVERY change.

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap38/ # track Chap38
scripts/holes.sh src/Chap39/ # track Chap39
scripts/holes.sh src/Chap41/ # track Chap41
```

## Target

**Reverts**: All ~71 accepts reverted. This is step 1, non-negotiable.
**Chap38**: Close BSTParaStEph. BSTParaMtEph: prove at least 5.
**Chap39**: Prove at least 5 ext_body in BSTTreapMtEph/BSTSetTreapMtEph.
**Chap41**: AVLTreeSetStEph ≤ 10 assumes. AVLTreeSetStPer ≤ 7 assumes.

## When Done

Push to `agent3/ready`. Write `plans/agent3-round9-report.md`.
Include a table showing: file, accepts reverted, holes before revert, holes after proof work.
