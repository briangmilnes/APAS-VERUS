# Agent 2 Work Plan — BST + Collections (Chap37/39/40/41/42)

## Current Baseline

| # | Chap | Holes | assume | external_body | trivial_wf | Status |
|---|------|-------|--------|---------------|------------|--------|
| 1 | 37 | 47 | 7 | 39 | 1 | In progress |
| 2 | 39 | 38 | 0 | 38 | 0 | Not started |
| 3 | 40 | 0 | — | — | — | Clean |
| 4 | 41 | 90 | 56 | 33 | 1 | Not started |
| 5 | 42 | 19 | 3 | 16 | 0 | Not started |
| | **Total** | **194** | **66** | **126** | **2** | |

## File-by-File Breakdown

### Chap37 (47 holes, 19 files, 4 clean)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | AVLTreeSeq.rs | 3 | 1 assume, 2 ext_body | Prove assume; iterator+clone stay |
| 2 | AVLTreeSeqMtPer.rs | 14 | 14 ext_body | Coarse RwLock migration |
| 3 | AVLTreeSeqStEph.rs | 9 | mixed | Prove — real proof work |
| 4 | AVLTreeSeqStPer.rs | 14 | mixed | Prove — real proof work |
| 5 | BSTAVLMtEph.rs | 0 | — | Done (has accepts) |
| 6 | BSTBBAlphaMtEph.rs | 0 | — | Done (has accepts) |
| 7 | BSTPlainMtEph.rs | 0 | — | Done (has accepts) |
| 8 | BSTRBMtEph.rs | 0 | — | Done (has accepts, fn_missing warns) |
| 9 | BSTSplayMtEph.rs | 0 | — | Done (has accepts, fn_missing warns) |
| 10 | BSTSetAVLMtEph.rs | 1 | ext_body iter | Iterator stays external_body |
| 11 | BSTSetBBAlphaMtEph.rs | 1 | ext_body iter | Iterator stays external_body |
| 12 | BSTSetPlainMtEph.rs | 1 | ext_body iter | Iterator stays external_body |
| 13 | BSTSetRBMtEph.rs | 1 | ext_body iter | Iterator stays external_body |
| 14 | BSTSetSplayMtEph.rs | 1 | ext_body iter | Iterator stays external_body |
| 15 | BSTSplayStEph.rs | 2 | 1 ext_body clone, 1 trivial wf | Write real wf; clone TBD |

Note: BSTSet*MtEph iterator external_body (5 files, 5 holes) — these are wrapping
iterators per the standard. They delegate to the underlying BST iterator. The
external_body on next() is expected. These can become accepts if the wrapping
iterator standard supports it.

