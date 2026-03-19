# R41 Agent 2: Prove OrderedTableStPer Delegation Wrappers (20 remaining)

## Baseline
- Main at `29641a5e`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4281 verified, 192 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R39, you restructured `OrderedTableStPer.rs` from `TableStPer<K,V>` to
`AVLTreeSetStPer<Pair<K,V>>` backing. In R40, you proved 6 methods: `size`, `singleton`,
`find`, `insert`, `delete`, and `from_sorted_entries`, plus 6 bridge lemmas.

20 external_body methods remain.

## Assignment: Prove Remaining 20 Methods

Current external_body methods in OrderedTableStPer.rs:

**Tier 1: Direct delegations** (start here)

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | domain | 915 | Iterate pairs, collect keys into ArraySetStEph |
| 2 | collect | 1089 | Return entries as sorted sequence |
| 3 | first_key | 1095 | Delegate to base_set.first() |
| 4 | last_key | 1118 | Delegate to base_set.last() |
| 5 | previous_key | 1140 | Delegate to base_set — find predecessor |
| 6 | next_key | 1164 | Delegate to base_set — find successor |

**Tier 2: Ordering operations**

| # | Method | Line | Notes |
|---|--------|------|-------|
| 7 | split_key | 1188 | Partition into < k and > k |
| 8 | join_key | 1211 | Join two ordered tables |
| 9 | get_key_range | 1217 | Extract k1 ≤ key ≤ k2 |
| 10 | rank_key | 1234 | Count elements < k |
| 11 | select_key | 1251 | Get i-th element |
| 12 | split_rank_key | 1272 | Split at rank i |

**Tier 3: Higher-order operations** (closures)

| # | Method | Line | Notes |
|---|--------|------|-------|
| 13 | tabulate | 929 | Build table from function + key set |
| 14 | map | 946 | Apply function to all values |
| 15 | filter | 962 | Filter entries by predicate |
| 16 | intersection | 979 | Set intersection with combine |
| 17 | union | 1001 | Set union with combine |
| 18 | difference | 1037 | Remove entries in other |
| 19 | restrict | 1055 | Keep only entries whose keys are in set |
| 20 | subtract | 1072 | Remove entries whose keys are in set |

### Pattern from R40

In R40, you established the delegation pattern with bridge lemmas:
- `lemma_view_from_set_view` connects AVLTreeSetStPer view to OrderedTable view
- `lemma_pair_view_key_value` connects Pair views to tuple components
- The StPer struct is persistent (returns new Self instead of &mut self)

Use the same bridge lemma pattern for the remaining methods.

### Key Observations

- StPer methods return `Self` (persistent), not `&mut self` (ephemeral).
- `base_set` is `AVLTreeSetStPer<Pair<K,V>>`. Its methods return sets of Pair values.
- The View mapping: `OrderedTableStPer@` is a `Map<K::V, V::V>` while `base_set@` is
  a `Set<Pair<K,V>::V>`. Bridge lemmas translate between these representations.
- For Tier 1, the AVLTreeSetStPer already has `first()`, `last()`, `previous()`, `next()`
  etc. that return `Option<T>` — you need to extract the key from the Pair.
- For Tier 3, read `src/standards/using_closures_standard.rs` before starting.

### Priority

1. Tier 1 (6 methods) — direct delegations, should use existing bridge lemmas
2. Tier 2 (6 methods) — ordering operations on sorted structure
3. Tier 3 (8 methods) — closure reasoning, hardest

### Expected Results

Conservative: 8-10 methods proved.
Optimistic: 15-20 methods proved.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent2-r41-report.md`.
