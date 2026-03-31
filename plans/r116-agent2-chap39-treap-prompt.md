# R116 Agent 1 — Lift BSTTreapMtEph internals into trait, strengthen specs. AFK.

## Problem

BSTTreapMtEph has all the core tree algorithms implemented as free functions
outside the trait: `rotate_left`, `rotate_right`, `insert_link`, `delete_link`,
`find_link`, `min_link`, `max_link`, `height_link`, `in_order_collect`,
`pre_order_collect`, `size_link`, `update`, `clone_link`. The StEph variant
puts these in the trait with full specs so they're testable and verified at
the trait level.

`veracity-compare-par-mut` reports 28 warnings: 38 missing functions plus
weak specs on every public function.

## Objective

1. Lift the internal algorithms into `BSTTreapMtEphTrait`.
2. Add missing spec functions to the trait.
3. Strengthen public function specs to match StEph.

## What StEph has in its trait that MtEph doesn't

### Spec functions (8 missing from trait)

StEph `BSTTreapStEphTrait` declares:
- `spec_in_order_link(link: &Link<T>) -> Seq<T>`
- `spec_pre_order_link(link: &Link<T>) -> Seq<T>`
- `spec_min_link(link: &Link<T>) -> Option<T>`
- `spec_max_link(link: &Link<T>) -> Option<T>`
- `spec_size(self) -> nat`
- `spec_bst(self) -> bool`
- `spec_height(self) -> nat`
- `spec_contains(self, target: T) -> bool`
- `spec_min(self) -> Option<T>`
- `spec_max(self) -> Option<T>`
- `spec_in_order(self) -> Seq<T>`
- `spec_pre_order(self) -> Seq<T>`

MtEph `LinkTrait` has 4 of the link-level specs (`spec_size_link`,
`spec_bst_link`, `spec_link_size_wf`, `spec_height_link`) but missing
the rest. MtEph `BSTTreapMtEphTrait` has none of the self-level specs.

### Proof lemmas (8 missing from trait)

StEph trait has:
- `lemma_height_le_size`
- `lemma_size_wf_child_bounded`
- `lemma_wf_decompose`
- `lemma_wf_assemble_node`
- `lemma_contains_left`
- `lemma_contains_right`
- `lemma_bst_decompose`
- `lemma_contains_root`

MtEph has some of these as free proof fns (lines 122-229) but not in the
trait.

### Exec functions (10 missing from trait)

These exist as free functions in MtEph but are not in the trait:
- `new_node` — not present, constructed inline
- `size_link` (line 388) — free fn
- `update` (line 398) — free fn
- `rotate_left` (line 416) — free fn
- `rotate_right` (line 490) — free fn
- `clone_link` (line 352) — free fn
- `height_link` (line 967) — free fn
- `insert_link` (line 566) — free fn
- `delete_link` (line 671) — free fn
- `find_link` (line 855) — free fn
- `min_link` (line 919) — free fn
- `max_link` (line 944) — free fn
- `in_order_collect` / `pre_order_collect` (lines 996, 1010) — free fns

### Weak public specs (the remaining warnings)

After lifting internals, strengthen the public trait specs:

| Function | Missing from MtEph | StEph has |
|----------|-------------------|-----------|
| `new` | `spec_bst()` ensures | `empty_tree.spec_bst()` |
| `insert` | wf requires, size+1 bound, 4 ensures | `spec_bst` preserved, `spec_contains(value)`, size bounds |
| `delete` | wf requires, len bound, 3 ensures | `spec_bst` preserved, containment, size bound |
| `find` | `spec_bst` requires, 1 ensures | `found.is_some() ==> *found.unwrap() == *target` |
| `contains` | `spec_bst` requires, ensures | `found == self.spec_contains(*target)` |
| `height` | everything | requires wf, ensures `h == self.spec_height()` |
| `minimum` | 1 ensures | match pattern with `spec_min` |
| `maximum` | 1 ensures | match pattern with `spec_max` |
| `in_order` | 1 ensures | `forall|v| self@.contains(v) <==> seq@.contains(v)` (needs spec_in_order) |

## Strategy

1. Read both files fully.
2. Add missing spec fns to `BSTTreapMtEphTrait` (or `LinkTrait`). Copy
   the open spec fn bodies from StEph, adapting `StT` → `StTInMtT`.
3. Move proof lemmas into the trait. They're already free fns in MtEph —
   just needs signature + body moved into the trait.
4. Move exec free functions into the trait. The implementations exist —
   they just need trait signatures with specs.
5. Strengthen public function specs.
6. Validate after each major group (specs, lemmas, exec lift, public specs).

## Important

- The free functions already have specs (requires/ensures). When you move
  them into the trait, KEEP their existing specs and strengthen to match
  StEph where the MtEph spec is weaker.
- Mt standalone: do NOT import from StEph. Copy spec fn bodies.
- The `LinkTrait` currently holds 4 spec fns. You may expand it or put
  new specs directly in `BSTTreapMtEphTrait` — follow whichever pattern
  StEph uses.
- StEph uses `StT + Ord + IsLtTransitive`, MtEph uses
  `StTInMtT + Ord + IsLtTransitive`. Adapt bounds accordingly.

## Read first

- `src/Chap39/BSTTreapStEph.rs` — full file, especially the trait (lines 410-520)
- `src/Chap39/BSTTreapMtEph.rs` — full file
- `tests/Chap39/TestBSTTreapStEph.rs` — to understand what's tested

## Validate

Use `scripts/validate.sh isolate Chap39`.
Run `scripts/rtt.sh Chap39` after.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from StEph.
- No subagents.
- Commit working intermediate states.
- If moving a function into the trait breaks verification, report what
  broke and why. Do NOT revert — leave the corpse.

## STEP 40

## Report

Write `plans/agent1-r116-chap39-treap-report.md`. Include before/after
warning counts from `veracity-compare-par-mut --chapter Chap39`.
