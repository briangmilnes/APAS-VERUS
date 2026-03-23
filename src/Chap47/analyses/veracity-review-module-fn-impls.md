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
| 2 | Chap47 | DoubleHashFlatHashTableStEph | 0 | 0 | 7 | 5 | 12 | 0 | 5 | 1 | 6 |
| 3 | Chap47 | FlatHashTable | 2 | 5 | 0 | 0 | 7 | 0 | 6 | 0 | 1 |
| 4 | Chap47 | LinProbFlatHashTableStEph | 0 | 0 | 6 | 4 | 10 | 0 | 4 | 0 | 6 |
| 5 | Chap47 | LinkedListChainedHashTableStEph | 0 | 5 | 5 | 2 | 9 | 0 | 3 | 0 | 6 |
| 6 | Chap47 | ParaHashTableStEph | 9 | 0 | 0 | 8 | 17 | 0 | 14 | 2 | 1 |
| 7 | Chap47 | QuadProbFlatHashTableStEph | 0 | 0 | 6 | 9 | 15 | 0 | 9 | 0 | 6 |
| 8 | Chap47 | StructChainedHashTable | 0 | 7 | 5 | 4 | 13 | 0 | 5 | 1 | 7 |
| 9 | Chap47 | VecChainedHashTableStEph | 0 | 5 | 5 | 2 | 9 | 0 | 4 | 0 | 5 |

## Function-by-Function Detail

### Chap47/ChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 1 | `lemma_seq_pairs_to_map_remove_preserves_other_keys` |  |  |  | Y | Y |  |  | unknown | 23&#8209;37 |
| 2 | `lemma_seq_pairs_remove_key_then_push` |  |  |  | Y | Y |  |  | unknown | 52&#8209;63 |
| 3 | `lemma_seq_pairs_no_key_not_in_map` |  |  |  | Y | Y |  |  | unknown | 83&#8209;91 |
| 4 | `lemma_seq_pairs_has_key_in_map` |  |  |  | Y | Y |  |  | unknown | 103&#8209;113 |
| 5 | `lemma_seq_pairs_last_key_gives_value` |  |  |  | Y | Y |  |  | unknown | 125&#8209;137 |
| 6 | `hash_index` | Y |  |  |  | Y |  |  | unknown | 164&#8209;169 |

### Chap47/DoubleHashFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 7 | `lemma_all_empties_count` |  |  |  | Y | Y |  |  | unknown | 92&#8209;95 |
| 8 | `lemma_empties_positive_implies_exists_empty` |  |  |  | Y | Y |  |  | unknown | 108&#8209;113 |
| 9 | `lemma_one_slot_change_empties` |  |  |  | Y | Y |  |  | unknown | 126&#8209;138 |
| 10 | `lemma_spec_second_hash_value` |  |  |  | Y | Y |  |  | unknown | 167&#8209;168 |
| 11 | `lemma_probe_mod_identity` |  |  |  | Y | Y |  |  | unknown | 172&#8209;174 |
| 12 | `second_hash` |  |  | Y |  | Y |  |  | hole | 197&#8209;202 |
| 13 | `insert` |  |  | Y |  | Y |  | Y |  | 242 |
| 14 | `lookup` |  |  | Y |  | Y |  | Y |  | 521 |
| 15 | `delete` |  |  | Y |  | Y |  | Y |  | 647 |
| 16 | `resize` |  |  | Y |  | Y |  | Y |  | 817&#8209;820 |
| 17 | `probe` |  |  | Y |  | Y |  | Y |  | 966 |
| 18 | `find_slot` |  |  | Y |  | Y |  | Y |  | 974 |

### Chap47/FlatHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 19 | `probe` | Y |  |  |  | Y |  |  | unknown | 63&#8209;67 |
| 20 | `find_slot` | Y |  |  |  | Y |  |  | unknown | 72&#8209;77 |
| 21 | `new` |  | Y |  |  | Y |  |  | unknown | 94&#8209;95 |
| 22 | `insert` |  | Y |  |  | Y |  |  | unknown | 100&#8209;103 |
| 23 | `lookup` |  | Y |  |  | Y |  |  | unknown | 108&#8209;111 |
| 24 | `delete` |  | Y |  |  | Y |  |  | unknown | 123&#8209;127 |
| 25 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 143 |

