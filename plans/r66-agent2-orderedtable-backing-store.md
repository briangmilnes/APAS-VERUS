# R66 Agent 2: OrderedTableStEph — Backing Store Rewire + Recursive Defaults

Read these files first:
- `src/standards/iterative_vs_recursive_standard.rs` — the naming pattern
- `src/Chap38/BSTParaStEph.rs` — the target backing store (full trait + impl)
- `src/Chap43/OrderedTableStEph.rs` — the file you're changing
- `src/Types.rs` — `Pair<K, V>` derives Ord (lexicographic by K then V), View is `(K::V, V::V)`
- `plans/iterative-vs-recursive-plan.md` — context (you're doing Tier 0b + Tier 3 for this file)

## Goal

Change OrderedTableStEph's backing store from AVLTreeSeqStEph (flat sorted array of
Pair<K,V>) to BSTParaStEph<Pair<K,V>> (Ch38 parametric BST). Then write recursive
defaults for the 11 renamed functions. The `_iter` variants become the alternative;
the defaults become recursive.

## Why BSTParaStEph

BSTParaStEph<Pair<K,V>> stores key-value pairs in a BST ordered by Pair's derived Ord
(lexicographic: K first, then V). It provides recursive split, join, union, intersect,
difference, filter, find, expose, min_key — everything needed for O(log n) table ops.

Its View is `Set<(K::V, V::V)>`. OrderedTableStEph's View is `Map<K::V, V::V>`. You
need a spec function to convert between them (similar to the existing
`spec_entries_to_map`).

## Key Challenge: Set of Pairs vs Map

BSTParaStEph views as `Set<(K::V, V::V)>`. The table needs `Map<K::V, V::V>`. This
means:

1. The View impl converts: `spec_pairs_set_to_map(self.tree@)` or similar.
2. The wf predicate must ensure **key uniqueness**: no two pairs in the BST share the
   same K. This is NOT guaranteed by BST structure alone (Pair(k, v1) and Pair(k, v2)
   are different values and could both be in the set).
3. Insert with combine function: if key exists, find the old pair, remove it, combine
   values, insert new pair. All O(log n) via BSTParaStEph's methods.

Write or adapt spec functions for the Set↔Map conversion. The existing
`spec_entries_to_map` works on `Seq<Pair<K,V>>` — you need one for
`Set<(K::V, V::V)>`.

## Step 1: Change the struct and View

Current:
```rust
pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
    pub base_seq: AVLTreeSeqStEphS<Pair<K, V>>,
}
```

Target:
```rust
pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
    pub tree: BSTParaStEph<Pair<K, V>>,
}
```

View: convert `self.tree@` (a `Set<(K::V, V::V)>`) to `Map<K::V, V::V>`.

## Step 2: Update wf

New wf should include:
- `self.tree.spec_bstparasteph_wf()`
- Key uniqueness: no two entries share a key
- `self@.dom().finite()` — fold finite into wf (per finite-in-wf plan)
- feq predicates

## Step 3: Recursive defaults for 11 functions

The 11 `_iter` functions: find, insert, delete, first_key, last_key, previous_key,
next_key, split_key, get_key_range, rank_key, split_rank_key.

For each, the default (recursive) implementation delegates to BSTParaStEph operations:

- **find**: `self.tree.find(&Pair(k, ...))` — but BSTParaStEph finds by the full Pair,
  not just key. You may need to use `split` by a sentinel pair, or walk the tree via
  `expose`. Think about this carefully.
- **insert**: split by key, combine if exists, join back.
- **delete**: split by key, discard middle, join.
- **first_key / last_key**: `self.tree.min_key()` / traverse rightmost.
- **previous_key / next_key**: split at key, find max of left / min of right.
- **split_key**: split the BST at key, convert halves back to OrderedTableStEph.
- **get_key_range**: two splits.
- **rank_key**: split, count left size.
- **split_rank_key**: similar to split by rank.

Some of these may need intermediate lemmas to connect BSTParaStEph's Set-based ensures
to Map-based ensures. The proofs are the hard part.

## Step 4: Update spec helpers

Remove or rewrite helpers referencing `self.base_seq`. The sortedness helpers
(`spec_keys_no_dups`, etc.) need to be rethought for the BST structure.

## Step 5: Fold finite into wf

Add `self@.dom().finite()` to the wf predicate. Then remove redundant
`.dom().finite()` from ensures clauses where wf is already ensured.

## Step 6: Check callers

- `src/Chap43/AugOrderedTableStEph.rs` wraps OrderedTableStEph. Check it still compiles.
- `src/Chap43/OrderedTableMtEph.rs` wraps OrderedTableStEph under RwLock. Check it.
- Tests in `tests/` and `rust_verify_test/`.

## Validation

`scripts/validate.sh` after completing the rewrite. Fix failures before done.
Run `scripts/rtt.sh` and `scripts/ptt.sh` after validate is clean.

## Constraints

- Do NOT change BSTParaStEph.rs. It is a dependency, not your file.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT change the OrderedTableStEphTrait signatures or specs.
  The trait is the contract. Only the struct, View, wf, and impl bodies change.
- Do NOT touch OrderedTableStPer.rs — that's a follow-up.
- Commit: `R66: OrderedTableStEph backing store rewire to BSTParaStEph + recursive defaults`
- Push to `agent2/ready`.

DOT. AFK.
