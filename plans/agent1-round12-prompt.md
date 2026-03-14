# Agent 1 — Round 12 Prompt

## Mission

Continue Chap43 (135 holes). You have momentum — keep going. Also pick up Chap37
(5 holes) and Chap45 (3 holes) if time permits.

## Your Files

**Chap43** (135 holes across 10 files):
- `OrderedSetMtEph.rs` — 36 assume, 3 external_body
- `OrderedTableMtPer.rs` — 14 assume, 8 external_body
- `OrderedTableStEph.rs` — 14 external_body (you got -2 in R11)
- `OrderedTableMtEph.rs` — 15 external_body (you got -1 in R11)
- `OrderedSetStEph.rs` — 14 external_body, 1 assume (you got -1 in R11)
- `OrderedSetStPer.rs` — 12 external_body (you got -1 in R11)
- `OrderedTableStPer.rs` — 10 external_body
- `AugOrderedTableStEph.rs` — 3 external_body (you got -2 in R11)
- `AugOrderedTableMtEph.rs` — 5 external_body
- `AugOrderedTableStPer.rs` — 2 assume

**Chap37** (5 holes):
- `AVLTreeSeq.rs` — 1 external_body (iterator next, standard pattern)
- `AVLTreeSeqMtPer.rs` — 2 external_body (parallel build, stretch)
- `AVLTreeSeqStPer.rs` — 1 assume (slice indexing)
- `BSTSplayStEph.rs` — 1 trivial_wf

**Chap45** (3 holes):
- `BalancedTreePQ.rs` — 1 external
- `BinaryHeapPQ.rs` — 1 assume
- `Example45_2.rs` — 1 external (skip, Example file)

## Priority Order

1. **Chap43 OrderedSetMtEph.rs** (39 holes) — The 36 assumes are RwLock ghost-state
   sync. Agent 3 just proved a broadcast trigger pattern (`Pair_feq_trigger +
   group_Pair_axioms`) that eliminates feq assumes. Study what Agent 3 did in
   `Chap42/TableMtEph.rs` — the same technique may apply here.

2. **Chap43 OrderedTableMtPer.rs** (22 holes) — Same RwLock ghost-state pattern as
   OrderedSetMtEph. The 14 assumes should follow from the RwLock invariant.

3. **Chap43 OrderedTableStEph.rs** (14 holes) — Continue proving AVL tree bodies.
   You already proved singleton + delete. Try: first, last, get_range, split.

4. **Chap37** (5 holes) — Quick wins: iterator next (1), trivial wf (1), slice
   indexing (1). The 2 MtPer external_body are stretch goals.

5. **Chap45** (2 real holes) — Skip Example45_2. Try BalancedTreePQ external and
   BinaryHeapPQ assume.

## Specific Guidance

### RwLock Ghost-State Pattern (50 holes in OrderedSetMtEph + OrderedTableMtPer)

These assumes look like:
```
assume(self.ghost_locked_set@ == locked_val@)
assume(self@.finite())
```

Fix approach:
1. Read the `*Inv` struct's `inv` function
2. If the invariant captures ghost↔locked relationship, replace assume with assert
3. If the invariant is weak, strengthen it (add ghost field tracking)
4. Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`

### Agent 3's feq Broadcast Trick

Agent 3 created `Pair_feq_trigger` and `group_Pair_axioms` broadcast proofs to
eliminate feq/clone assumes. Check `src/Chap42/TableMtEph.rs` for the pattern.
If it works for Pair<K,V>, adapt it for the Chap43 ordered types.

## DO NOT TOUCH

- Chap41, Chap53 — Agent 4
- Chap42, Chap47 — Agent 2
- Chap38, Chap39 — Agent 3

## Rules

- Read standards before writing code.
- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent1/ready`. Write `plans/agent1-round12-report.md`.

## Targets

- Chap43: 135 → ≤ 110 (-25)
- Chap37: 5 → ≤ 2
- Chap45: 3 → ≤ 2
