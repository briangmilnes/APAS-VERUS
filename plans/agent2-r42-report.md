# Agent 2 Round 42 Report

## Summary

Added well-formedness `requires` to OrderedTableStPer and AugOrderedTableStPer trait methods,
then proved all 7 unblocked methods by removing `#[verifier::external_body]` and writing
full proofs with loop invariants.

## Verification

- Before: 4320 verified, 0 errors, 153 holes
- After: 4333 verified, 0 errors, 146 holes
- RTT: 2613 tests passed

## Holes Before/After by File

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 43 | OrderedTableStPer.rs | 9 | 2 | -7 |
| 2 | 43 | AugOrderedTableStPer.rs | 3 | 3 | 0 |

## Methods Proved (7)

| # | Chap | File | Method | Technique |
|---|------|------|--------|-----------|
| 1 | 43 | OrderedTableStPer.rs | `collect` | Clone elements seq + finite/len lemmas |
| 2 | 43 | OrderedTableStPer.rs | `domain` | Loop with clone_plus + bidirectional domain proof |
| 3 | 43 | OrderedTableStPer.rs | `difference` | Subtract pattern: result_src/result_idx ghost tracking + from_sorted_entries |
| 4 | 43 | OrderedTableStPer.rs | `first_key` | Ghost vals seq + TotalOrder min loop with transitive chains |
| 5 | 43 | OrderedTableStPer.rs | `last_key` | Ghost vals seq + TotalOrder max loop with transitive chains |
| 6 | 43 | OrderedTableStPer.rs | `previous_key` | Ghost vals + filtered predecessor tracking with TotalOrder cmp/total/antisymmetric |
| 7 | 43 | OrderedTableStPer.rs | `next_key` | Ghost vals + filtered successor tracking with TotalOrder cmp/total/antisymmetric |

## Part A: WF Requires Added

Added `self.spec_orderedtablestper_wf()` requires to: domain, collect, first_key, last_key,
previous_key, next_key. Added `other.spec_orderedtablestper_wf()` to difference. Added wf
ensures to split_key, get_key_range, split_rank_key return values.

Parallel changes in AugOrderedTableStPer trait with `spec_augorderedtablestper_wf()`. Added
`base.spec_orderedtablestper_wf()` to calculate_reduction. Fixed Aug impl proof blocks with
`lemma_reducer_clone_total` calls for split_key, split_rank_key, get_key_range.

## Remaining Holes in OrderedTableStPer.rs (2)

| # | Chap | File | Method | Blocker |
|---|------|------|--------|---------|
| 1 | 43 | OrderedTableStPer.rs | `rank_key` | TotalOrder + Set::filter proof (hard) |
| 2 | 43 | OrderedTableStPer.rs | `select_key` | TotalOrder + Set::filter proof (hard) |

## Techniques Used

- **Ghost vals pattern**: Incrementally built `Seq<Pair<K, V>>` tracking actual values
  alongside the persistent tree's view-level sequence, enabling TotalOrder reasoning
  on concrete K values (adapted from OrderedSetStPer).
- **Result src/idx ghost tracking**: Bidirectional index mapping between source and result
  sequences for filter-like operations (adapted from proven restrict/subtract methods).
- **TotalOrder proof chains**: reflexive for base case, transitive for update propagation,
  antisymmetric for Greater-but-le contradiction, total for non-update case.
- **lemma_entries_to_map_***: Family of lemmas bridging seq-of-pairs to Map domain/values.
