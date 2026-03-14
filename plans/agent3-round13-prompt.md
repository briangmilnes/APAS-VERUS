# Agent 3 — Round 13 Prompt

## Mission

Apply your feq broadcast proof trick everywhere it can go. Eliminate feq assumes
in Chap41 (17 holes) and Chap38 (15 holes). You proved the technique works in
Round 11 — now scale it. Target: -15 holes.

## Your Files

**Chap41** (24 holes across 2 files):
- `ArraySetStEph.rs` — 3 assume (feq) — HIGHEST PRIORITY, unblocks Chap42
- `AVLTreeSetStEph.rs` — 14 assume (feq + set ops)
- `AVLTreeSetStPer.rs` — 10 assume (same patterns as StEph)

**Chap38** (15 holes):
- `BSTParaStEph.rs` — 15 assume (set ops, you got -4 in R10)

## Step 1: ArraySetStEph.rs (3 holes) — DO THIS FIRST

The 3 assumes are `assume(obeys_feq_full::<T>())`. Create a broadcast proof
that makes this provable from `T: StT` bounds, same as your Pair_feq_trigger.
If the broadcast approach doesn't fit, add `requires obeys_feq_full::<T>()` to
empty/singleton/find and cascade to callers IN YOUR FILES. Document callers in
Chap42/43 for other agents.

## Step 2: AVLTreeSetStEph.rs (14 holes)

Same feq technique. Also try:
- filter subset: loop invariant + intermediate assertions
- intersection/union/difference: set algebra lemmas from vstd::set_lib
- insert/delete: chain from inner AVLTreeSeq ensures

## Step 3: AVLTreeSetStPer.rs (10 holes)

Mirror of StEph. Same patterns apply.

## Step 4: BSTParaStEph.rs (15 holes)

Continue from R10. Try broadcast proof for the 11 set op assumes.

## DO NOT TOUCH

- Chap43 — Agents 1 and 2
- Chap42 — Agent 4
- Chap41 Mt files — Agent 4
- Chap39 — Agent 4

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent3/ready`. Write `plans/agent3-round13-report.md`.
- **Prove or move on.** Don't spend more than 10 minutes on any single hole.

## Target: ArraySetStEph 3 → 0. AVLTreeSetStEph 14 → ≤ 8. AVLTreeSetStPer 10 → ≤ 6. BSTParaStEph 15 → ≤ 10. Total -15.
