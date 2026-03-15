# Agent 2 — Round 15

## Status: 149 holes, 4078 verified, 38 clean chapters.

Round 13: you proved 12 Chap43 St ordering ops. Now continue with the remaining Chap43
files — you own the entire chapter this round.

## Your files — Chap43 (56 holes total, all files)

### Priority 1: St files (38 holes)

**OrderedSetStEph.rs (12 holes: 1 assume + 11 external_body)**
- The 11 ext_body are St operations: `insert`, `delete`, `find`, `to_seq`, `previous`,
  `next`, `rank_key`, `split`, `get_key_range`, `split_rank`, `from_sorted_elements`
- You already proved `first`, `last`, `select`, `previous`, `next`, `from_sorted_elements`,
  `into_iter`, `rank_key` in R13. Continue with the remaining.
- Use collect+while loop + inner AVLTreeSet StEph methods.
- The 1 assume (`result@ =~= eph_seq@`) is a clone/view bridge — may need feq axiom.

**OrderedTableStEph.rs (12 external_body)**
- Same pattern as OrderedSetStEph but for key-value tables.
- Inner type is AVLTreeSetStEph with Pair<K,V> entries.
- Functions: `collect`, `from_sorted_entries`, `previous_key`, `next_key`, `rank_key`,
  `get_key_range`, `split_key`, `split_rank_key`, `map`, `filter`, `reduce`.
- Agent 1 proved the identical functions for OrderedTableMtEph in R14. Study that code
  for the pattern (collect+while loop, Pair clone decomposition, cmp pattern).

**OrderedTableStPer.rs (9 external_body)**
- Persistent version. Same functions, persistent semantics (returns new table).
- Same collect+while loop pattern.

**OrderedSetStPer.rs (5 external_body)**
- Persistent set operations. Smaller set of functions.

### Priority 2: Remaining files (18 holes)

**OrderedSetMtEph.rs (9 holes: 7 assume + 2 external_body)**
- 5 assumes are `spec_orderedsetsteph_wf()` after split/range — need wf in StEph ensures
- 2 assumes are ghost-locked reader gaps (size, find)
- 2 ext_body are MtEph wrappers

**OrderedTableMtPer.rs (2 assume)**
- Both from R13 reduction (21 → 2). These are the hard remainder.

**AugOrderedTableMtEph.rs (2 external_body)**
**AugOrderedTableStEph.rs (3 external_body)**
**AugOrderedTableStPer.rs (2 assume)**
- AugOrdered files build on OrderedTable. Fix OrderedTable first, then these.
- The 2 StPer assumes need closure requires propagation — read
  `src/standards/using_closures_standard.rs`.

## DO NOT

- Touch Chap42 (Agent 1)
- Touch Chap41 (Agents 1 and 4)
- Touch Chap39 (Agent 3)
- Touch Chap37, Chap47, Chap45 (Agent 4)
- Touch Chap38 (Agent 3)

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversion.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Read `src/standards/partial_eq_eq_clone_standard.rs` for feq/clone patterns.
- Study Agent 1's R14 OrderedTableMtEph proofs for the collect+while loop + Pair
  clone decomposition + cmp pattern.
- Push to `agent2/ready`. Write `plans/agent2-round15-report.md`.

## Target: -15 (stretch -30). Focus on St files first.
