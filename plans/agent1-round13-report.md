# Agent 1 Round 13 Report

## Mission

Prove Mt holes in Chap43 by strengthening RwLock invariants, replacing assumes with
type_invariant + use_type_invariant, and proving external_body wrappers.

## Results Summary

| # | Metric | Value |
|---|--------|-------|
| 1 | Verified functions | 4006 |
| 2 | Errors | 0 |
| 3 | Chap43 Mt holes before | 81 |
| 4 | Chap43 Mt holes after | 46 |
| 5 | Holes eliminated | -35 |

## Per-File Results

| # | Chap | File | Before | After | Delta | Techniques |
|---|------|------|--------|-------|-------|------------|
| 1 | 43 | OrderedSetMtEph.rs | 39 | 23 | -16 | type_invariant, use_type_invariant, borrow-from-lock |
| 2 | 43 | OrderedTableMtPer.rs | 21 | 10 | -11 | type_invariant, use_type_invariant, inv-proves-wf |
| 3 | 43 | OrderedTableMtEph.rs | 15 | 11 | -4 | clone proved, collect ensures strengthened, first/last/select_key proved |
| 4 | 43 | AugOrderedTableMtEph.rs | 5 | 2 | -3 | clone proved, recalculate_reduction proved, join_key rewritten |
| 5 | 43 | Total Mt | 80 | 46 | -34 | |

Note: Starting count 80 differs from prompt's 61 because prompt counted only two primary
files. OrderedTableMtPer had 21 holes (not 22; the 3 accepts are info not errors).

## Techniques Used

### type_invariant + use_type_invariant (OrderedSetMtEph, OrderedTableMtPer)
Added `#[verifier::type_invariant]` inherent impl block with `self.ghost_locked_set@.finite()`
(or `.dom().finite()` for tables). Changed View from `closed spec fn` to `open spec fn`
calling a `pub closed spec fn` accessor. Added `proof { use_type_invariant(self); }` to all
`&self` methods that need finiteness. Eliminated 8 finite assumes per file.

Key discovery: `use_type_invariant` only works on `&self`, not `&mut self`. For `&mut self`
methods, finiteness is proven from the inner operation's ensures chain.

### RwLock inv proves wf (OrderedTableMtPer size, empty)
After `acquire_read`, the RwLock inv guarantees `inner.spec_orderedtablestper_wf()`. Removed
the redundant `assume(inner.wf)` in `size()`. For `empty()`, StPer::empty() ensures wf
directly — removed that assume too.

### Borrow-from-lock pattern (OrderedSetMtEph intersection/union/difference)
Instead of cloning `other` from the lock (which loses wf), keep the read handle open and
pass `other_read.borrow()` directly to the StEph method. Eliminates `assume(other_inner.wf)`.

### Strengthened collect ensures (OrderedTableMtEph)
Added `collected.spec_avltreeseqstper_wf()` to collect's ensures (trait + impl). This
enabled removing external_body from first_key, last_key, select_key — their bodies call
collect() then length()/nth() which require wf.

### Rewritten join_key (AugOrderedTableMtEph)
Original body called `size()` (requires wf not in scope). Rewrote to:
`self.base_table.join_key(other.base_table); self.cached_reduction = recalculate_reduction(self);`
Avoids wf requirement entirely. Simpler and verified.

## Remaining Holes (46 across 4 Mt files)

### OrderedSetMtEph (23 holes)
- 7x ghost-to-inner bridge assumes (`self.ghost_locked_set@ == locked_val@`) — structural
- 3x operation result bridges (`self@ == old(self)@.intersect/union/difference(other@)`) — structural
- 2x lock-boundary bridges (size count, find result) — structural
- 4x wf on split/split_rank parts — StEph split doesn't ensure wf on parts
- 2x wf for release_write after split/split_rank — split mutates locked_val, wf not ensured
- 1x other_inner wf in join — clone doesn't carry wf
- 1x range wf in get_range — StEph get_range doesn't ensure wf
- 3x external_body (filter, to_seq, from_seq)

### OrderedTableMtPer (10 holes)
- 1x inner wf in from_st_table — callers (split_key, etc.) can't prove wf
- 1x count bridge in size — structural
- 8x external_body (singleton, find, insert, delete, domain, map, filter, join_key) — removing would add feq assumes (net worse)

### OrderedTableMtEph (11 holes)
- 11x external_body (map, filter, reduce, collect, previous_key, next_key, split_key, get_key_range, rank_key, split_rank_key, from_sorted_entries) — bodies use for-loops with Vec/break/rev that need loop invariants or closure requires

### AugOrderedTableMtEph (2 holes)
- 2x external_body (calculate_reduction, reduce_range_parallel) — closure requires + ParaPair thread boundary

## What Would Unblock More

1. **StEph split/split_rank ensuring wf on parts** — would eliminate 4 assumes in OrderedSetMtEph
2. **StEph get_range ensuring wf** — would eliminate 1 assume in OrderedSetMtEph
3. **Proving for-loop bodies with Vec construction** — would enable proving OrderedTableMtEph collect, rank_key, etc.
4. **feq/obeys_view_eq implied by MtKey bounds** — would enable proving OrderedTableMtPer external_body wrappers

## Commit

Committed on branch `agent1/ready`.
