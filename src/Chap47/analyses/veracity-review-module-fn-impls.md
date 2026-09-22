<style>
  body { max-width: 98%; margin: auto; font-size: 16px; }
  table { width: 100%; border-collapse: collapse; }
  th, td { padding: 4px 8px; }
</style>

# Module Function Implementations Review

## Specification Summary by Module

| Abbr | Meaning |
|------|---------|
| Tr | declared in a `trait` block |
| IT | in `impl Trait for Type` |
| IBI | in bare `impl Type` |
| ML | module-level free fn |
| V! | inside `verus!` macro |
| -V! | outside `verus!` macro |
| Unk | has requires/ensures (strength not assessed) |
| Hole | contains `assume()`, `admit()`, or `#[verifier::external_body]` |
| NoSpec | no spec |

| # | Dir | Module | Tr | IT | IBI | ML | V! | -V! | Unk | Hole | NoSpec |
|---|-----|--------|:--:|:--:|:---:|:--:|:--:|:---:|:---:|:----:|:------:|
| 1 | Chap47 | ChainedHashTable | 1 | 0 | 0 | 5 | 6 | 0 | 6 | 0 | 0 |
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 1 | 8 | 0 | 1 | 1 | 6 |
| 3 | Chap47 | FlatHashTable | 2 | 5 | 0 | 4 | 11 | 0 | 10 | 0 | 1 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 0 | 6 | 0 | 0 | 0 | 6 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 5 | 5 | 2 | 9 | 0 | 3 | 0 | 6 |
| 6 | Chap47 | ParaHashTableStEph | 9 | 0 | 0 | 8 | 17 | 0 | 16 | 0 | 1 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 6 | 12 | 0 | 6 | 0 | 6 |
| 8 | Chap47 | StructChainedHashTable | 0 | 7 | 5 | 4 | 13 | 0 | 6 | 0 | 7 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 5 | 5 | 2 | 9 | 0 | 4 | 0 | 5 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_seq_pairs_to_map_remove_preserves_other_keys` |  |  |  | Y | Y |  |  | unknown | 30&#8209;44 |
| 2 | `lemma_seq_pairs_remove_key_then_push` |  |  |  | Y | Y |  |  | unknown | 58&#8209;69 |
| 3 | `lemma_seq_pairs_no_key_not_in_map` |  |  |  | Y | Y |  |  | unknown | 94&#8209;102 |
| 4 | `lemma_seq_pairs_has_key_in_map` |  |  |  | Y | Y |  |  | unknown | 110&#8209;120 |
| 5 | `lemma_seq_pairs_last_key_gives_value` |  |  |  | Y | Y |  |  | unknown | 131&#8209;143 |
| 6 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 166&#8209;171 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_spec_second_hash_value` |  |  |  | Y | Y |  |  | unknown | 91&#8209;92 |
| 8 | `second_hash` |  |  | Y |  | Y |  |  | hole | 106&#8209;111 |
| 9 | `insert` |  |  | Y |  | Y |  | Y |  | 152 |
| 10 | `lookup` |  |  | Y |  | Y |  | Y |  | 358 |
| 11 | `delete` |  |  | Y |  | Y |  | Y |  | 499 |
| 12 | `resize` |  |  | Y |  | Y |  | Y |  | 663&#8209;666 |
| 13 | `probe` |  |  | Y |  | Y |  | Y |  | 823 |
| 14 | `find_slot` |  |  | Y |  | Y |  | Y |  | 831 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 15 | `lemma_all_empties_count` |  |  |  | Y | Y |  |  | unknown | 74&#8209;77 |
| 16 | `lemma_empties_positive_implies_exists_empty` |  |  |  | Y | Y |  |  | unknown | 85&#8209;90 |
| 17 | `lemma_one_slot_change_empties` |  |  |  | Y | Y |  |  | unknown | 101&#8209;113 |
| 18 | `lemma_probe_mod_identity` |  |  |  | Y | Y |  |  | unknown | 139&#8209;141 |
| 19 | `probe` | Y |  |  |  | Y |  |  | unknown | 166&#8209;171 |
| 20 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 176&#8209;182 |
| 21 | `new` |  | Y |  |  | Y |  |  | unknown | 200&#8209;201 |
| 22 | `insert` |  | Y |  |  | Y |  |  | unknown | 206&#8209;209 |
| 23 | `lookup` |  | Y |  |  | Y |  |  | unknown | 214&#8209;217 |
| 24 | `delete` |  | Y |  |  | Y |  |  | unknown | 229&#8209;233 |
| 25 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 250 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `insert` |  |  | Y |  | Y |  | Y |  | 110 |
| 27 | `lookup` |  |  | Y |  | Y |  | Y |  | 347 |
| 28 | `delete` |  |  | Y |  | Y |  | Y |  | 469 |
| 29 | `resize` |  |  | Y |  | Y |  | Y |  | 622&#8209;626 |
| 30 | `probe` |  |  | Y |  | Y |  | Y |  | 787 |
| 31 | `find_slot` |  |  | Y |  | Y |  | Y |  | 794 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 32 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 50 |
| 33 | `clone_linked_list_entry` |  |  |  | Y | Y |  |  | unknown | 57&#8209;63 |
| 34 | `new` |  | Y |  |  | Y |  | Y |  | 93 |
| 35 | `insert` |  | Y | Y |  | Y |  |  | unknown | 97&#8209;103 |
| 36 | `lookup` |  | Y | Y |  | Y |  | Y |  | 124 |
| 37 | `delete` |  | Y | Y |  | Y |  |  | unknown | 139&#8209;142 |
| 38 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 162 |
| 39 | `resize` |  |  | Y |  | Y |  | Y |  | 489&#8209;492 |
| 40 | `hash_index` |  |  | Y |  | Y |  | Y |  | 668 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 41 | `lemma_table_to_map_push_empty` |  |  |  | Y | Y |  |  | unknown | 153&#8209;161 |
| 42 | `lemma_table_to_map_update_contains` |  |  |  | Y | Y |  |  | unknown | 173&#8209;184 |
| 43 | `lemma_table_to_map_not_contains` |  |  |  | Y | Y |  |  | unknown | 202&#8209;211 |
| 44 | `lemma_table_to_map_update_insert` |  |  |  | Y | Y |  |  | unknown | 220&#8209;235 |
| 45 | `lemma_table_to_map_update_remove` |  |  |  | Y | Y |  |  | unknown | 269&#8209;283 |
| 46 | `lemma_table_to_map_unique_entry_value` |  |  |  | Y | Y |  |  | unknown | 307&#8209;320 |
| 47 | `new` | Y |  |  |  | Y |  |  | unknown | 343&#8209;344 |
| 48 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 444&#8209;458 |
| 49 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 464&#8209;471 |
| 50 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 476&#8209;486 |
| 51 | `clone_entry` | Y |  |  |  | Y |  | Y |  | 359 |
| 52 | `createTable` | Y |  |  |  | Y |  |  | unknown | 387&#8209;398 |
| 53 | `metrics` | Y |  |  |  | Y |  |  | unknown | 490&#8209;492 |
| 54 | `loadAndSize` | Y |  |  |  | Y |  |  | unknown | 499&#8209;503 |
| 55 | `resize` | Y |  |  |  | Y |  |  | unknown | 517&#8209;529 |
| 56 | `clone_elem` |  |  |  | Y | Y |  |  | unknown | 537&#8209;539 |
| 57 | `call_hash_fn` |  |  |  | Y | Y |  |  | unknown | 553&#8209;559 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 58 | `lemma_consecutive_even` |  |  |  | Y | Y |  |  | unknown | 106&#8209;107 |
| 59 | `lemma_tri_step` |  |  |  | Y | Y |  |  | unknown | 125&#8209;126 |
| 60 | `lemma_odd_factor_pow2` |  |  |  | Y | Y |  |  | unknown | 140&#8209;147 |
| 61 | `lemma_triangular_injective` |  |  |  | Y | Y |  |  | unknown | 229&#8209;234 |
| 62 | `lemma_mod_add_cancel` |  |  |  | Y | Y |  |  | unknown | 311&#8209;319 |
| 63 | `lemma_empty_slot_reachable` |  |  |  | Y | Y |  |  | unknown | 349&#8209;368 |
| 64 | `insert` |  |  | Y |  | Y |  | Y |  | 457 |
| 65 | `lookup` |  |  | Y |  | Y |  | Y |  | 670 |
| 66 | `delete` |  |  | Y |  | Y |  | Y |  | 800 |
| 67 | `resize` |  |  | Y |  | Y |  | Y |  | 962&#8209;965 |
| 68 | `probe` |  |  | Y |  | Y |  | Y |  | 1120 |
| 69 | `find_slot` |  |  | Y |  | Y |  | Y |  | 1128 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 70 | `chain_insert` |  |  |  | Y | Y |  |  | unknown | 95&#8209;105 |
| 71 | `chain_lookup` |  |  |  | Y | Y |  |  | unknown | 155&#8209;166 |
| 72 | `chain_delete` |  |  |  | Y | Y |  |  | unknown | 207&#8209;218 |
| 73 | `new` |  | Y |  |  | Y |  | Y |  | 263 |
| 74 | `insert` |  | Y | Y |  | Y |  |  | unknown | 267&#8209;269 |
| 75 | `lookup` |  | Y | Y |  | Y |  | Y |  | 277 |
| 76 | `delete` |  | Y | Y |  | Y |  |  | unknown | 287&#8209;289 |
| 77 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 298 |
| 78 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 343 |
| 79 | `resize` |  |  | Y |  | Y |  | Y |  | 491&#8209;494 |
| 80 | `hash_index` |  |  | Y |  | Y |  | Y |  | 850 |
| 81 | `eq` x2 |  | Y |  |  | Y |  |  | unknown | 887&#8209;889 |
| 82 | `default` |  | Y |  |  | Y |  | Y |  | 917 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 83 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 49 |
| 84 | `clone_vec_pairs` |  |  |  | Y | Y |  |  | unknown | 56&#8209;60 |
| 85 | `new` |  | Y |  |  | Y |  |  | unknown | 90&#8209;91 |
| 86 | `insert` |  | Y | Y |  | Y |  |  | unknown | 96&#8209;102 |
| 87 | `lookup` |  | Y | Y |  | Y |  | Y |  | 123 |
| 88 | `delete` |  | Y | Y |  | Y |  |  | unknown | 138&#8209;141 |
| 89 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 161 |
| 90 | `resize` |  |  | Y |  | Y |  | Y |  | 498&#8209;501 |
| 91 | `hash_index` |  |  | Y |  | Y |  | Y |  | 676 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
