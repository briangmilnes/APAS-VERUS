# Agent 4 Round 163 Report

## Task: R163 Prompt D — Compress Proof Lines

Target functions: compress ugly/repetitive proof blocks in Chap43, Chap45, Chap55.

---

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 43   | OrderedTableStEph.rs | 0 | 0 | 0 |
| 2 | 45   | SortedListPQ.rs | 0 | 0 | 0 |
| 3 | 55   | CycleDetectStEph.rs | 0 | 0 | 0 |
| 4 | 55   | TopoSortStEph.rs | 0 | 0 | 0 |

No holes were introduced or removed — all chapters already clean. Compression
targeted proof *length* (lines), not proof *completeness* (holes).

---

## Line Savings (git diff --stat HEAD)

| # | Chap | File | Lines Before | Lines After | Saved |
|---|------|------|-------------|-------------|-------|
| 1 | 45   | SortedListPQ.rs | ~237 (meld) | ~145 (meld) | ~92 |
| 2 | 55   | TopoSortStEph.rs | ~223 (dfs) | ~187 (dfs) | ~36 |
| 3 | 55   | CycleDetectStEph.rs | ~328 (dfs) | ~276 (dfs) | ~52 |
| 4 | 43   | OrderedTableStEph.rs | — | — | 0 (already #[cfg(never)]) |

**Total: ~180 lines removed across 3 files** (git diff: +105 / -217 = net -112).

---

## Chapters Closed

No chapters were opened or closed. All target chapters remain 0-hole clean.

---

## Verification Counts

- **Verified**: 5744 (up from previous round)
- **RTT**: 3776 passed, 0 failed
- **PTT**: 221 passed, 0 failed

---

## Techniques Used

### 1. `lemma_bool_array_set_view` (Chap55)

Extracted a new `pub proof fn` in `TopoSortStEph.rs` Section 7 that collapses the
11-line forall bridge (`a@ =~= old_view.update(vertex, val)`) into 2 lines.
Root cause of the verbosity: `ArraySeqStEphS::set` ensures only `spec_index` changes,
not the view directly. The bridge lemma connects spec_index to the view in one call.

Applied at 2 sites in `TopoSortStEph.rs::dfs_finish_order` and 3 sites in
`CycleDetectStEph.rs::dfs_check_cycle`.

**Key insight**: Must call `lemma_bool_view_eq_spec_index(a)` BEFORE `a.set(...)` to
pre-establish the `a@[j] == a.spec_index(j)` quantifier in Z3's context. The lemma
then uses `old(a)` state in post-mutation proofs.

Also made `lemma_bool_view_eq_spec_index` `pub` in TopoSortStEph so CycleDetectStEph
could import and reuse it, removing a duplicate local copy.

### 2. `lemma_append_push_bridge` (Chap45)

Added a new `proof fn` in `SortedListPQ.rs` Section 7 (50 lines) that collapses the
18-line dual-forall bridge in each `meld` loop iteration into ~4 lines.
Root cause: `ArraySeqStPerS::append` ensures only `spec_index` equality, not
`seq@ =~= old_seq.push(elem)` directly. The bridge proves both the `seq@` push and
the `@` (view) push from `spec_index` witnesses.

Applied at all 4 call sites in `meld`:
- Less/Equal branch in the merged while loop
- Greater branch in the merged while loop
- while i<n cleanup loop
- while j<m cleanup loop

### 3. Chap43 — No work needed

`union_bypassed_r158` in `OrderedTableStEph.rs` is already dead code under
`#[cfg(never)]`. Confirmed and skipped.

---

## Remaining Long Functions (not compressed this round)

| # | Chap | File | Function | Lines | Reason not compressed |
|---|------|------|----------|-------|----------------------|
| 1 | 45   | SortedListPQ.rs | `insert` | ~138 | Single non-duplicated block; high risk |
| 2 | 55   | CycleDetectStPer.rs | `dfs_check_cycle` | ~212 | StPer uses Vec, different proof structure |
| 3 | 55   | TopoSortStPer.rs | `dfs_finish_order` | ~193 | StPer uses Vec, different proof structure |

StPer files use `Vec<bool>` which already has `a@ =~= old(a)@.update(i, v)` from
Verus's Vec::set ensures — the verbosity problem is StEph-specific. StPer proofs
are already as short as they can reasonably be.

---

## Pre-existing Errors (not introduced by this work)

- `src/Chap37/AVLTreeSeqMtPer.rs:389 rotate_right` — rlimit exceeded (pre-existing)
- `src/Chap37/BSTSplayMtEph.rs` — sporadic Z3 conjunction flakiness (pre-existing)
