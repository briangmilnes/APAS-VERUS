# Agent 2 — Round 26: Chap40 Delete + Chap39 BSTTreap Proofs

## R25 Feedback

Good work proving insert in BSTKeyValueStEph and BSTReducedStEph. The Map algebraic
lemma chain and the `reveal_with_fuel(spec_ordered_link, 2)` technique are solid. The
trait strengthening (adding `spec_ordered_link` to rotation ensures) was the right call.

Delete is the remaining challenge. Your report identified the blocker: collect-filter-rebuild
pattern needs content specs on `collect_in_order_kvp`, `filter_by_key_kvp`,
`build_treap_from_vec`. That's your entry point this round.

## Mission

1. Prove delete in all 3 Chap40 files (3 holes — the last holes in Chap40)
2. Prove find and insert_link in Chap39 BSTTreapStEph.rs (2 holes)

## Part 1: Chap40 Delete (Priority 1)

### Current State (3 holes)

| # | Chap | File | Function | Blocker |
|---|------|------|----------|---------|
| 1 | 40 | BSTKeyValueStEph.rs | `delete` | Content specs on collect/filter/rebuild |
| 2 | 40 | BSTReducedStEph.rs | `delete` | Same pattern |
| 3 | 40 | BSTSizeStEph.rs | `delete` | Same pattern |

### Delete Strategy

All three files use the same delete pattern: find the key, then restructure. Read the
actual delete bodies to understand which variant they use:

**Variant A: In-place removal** (find node, replace with child or successor)
- Find node with target key
- If leaf: remove
- If one child: replace with child
- If two children: find in-order successor (min of right), swap, delete successor
- Ordering: successor is > all left keys and < all remaining right keys

**Variant B: Collect-filter-rebuild** (your R25 report suggests this)
- Collect all elements in order
- Filter out the target key
- Rebuild tree from filtered sequence
- Ordering: filtering preserves sortedness; building from sorted produces BST

For Variant B, the proof chain is:
1. `collect_in_order_kvp` produces a sorted sequence matching tree content
2. `filter_by_key_kvp` removes one element while preserving sortedness
3. `build_treap_from_vec` from a sorted sequence produces a valid BST

You need to add ensures to each helper:
- `collect_in_order_kvp`: result is sorted, result's content == tree's content
- `filter_by_key_kvp`: result is sorted (if input sorted), result == input minus target
- `build_treap_from_vec`: result is valid BST with content matching input

### Proof Techniques (from your R25 toolkit)

- `reveal_with_fuel(spec_ordered_link, 2)` for ordering through structural changes
- Ghost capture before mutations
- Map algebraic lemma chain for content preservation
- `obeys_cmp_spec` / `TotalOrder` reasoning per file's convention

### What Would Close Chap40

Proving all 3 deletes closes Chap40 completely (0 holes). Chap40 has clean deps — no
external blockers.

## Part 2: Chap39 BSTTreapStEph (Priority 2)

### Current State (2 holes)

| # | Chap | File | Function | Type |
|---|------|------|----------|------|
| 1 | 39 | BSTTreapStEph.rs | `find` | external_body |
| 2 | 39 | BSTTreapStEph.rs | `insert_link` | external_body |

These are the same BST operations you proved in Chap40. The treap adds random priorities
but the BST ordering invariant is the same — find uses key comparison, insert maintains
ordering through rotations.

### find Strategy

Read `BSTTreapStEph.rs` line 519. `find` searches by key comparison — identical to the
find you proved in Chap40. The treap priority doesn't affect find at all.

Proof: `spec_is_bst_link` (or equivalent) ensures keys are ordered. Recurse left/right
based on comparison. The found element matches the target key.

### insert_link Strategy

Read `BSTTreapStEph.rs` line 748. Insert in a treap: BST insert by key, then rotate up
to restore heap property on priorities. The BST ordering is maintained through rotations
(same proof technique as Chap40 insert). The heap property on priorities is a separate
invariant.

You may need to prove only the BST ordering part (key correctness) and leave the priority
heap property for later if it's complex.

### What This Enables

BSTTreapStEph has 16 clean proof functions. Proving find + insert_link takes it from 2
holes to 0, closing the file. BSTTreapMtEph (6 assume holes) depends on BSTTreapStEph
being correct — closing StEph strengthens the foundation.

## Part 3: Remaining `requires true` in Chap39 (Bonus)

BSTTreapMtEph.rs has 5 `requires true` (lines 302, 338, 802, 832, 857) on:
- `clone_link` — no ordering needed, but add ensures (content preservation)
- `size_link` — no ordering needed, add ensures `result == spec_size_link`
- `find_link` — needs BST ordering precondition
- `min_link` — needs BST ordering precondition
- `max_link` — needs BST ordering precondition

Replace with real specs. Read BSTTreapStEph.rs for the corresponding specs.

## Important

- You MAY add requires/ensures and strengthen specs.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`. Omit requires if genuinely no precondition.
- Partial progress is real — if delete proves in 1 or 2 files but not all 3, commit the wins.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- Delete proved in BSTKeyValueStEph, BSTReducedStEph, BSTSizeStEph (3 holes)
- find + insert_link proved in BSTTreapStEph (2 holes)
- `requires true` replaced in BSTTreapMtEph (5 instances, bonus)
- `plans/agent2-round26-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
