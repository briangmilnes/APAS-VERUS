# R40 Agent 1: Prove OrderedTableStEph Delegation Wrappers + Fix MtEph Reduce

## Baseline
- Main at `8fe452e7`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4264 verified, 204 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R39, you restructured `OrderedTableStEph.rs` from `TableStEph<K,V>` to
`AVLTreeSeqStEphS<Pair<K,V>>`. The struct and collect() are verified, but many methods
are `external_body`. After merging with Agent 4's MtEph restructure, the MtEph file
needed fixes for the `base_table` → `base_seq` rename.

There is also 1 ignored RTT (`test_string_concatenation_multithreaded` in
`tests/Chap43/TestAugOrderedTableMtEph.rs`) caused by iteration ordering after the
restructure.

## Assignment

### Part A: Prove OrderedTableStEph Delegation Wrappers

`OrderedTableStEph.rs` now has 26 holes. Many are `external_body` wrappers that should
delegate to `AVLTreeSeqStEphS` methods.

Read your R39 code to see which methods you verified and which are still `external_body`.
Then prove as many as possible.

**Tier 1: Direct delegations** (should be straightforward)

| # | Method | Notes |
|---|--------|-------|
| 1 | size() | delegate to base_seq.length() |
| 2 | singleton(k, v) | already done? check |
| 3 | find(k) | iterate base_seq to find pair with key k |
| 4 | insert(k, v, combine) | find existing, compute combined, rebuild |
| 5 | delete(k) | find pair, remove from sequence |
| 6 | domain() | iterate pairs, collect keys into ArraySetStEph |

**Tier 2: Ordering operations** (leverage sorted sequence)

| # | Method | Notes |
|---|--------|-------|
| 7 | previous_key(k) | scan sequence for predecessor |
| 8 | next_key(k) | scan sequence for successor |
| 9 | rank_key(k) | count elements < k |
| 10 | select_key(i) | get i-th element |
| 11 | split_key(k) | partition into < k and > k |
| 12 | get_key_range(k1, k2) | extract k1 ≤ key ≤ k2 |

**Tier 3: Higher-order operations** (closure reasoning)

| # | Method | Notes |
|---|--------|-------|
| 13 | tabulate(f, keys) | build table from function + key set |
| 14 | map(f) | apply function to all values |
| 15 | filter(f) | filter entries by predicate |
| 16+ | intersection, union, difference, restrict, subtract | set operations |

### Part B: Fix MtEph Reduce Ordering Bug

The ignored RTT test expects keys iterated in sorted order: 1→2→3. After the restructure,
`calculate_reduction` in `AugOrderedTableMtEph.rs` iterates in wrong order (1→3→2).

The bug is in `calculate_reduction` (line ~80 in AugOrderedTableMtEph.rs). It calls
`base.collect()` which now returns entries from the new AVLTreeSeqStEphS backing. The
ordering depends on how `collect()` iterates the sequence.

**Fix**: Ensure `OrderedTableMtEph.collect()` returns entries sorted by key. This should
be true if the backing AVL tree sequence is sorted. Check whether `collect()` delegates
correctly through the RwLock to the StEph `collect()` which you verified in R39.

After fixing, remove the `#[ignore]` from the test and verify it passes.

### Part C: Fix MtEph split_key and from_sorted_entries

During the merge, `split_key` and `from_sorted_entries` in `OrderedTableMtEph.rs` were
wrapped in `external_body` because they accessed `base_table.entries` (now `base_seq`).
The field references were fixed but the code still reaches into StEph internals.

**Better approach**: Rewrite `split_key` to delegate to `OrderedTableStEph::split_key()`
through the RwLock, instead of manually iterating entries. The StEph split_key is already
implemented. Same for from_sorted_entries — use `OrderedTableStEph::from_sorted_entries()`
if it exists, or construct via the API.

### Priority

1. Part B (fix RTT) — unignore the test, quick impact
2. Part A Tier 1 (5-6 methods) — direct delegations
3. Part C (fix MtEph external_body) — clean up merge artifacts
4. Part A Tiers 2-3 as time permits

### Expected Results

Conservative: Fix RTT + 5-8 delegation proofs.
Optimistic: Fix RTT + 10-15 delegation proofs + MtEph cleanup.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass (including the unignored test!).
Write your report to `plans/agent1-r40-report.md`.
