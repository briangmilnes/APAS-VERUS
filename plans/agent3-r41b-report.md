# Agent 3 — Round 41b Report

## Assignment

Chap41 AVLTreeSetMtPer delegation wrappers: restructure MtPer to delegate
through `Arc<RwLock<AVLTreeSetStPer<T>, AVLTreeSetMtPerInv>>` to inner
AVLTreeSetStPer, removing all 7 `external_body` methods.

## Results

### Verification

- **Before**: 4286 verified, 187 holes, 215 clean modules
- **After**: 4286 verified (+0), 180 holes (-7), 0 errors
- **RTT**: 2612 tests pass, 1 skipped
- **Clean modules**: 216 (84%, +1)

### Chap41 Hole Summary

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 41 | AVLTreeSetMtPer.rs | 7 | 0 | -7 |
| 2 | 41 | AVLTreeSetStEph.rs | 2 | 2 | 0 |
| 3 | 41 | Example41_3.rs | 4 | 4 | 0 (skip) |

**Chap41 total**: 13 → 6 (-7 actionable holes)
**Real (excl Example)**: 9 → 2 (-7)

## Changes Made

### AVLTreeSetMtPer.rs — 7 external_body → 0 holes

**Structural refactoring**: Replaced `AVLTreeSeqMtPerS<T>` backing store with
`Arc<RwLock<AVLTreeSetStPer<T>, AVLTreeSetMtPerInv>>` + ghost shadow. Same
architecture as AVLTreeSetMtEph.

**New type layout:**
```rust
pub struct AVLTreeSetMtPerInv;
pub struct AVLTreeSetMtPer<T: StTInMtT + Ord + 'static> {
    pub locked_set: Arc<RwLock<AVLTreeSetStPer<T>, AVLTreeSetMtPerInv>>,
    pub ghost_set_view: Ghost<Set<<T as View>::V>>,
}
```

**Lock invariant**: `v.spec_avltreesetstper_wf()` — ensures the inner StPer is
well-formed on every lock release.

**Method conversion:**

| # | Chap | Method | Pattern | Assumes |
|---|------|--------|---------|---------|
| 1 | 41 | from_seq | Extract Vec via values_in_order, build StPer iteratively via insert loop, wrap | 0 |
| 2 | 41 | filter | Acquire read, call inner.filter(), release, wrap | 3 (subset_of + 2 membership) |
| 3 | 41 | intersection | Acquire 2 reads, call inner.intersection(), release both, wrap | 1 (set equality) |
| 4 | 41 | difference | Acquire 2 reads, call inner.difference(), release both, wrap | 1 (set equality) |
| 5 | 41 | union | Acquire 2 reads, call inner.union(), release both, wrap | 1 (set equality) |
| 6 | 41 | delete | Acquire read, call inner.delete(), release, wrap | 1 (set equality) |
| 7 | 41 | insert | Acquire read, call inner.insert(), release, wrap | 1 (set equality) |

All assumes classified by veracity as `structural_false_positive RWLOCK_GHOST`
(not actionable holes). The reader accept pattern bridges the inner StPer view
to the MtPer ghost shadow.

**Additional methods rewritten (were already verified, needed adaptation for new struct):**

| # | Chap | Method | Notes |
|---|------|--------|-------|
| 1 | 41 | size | Delegation with 1 reader accept assume |
| 2 | 41 | to_seq | Delegation + values_in_order + MtPerS::from_vec; 1 assume + 1 proved forall |
| 3 | 41 | empty | Direct StPer::empty construction, 0 assumes |
| 4 | 41 | singleton | Direct StPer::singleton construction, 0 assumes |
| 5 | 41 | find | Delegation with 1 reader accept assume |
| 6 | 41 | Clone::clone | clone_arc_rwlock + ghost copy, 0 assumes |
| 7 | 41 | PartialEq::eq | Delegation to StPer's PartialEq, 1 assume (eq_clone_workaround) |
| 8 | 41 | Ord::cmp | external_body retained (std trait, not algorithmic) |
| 9 | 41 | Default::default | Delegates to empty() |

**Key proof insight**: The `to_seq` forall postcondition
(`forall|i| 0 <= i < seq@.len() ==> self@.contains(seq@[i])`) was proved
from the `to_set` assume rather than assumed separately. The chain:
`seq@[i]` is in `seq@` → in `seq@.to_set()` → in `self@` (via to_set =~= self@).

**Removed**: `use crate::ParaPair`, `SEQUENTIAL_CUTOFF` constant, `feq` imports,
all parallel divide-and-conquer code.

**Added**: `AVLTreeSetMtPerInv` struct with Debug/Display, `Send`/`Sync` unsafe
impls, RwLockPredicate impl.

## Techniques Used

- **RwLock delegation**: acquire_read → call inner StPer → release → wrap in
  new MtPer with new_arc_rwlock. Inner StPer's ensures carry the proof;
  only the ghost shadow bridge needs assumes.
- **Reader accept pattern**: Single assume per method bridging inner@ to self@.
  Classified as structural false positive by veracity (not actionable holes).
- **Lock invariant assertion**: `assert(AVLTreeSetMtPerInv.inv(result_st))`
  after each StPer method call — proven automatically from StPer's wf ensures.
- **Iterative construction for from_seq**: StPer::empty → insert loop → wrap.
  No assumes needed; StPer maintains wf through insert chain.
- **to_set membership proof**: Eliminated quantified assume by proving forall
  from the to_set extensional equality assume.
