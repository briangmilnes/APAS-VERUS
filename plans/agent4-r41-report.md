# Agent 4 — Round 41 Report

## Baseline
- Main at `29641a5e`, branch `agent4/ready`
- 4281 verified, 0 errors, 192 holes, 30 clean chapters, 2612 RTT pass

## Results
- 4276 verified, 0 errors, **187 holes (-5)**, 30 clean chapters, 2612 RTT pass
- Chap47: 14 → 9 holes (-5), 19 → 16 warnings (-3), 2 → 4 clean files

## Changes

### Chap47 — clone_elem Consolidation (-5 holes, -3 warnings)

Moved 6 duplicate `clone_elem<T: Clone>` functions from individual files into a single
`pub fn clone_elem` in `ParaHashTableStEph.rs`. All 6 implementations had identical
`assume(c == *x)` bridges. Centralizing eliminates 5 duplicate assumes and 3
`fn_missing_requires` warnings.

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Added centralized `pub fn clone_elem` |
| 2 | 47 | DoubleHashFlatHashTableStEph.rs | Removed local clone_elem |
| 3 | 47 | LinProbFlatHashTableStEph.rs | Removed local clone_elem |
| 4 | 47 | QuadProbFlatHashTableStEph.rs | Removed local clone_elem |
| 5 | 47 | StructChainedHashTable.rs | Removed local clone_elem |
| 6 | 47 | VecChainedHashTableStEph.rs | Removed local clone_elem |
| 7 | 47 | LinkedListChainedHashTableStEph.rs | Removed local clone_elem |

Files now clean: LinkedListChainedHashTableStEph.rs, VecChainedHashTableStEph.rs (+2).

### Chap47 — assume(false) + diverge() (Part A)

Added `diverge::<()>()` after `assume(false)` in DoubleHash and LinProb insert functions.
This matches the standard unreachable-arm pattern (`assume(false); diverge()`).

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | Added `diverge::<()>()` + import |
| 2 | 47 | LinProbFlatHashTableStEph.rs | Added `diverge::<()>()` + import |

**QuadProb excluded**: The "table full" path IS reachable at runtime with quadratic
probing (APAS Lemma 47.1: only first ceil(m/2) probes are distinct). Test
`test_max_attempts_ceiling_m_over_2` exercises this path. Adding `diverge()` would
cause an infinite loop. The `assume(false)` without diverge is correct for QuadProb.

**Note**: Veracity still counts `assume(false); diverge()` as holes (it flags all
`assume(false)` regardless of diverge). The code is now correct but the hole count
is unchanged from this change.

## Remaining Chap47 Holes (9)

| # | Chap | File | Hole | Status |
|---|------|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | assume(c == *x) in clone_elem | Irreducible: clone bridge |
| 2 | 47 | ParaHashTableStEph.rs | external_body call_hash_fn | Irreducible: opaque Fn closure |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | assume(false) insert | Unreachable with load < 1 |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | assume(forall) insert wf | Irreducible: opaque 2nd hash |
| 5 | 47 | DoubleHashFlatHashTableStEph.rs | assume(forall) lookup wf | Irreducible: opaque 2nd hash |
| 6 | 47 | DoubleHashFlatHashTableStEph.rs | assume(forall) delete wf | Irreducible: opaque 2nd hash |
| 7 | 47 | LinProbFlatHashTableStEph.rs | assume(false) insert | Unreachable with load < 1 |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | assume(false) insert | Reachable at runtime |
| 9 | 47 | StructChainedHashTable.rs | external_body resize | Provable but complex |

### Analysis of Irreducible Holes

**DoubleHash wf bridges (rows 4-6)**: The wf predicate has `exists |s: int|` (abstract
step). Runtime computes `step = compute_second_hash(key, m)` which is `external_body`
with only `ensures step >= 1`. No spec connects the runtime step to the wf existential
witness. These are irreducible without either (a) making `compute_second_hash` transparent,
or (b) adding the step as a concrete wf field.

**StructChained resize (row 9)**: Requires proving linked list chain traversal collects
all entries, connecting `spec_chain_to_map` to collected pairs via `spec_seq_pairs_to_map`.
Needs: (1) a `spec_chain_to_pairs` function, (2) a lemma connecting chain-pairs to
chain-map, (3) inner/outer loop invariants for chain traversal. Estimated 50-80 lines
of proof. Deferred to future round.

## Remaining Chap47 Warnings (16)

| # | Category | Count | Status |
|---|----------|-------|--------|
| 1 | fn_missing_requires (clone_elem) | 1 | Genuinely no precondition; user should add `// veracity: no_requires` |
| 2 | fn_missing_wf_requires (ParaHash trait) | 6 | False positive: `spec_impl_wf` IS the wf predicate |
| 3 | fn_missing_wf_ensures (ParaHash trait) | 2 | False positive: `spec_impl_wf` in ensures |
| 4 | assume_eq_clone_workaround | 7 | Structural: Clone/Eq workaround pattern |

## Part F: Chap43 from_sorted_elements (Report Only)

Both `OrderedSetStEph::from_sorted_elements` and `OrderedSetStPer::from_sorted_elements`
genuinely have no precondition. The chain is:
- `from_sorted_elements(elements)` calls `AVLTreeSeqStPerS::from_vec(elements)`
- `from_vec` has NO requires (StPer version)
- `from_vec` ensures `tree.spec_avltreeseqstper_wf()`
- `from_seq` requires `seq.spec_avltreeseqstper_wf()` — satisfied by from_vec's ensures

The function works for any Vec<T>, sorted or not. The name is misleading — it builds an
ordered set from any input. User should add `// veracity: no_requires` to both.

## Techniques Used
- Clone bridge consolidation: 6 → 1 centralized function, -5 holes
- Code pattern: `diverge::<()>()` after `assume(false)` for unreachable paths
- Analysis: quadratic probing Lemma 47.1 limits slot reachability
