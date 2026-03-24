# R66 Agent2: OrderedTableStEph Backing Store Rewrite

## Objective
Rewrite `src/Chap43/OrderedTableStEph.rs` to use `ParamBST<Pair<K,V>>` (Chap38)
instead of `AVLTreeSeqStEphS<Pair<K,V>>` (Chap37) as backing store.

## Key Design Decisions

### Struct Change
```rust
// OLD:
pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
    pub base_seq: AVLTreeSeqStEphS<Pair<K, V>>,
}
// NEW:
pub struct OrderedTableStEph<K: StT + Ord, V: StT + Ord> {
    pub tree: ParamBST<Pair<K, V>>,
}
```
- V gets `+ Ord` because `ParamBST<T>` requires `T: StT + Ord`, and
  `Pair<K,V>: Ord` requires `V: Ord`.
- Trait keeps `V: StT` (not changed). Impl adds `V: Ord`.

### View
```rust
// spec_pair_set_to_map converts Set<(KV,VV)> to Map<KV,VV>
open spec fn view(&self) -> Map<K::V, V::V> {
    spec_pair_set_to_map(self.tree@)
}
```

### New Spec Functions
- `spec_pair_set_to_map<KV,VV>(s)` — Map::new over set projection
- `spec_key_unique_pairs_set<KV,VV>(s)` — no duplicate keys in set of pairs
- `spec_pair_key_determines_order<K,V>()` — key ordering determines pair ordering

### WF Predicate
```
self.tree.spec_bstparasteph_wf()
&& spec_key_unique_pairs_set(self.tree@)
&& self.tree@.len() < usize::MAX
&& obeys_feq_fulls::<K, V>()
&& obeys_cmp_spec::<Pair<K,V>>()
&& view_ord_consistent::<Pair<K,V>>()
&& spec_pair_key_determines_order::<K,V>()
&& obeys_cmp_spec::<K>()
&& view_ord_consistent::<K>()
```

### Proof of dom().finite()
- `spec_pair_set_to_map(s).dom()` ⊆ `s.map(|p| p.0)` (extensional)
- `s.finite() ==> s.map(f).finite()` (vstd::set_lib::Set::lemma_map_finite)
- Subset of finite is finite (vstd::set_lib::lemma_len_subset)

### Recursive Defaults (Tier 3)
11 functions get new recursive bodies using expose/join_mid:
find, insert, delete, first_key, last_key, previous_key, next_key,
split_key, get_key_range, rank_key, split_rank_key.

### _iter Variants
Use `tree.in_order()` to collect sorted `ArraySeqStPerS<Pair<K,V>>`,
then iterate linearly. Proof connects via set containment.

### Caller Updates
- `AugOrderedTableStEph.rs`: add `V: StT + Ord` to struct bounds
- `OrderedTableMtEph.rs`: add `V: MtVal + Ord` to struct bounds

## Execution Order
1. Write new OrderedTableStEph.rs (all sections)
2. Update callers (AugOrderedTableStEph, OrderedTableMtEph)
3. Validate + fix
4. Commit + push to agent2/ready