### Chap39 (38 holes, 4 files, 1 clean)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | BSTParaTreapMtEph.rs | 18 | 18 ext_body | EXCLUDED — concurrent BST, per-node locking |
| 2 | BSTTreapMtEph.rs | 10 | 10 ext_body | Arc→plain RwLock + standard |
| 3 | BSTSetTreapMtEph.rs | 10 | 10 ext_body | Coarse RwLock migration (after #2) |
| 4 | BSTTreapStEph.rs | 0 | — | Clean |

BSTParaTreapMtEph: 18 holes stay. Genuinely concurrent BST with fine-grained
per-node locking. Not a coarse-RwLock candidate.

### Chap41 (90 holes, 7 files, 0 clean)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | AVLTreeSetMtEph.rs | 17 | 16 ext_body, 1 trivial wf | Arc→plain RwLock + standard |
| 2 | AVLTreeSetMtPer.rs | 13 | 6 assume, 7 ext_body | Mixed: some accepts, some proofs |
| 3 | AVLTreeSetStEph.rs | 26 | 26 assume | Real proof work (hardest file) |
| 4 | AVLTreeSetStPer.rs | 14 | 14 assume | Real proof work |
| 5 | ArraySetEnumMtEph.rs | 7 | 1 assume, 6 ext_body | Coarse RwLock migration |
| 6 | ArraySetStEph.rs | 9 | 9 assume | Partial accept, partial proof |
| 7 | Example41_3.rs | 4 | 4 ext_body | Examples stay external_body |

#### Assume Classification for Chap41

The 56 assumes fall into categories:

**Potentially acceptable (Verus limitation, not algorithmic):**
- `assume(obeys_feq_clone::<T>())` — 1 in ArraySetStEph. Standard says accept.
- `assume(obeys_feq_full::<T>())` — 10 across files. NOT in standard. Needs decision.
- `assume(self.elements.spec_*_wf())` — ~12 across files. Wf propagation from inner
  collection. May be provable if inner collection maintains wf.
- `assume(result_vec@.len() < usize::MAX)` — capacity bounds, 2 instances. Provable.

**Real algorithmic proof obligations:**
- `assume(combined@ == self@.union(other@))` — set operation postconditions, ~5 instances
- `assume(self@ == old(self)@.insert(x_view))` — insert postcondition, ~4 instances
- `assume(self@ == old(self)@.remove(x@))` — delete postcondition, ~2 instances
- `assume(common@ == self@.intersect(other@))` — intersection postcondition, ~2 instances
- `assume(remaining@ == self@.difference(other@))` — difference postcondition, ~2 instances
- `assume(filtered@.subset_of(self@))` — filter postcondition, ~2 instances
- `assume(!self@.contains(x@))` — search completeness, ~3 instances
- `assume(f.requires((&*elem,)))` — predicate callability, ~2 instances

### Chap42 (19 holes, 4 files, 1 clean)

| # | File | Holes | Type | Action |
|---|------|-------|------|--------|
| 1 | TableMtEph.rs | 16 | 1 assume, 15 ext_body | Coarse RwLock migration |
| 2 | TableStEph.rs | 1 | 1 assume | Accept (obeys_feq_clone) |
| 3 | TableStPer.rs | 2 | 1 assume, 1 ext_body | Accept + investigate ext_body |
| 4 | Example42_1.rs | 0 | — | Clean |

## Work Phases

### Phase 1: Quick Wins — Accept Conversions (4 holes → 0)

Convert `assume(obeys_feq_clone::<...>())` to `accept()` per standard:
1. Chap41/ArraySetStEph.rs:423 — 1 hole
2. Chap42/TableMtEph.rs:809 — 1 hole
3. Chap42/TableStEph.rs:1652 — 1 hole (makes file clean!)
4. Chap42/TableStPer.rs:1727 — 1 hole

Expected reduction: 4 holes. TableStEph goes clean.

### Phase 2: Coarse RwLock Migration (65 ext_body → accepts)

Priority by ROI (holes eliminated per file):

| # | Chap | File | ext_body | Prereq | Effort |
|---|------|------|----------|--------|--------|
| 1 | 42 | TableMtEph.rs | 15 | None | Medium |
| 2 | 41 | AVLTreeSetMtEph.rs | 16 | None (Arc→plain) | High |
| 3 | 37 | AVLTreeSeqMtPer.rs | 14 | None | Medium |
| 4 | 39 | BSTTreapMtEph.rs | 10 | None (Arc→plain) | High |
| 5 | 39 | BSTSetTreapMtEph.rs | 10 | #4 done | Medium |
| 6 | 41 | ArraySetEnumMtEph.rs | 6 | None | Medium |

After migration, external_body methods become lock-acquire + exec-check + delegate
+ lock-release, with 3 accept categories per the standard.

Expected reduction: ~65 external_body holes become ~65 accepts (net -65 holes).

Note: AVLTreeSetMtEph.rs also has 1 trivial_wf to fix.

### Phase 3: Small Fixes (5 holes)

| # | File | Hole | Action |
|---|------|------|--------|
| 1 | Chap37/BSTSplayStEph.rs | trivial spec_wf | Write real spec_bstsplaysteph_wf |
| 2 | Chap37/BSTSplayStEph.rs | ext_body clone | Convert to accept per standard |
| 3 | Chap37/AVLTreeSeq.rs | assume in insert_at_link | Investigate — nat_max proof |
| 4 | Chap42/TableStPer.rs | ext_body collect_by_key | Investigate feasibility |
| 5 | Chap41/AVLTreeSetMtEph.rs | trivial spec_wf | Write real wf in migration |

### Phase 4: Hard Proof Work — Chap41 St Files (49 assumes)

This is the hardest work. The St files implement set operations (union, intersect,
difference, filter, insert, delete, find) on sorted sequences. Most assumes are
algorithmic postconditions that need loop invariant proofs.

Strategy: start with ArraySetStEph (simpler, flat array) then AVLTreeSetStEph
(harder, tree-backed), then AVLTreeSetStPer (persistent variant).

| # | File | Assumes | Key challenges |
|---|------|---------|----------------|
| 1 | ArraySetStEph.rs | 9 | 8× obeys_feq_full, 1× obeys_feq_clone (done in Phase 1) |
| 2 | AVLTreeSetStPer.rs | 14 | Set op postconditions, wf maintenance |
| 3 | AVLTreeSetStEph.rs | 26 | Same but mutable, wf propagation harder |
| 4 | AVLTreeSetMtPer.rs | 6 | Wf, feq, contains — depends on inner |

The `obeys_feq_full` assumes (10 total across files) need a decision: can these be
accepted as Verus limitations, or must they be proved? **Flag for user review.**

### Phase 5: Hard Proof Work — Chap37 AVLTreeSeq (23 holes)

AVLTreeSeqStEph.rs (9) and AVLTreeSeqStPer.rs (14) implement sequence operations
on AVL trees. These need investigation before planning.

## Excluded from Plan (not reducible)

| # | File | Holes | Reason |
|---|------|-------|--------|
| 1 | Chap39/BSTParaTreapMtEph.rs | 18 | Concurrent BST, fine-grained locking |
| 2 | Chap41/Example41_3.rs | 4 | Example functions, ext_body expected |
| 3 | Chap37/BSTSet*MtEph.rs (5) | 5 | Iterator next() ext_body, wrapping pattern |

Total excluded: 27 holes (stay as-is).

## Projected Outcome

| Phase | Holes Fixed | Running Total |
|-------|------------|---------------|
| Start | — | 194 |
| Phase 1: Accept conversions | -4 | 190 |
| Phase 2: RwLock migration | -65 | 125 |
| Phase 3: Small fixes | -5 | 120 |
| Phase 4: Chap41 St proofs | -49 (optimistic) | 71 |
| Phase 5: Chap37 AVLTreeSeq | -23 (optimistic) | 48 |
| Excluded (untouched) | — | 48 |

Realistic target: ~90-100 holes remaining (Phases 1-3 solid, Phases 4-5 partial).

## Decision Points for User

1. **obeys_feq_full:** 10 assumes across Chap41. Accept or prove?
2. **BSTSet*MtEph iterators:** 5 wrapping iterator external_body holes. Accept per
   wrapping_iterators_standard, or leave as external_body?
3. **BSTRBMtEph/BSTSplayMtEph fn_missing warnings:** These migrated files have 20+
   fn_missing_requires_ensures on Layer 1 helper functions. Add specs or leave?
4. **Phase order:** Start with Phase 1+2 (migration, high ROI) or Phase 4 (harder
   but higher proof value)?
