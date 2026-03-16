# Agent 1 — Round 25: Fix `requires true` with Real Invariants + Chap45

## Mission

Your R24 added `requires true` to 33 functions. That's wrong — `requires true` is a
vacuous precondition that says nothing. This round: replace every `requires true` you
added with the **real** well-formedness predicate, then prove Chap45 BinaryHeapPQ::find_min.

**New CLAUDE.md rule (read it):** "DO NOT ADD `requires true` TO FIX `fn_missing_requires`
WARNINGS." The fix is the real precondition — typically `spec_<module>_wf()` on `&self`
methods, or structural predicates on `Link<T>` arguments.

## Part 1: BSTSplayMtEph.rs — Replace 17 `requires true` (Priority 1)

The module already has `spec_bstsplaymteph_wf` (line 609) used correctly by trait-level
functions. But 17 internal Layer 1 helpers have `requires true` instead of real predicates.

### What Each Helper Should Require

These are BST operations on `Link<T>` (Option<Box<Node<T>>>). They need the BST ordering
invariant on their link argument. The StEph counterpart (BSTSplayStEph.rs) already has
the real specs proved in R23 — **read BSTSplayStEph.rs and copy the corresponding
requires/ensures**.

Key functions and their likely real requires:

| # | Function | Real Requires |
|---|----------|---------------|
| 1 | `new_node(v)` | none needed (constructor) — but add ensures about result |
| 2 | `size_link(link)` | none needed (pure traversal) — but add ensures `result == spec_size_link(link)` |
| 3 | `update(root)` | none needed (size recalculation) |
| 4 | `splay(root, v)` | `spec_is_bst_link(&Some(root))` |
| 5 | `bst_insert(link, v)` | `spec_is_bst_link(link)` |
| 6 | `insert_link(link, v)` | `spec_is_bst_link(link)` |
| 7 | `find_link(link, v)` | `spec_is_bst_link(link)` |
| 8 | `min_link(link)` | `spec_is_bst_link(&Some(link))` |
| 9 | `max_link(link)` | `spec_is_bst_link(&Some(link))` |
| 10 | `in_order_collect(link)` | none (traversal) — add ensures about result length |
| 11 | `pre_order_collect(link)` | none (traversal) — add ensures about result length |
| 12 | `in_order_parallel(link)` | none (traversal) |
| 13 | `pre_order_parallel(link)` | none (traversal) |
| 14 | `build_balanced(sorted)` | `sorted.len() > 0` or similar |
| 15 | `filter_parallel(link, f)` | closure requires per standard |
| 16 | `reduce_parallel(link, f, id)` | closure requires per standard |
| 17 | `height_rec(link)` | none (pure traversal) |

**Important**: Mt files must NOT import from St counterparts. The `spec_is_bst_link`
definition should already exist in BSTSplayMtEph.rs — if not, duplicate it from
BSTSplayStEph.rs.

**Process**: For each function:
1. Read the corresponding function in BSTSplayStEph.rs
2. Copy its `requires`/`ensures` to BSTSplayMtEph.rs
3. If the function is trivial and genuinely has no precondition, use NOTHING — leave
   the `requires` clause off entirely rather than writing `requires true`

## Part 2: BSTRBMtEph.rs — Replace 20 `requires true` (Priority 2)

Same pattern. `spec_bstrbmteph_wf` exists at line 608. The StEph counterpart is
BSTRBStEph.rs. 20 internal helpers need real predicates.

Additional RB-tree helpers not in Splay:
- `is_red(link)` — no requires needed (pure check)
- `rotate_left(root)` / `rotate_right(root)` — requires BST ordering
- `flip_colors(root)` — no ordering change
- `fix_up(root)` — requires BST ordering

Read BSTRBStEph.rs and propagate its specs.

## Part 3: BSTSet*MtEph Files — Replace `requires true` (Priority 3)

5 files in Chap37 with 2-3 `requires true` each:
- BSTSetAVLMtEph.rs (3)
- BSTSetBBAlphaMtEph.rs (3)
- BSTSetPlainMtEph.rs (3)
- BSTSetRBMtEph.rs (2)
- BSTSetSplayMtEph.rs (3)

These are thin wrappers. Each has a `spec_<module>_wf` predicate. Replace `requires true`
with the appropriate wf predicate on `&self` methods.

## Part 4: BSTSplayStEph.rs — Fix 5 fn_missing_requires (Priority 4)

5 functions have NO requires at all (not even `requires true`):
- `new_node` (line 251) — add ensures
- `size_link` (line 269) — add ensures
- `update` (line 301) — add ensures
- `in_order_collect` (line 1605) — add ensures
- `pre_order_collect` (line 1619) — add ensures

These are your R23 functions. You know their specs — add the real ensures. Be careful
not to destabilize the splay proof (you noted SMT sensitivity in R24).

## Part 5: Chap45 BinaryHeapPQ::find_min (Priority 5)

Line 662 in BinaryHeapPQ.rs. Returns the root of a min-heap.

Your R24 report said this is structurally blocked because the wf predicate lacks a heap
property and `spec_leq_view` is disconnected from `TotalOrder::le`. The fix:

1. **Strengthen `spec_binaryheappq_wf`** to include the heap ordering property:
   parent ≤ children for all nodes.
2. **Connect `spec_leq_view` to `TotalOrder::le`** in the spec or as a requires.
3. Remove `external_body` from `find_min` and prove from the strengthened invariant.

This is spec strengthening work — exactly what this project needs.

## Priority Order

1. BSTSplayMtEph.rs — replace 17 requires true with real specs
2. BSTRBMtEph.rs — replace 20 requires true with real specs
3. BSTSet*MtEph files — replace ~14 requires true
4. BSTSplayStEph.rs — fix 5 fn_missing_requires
5. BinaryHeapPQ::find_min — strengthen wf and prove

## Important

- You MAY add requires/ensures and strengthen wf predicates — that's the goal.
- Do NOT add `requires true`. If a function genuinely needs no precondition, omit requires.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- Mt files must NOT import from St counterparts. Duplicate shared specs.
- Read `src/standards/helper_function_placement_standard.rs` — Layer 1 free functions
  use `pub(crate)`, real requires/ensures. Layer 2 trait methods wrap with RwLock.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- All `requires true` replaced with real predicates in BSTSplayMtEph, BSTRBMtEph, BSTSet*MtEph
- fn_missing_requires fixed in BSTSplayStEph.rs
- BinaryHeapPQ::find_min proved (if time permits after the requires work)
- `plans/agent1-round25-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
