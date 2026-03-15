# Chap40 Spec Audit — Augmented Binary Search Trees

## Summary

Constructors and size queries **strong**. Core dictionary ops (find/contains/get/insert/delete) **strengthened to strong in R20** (external_body on impls). Navigation and augmentation ops remain **weak**.

## BSTKeyValueStEph.rs (View = Map<K, V>)

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | size==0, view==empty map | **strong** |
| 2 | size | — | count == spec_size | **strong** |
| 3 | is_empty | — | result == (size==0) | **strong** |
| 4 | height | height < MAX | result == spec_height | **strong** |
| 5 | insert | size < MAX | self@ == old@.insert(k,v), contains, size bounds | **strong** (R20) |
| 6 | delete | wf | self@ == old@.remove(*k), size bounds | **strong** (R20) |
| 7 | find | wf | Some <==> contains_key, value == map[key] | **strong** (R20) |
| 8 | contains | wf | contains == self@.contains_key(*key) | **strong** (R20) |
| 9 | get | wf | Some <==> contains_key, value == map[key] | **strong** (R20) |
| 10 | keys | wf | len == size | **weak** |
| 11 | values | wf | len == size | **weak** |
| 12 | minimum_key | wf | empty→None, non-empty→Some, matches spec_min_key | **partial** |
| 13 | maximum_key | wf | empty→None, non-empty→Some, matches spec_max_key | **partial** |

## BSTSizeStEph.rs (View = Set<T>)

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | size==0, wf, view==empty set | **strong** |
| 2 | size | — | count == spec_size | **strong** |
| 3 | is_empty | — | result == (size==0) | **strong** |
| 4 | height | size<MAX, wf | result == spec_height | **strong** |
| 5 | insert | size+1<=MAX, wf | self@ == old@.insert(v), wf, size bounds | **strong** (R20) |
| 6 | delete | wf | self@ == old@.remove(*k), wf, size bounds | **strong** (R20) |
| 7 | find | wf | Some <==> self@.contains(*target) | **strong** (R20) |
| 8 | contains | wf | contains == self@.contains(*target) | **strong** (R20) |
| 9 | minimum | wf | empty→None, non-empty→Some | **weak** |
| 10 | maximum | wf | empty→None, non-empty→Some | **weak** |
| 11 | in_order | wf | len == size | **weak** |
| 12 | rank | size<MAX, wf | rank <= size | **weak** |
| 13 | select | — | (rank==0 or rank>size) → None | **weak** |
| 14 | split_rank | wf | wf on both results | **weak** |

## BSTReducedStEph.rs (View = Map<K, V>)

| # | Function | requires | ensures | Classification |
|---|----------|----------|---------|----------------|
| 1 | new | — | size==0, wf, view==empty map | **strong** |
| 2 | size | — | count == spec_size | **strong** |
| 3 | is_empty | — | result == (size==0) | **strong** |
| 4 | height | height<MAX | result == spec_height | **strong** |
| 5 | insert | size+1<=MAX, wf | self@ == old@.insert(k,v), wf, size bounds | **strong** (R20) |
| 6 | delete | wf | self@ == old@.remove(*k), wf, size bounds | **strong** (R20) |
| 7 | find | wf | Some <==> contains_key, value == map[key] | **strong** (R20) |
| 8 | contains | wf | contains == self@.contains_key(*key) | **strong** (R20) |
| 9 | get | wf | Some <==> contains_key, value == map[key] | **strong** (R20) |
| 10 | keys | wf | len == size | **weak** |
| 11 | values | wf | len == size | **weak** |
| 12 | minimum_key | wf | empty→None, non-empty→Some | **weak** |
| 13 | maximum_key | wf | empty→None, non-empty→Some | **weak** |
| 14 | reduced_value | wf | empty → identity | **weak** |
| 15 | range_reduce | wf | empty → identity | **weak** |

## R20 Changes

Strengthened core dictionary operations (insert/delete/find/contains/get) across all three files to encode full map/set semantics. Added external_body to impl methods (+15 holes) since proofs need BST invariant connection to View.

## Remaining Gaps

- **keys/values**: Should ensure sorted order and domain/range correspondence with abstract map.
- **minimum_key/maximum_key**: Should ensure TotalOrder minimality/maximality over the key domain.
- **rank**: Should ensure rank == |{x in self@ | TotalOrder::le(x, key)}| (APAS Algorithm 40.1).
- **select**: Should ensure select returns the element with the given rank.
- **split_rank**: Should ensure partition by rank: left has elements with rank < r, right has rank >= r.
- **reduced_value**: Should ensure result == Op::spec_combine over all values in tree.
- **range_reduce**: Should ensure result == Op::spec_combine over values in key range [low, high].
