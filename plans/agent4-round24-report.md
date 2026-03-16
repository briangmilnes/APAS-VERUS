# Agent 4 — Round 24 Report: Chap42 Tables — Prove Closure Functions

## Summary

Proved 11 `external_body` holes across 2 files in Chapter 42 (Table implementations).
All closure-using functions (tabulate, map, filter, intersection, union, insert) are now
fully verified in both TableStEph.rs and TableStPer.rs.

## Results

- **4045 verified, 0 errors**
- **2613 RTT pass**
- **224 total holes** (project-wide)
- **Chap42: 15 → 4 holes** (-11)

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 42 | TableStEph.rs | 6 | 0 | -6 |
| 2 | 42 | TableStPer.rs | 5 | 0 | -5 |
| 3 | 42 | TableMtEph.rs | 4 | 4 | 0 |

## Functions Proved

### TableStEph.rs (6 external_body removed)

| # | Function | Technique |
|---|----------|-----------|
| 1 | tabulate | Manual loop with f.ensures tracking in invariant |
| 2 | map | f.ensures tracking per entry, post-loop existential bridge |
| 3 | filter | f.ensures-to-spec_pred bridge + completeness invariant |
| 4 | intersection | combine.ensures tracking with self_srcs/other_srcs witnesses |
| 5 | union | insert-in-loop with combine.ensures tracking across iterations |
| 6 | insert | match_index sentinel (avoids V clone) + combine postcondition |

### TableStPer.rs (5 external_body removed)

| # | Function | Technique |
|---|----------|-----------|
| 1 | tabulate | Rewrote from ArraySeqStPerS::tabulate to manual loop (tabulate doesn't propagate f.ensures) |
| 2 | map | f.ensures tracking via self.entries.spec_index(j).1 (immutable self gives stable raw values) |
| 3 | filter | Completeness invariant tracking which source entries matched spec_pred |
| 4 | intersection | combine.ensures with self_srcs/other_srcs; added other.spec_tablestper_wf() to requires |
| 5 | union | Clone-then-insert-loop; unprocessed-both-keys invariant + combine.ensures tracking per processed entry |

## Cascade Changes

Added `other.spec_*_wf()` to intersection requires (needed for `lemma_entries_to_map_get` on other):

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 42 | TableStPer.rs | Added `other.spec_tablestper_wf()` to intersection requires |
| 2 | 43 | OrderedTableStPer.rs | Added `other.spec_orderedtablestper_wf()` to intersection requires |
| 3 | 43 | AugOrderedTableStPer.rs | Added `other.spec_augorderedtablestper_wf()` to intersection requires |
| 4 | 43 | OrderedTableStEph.rs | Added `old(self).spec_orderedtablesteph_wf()` to insert requires |
| 5 | 43 | AugOrderedTableStEph.rs | Added `old(self).spec_augorderedtablesteph_wf()` to insert requires |

## Key Techniques

- **match_index sentinel**: Use `usize` sentinel (= n) instead of `Option<V>` to avoid
  cloning V. `clone_plus` on V requires `obeys_feq_full::<V>()` which is NOT implied by
  `obeys_feq_full::<Pair<K, V>>()`.

- **f.ensures/combine.ensures tracking**: Loop invariant carries per-entry closure ensures
  facts using source indices. Post-loop existential witness proof connects entry indices
  to map keys via `lemma_entries_to_map_get` + `lemma_entries_to_map_key_in_seq`.

- **ArraySeqStPerS::tabulate bypass**: The library tabulate doesn't propagate closure
  ensures through its result. Rewrite with manual loop to track f.ensures per entry.

- **Unprocessed-both-keys invariant** (union): Track that self values are preserved for
  keys in self intersect other whose other entry hasn't been processed yet. Invariant
  uses inner forall to express "no prior other entry has this key".

## Remaining Holes (Chap42)

4 holes in TableMtEph.rs — all blocked by join() with nested closures:

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 42 | TableMtEph.rs | tabulate | join() + Arc + nested closures |
| 2 | 42 | TableMtEph.rs | map | join() + Arc + nested closures |
| 3 | 42 | TableMtEph.rs | filter | join() + Arc + nested closures |
| 4 | 42 | TableMtEph.rs | insert | join() + Arc + nested closures |