### Chap47/LinProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 26 | `lemma_all_empties_count` |  |  |  | Y | Y |  |  | unknown | 84&#8209;87 |
| 27 | `lemma_empties_positive_implies_exists_empty` |  |  |  | Y | Y |  |  | unknown | 100&#8209;105 |
| 28 | `lemma_one_slot_change_empties` |  |  |  | Y | Y |  |  | unknown | 118&#8209;130 |
| 29 | `lemma_probe_mod_identity` |  |  |  | Y | Y |  |  | unknown | 158&#8209;160 |
| 30 | `insert` |  |  | Y |  | Y |  | Y |  | 196 |
| 31 | `lookup` |  |  | Y |  | Y |  | Y |  | 473 |
| 32 | `delete` |  |  | Y |  | Y |  | Y |  | 578 |
| 33 | `resize` |  |  | Y |  | Y |  | Y |  | 728&#8209;731 |
| 34 | `probe` |  |  | Y |  | Y |  | Y |  | 881 |
| 35 | `find_slot` |  |  | Y |  | Y |  | Y |  | 888 |

### Chap47/LinkedListChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 36 | `_linked_list_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 40 |
| 37 | `clone_linked_list_entry` |  |  |  | Y | Y |  |  | unknown | 44&#8209;47 |
| 38 | `new` |  | Y |  |  | Y |  | Y |  | 76 |
| 39 | `insert` |  | Y | Y |  | Y |  |  | unknown | 80&#8209;86 |
| 40 | `lookup` |  | Y | Y |  | Y |  | Y |  | 107 |
| 41 | `delete` |  | Y | Y |  | Y |  |  | unknown | 122&#8209;125 |
| 42 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 144 |
| 43 | `resize` |  |  | Y |  | Y |  | Y |  | 437&#8209;440 |
| 44 | `hash_index` |  |  | Y |  | Y |  | Y |  | 588 |

### Chap47/ParaHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 45 | `clone_elem` |  |  |  | Y | Y |  |  | hole | 116&#8209;117 |
| 46 | `lemma_table_to_map_push_empty` |  |  |  | Y | Y |  |  | unknown | 125&#8209;133 |
| 47 | `lemma_table_to_map_update_contains` |  |  |  | Y | Y |  |  | unknown | 145&#8209;156 |
| 48 | `lemma_table_to_map_not_contains` |  |  |  | Y | Y |  |  | unknown | 184&#8209;193 |
| 49 | `lemma_table_to_map_update_insert` |  |  |  | Y | Y |  |  | unknown | 209&#8209;224 |
| 50 | `lemma_table_to_map_update_remove` |  |  |  | Y | Y |  |  | unknown | 263&#8209;277 |
| 51 | `lemma_table_to_map_unique_entry_value` |  |  |  | Y | Y |  |  | unknown | 306&#8209;319 |
| 52 | `call_hash_fn` |  |  |  | Y | Y |  |  | hole | 343&#8209;347 |
| 53 | `new` | Y |  |  |  | Y |  |  | unknown | 361&#8209;362 |
| 54 | `insert` x2 | Y |  |  |  | Y |  |  | unknown | 463&#8209;475 |
| 55 | `lookup` x2 | Y |  |  |  | Y |  |  | unknown | 480&#8209;487 |
| 56 | `delete` x2 | Y |  |  |  | Y |  |  | unknown | 492&#8209;502 |
| 57 | `clone_entry` | Y |  |  |  | Y |  | Y |  | 375 |
| 58 | `createTable` | Y |  |  |  | Y |  |  | unknown | 412&#8209;422 |
| 59 | `metrics` | Y |  |  |  | Y |  |  | unknown | 507&#8209;509 |
| 60 | `loadAndSize` | Y |  |  |  | Y |  |  | unknown | 516&#8209;520 |
| 61 | `resize` | Y |  |  |  | Y |  |  | unknown | 533&#8209;545 |

