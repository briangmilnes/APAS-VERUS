# Agent 4 — Round 25 Report

## Mission
- Prove 4 remaining holes in `src/Chap42/TableMtEph.rs` (tabulate, map, filter, insert)
- Assess Chap58/59 holes
- Fix Clone derive warning in DijkstraStEphI64/F64

## Results Summary

| Metric | Value |
|---|---|
| Verified | 4087 |
| Errors | 0 |
| RTT | 2613 pass |
| PTT | 147 pass |
| Total holes | 216 (was 220, -4 net) |

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 42 | TableMtEph.rs | 4 | 0 | -4 |
| 2 | 43 | OrderedTableMtEph.rs | 7 | 7 | 0 |
| 3 | 43 | AugOrderedTableMtEph.rs | 8 | 8 | 0 |
| 4 | 57 | DijkstraStEphI64.rs | 0 | 0 | 0 |
| 5 | 57 | DijkstraStEphF64.rs | 0 | 0 | 0 |

## Chapters Closed

**Chap42 — Table**: 4 → 0 holes. All 4 modules clean (100%).

## What Was Done

### 1. TableMtEph.rs — 4 holes removed (tabulate, map, filter, insert)

All four `external_body` functions replaced with verified implementations using sequential
loops over the backing `ArraySeqMtEphS<Pair<K,V>>`. Each follows the same pattern:

- **tabulate**: Iterates over `key_seq` from `keys.to_seq()`, calls `f(&key)` for each,
  builds `Vec<Pair<K,V>>`, constructs new table via `from_vec`. Proof uses
  `lemma_entries_to_map_dom_same_keys` and `lemma_entries_to_map_contains_key`.

- **map**: Iterates over entries, calls `f(&value)` for each, clones key via `clone_plus()`,
  builds new entries. Proof preserves domain (same keys) and no-dups.

- **filter**: Iterates over entries with ghost `sources: Seq<int>` tracking provenance.
  Four post-loop proof blocks: subset, no-dups, value preservation, completeness.
  Requires `spec_pred` ghost parameter matching closure behavior.

- **insert**: 2-phase design preserving entry order:
  - Phase 1: scan loop to find `match_index` (position of existing key, or n if not found)
  - Phase 2a (found): copy all entries, replace value at `match_index` with `combine(old, new)`
  - Phase 2b (not found): copy all entries, append `Pair(key, value)`
  - Order preservation avoids breaking `first_key`/`last_key`/`rank_key` in callers

### 2. Trait strengthening cascade

Added closure requires and type axiom preconditions to trait specs:

| Chap | File | Functions | New requires |
|---|---|---|---|
| 42 | TableMtEph.rs | tabulate | `keys.spec_arraysetsteph_wf()`, `f.requires`, `obeys_feq_full::<K>()` |
| 42 | TableMtEph.rs | map | `spec_tablemteph_wf()`, `f.requires`, `obeys_feq_clone::<K>()` |
| 42 | TableMtEph.rs | filter | `spec_tablemteph_wf()`, `f.requires`, `obeys_feq_full::<Pair<K,V>>()` |
| 42 | TableMtEph.rs | insert | `spec_tablemteph_wf()`, `combine.requires`, `obeys_view_eq::<K>()`, `obeys_feq_clone::<K>()`, `obeys_feq_full::<Pair<K,V>>()` |
| 43 | OrderedTableMtEph.rs | insert, tabulate | Propagated all new requires from TableMtEph |
| 43 | AugOrderedTableMtEph.rs | insert, tabulate | Propagated all new requires from OrderedTableMtEph |

### 3. OrderedTableMtEph runtime fixes

Insert's order-preserving design required fixing OrderedTableMtEph functions that previously
assumed sorted entry order:

- **first_key**: Changed from `collect().nth(0)` to scanning all entries for minimum key
- **last_key**: Changed to scanning all entries for maximum key
- **previous_key**: Changed to scanning all entries for max key < k
- **next_key**: Changed to scanning all entries for min key > k
- **rank_key**: Changed from early-termination scan to full scan counting keys < k
- **select_key**: Changed to collect+sort+index approach
- Fixed duplicate `#[verifier::external_body]` on `first_key`

### 4. Clone derive warning fix (DijkstraStEphI64/F64)

Replaced `#[derive(Clone, ...)]` with manual `Clone` impl per `partial_eq_eq_clone_standard.rs`:
- **DijkstraStEphI64.rs:50**: `PQEntry { dist: i64, vertex: usize }` — manual Clone with
  `ensures result@ == self@` (View = Self)
- **DijkstraStEphF64.rs:25**: `PQEntry { dist: WrappedF64, vertex: usize }` — manual Clone with
  `ensures result.dist == self.dist, result.vertex == self.vertex` (no View impl)

### 5. Chap58/59 Assessment

| # | Chap | File | Holes | Nature |
|---|------|------|-------|--------|
| 1 | 58 | BellmanFordStEphI64.rs | 2 | `external_body` on `neg_cycle_error_string()`, `algorithm_error_string()` |
| 2 | 59 | JohnsonStEphI64.rs | 1 | `external_body` on `neg_cycle_error_string()` |

All 3 holes are `"literal".to_string()` — Verus infrastructure limitation (String construction
from string literals). Not algorithmic logic. Cannot be proved until Verus adds String support.

## Key Techniques

- **2-phase insert for order preservation**: Scan for match first, then build result array
  with separate branches for found/not-found. Avoids skip+append pattern that changes entry
  ordering and breaks callers that depend on position.
- **Ghost sources for filter**: `Seq<int>` tracking which original indices were kept,
  enabling completeness proof (every original entry matching the predicate appears in result).
- **lemma_entries_to_map_dom_same_keys**: New proof lemma showing two sequences with
  identical keys at each position have the same map domain. Used by map and insert proofs.
- **clone_plus + obeys_feq_clone**: Key cloning in insert requires `obeys_feq_clone::<K>()`
  to ensure `clone_plus().view() == original.view()`.

## Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 42 | TableMtEph.rs | 4 external_body → verified; new lemma; strengthened trait specs |
| 2 | 43 | OrderedTableMtEph.rs | Strengthened insert/tabulate requires; fixed first/last/prev/next/rank/select |
| 3 | 43 | AugOrderedTableMtEph.rs | Strengthened insert/tabulate requires; added import |
| 4 | 57 | DijkstraStEphI64.rs | Manual Clone impl replacing derive |
| 5 | 57 | DijkstraStEphF64.rs | Manual Clone impl replacing derive |
