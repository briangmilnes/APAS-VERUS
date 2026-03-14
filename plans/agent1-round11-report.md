# Agent 1 — Round 11 Report

## Summary

**PRIMARY**: Chap43 (Ordered Tables and Sets) — largest remaining chapter.
**SECONDARY**: Chap37 and Chap45 investigated, no actionable holes found.

**Baseline**: 3986 verified, 0 errors, 289 total holes (post-rebase with Agent 4 merge)
**Final**: 3993 verified, 0 errors, 287 total holes
**Net**: +7 verified, -2 total holes (Chap43: 143→135, -8 holes; recount adjustments elsewhere)

## Holes Before/After Per File

| # | Chap | File | Before | After | Delta | Technique |
|---|------|------|--------|-------|-------|-----------|
| 1 | 43 | OrderedTableStEph.rs | 16 | 14 | -2 | Proved singleton + delete; added ensures to find/lookup trait |
| 2 | 43 | AugOrderedTableStEph.rs | 5 | 3 | -2 | Proved find + lookup (delegation after parent trait fix) |
| 3 | 43 | OrderedSetStEph.rs | 15 | 14 | -1 | Proved iter() with requires |
| 4 | 43 | OrderedSetStPer.rs | 13 | 12 | -1 | Proved iter() with requires |
| 5 | 43 | OrderedTableMtEph.rs | 16 | 15 | -1 | Proved delete with requires + assert chain |
| 6 | 43 | OrderedTableMtPer.rs | 22 | 21 | -1 | Lifted dom().finite() requires on from_st_table, eliminated 1 assume |
| 7 | 43 | OrderedSetMtEph.rs | 39 | 39 | 0 | Investigated: all 36 assumes are ghost bridges (structural) |
| 8 | 43 | AugOrderedTableStPer.rs | 2 | 2 | 0 | Closure requires blocked |
| 9 | 43 | AugOrderedTableMtEph.rs | 5 | 5 | 0 | Clone + closure blocked; propagated delete requires |
| 10 | 43 | OrderedTableStPer.rs | 10 | 10 | 0 | All ordered ops blocked by clone/comparison bridge |

## Techniques Used

1. **Requires lifting**: Added missing requires to trait signatures (singleton, delete, find, lookup, iter) to enable proof of impl bodies.
2. **Ensures propagation**: Added missing ensures to OrderedTableStEph find/lookup trait, unblocking AugOrderedTableStEph delegation proofs.
3. **Assert chains**: `assert(self.base_table@ =~= old(self).base_table@.remove(k@))` pattern for view delegation proofs.
4. **lemma_entries_to_map_finite**: Used to prove `@.dom().finite()` after table mutations.
5. **fn_missing_requires fixes**: Lifted dom().finite() into from_st_table requires, eliminating one assume.

## Chapters Closed

None. Chap43 has 135 remaining holes.

## Remaining Holes — What Blocks Them

### Clone/Comparison Bridge (blocks ~70+ holes across all files)
All ordered operations (first/last/previous/next/rank/select/split/get_range/split_rank) go through `to_seq()` + scan + `.clone()`. Without `obeys_feq_clone` or Ord spec bridges, Verus can't prove view equality after clone or ordering after comparison. This blocks:
- OrderedSetStEph: 13 external_body (ordered ops + clone + from_sorted_elements)
- OrderedSetStPer: 12 external_body
- OrderedTableStEph: 14 external_body (ordered ops + map/filter/reduce/collect)
- OrderedTableStPer: 10 external_body (ordered ops + collect)
- OrderedTableMtEph: 15 external_body (same pattern)

### Closure Requires (blocks ~7 holes)
- AugOrderedTableStPer: 2 assumes on `reducer.requires(...)`
- AugOrderedTableStEph: calculate_reduction, join_key (3 external_body)
- AugOrderedTableMtEph: calculate_reduction, recalculate_reduction, join_key

### Mt Ghost Bridge (blocks 36+ assumes)
- OrderedSetMtEph: 36 assumes — RwLock invariant stores only wf, not ghost-to-value mapping
- OrderedTableMtPer: 14 assumes — same pattern

### Iterator::next structural limitation
- Cannot add `requires` to `std::iter::Iterator::next` trait method implementations in Verus
- Affects iter next() in OrderedSetStEph, OrderedSetStPer, OrderedTableMtEph

## Commit

```
git log --oneline -1
```
(To be filled after commit)