### Chap47/QuadProbFlatHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 62 | `lemma_consecutive_even` |  |  |  | Y | Y |  |  | unknown | 95&#8209;96 |
| 63 | `lemma_tri_step` |  |  |  | Y | Y |  |  | unknown | 115&#8209;116 |
| 64 | `lemma_odd_factor_pow2` |  |  |  | Y | Y |  |  | unknown | 131&#8209;138 |
| 65 | `lemma_triangular_injective` |  |  |  | Y | Y |  |  | unknown | 241&#8209;246 |
| 66 | `lemma_mod_add_cancel` |  |  |  | Y | Y |  |  | unknown | 327&#8209;335 |
| 67 | `lemma_empty_slot_reachable` |  |  |  | Y | Y |  |  | unknown | 365&#8209;384 |
| 68 | `lemma_all_empties_count` |  |  |  | Y | Y |  |  | unknown | 463&#8209;466 |
| 69 | `lemma_empties_positive_implies_exists_empty` |  |  |  | Y | Y |  |  | unknown | 479&#8209;484 |
| 70 | `lemma_one_slot_change_empties` |  |  |  | Y | Y |  |  | unknown | 497&#8209;509 |
| 71 | `insert` |  |  | Y |  | Y |  | Y |  | 560 |
| 72 | `lookup` |  |  | Y |  | Y |  | Y |  | 851 |
| 73 | `delete` |  |  | Y |  | Y |  | Y |  | 965 |
| 74 | `resize` |  |  | Y |  | Y |  | Y |  | 1133&#8209;1136 |
| 75 | `probe` |  |  | Y |  | Y |  | Y |  | 1279 |
| 76 | `find_slot` |  |  | Y |  | Y |  | Y |  | 1287 |

### Chap47/StructChainedHashTable.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 77 | `_struct_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 89 |
| 78 | `default` |  | Y |  |  | Y |  | Y |  | 119 |
| 79 | `chain_insert` |  |  |  | Y | Y |  |  | unknown | 125&#8209;135 |
| 80 | `chain_lookup` |  |  |  | Y | Y |  |  | unknown | 179&#8209;188 |
| 81 | `chain_delete` |  |  |  | Y | Y |  |  | unknown | 219&#8209;229 |
| 82 | `new` |  | Y |  |  | Y |  | Y |  | 271 |
| 83 | `insert` |  | Y | Y |  | Y |  |  | unknown | 275&#8209;276 |
| 84 | `lookup` |  | Y | Y |  | Y |  | Y |  | 284 |
| 85 | `delete` |  | Y | Y |  | Y |  |  | unknown | 291&#8209;293 |
| 86 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 301 |
| 87 | `resize` |  |  | Y |  | Y |  | Y |  | 461&#8209;464 |
| 88 | `hash_index` |  |  | Y |  | Y |  | Y |  | 823 |
| 89 | `eq` x2 |  | Y |  |  | Y |  |  | hole | 849&#8209;851 |

### Chap47/VecChainedHashTableStEph.rs

| # | Function | Trait | IT | IBI | ML | V! | -V! | NoSpec | SpecStr | Lines |
|---|----------|:-----:|:--:|:--:|:--:|:--:|:---:|:------:|:-------:|------:|
| 90 | `_vec_chained_hash_table_verified` |  |  |  | Y | Y |  | Y |  | 40 |
| 91 | `clone_vec_pairs` |  |  |  | Y | Y |  |  | unknown | 44&#8209;45 |
| 92 | `new` |  | Y |  |  | Y |  |  | unknown | 74&#8209;75 |
| 93 | `insert` |  | Y | Y |  | Y |  |  | unknown | 80&#8209;86 |
| 94 | `lookup` |  | Y | Y |  | Y |  | Y |  | 107 |
| 95 | `delete` |  | Y | Y |  | Y |  |  | unknown | 122&#8209;125 |
| 96 | `clone_entry` |  | Y |  |  | Y |  | Y |  | 144 |
| 97 | `resize` |  |  | Y |  | Y |  | Y |  | 440&#8209;443 |
| 98 | `hash_index` |  |  | Y |  | Y |  | Y |  | 590 |


### Legend

- **Trait** = function declared in a `trait` block (with spec).
- **IT** = implemented in `impl Trait for Type` (inherits trait spec).
- **IBI** = implemented in bare `impl Type` (own spec).
- **ML** = module-level free function.
- **V!** = inside `verus!` macro.
- **-V!** = outside `verus!` macro.
- **NoSpec** = no requires/ensures.
- **SpecStr** = spec strength: unknown = has requires/ensures (strength not assessed); hole = contains `assume()`, `admit()`, or `#[verifier::external_body]`.
