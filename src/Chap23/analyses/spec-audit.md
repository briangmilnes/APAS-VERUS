# Chap23 Spec Audit — Tree Sequences

Audited: 2026-03-15, Agent 4, Round 19.
Prose source: prompts/Chap23.txt (Data Type 23.1, Cost Spec 23.2, Algorithm 23.3).

## Summary

2 files, 0 holes, all specs **strong**. No changes needed.

Chapter 23 defines the Primitive Tree Sequence ADT (Data Type 23.1) with `expose` and
`join` as the core operations, then implements the full sequence ADT on top (Algorithm
23.3). The APAS-VERUS implementation provides:

- `BalBinTreeStEph.rs`: Balanced binary tree with traversals (in/pre/post order).
- `PrimTreeSeqStPer.rs`: Primitive tree sequence backed by Vec, implementing all
  Algorithm 23.3 operations.

## Per-File Classification

| # | File | Holes | Fns | Classification | Notes |
|---|------|:-----:|:---:|:--------------:|-------|
| 1 | BalBinTreeStEph.rs | 0 | 10 | Strong | Tree structure + traversals |
| 2 | PrimTreeSeqStPer.rs | 0 | 16 | Strong | DT 23.1 + Alg 23.3 |

## Spec-vs-Prose Detail

### BalBinTreeStEph.rs

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `leaf` | size=0, height=0, traversals=[] | Def: empty tree | Strong |
| 2 | `node` | size=1+l+r, height, traversals | Def: tree node | Strong |
| 3 | `is_leaf` | b == (size==0) | - | Strong |
| 4 | `size` | count == spec_size | Def 6.24: \|T\| | Strong |
| 5 | `height` | h == spec_height | Def 6.25: tree height | Strong |
| 6 | `in_order` | result == spec_in_order | L,root,R traversal | Strong |
| 7 | `pre_order` | result == spec_pre_order | root,L,R traversal | Strong |
| 8 | `post_order` | result == spec_post_order | L,R,root traversal | Strong |

Plus two proof lemmas (traversal permutations) with correct ensures.

### PrimTreeSeqStPer.rs

| # | Function | Ensures | Prose | Strength |
|---|----------|---------|-------|:--------:|
| 1 | `empty` | len=0 | Alg 23.3: empty | Strong |
| 2 | `singleton` | len=1, index(0)=value | Alg 23.3: singleton | Strong |
| 3 | `from_vec` | len, pointwise index eq | N/A (Verus utility) | Strong |
| 4 | `length` | len == spec_len | CS 23.2: O(1) | Strong |
| 5 | `nth` | result == spec_index(i) | Alg 23.3: nth | Strong |
| 6 | `expose` | Zero/One/Two with split | DT 23.1: expose | Strong |
| 7 | `join` | Zero->empty, One->single, Two->concat | DT 23.1: join | Strong |
| 8 | `append` | len, pointwise index eq | Alg 23.3: append | Strong |
| 9 | `subseq` | len=length, index shift | Alg 23.3: subseq | Strong |
| 10 | `update` | len preserved, index updated | Alg 23.3: update | Strong |
| 11 | `map` | len preserved, f applied | Alg 23.3: map | Strong |
| 12 | `tabulate` | len=length, f(i) at each index | Alg 23.3: tabulate | Strong |
| 13 | `filter` | multiset filter, predicate holds | Alg 23.3: filter | Strong |
| 14 | `drop` | len=n-drop, index shift | Alg 23.3: drop | Strong |
| 15 | `flatten` | vec flatten of inner sequences | Alg 23.3: flatten | Strong |
| 16 | `as_slice` / `into_vec` | view equality | Utility | Strong |

Key observations:
- `expose` correctly handles the three cases from DT 23.1 (Zero, One, Two) with
  the split-concatenation invariant: `Two.0 + Two.1 == self@`.
- `join` is the inverse of `expose`, matching DT 23.1.
- `filter` has a multiset-based ensures (filtered elements are a multiset-filter
  of the input), which is stronger than just length bounds.

## Verdict

No spec changes needed. All Chap23 specs faithfully encode DT 23.1 and Algorithm 23.3.
