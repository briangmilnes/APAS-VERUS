# Agent 2 Round 36 Report

## Assignment
Prove ordering operations in Chap43 OrderedTableStEph.rs and OrderedTableStPer.rs.

## Results

| # | Chap | File | Holes Before | Holes After | Proved |
|---|------|------|-------------|-------------|--------|
| 1 | 43 | OrderedTableStEph.rs | 10 | 5 | 5 |
| 2 | 43 | OrderedTableStPer.rs | 9 | 4 | 5 |
| 3 | 19 | ArraySeqStPer.rs | 0 | 0 | (added lemma_view_index) |

**Verified count**: 4285 (up from 4257 baseline, +28 net)
**External bodies removed**: 10

## Functions Proved

### OrderedTableStEph.rs (5 proved)
- `first_key` — finds minimum key via linear scan with TotalOrder bridging
- `last_key` — finds maximum key via linear scan with TotalOrder bridging
- `previous_key` — finds largest key strictly less than k
- `next_key` — finds smallest key strictly greater than k
- `get_key_range` — filters entries by key range [k1, k2], uses ghost src_idx witness sequence

### OrderedTableStPer.rs (5 proved, mirrored from StEph)
- `first_key` — mirror of StEph
- `last_key` — mirror of StEph
- `previous_key` — mirror of StEph
- `next_key` — mirror of StEph
- `get_key_range` — mirror of StEph; added `requires self.spec_orderedtablestper_wf()` to trait for consistency with StEph

### Supporting Changes
- Added `lemma_view_index` to `ArraySeqStPerS` in `src/Chap19/ArraySeqStPer.rs` (connects `self@[i] == self.spec_index(i)@`)

## Techniques Used
- **TotalOrder bridging**: `K::reflexive`, `K::transitive`, `K::antisymmetric`, `K::total` for comparison proofs
- **Direct entry iteration**: Access `self.base_table.entries` (ArraySeqStEphS/StPerS) directly instead of `collect()`
- **Ghost src_idx witness sequence**: Eliminates `exists` quantifiers in loop invariants by tracking source indices explicitly
- **src_idx monotonicity**: Derives `spec_keys_no_dups` on range entries from monotonic source indices + no-dup on source
- **lemma_view_index**: Connects view-level (`entries@[i]`) with exec-level (`entries.spec_index(i)@`)
- **lemma_cloned_view_eq**: Preserves view equality after clone_plus

## Remaining Holes

### OrderedTableStEph.rs (5 remaining)
| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 43 | OrderedTableStEph.rs | collect | Complex: sorts entries, needs sorted sequence guarantees |
| 2 | 43 | OrderedTableStEph.rs | filter | Complex: closure spec propagation |
| 3 | 43 | OrderedTableStEph.rs | split_key | Trait lacks `requires wf`, postcondition `disjoint` unprovable |
| 4 | 43 | OrderedTableStEph.rs | rank_key | Complex: `dom().filter()` postcondition |
| 5 | 43 | OrderedTableStEph.rs | select_key | Complex: `dom().filter()` postcondition |

### OrderedTableStPer.rs (4 remaining)
| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 43 | OrderedTableStPer.rs | collect | Complex: sorts entries, needs sorted sequence guarantees |
| 2 | 43 | OrderedTableStPer.rs | split_key | Trait lacks `requires wf`, postcondition `disjoint` unprovable |
| 3 | 43 | OrderedTableStPer.rs | rank_key | Complex: `dom().filter()` postcondition |
| 4 | 43 | OrderedTableStPer.rs | select_key | Complex: `dom().filter()` postcondition |

## Pre-existing Error
- `src/Chap43/AugOrderedTableStEph.rs:516` — `obeys_feq_clone::<Pair<K, V>>()` precondition not satisfied (not caused by this round's changes)
