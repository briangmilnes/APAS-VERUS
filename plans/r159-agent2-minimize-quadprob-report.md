# R159 Agent 2 Report: Minimize Hash Table Insert Proofs

## Summary

Minimized `insert` proof assertions in three Chap47 flat hash table files by
stripping redundant intermediate assertions from `assert forall ... by { ... }`
bodies while preserving structural proof elements (case splits, choose expressions,
lemma calls, trigger activators).

## Results

| # | Chap | File | Asserts Before | Asserts After | Delta | Notes |
|---|------|------|---------------|--------------|-------|-------|
| 1 | 47 | QuadProbFlatHashTableStEph.rs | 65 | 36 | -29 | Already in HEAD via R160 commit |
| 2 | 47 | LinProbFlatHashTableStEph.rs | 60 | 37 | -23 | New reduction |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 65 | 42 | -23 | New reduction |

Total: 190 → 115 asserts (-75, -39%)

## Validation

- Isolate Chap47: 1257 verified, 0 errors, 14s
- Full validation: 5748 verified, 0 errors, 176s
- RTT: 3776 passed, 0 skipped

## What was removed

Across all three insert functions, the same categories of assertions were removed:

### Overwrite path (Occupied, key match)
- **Removed**: `assert(spec_flat_has_key(table.table@[slot], k) ==> k == key)` — Z3 derives
  from the Occupied constructor and feq result.
- **Removed**: `assert(table.table@[j] == old_table_seq[j])` in no-dup body — Z3 derives
  from `set()` semantics when j != slot.
- **Removed**: `assert(!spec_flat_has_key(old_table_seq[j], key))` in no-dup body — follows
  from the earlier `assert forall` that established no other slot has key.
- **Removed**: `assert(spec_flat_has_key(old_table_seq[i], k))` — Z3 transfers from unchanged
  slot equality.
- **Removed**: `assert(i != slot as int)` — tautological in else branch.
- **Removed**: Probe chain inner asserts about spec_flat_has_key at slot — Z3 handles the
  case split with just `if k == key {}`.

### Insert-new path (Empty slot)
- **Removed**: `assert(old_table_seq[slot] is Empty)` — follows from match arm.
- **Removed**: `assert(old_table_seq[slot].spec_entry_to_map() =~= Map::empty())` — follows
  from Empty variant definition.
- **Removed**: Various `assert(spec_flat_has_key(table.table@[slot], k) ==> k == key)` —
  Z3 derives from Occupied constructor.

### Kept (irreducible)
- All lemma calls (lemma_reveal_view_injective, lemma_table_to_map_update_insert, etc.)
- All `assert forall` headers — these ARE the proof obligations
- All `choose` expressions — existential witnesses Z3 can't guess
- All modular arithmetic proofs (slot advancement, probe chain)
- `assert(spec_other_slots_preserved(...))` — Z3 can't derive this exists witness
- `assert(old_table_seq =~= old(table).table@)` — needed to bridge ghost variable to old()
- `assert(table.table@[pos] == old_table_seq[pos])` in probe chain inner bodies — needed
  for Z3 to connect unchanged slots across the set() mutation
- `assert(new_entry.spec_entry_to_map() =~= ...)` — preconditions for lemma calls

## Technique

For each `assert forall ... by { ... }` body:
1. Keep the case-split `if i == slot { ... } else { ... }` structure
2. Keep empty `if` bodies that serve as trigger activators (e.g., `if spec_flat_has_key(...) {}`)
3. Remove intermediate assertions that restate what Z3 can derive from:
   - Vec::set() semantics (unchanged slots)
   - Enum variant matching (Occupied has key)
   - Previously established quantified facts
4. Keep assertions that provide trigger terms Z3 needs to instantiate quantifiers

Key learning: LinProb's insert-new no-dup proof needs MORE explicit hints than QuadProb/DoubleHash.
Specifically, `assert(table.table@[j] == old_table_seq[j])` and
`assert(!old_table_seq[j].spec_entry_to_map().dom().contains(key))` were both needed to connect
the not-contains fact from `lemma_table_to_map_not_contains` through to `!spec_flat_has_key`.

## Steps used: 5 of 15

1. Strip QuadProb insert — 1 error (missing spec_other_slots_preserved)
2. Fix QuadProb — clean (found bug: assert inside wrong scope)
3. Fix QuadProb again — clean (assert was inside assert forall body, not proof block)
4. Strip LinProb insert — 1 error (insert-new no-dup needs more hints than QuadProb)
5. Fix LinProb, strip DoubleHash — both clean
