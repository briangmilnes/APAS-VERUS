# Agent 1 — Round 9: Close Chap37 + Close Chap45

## Mission

Close Chap37 (15 holes) and Chap45 (4 holes). Chap37 now has clean external deps —
nothing blocks you. Closing Chap37 unblocks Chap41/42/43/45/52/53.

**Your success metric is holes eliminated and chapters closed.** Chap45 has 4 holes —
close it. Chap37 has 15 remaining holes across 12 files. Get it to ≤ 5.

## Your Files (ONLY touch these)

Chap37 — AVLTreeSeq files (carry-forward from Round 8):
1. `src/Chap37/AVLTreeSeq.rs` — 2 ext_body (iterator stubs)
2. `src/Chap37/AVLTreeSeqStEph.rs` — 1 assume + 2 ext_body
3. `src/Chap37/AVLTreeSeqStPer.rs` — 4 ext_body
4. `src/Chap37/AVLTreeSeqMtPer.rs` — 1 assume + 3 ext_body

Chap37 — BST MtEph files (picked up from Agent 2):
5. `src/Chap37/BSTRBMtEph.rs` — requires_true warnings (NO real holes, just weak specs)
6. `src/Chap37/BSTSplayMtEph.rs` — requires_true warnings
7. `src/Chap37/BSTSplayStEph.rs` — 1 ext_body
8. `src/Chap37/BSTSetAVLMtEph.rs` — requires_true warnings
9. `src/Chap37/BSTSetBBAlphaMtEph.rs` — requires_true warnings
10. `src/Chap37/BSTSetPlainMtEph.rs` — requires_true warnings
11. `src/Chap37/BSTSetRBMtEph.rs` — requires_true warnings
12. `src/Chap37/BSTSetSplayMtEph.rs` — requires_true warnings

Chap45:
13. `src/Chap45/BalancedTreePQ.rs` — 1 ext_body
14. `src/Chap45/BinaryHeapPQ.rs` — 1 assume + 2 external

**DO NOT touch files in any other chapter.**

## Key Guidance

### BST*MtEph requires_true Fix
The ❌ files (BSTRBMtEph, BSTSplayMtEph, BSTSet*MtEph) have NO proof holes — just
`requires true` warnings. The clean StEph counterparts (BSTRBStEph, BSTPlainStEph,
BSTBBAlphaStEph, BSTAVLStEph) have ZERO `requires true`. Copy the requires from the
StEph trait signatures into the MtEph versions. This is spec-matching work, not proof
work — should be fast.

### AVLTreeSeq* remaining ext_body
These are iterator stubs and from_vec/clone stubs. Read
`src/standards/collection_iterators_standard.rs` and clean iterator implementations
in `src/Chap18/ArraySeqStEph.rs` as reference.

### feq broadcast pattern (from Round 8)
Use `obeys_feq_clone::<T>()` in requires for internal helpers, assume once at entry.
See rule 5 in `src/standards/partial_eq_eq_clone_standard.rs`.

## Standards to Read First

1. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone + feq propagation (rule 5)
2. `src/standards/collection_iterators_standard.rs` — iterator pattern
3. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — MtEph/MtPer pattern

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap37/ # track Chap37
scripts/holes.sh src/Chap45/ # track Chap45
```

## Target

**Chap37**: 15 → ≤ 5. Close all BST*MtEph requires_true issues. Prove AVLTreeSeq* ext_body.
**Chap45**: Closed (0 holes).

## When Done

Push to `agent1/ready`. Write `plans/agent1-round9-report.md`.
