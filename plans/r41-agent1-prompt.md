# R41 Agent 1: Prove OrderedTableStEph Delegation Wrappers + Fix MtEph

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

In R39, you restructured `OrderedTableStEph.rs` from `TableStEph<K,V>` to
`AVLTreeSeqStEphS<Pair<K,V>>`. The struct, `new()`, `collect()`, `singleton()`, `find()`,
`first_key()`, `last_key()`, `previous_key()`, `next_key()`, and `get_key_range()` are
verified. But 18 methods remain as `external_body`.

There is also 1 ignored RTT (`test_string_concatenation_multithreaded` in
`tests/Chap43/TestAugOrderedTableMtEph.rs`) caused by iteration ordering after the
restructure. And 1 external_body in `OrderedTableMtEph.rs` (`from_sorted_entries`).

## Assignment

### Part A: Prove OrderedTableStEph Delegation Wrappers (18 holes)

Current external_body methods in OrderedTableStEph.rs:

| # | Method | Line | Notes |
|---|--------|------|-------|
| 1 | avl_seq_length | 395 | Helper: get AVL sequence length |
| 2 | avl_seq_nth | 403 | Helper: get nth element from AVL sequence |
| 3 | insert | 497 | Find existing pair, compute combined, rebuild |
| 4 | delete | 525 | Find pair with key k, remove from sequence |
| 5 | domain | 547 | Iterate pairs, collect keys into ArraySetStEph |
| 6 | tabulate | 561 | Build table from function + key set |
| 7 | map | 578 | Apply function to all values |
| 8 | filter | 594 | Filter entries by predicate |
| 9 | reduce | 615 | Fold over entries |
| 10 | intersection | 630 | Set intersection with combine function |
| 11 | union | 649 | Set union with combine function |
| 12 | difference | 685 | Remove entries in other |
| 13 | restrict | 702 | Keep only entries whose keys are in set |
| 14 | subtract | 719 | Remove entries whose keys are in set |
| 15 | split_key | 1193 | Partition into < k and > k |
| 16 | rank_key | 1246 | Count elements < k |
| 17 | select_key | 1265 | Get i-th element by sorted position |
| 18 | from_sorted_entries | 1589 | Constructor from sorted Vec |

**Tier 1: Helpers + Direct delegations** (start here)

Methods 1-2 (avl_seq_length, avl_seq_nth) are helper functions that wrap AVLTreeSeqStEphS
operations. These should be straightforward — call the AVL method, ensures match.

Methods 3-5 (insert, delete, domain) are core operations. For insert: scan the sorted
sequence for an existing pair with key k, apply combine if found, otherwise insert in sorted
position. For delete: find and remove. For domain: iterate pairs and collect keys.

Method 18 (from_sorted_entries) constructs from a Vec of Pair<K,V>.

**Tier 2: Higher-order operations** (these take closures)

Methods 6-14 (tabulate through subtract) involve closures. Read
`src/standards/using_closures_standard.rs` for patterns. These iterate the backing
AVLTreeSeqStEphS, applying the closure to each entry.

**Tier 3: Ordering operations**

Methods 15-17 (split_key, rank_key, select_key) leverage the sorted structure.

### Part B: Fix MtEph Reduce Ordering Bug

The ignored RTT test expects keys iterated in sorted order: 1→2→3. After the restructure,
`calculate_reduction` in `AugOrderedTableMtEph.rs` iterates in wrong order.

The bug is in how `collect()` delegates through the RwLock to StEph's `collect()`. Check
whether the backing AVL tree sequence maintains sort order through collect().

After fixing, remove the `#[ignore]` from the test and verify it passes.

### Part C: Fix MtEph from_sorted_entries

`OrderedTableMtEph.rs` line 762: `from_sorted_entries` is `external_body`. It should
construct an `OrderedTableStEph` from the sorted entries, then wrap it in the RwLock
following the standard `from_st` pattern.

### Priority

1. Part A Tier 1 (helpers + insert/delete/domain/from_sorted_entries) — 6 methods
2. Part B (fix RTT) — unignore the test
3. Part C (fix MtEph from_sorted_entries) — 1 method
4. Part A Tier 2 (closure operations) — 9 methods
5. Part A Tier 3 (ordering operations) — 3 methods

### Expected Results

Conservative: 6-8 delegation proofs + fix RTT.
Optimistic: 12-18 delegation proofs + fix RTT + MtEph cleanup.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass (including the unignored test!).
Write your report to `plans/agent1-r41-report.md`.
