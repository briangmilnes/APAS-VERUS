# R157 Agent 1 Report — OrdKeyMap domain/tabulate/restrict/subtract

## Summary

Added four new operations to OrdKeyMap that use ArraySetStEph, now possible because
OrdKeyMap moved from Chap38 to Chap41 (eliminating circular dependency).

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 41 | OrdKeyMap.rs | Added `domain`, `tabulate`, `restrict`, `subtract` to trait + impl |
| 2 | 41 | OrdKeyMap.rs | Added imports: `Chap19::ArraySeqStEph`, `Chap41::ArraySetStEph` |
| 3 | 41 | TestOrdKeyMap.rs | Added 11 RTTs for the four new operations |

## New Operations

| # | Chap | File | Operation | Signature | Work/Span |
|---|------|------|-----------|-----------|-----------|
| 1 | 41 | OrdKeyMap.rs | `domain` | `&self -> ArraySetStEph<K>` | O(n) / O(n) |
| 2 | 41 | OrdKeyMap.rs | `tabulate` | `keys: &ArraySetStEph<K>, f: &F -> Self` | O(n lg n) / O(n lg n) |
| 3 | 41 | OrdKeyMap.rs | `restrict` | `&self, keys: &ArraySetStEph<K> -> Self` | O(n*m) / O(n*m) |
| 4 | 41 | OrdKeyMap.rs | `subtract` | `&self, keys: &ArraySetStEph<K> -> Self` | O(n*m) / O(n*m) |

## Verification

- Isolate Chap41: 2280 verified, 0 errors
- Full validate: 5772 verified, 0 errors
- RTT: 3763 passed (3752 → 3763, +11 new)
- No assumes, accepts, or external_body added

## Approach

Proof patterns copied from `Chap43/OrderedTableStEph.rs` which has identical operations.
Adapted for OrdKeyMap's direct `self.inner` access (vs OrderedTableStEph's `self.tree.inner`).
Key proof infrastructure:
- `lemma_view_gen_subset` for restrict/subtract (new tree is subset of old)
- `lemma_view_gen_insert` for tabulate (fresh pairs inserted)
- `lemma_sorted_keys_pairwise_distinct` for key freshness in loops
- `lemma_key_unique_insert` for key uniqueness preservation
