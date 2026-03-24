# R67 Agent 3: OrderedTableStPer Backing Store Rewire

## Goal

Rewire `OrderedTableStPer<K, V>` from wrapping `AVLTreeSetStPer<Pair<K, V>>` to using
`ParamBST<Pair<K, V>>` directly. This mirrors agent2's R66 rewire of OrderedTableStEph.

## Reference Implementation

**Read first**: `src/Chap43/OrderedTableStEph.rs` — the completed StEph rewire by agent2.
Your StPer version should follow the same patterns: struct field, view via
`spec_pair_set_to_map`, wf, delegation to ParamBST methods.

**Current file**: `src/Chap43/OrderedTableStPer.rs`

**Key differences between StEph and StPer**:
- StPer is persistent — operations return new values rather than mutating.
- Where StEph has `fn insert(&mut self, k: K, v: V, ...)`, StPer has
  `fn insert(&self, k: K, v: V, ...) -> (updated: Self)`.
- StPer has `spec_orderedtablestper_find_wf` (a weaker wf for find-only operations).

## Current Structure

```rust
pub struct OrderedTableStPer<K: StT + Ord, V: StT + Ord> {
    pub base_set: AVLTreeSetStPer<Pair<K, V>>,
}
```

wf: `self.base_set.spec_avltreesetstper_wf() && spec_keys_no_dups(...) && obeys_feq_fulls::<K, V>() && obeys_feq_full::<Pair<K, V>>()`

39 trait methods including `_iter` variants.

## Steps

### Step 1: Change struct field

```rust
pub struct OrderedTableStPer<K: StT + Ord, V: StT + Ord> {
    pub tree: ParamBST<Pair<K, V>>,
}
```

Add `#[verifier::reject_recursive_types(K)]` and `#[verifier::reject_recursive_types(V)]`.

### Step 2: View — spec_pair_set_to_map bridge

ParamBST views as `Set<(K::V, V::V)>`. OrderedTable views as `Map<K::V, V::V>`. You need
a spec function to bridge these. Copy the pattern from the StEph version:

```rust
pub open spec fn spec_pair_set_to_map<KV, VV>(s: Set<(KV, VV)>) -> Map<KV, VV> {
    Map::new(|k: KV| exists|v: VV| s.contains((k, v)), |k: KV| choose|v: VV| s.contains((k, v)))
}
```

View becomes: `spec_pair_set_to_map(self.tree@)`

### Step 3: Update wf

Follow the StEph pattern. Include key uniqueness:
```rust
self.tree.spec_bstparasteph_wf()
&& spec_key_unique_pairs_set(self.tree@)
&& self.tree@.len() < usize::MAX as nat
&& obeys_feq_fulls::<K, V>()
&& obeys_feq_full::<Pair<K, V>>()
&& vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()
&& view_ord_consistent::<Pair<K, V>>()
&& spec_pair_key_determines_order::<K, V>()
&& vstd::laws_cmp::obeys_cmp_spec::<K>()
&& view_ord_consistent::<K>()
```

Note: the StEph version includes axioms in wf. This works because `empty()` and
`singleton()` don't ensure wf in the OrderedTable trait. Follow whatever pattern the
StEph version uses — consistency between StEph and StPer matters more than perfection.

### Step 4: Rewrite method bodies

Each method converts from Map operations to ParamBST tree operations on `Pair<K, V>`.
The StEph version has working implementations for all of these — use them as templates.

Key operations:
- **find**: Split by key, check if found
- **insert**: Delete old key (if exists), insert new Pair
- **delete**: Split by key, rejoin without it
- **first_key/last_key**: Use `min_key` on tree or expose to find min/max
- **previous_key/next_key**: Split, find max of left / min of right
- **rank_key**: Split, count left side
- **split_key**: Split tree by key, build two tables
- **get_key_range**: Two splits to extract range
- **split_rank_key**: In-order traversal, split at index

### Step 5: Proof infrastructure

You'll need the same lemma infrastructure as the StEph version:
- `lemma_key_unique_insert` — inserting into unique-key set preserves uniqueness
- `lemma_key_unique_subset` — subset of unique-key set has unique keys
- `spec_key_unique_pairs_set` — predicate for key uniqueness in a set of pairs
- `spec_pair_key_determines_order` — keys determine Pair ordering

Copy these from the StEph version if they're not already importable.

### Step 6: Check callers

- `AugOrderedTableStPer` (Chap43) — extends OrderedTableStPer with augmented operations.
  Check if it compiles. Update imports/type bounds if needed (may need `+Ord` on V).
- `OrderedSetStPer` (Chap43) — already commented out. Leave it.

### Step 7: Verify

- `scripts/validate.sh` — 0 errors
- `scripts/rtt.sh` — all pass
- `scripts/ptt.sh` — all pass
- `scripts/holes.sh src/Chap43/` — 0 holes

## Constraints

- Do NOT modify BSTParaStEph.rs, OrderedTableStEph.rs, or AVLTreeSetStPer.rs.
- Do NOT add `assume`, `accept`, or `external_body` on algorithmic logic.
- Do NOT uncomment OrderedSetStPer or OrderedTableMtPer — those are separate tasks.
- Run validate, rtt, ptt sequentially.
