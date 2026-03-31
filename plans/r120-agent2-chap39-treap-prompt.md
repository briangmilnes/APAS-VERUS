# R120 Agent 2 — Strengthen BSTTreapMtEph public specs (Chap39). AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 29 warnings on Chap39. R116 (agent2)
lifted internals into the trait and added link-level specs. This round:
strengthen the public function specs to match StEph.

## Warnings

### Wf naming (1 warning)

- `BSTTreapStEph.rs:2740`: wf name `spec_parambsttreapsteph_wf` doesn't
  follow pattern `spec_bsttreap*_wf`. This is a StEph naming issue —
  rename to `spec_bsttreapsteph_wf`. Update all references.

### Missing 23 fns (1 warning)

MtEph missing 23 fns from StEph trait. R116 already lifted 15 (specs,
proofs, read-only exec). The remaining 8 are:

- `spec_bst`, `spec_height`, `spec_min`, `spec_max`, `spec_in_order`,
  `spec_pre_order` — self-level specs that need the tree structure.
  These may not be expressible on MtEph (behind RwLock). Assess and
  document if blocked.
- `new_node`, `update_size` — internal helpers.
- `rotate_left`, `rotate_right`, `insert_link`, `delete_link` — blocked
  by Verus take() bug (documented in R116 report). Skip these.
- `clone_link`, `in_order_vec`, `pre_order_vec` — may be liftable.

### Param type mismatch (1 warning)

- `lemma_wf_assemble_node`: param `&Node<T>` vs StEph `&Box<Node<T>>`.
  This is an intentional type difference. False positive.

### Weak public specs (26 warnings)

**height** (2 warnings): missing requires and ensures entirely.
Add `requires self.spec_bsttreapmteph_wf()` and appropriate ensures.

**new** (1 warning): missing `spec_bst()` ensures.

**insert** (7 warnings):
- Missing requires: `spec_size() + 1 <= usize::MAX`
- Missing 5 ensures: containment preservation, spec_bst preservation,
  spec_contains(value), size bounds (upper and lower)

**delete** (4 warnings):
- Missing requires from StEph
- Missing 3 ensures: containment, spec_bst preservation, size bound

**find** (3 warnings):
- Missing `spec_bst()` requires
- Missing `found.is_some() ==> *found.unwrap() == *target` ensures

**contains** (3 warnings):
- Missing `spec_bst()` requires
- Missing `found == self.spec_contains(*target)` ensures

**minimum** (1 warning): missing match ensures with spec_min

**maximum** (1 warning): missing match ensures with spec_max

## Important context from R116

- The `spec_bst()`, `spec_contains()` etc. spec fns were added to
  `BSTTreapMtEphTrait` in R116 as `spec_size` and `spec_contains`.
  Check which self-level specs exist and which are missing.
- The `take()` bug blocks lifting rotate/insert_link/delete_link.
- MtEph ensures `self@ =~= old(self)@.insert(value@)` for insert,
  which is actually STRONGER than StEph's individual clauses at the
  set level. But compare-par-mut counts clause-by-clause.

## Strategy

1. Read `src/Chap39/BSTTreapMtEph.rs` — full file, know what R116 added.
2. Read `src/Chap39/BSTTreapStEph.rs` — the StEph trait specs.
3. Rename `spec_parambsttreapsteph_wf` in StEph (1 warning).
4. Strengthen public specs: height, new, insert, delete, find, contains,
   minimum, maximum.
5. Add self-level spec fns if expressible, skip if RwLock-blocked.
6. Validate: `scripts/validate.sh isolate Chap39`.
7. RTT: `scripts/rtt.sh Chap39`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from StEph.
- Renaming wf: update ALL references across the file.
- The existing MtEph set-level ensures (e.g. `self@ =~= old(self)@.insert(v@)`)
  are STRONGER than StEph's clause-by-clause specs. Do NOT remove them
  to match StEph. ADD the missing clauses alongside the existing ones.
- No subagents.

## STEP 30

## Report

Write `plans/agent2-r120-chap39-treap-report.md`. Include before/after
warning count.
