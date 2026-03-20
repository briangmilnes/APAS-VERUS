# Agent 3 — Round 46 Report

## Summary

Closed 1 Chap47 hash table hole and strengthened the ParaHashTable insert trait spec
across all 6 implementations. Added counting infrastructure for flat hash table empties
to support the LinProb resize proof.

Baseline: 69 holes, 4396 verified.
Result: 68 holes, 4400 verified.

## Holes Before/After

| # | Chap | File | Before | After | Notes |
|---|------|------|--------|-------|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 1 | 0 | assume(false) closed via proof-by-contradiction |
| 2 | 47 | DoubleHashFlatHashTableStEph.rs | 1 | 1 | assume(false) remains; probing doesn't visit all slots |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 1 | 1 | assume(false) remains; probing doesn't visit all slots |
| 4 | 47 | ParaHashTableStEph.rs | 2 | 2 | clone_elem assume + call_hash_fn external_body (irreducible) |

## Changes Made

### FlatHashTable.rs (Chap47)
- Added `spec_count_empties` recursive spec function for counting Empty entries.
- Added 3 proof lemmas: `lemma_all_empties_count`, `lemma_empties_positive_implies_exists_empty`,
  `lemma_one_slot_change_empties`.

### ParaHashTableStEph.rs (Chap47)
- Added `spec_other_slots_preserved` helper spec function (trigger for existential).
- Added `spec_has_insert_capacity` and `spec_resize_ok` overridable trait methods.
- Strengthened `insert` requires with `spec_has_insert_capacity`.
- Strengthened `insert` ensures with `exists |s| spec_other_slots_preserved(...)`.
- Strengthened `resize` requires with `spec_resize_ok`.

### LinProbFlatHashTableStEph.rs (Chap47)
- Overrode `spec_has_insert_capacity`: exists an Empty slot in the table.
- Overrode `spec_resize_ok`: new_size > current_size.
- Replaced `assume(false)` with proof-by-contradiction: linear probing visits all m slots
  via `lemma_probe_mod_identity`, so if an Empty exists (precondition) the loop must find it.
- Added `exists |s|` existential witnesses at both return points (overwrite + empty slot).
- Rewrote resize phase 3 reinsertion loop with empties-count tracking invariant.

### All 5 Other Hash Table Implementations (Chap47)
- VecChainedHashTableStEph.rs: Added `spec_other_slots_preserved` proof for insert ensures.
- LinkedListChainedHashTableStEph.rs: Same.
- StructChainedHashTable.rs: Same (StructChainedHashTableStEph impl).
- DoubleHashFlatHashTableStEph.rs: Added proof at both return points.
- QuadProbFlatHashTableStEph.rs: Added proof at both return points.

## Techniques Used

- **Proof by contradiction**: If precondition says "exists Empty" and loop visits all m
  positions (linear probing), and none are Empty, then contradiction via extensional
  equality with old table + precondition.
- **Counting spec functions**: Recursive `spec_count_empties` with inductive lemmas to
  track how many Empty slots remain through resize's reinsertion loop.
- **Existential witnessing via spec helpers**: `spec_other_slots_preserved` provides a
  triggerable function application for the `exists |s|` postcondition.
- **Assertion scoping**: Assertions inside `assert forall ... by { }` are sub-proofs of
  the forall, not standalone facts. Existential witnesses must be placed outside.

## Remaining Chap47 Holes (4)

| # | Chap | File | Hole | What Blocks It |
|---|------|------|------|----------------|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | assume(false) at fallthrough | Double hashing may cycle without visiting all m slots |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | assume(false) at fallthrough | Quadratic probing may cycle without visiting all m slots |
| 3 | 47 | ParaHashTableStEph.rs | assume() in clone_elem | Verus Clone bridge limitation (irreducible) |
| 4 | 47 | ParaHashTableStEph.rs | external_body on call_hash_fn | Verus closure calling limitation (irreducible) |

## Verification

- 4400 verified, 0 errors
- 2613 RTT passed, 0 skipped
- 68 total project holes (was 69)
