# Agent 2 — Round 25 Report: Chap40 Insert with Real Ordering Proofs

## Mission

Prove `insert` in BSTKeyValueStEph.rs and BSTReducedStEph.rs by removing
`#[verifier::external_body]` and writing real ordering/content proofs through
rotations. Remove vacuous `requires true` across all 3 Chap40 BST files.
Assess `delete` proofs.

## Results

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|:---:|:---:|:---:|
| 1 | 40 | BSTKeyValueStEph.rs | 2 | 1 | -1 |
| 2 | 40 | BSTReducedStEph.rs | 2 | 1 | -1 |
| 3 | 40 | BSTSizeStEph.rs | 1 | 1 | 0 |
| | | **Total** | **5** | **3** | **-2** |

### Holes Removed

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 40 | BSTKeyValueStEph.rs | `insert` | Removed external_body; proved content/ordering through rotations using TotalOrder + Map algebraic lemmas |
| 2 | 40 | BSTReducedStEph.rs | `insert` | Removed external_body; proved content/ordering through rotations using cmp_spec + Map algebraic lemmas |

### `requires true` Removed

| # | Chap | File | Function |
|---|------|------|----------|
| 1 | 40 | BSTKeyValueStEph.rs | `clone_link` | (previous session) |
| 2 | 40 | BSTKeyValueStEph.rs | `compare_kv_links` | (previous session) |
| 3 | 40 | BSTReducedStEph.rs | `clone_link` | |
| 4 | 40 | BSTReducedStEph.rs | `compare_reduced_links` | |
| 5 | 40 | BSTSizeStEph.rs | `compare_links` | |
| 6 | 40 | BSTSizeStEph.rs | `clone_link` | |

### Remaining Holes (3)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 40 | BSTKeyValueStEph.rs | `delete` | Needs content specs on `collect_in_order_kvp`, `filter_by_key_kvp`, `build_treap_from_vec` |
| 2 | 40 | BSTReducedStEph.rs | `delete` | Same collect-filter-rebuild pattern |
| 3 | 40 | BSTSizeStEph.rs | `delete` | Same pattern |

### Trait Signature Strengthening

- **BSTReducedStEph.rs**: `rotate_left`, `rotate_right`, `insert_link` — added
  `spec_ordered_link`, `spec_content_link`, `obeys_cmp_spec` to requires/ensures
- **BSTReducedStEph.rs**: `update_node` — added `node.value == old(node).value` to ensures
- **BSTReducedStEph.rs**: `insert` — added `obeys_cmp_spec` + equal-implies-equal to requires

## Verification

```
verification results:: 4085 verified, 0 errors
RTT: 2613 tests run: 2613 passed
PTT: 147 tests run: 147 passed
```

## Techniques

1. **Map algebraic lemmas for rotation content**: `union_prefer_right` is NOT commutative,
   so rotation content equality requires a chain of algebraic identities:
   `upr_insert_commute → upr_assoc → insert_commute → insert_upr_commute`.
   Shared pattern across BSTKeyValueStEph and BSTReducedStEph.

2. **reveal_with_fuel(spec_ordered_link, 2) instead of lemma_ordered_assemble_kv**:
   The assembler lemma was flaky in BSTKeyValueStEph (all four sub-assertions pass but
   conjunction fails due to SMT trigger interference). Revealing the open spec directly
   lets the solver fold components without the conjunction issue.

3. **cmp_spec proofs vs TotalOrder**: BSTReducedStEph uses `cmp_spec` while
   BSTKeyValueStEph uses `TotalOrder`. For cmp_spec, added transitivity lemmas
   (`lemma_cmp_transitivity_lt_reduced`, `lemma_cmp_transitivity_gt_reduced`) and
   used `reveal(obeys_partial_cmp_spec_properties)` to prove `key != node_key`.

4. **Ordering fact capture before mutations**: Must capture all four components of
   `spec_ordered_link` as explicit assertions BEFORE `take()` mutations, since the
   solver loses track of recursive ordering facts through structural changes.

5. **Trait method reveal_with_fuel limitation**: `reveal_with_fuel` does not work on
   trait methods (e.g., `Lnk::spec_content_link`). Since these are `open spec fn`,
   the solver auto-unfolds them. The `update_node` value preservation ensure was the
   real missing piece for content proofs in BSTReducedStEph.
