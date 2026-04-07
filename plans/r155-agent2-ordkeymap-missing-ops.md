# R155 Agent 2 — Add Missing OrdKeyMap Operations. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap38/OrdKeyMap.rs` — your file.
Read `src/Chap43/OrderedTableStEph.rs` — reference implementations.

Report file: `plans/r155-agent2-ordkeymap-missing-ops-report.md`

## Problem

OrderedTable can't fully delegate because OrdKeyMap lacks these operations:

### Priority 1 (enables most delegation)

- `first_key(&self) -> Option<K>` — minimum key. BST: leftmost node.
- `last_key(&self) -> Option<K>` — maximum key. BST: rightmost node.
- `domain(&self) -> Set<K::V>` — set of all keys. Spec-only: `self@.dom()`.

### Priority 2 (enables higher-order methods)

- `collect(&self) -> Vec<Pair<K,V>>` — in-order traversal. Delegate to
  `self.inner.in_order()`.
- `filter<F>(&self, pred: &F) -> Self` — filter entries by predicate.
- `map<F, U>(&self, f: &F) -> OrdKeyMap<K, U>` — map values.
- `tabulate(keys, f) -> Self` — build map from keys + function.

### Priority 3 (enables range operations)

- `restrict(&self, keys: &Set<K::V>) -> Self` — keep only keys in set.
- `subtract(&self, keys: &Set<K::V>) -> Self` — remove keys in set.
- `get_key_range(&self, lo: &K, hi: &K) -> Self` — entries with lo <= key <= hi.
- `split_rank_key(&self, rank: usize) -> (Self, Self)` — split at rank.

## Approach

**Do NOT write proofs from scratch.** For each operation:

1. Find the implementation in OrderedTableStEph
2. Copy the proof logic into OrdKeyMap
3. Adapt: `self.tree.inner` → `self.inner`, use OrdKeyMap's bridge lemmas

Start with Priority 1 — first_key, last_key, domain are simple and
unblock the most delegation. Then Priority 2 if time permits.

### first_key / last_key

These are trivial BST operations (walk left/right spine). ParamBST may
already have min/max. Check `spec_min_link` / `spec_max_link` in BSTTreapStEph.
If ParamBST exposes them, delegate. If not, walk the tree.

### domain

This is pure spec:
```rust
pub open spec fn spec_domain(&self) -> Set<K::V> {
    self@.dom()
}
```

No exec implementation needed if it's spec-only. But if OrderedTable
has an exec `domain()` returning a `SetStEph`, that needs to iterate.

## Validation

`scripts/validate.sh isolate Chap38` during development.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Rules

- Do NOT modify any Chap43 file.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.

## When done

RCP.
