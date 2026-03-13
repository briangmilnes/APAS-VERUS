# Agent 2 — Round 8: Chap37 BST*MtEph — The MT BST Variants

## Mission

Prove the 5 BST MtEph variants and their 5 BSTSet wrappers. The StEph counterparts
(BSTAVLStEph, BSTBBAlphaStEph, BSTPlainStEph, BSTRBStEph) are already clean — the
algorithms are proven. Your job is to connect those proofs through the Arc<RwLock>
threading layer.

**Your success metric is holes eliminated.** Do not write "fundamental blocker" or
"infrastructure limitation" — those are excuses. The StEph files prove these exact
algorithms. The MtEph files add threading. Read the standards, use the bridge functions,
prove the threading layer. If you get stuck on a specific lock predicate, say exactly
what the Verus error is and what you tried — not that it's hard.

## Your Files (ONLY touch these)

BST MtEph variants (5 files):
1. `src/Chap37/BSTAVLMtEph.rs`
2. `src/Chap37/BSTBBAlphaMtEph.rs`
3. `src/Chap37/BSTPlainMtEph.rs`
4. `src/Chap37/BSTRBMtEph.rs`
5. `src/Chap37/BSTSplayMtEph.rs`

BSTSet wrappers (5 files):
6. `src/Chap37/BSTSetAVLMtEph.rs`
7. `src/Chap37/BSTSetBBAlphaMtEph.rs`
8. `src/Chap37/BSTSetPlainMtEph.rs`
9. `src/Chap37/BSTSetRBMtEph.rs`
10. `src/Chap37/BSTSetSplayMtEph.rs`

**DO NOT touch AVLTreeSeq*, BST*StEph files, or any files outside Chap37. Agent 1 owns
the AVLTreeSeq files.**

## Strategy

### Read Standards First
1. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — THE pattern for MtEph.
   This shows exactly how to wrap a StEph type in Arc<RwLock>, define the RwLockPredicate,
   and delegate operations through lock acquire/release.
2. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridge functions.
3. `src/vstdplus/arc_rwlock.rs` — `new_arc_rwlock` and `clone_arc_rwlock`. Use these
   centralized bridges, do NOT write type-specific external_body helpers.

### The Pattern
Each BST*MtEph file should follow this structure:
- Struct wraps `Arc<RwLock<BST*StEph>>` (or the inner BST type).
- RwLockPredicate (named `BST*MtEphInv`) carries a real invariant — not `true`.
- Each trait method: acquire lock → delegate to StEph method → release lock.
- The StEph method's ensures give you the proof obligations for the MtEph ensures.

### BSTSet* Wrappers
These are thin wrappers: `BSTSetAVLMtEph` wraps `BSTAVLMtEph` and exposes set operations.
Once the BST*MtEph files are proven, these should follow with minimal work.

### Reference Clean Mt Files
Look at existing clean MtEph files for the exact pattern:
- `src/Chap06/DirGraphMtEph.rs` — clean MtEph graph with Arc<RwLock>
- `src/Chap18/ArraySeqMtEph.rs` — clean MtEph sequence
- `src/Chap26/DivConReduceMtPer.rs` — clean Mt reduce

### Common Hole Types in These Files
- **external_body on thread spawn**: Use `join()` from HFScheduler for fork-join.
  Read `src/standards/arc_rwlock_for_hfscheduler_standard.rs`.
- **assume on lock value linking**: The lock predicate should carry enough ghost state
  to establish the connection. Don't assume — prove via the predicate's `inv`.
- **requires_true**: Add real requires. Copy from the StEph trait's requires/ensures.

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap37/ # track hole reduction
```

## Target

Close at least 3 of the 5 BST*MtEph files. Get BSTSet* wrappers to follow.
Eliminate Chap37's internal dependency blockage (the 5 MtEph files blocking themselves).

## When Done

Push to `agent2/ready`. Write `plans/agent2-round8-report.md` with:
- Holes before/after per file (table)
- Verification counts
- Techniques used
- Remaining holes with what blocks them
- Commit hash
