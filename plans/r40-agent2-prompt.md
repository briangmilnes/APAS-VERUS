# R40 Agent 2: Prove OrderedTableStPer Delegation Wrappers

## Baseline
- Main at `c1a1e964`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4290 verified, 186 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R39, you restructured `OrderedTableStPer.rs` from `TableStPer<K,V>` to
`AVLTreeSetStPer<Pair<K,V>>`. The struct and runtime code are correct — all 2613 RTT pass.
But 26 methods are `external_body` because the proof bodies weren't written.

Your R39 report says the blocker is `AVLTreeSeqStPerS` lacking `spec_index`/`lemma_view_index`.
However, many of these delegations should be provable directly through the `AVLTreeSetStPer`
API without needing to reason about individual sequence elements.

## Assignment

Prove as many of the 26 `external_body` methods in `src/Chap43/OrderedTableStPer.rs` as
possible by delegating to `AVLTreeSetStPer<Pair<K,V>>` methods.

### Tier 1: Direct Delegations (should be straightforward)

These delegate 1:1 to AVLTreeSetStPer methods. The AVLTreeSetStPer method ensures should
directly satisfy the OrderedTableStPer trait ensures after connecting views.

| # | Method | AVLTreeSetStPer API |
|---|--------|-------------------|
| 1 | `size()` | `self.base_set.size()` |
| 2 | `singleton(k, v)` | `AVLTreeSetStPer::singleton(Pair(k, v))` |
| 3 | `collect()` | `self.base_set.to_seq()` — NO sort_by! |
| 4 | `first_key()` | `self.base_set.first()` → extract `.0` |
| 5 | `last_key()` | `self.base_set.last()` → extract `.0` |

Read `AVLTreeSetStPer.rs` to understand each method's ensures. The proof pattern:
1. Remove `#[verifier::external_body]`
2. Call the AVLTreeSetStPer method
3. Connect its ensures to OrderedTableStPer's ensures via the view conversion

The view converts `Set<(K::V, V::V)>` → `Map<K::V, V::V>`. You need helper lemmas
connecting properties of the set to properties of the map (e.g., set.len() == map.dom().len()
when keys are unique).

### Tier 2: Find/Insert/Delete (need key-uniqueness reasoning)

| # | Method | Pattern |
|---|--------|---------|
| 6 | `find(k)` | Iterate `to_seq()` or use set operations to find pair with key k |
| 7 | `insert(k, v)` | Delete old pair with key k (if exists), insert Pair(k, v) |
| 8 | `delete(k)` | Find pair with key k, delete it from set |

These need the `spec_keys_no_dups` invariant from your wf predicate. The tricky part
is connecting "key k exists in the map" to "some Pair(k, _) exists in the set".

### Tier 3: Ordering Operations (may need tree-specific reasoning)

| # | Method | Difficulty |
|---|--------|-----------|
| 9 | `previous_key(k)` | Need predecessor in tree ordering |
| 10 | `next_key(k)` | Need successor in tree ordering |
| 11 | `rank_key(k)` | Need tree rank operation |
| 12 | `select_key(i)` | Need tree select operation |
| 13 | `split_key(k)` | Need tree split |
| 14 | `split_rank_key(i)` | Need tree split by rank |
| 15 | `get_key_range(k1, k2)` | Need range extraction |
| 16 | `join_key(left, right)` | Need tree join |

### Tier 4: Higher-Order Operations (closure reasoning)

| # | Method | Blocker |
|---|--------|---------|
| 17 | `tabulate(f, keys)` | Closure requires propagation |
| 18 | `map(f)` | Closure requires propagation |
| 19 | `filter(f)` | Closure requires propagation |
| 20 | `intersection(other, f)` | Set operation + closure |
| 21 | `union(other, f)` | Set operation + closure |
| 22-26 | `difference`, `restrict`, `subtract`, `from_sorted_entries`, `reduce` | Various |

### Strategy

1. Read `src/Chap41/AVLTreeSetStPer.rs` — understand every method's ensures
2. Read `src/Chap43/OrderedSetStPer.rs` — see how OrderedSet delegates to AVLTreeSetStPer
   (same pattern you need, but for sets not tables)
3. Start with Tier 1 (5 methods). These are pure delegation.
4. Move to Tier 2 (3 methods). Find/insert/delete are the core table operations.
5. Try Tier 3 as time allows. These leverage tree structure.
6. Tier 4 last — closure reasoning is hardest.

### Key Spec Functions

Your view conversion from R39 is `spec_entries_to_map(self.base_set.elements@)`. You may
need lemmas like:
- `spec_entries_to_map` preserves cardinality when keys are unique
- First pair in sorted set has the minimum key
- Pair ordering: Pair(k1,v1) < Pair(k2,v2) iff k1 < k2 (when keys unique)

### Expected Results

Conservative: 5-8 holes closed (Tier 1 + partial Tier 2).
Optimistic: 10-15 holes closed (Tiers 1-3).

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent2-r40-report.md`.
