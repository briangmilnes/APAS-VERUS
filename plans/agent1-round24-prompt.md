# Agent 1 — Round 24: BSTSplayMtEph Specs + Chap45 Priority Queues

## Mission

You proved BSTSplayStEph in R23 — all 7 holes closed, 0 external_body, 0 assume, real
BST specs. Excellent work. Now propagate those specs to the Mt wrapper, fix fn_missing
warnings in Chap37, and prove Chap45 priority queue holes.

## What You Proved in R23 (BSTSplayStEph.rs)

Your R23 work established:
- `spec_is_bst_link` — BST ordering invariant using `TotalOrder::le` and `spec_contains_link`
- `spec_bstsplaysteph_wf` now = `spec_is_bst_link(&self.root)` (was `{ true }`)
- `splay` — BST + content preservation through all 6 rotation cases (zig, zig-zig, zig-zag
  and mirrors). Pre-mutation ghost capture technique.
- `bst_insert` — standard BST insert with ordering preservation
- `insert_link` — bst_insert + splay
- `find_link` — BST search with completeness (uses `T::antisymmetric` to rule out wrong subtree)
- `min_link` / `max_link` — leftmost/rightmost descent with `T::transitive`
- `in_order_collect` / `pre_order_collect` — traversals with length ensures
- `lemma_bst_deep_link` — exposes BST facts two levels deep with `reveal_with_fuel(3)`

Key proof patterns you used:
- `reveal_with_fuel(spec_is_bst_link, N)` for N=2,3,4 depending on depth
- `reveal_with_fuel(spec_contains_link, N)` for element tracking
- `T::transitive(a, b, c)` and `T::antisymmetric(a, b)` from TotalOrder
- Ghost capture before `.take()`: `let ghost orig_root_left = root.left;`

## Part 1: BSTSplayMtEph.rs — Propagate Specs (fn_missing_requires)

BSTSplayMtEph.rs has 17 `fn_missing_requires` warnings on Layer 1 helper functions. These
are copies of the same functions you proved in BSTSplayStEph.rs but without `requires`
clauses. The fix:

1. Read BSTSplayMtEph.rs.
2. For each Layer 1 helper that mirrors a BSTSplayStEph function, add the same
   `requires`/`ensures` from your StEph proof.
3. The proof bodies should be nearly identical — copy and adapt.

Target functions (all in BSTSplayMtEph.rs):
- `new_node`, `size_link`, `update` — scaffolding, add ensures
- `splay` — add `requires spec_is_bst_link`, `ensures` BST + content preservation
- `bst_insert`, `insert_link` — add requires/ensures matching StEph
- `find_link`, `min_link`, `max_link` — add requires/ensures
- `in_order_collect`, `pre_order_collect` — add ensures
- `in_order_parallel`, `pre_order_parallel` — parallel versions, add ensures
- `build_balanced`, `filter_parallel`, `reduce_parallel`, `height_rec` — add requires

**Important**: Mt files must NOT import from St counterparts. Duplicate specs locally.
The `spec_is_bst_link` and `spec_contains_link` definitions should already exist in the
Mt file (or need to be added).

## Part 2: BSTRBMtEph.rs — Same Pattern (fn_missing_requires)

BSTRBMtEph.rs has 19 fn_missing_requires warnings. Same approach: read the clean
BSTRBStEph.rs and propagate its requires/ensures to the Mt wrapper functions.

## Part 3: Chap37 Remaining Real Holes

| # | File | Holes | Function | Notes |
|---|------|:-----:|----------|-------|
| 1 | AVLTreeSeq.rs | 1 | Iterator `next` | May be structurally hard |
| 2 | AVLTreeSeqMtPer.rs | 2 | `build_balanced_from_slice`, `subseq_copy` | Thread boundary |

These are harder. Attempt them but don't burn the whole round on them.

## Part 4: Chap45 Priority Queues

Skip Example files (Example45_2.rs, HeapsortExample.rs). Three real targets:

### BinaryHeapPQ::find_min (trivial)

Line 662. Returns the root of a min-heap. The heap property guarantees root = minimum.
Remove `external_body`, prove from the heap invariant in `spec_binaryheappq_wf`.

### BinaryHeapPQ::extract_all_sorted (hard)

Line 975. `assume(Self::spec_sorted(result.seq@))` inside the extraction loop. Proving
sortedness requires showing each `delete_min` returns the next smallest element. This
needs the heap invariant maintained through each extraction step. Attempt if time permits
but this is genuinely hard — multiset reasoning + heap property across deletions.

### BalancedTreePQ::insert (medium)

Line 223. Uses `AVLTreeSeqStPer` as backing. The insert does a linear scan for position,
then rebuilds via `from_vec`. Depends on AVLTreeSeqStPer being clean enough to expose the
relevant specs.

### BalancedTreePQ external trait impl (skip for now)

Line 571. Entire `BalancedTreePQExtTrait` impl is external. Filter/partition/retain. Lower
priority.

## Priority Order

1. BSTSplayMtEph.rs fn_missing_requires (propagate your StEph proofs)
2. BSTRBMtEph.rs fn_missing_requires (propagate from BSTRBStEph)
3. BinaryHeapPQ::find_min (likely trivial)
4. AVLTreeSeq.rs iterator next (attempt)
5. BinaryHeapPQ::extract_all_sorted (attempt if time)
6. BalancedTreePQ::insert (attempt if time)

## Nipkow Splay Reference (for your information)

Your R23 splay proof closely mirrors Nipkow's Isabelle formalization from the Archive
of Formal Proofs. Key parallels:
- Nipkow: `inorder(splay x t) = inorder t` (content preservation via inorder)
- You: `forall|x: T| spec_contains_link(&Some(result), x) <==> spec_contains_link(&Some(root), x)`
- Nipkow: `bst t ==> bst (splay x t)` (BST preservation)
- You: `spec_is_bst_link(&Some(root)) ==> spec_is_bst_link(&Some(result))`
- Nipkow: `inv = λ_. True` (no structural invariant beyond BST)
- You: `spec_bstsplaysteph_wf = spec_is_bst_link` (same insight)

Reference files in `prompts/splaytree/` if you need them for the Mt wrapper.

## Important

- You MAY add requires/ensures and strengthen wf predicates.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Mt files must NOT import from St counterparts. Duplicate shared specs.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- fn_missing_requires fixed in BSTSplayMtEph.rs and BSTRBMtEph.rs
- Proven holes in Chap45 (at minimum BinaryHeapPQ::find_min)
- `plans/agent1-round24-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
