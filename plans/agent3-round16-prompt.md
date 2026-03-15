# Agent 3 — Round 16

## Project State

136 holes, 4097 verified, 38 clean chapters, 8 holed.

## Your Assignment: Chap39 (21 holes) + Chap38/BSTParaMtEph (17 ext_body)

### Chap39/BSTTreapMtEph.rs (6 assume)

This is an Mt wrapper around BSTTreapStEph (treap = tree + heap priority).
All 6 holes are `assume(...)` that bridge ghost state after lock operations.

**How to fix each assume:**
After `acquire_read`/`acquire_write`, the RwLock invariant guarantees
`ghost_locked_root@ == locked_val@`. The inner StEph method's `ensures` gives
the property on `locked_val`. Chain through the invariant to prove the assume.

Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` for the pattern.
Run `scripts/holes.sh src/Chap39/BSTTreapMtEph.rs` to see exact lines.

### Chap39/BSTParaTreapMtEph.rs (15 external_body)

Mt wrapper around the parallel treap. Inner BSTParaTreapStEph is fully proved (0 holes).

Same lock-call-bridge pattern:
1. Remove `#[verifier::external_body]`
2. Acquire lock, call inner method, bridge ghost state, release lock
3. For collection functions: use collect+while loop (NOT `for` loops)

### Chap38/BSTParaMtEph.rs (17 external_body)

Mt wrapper around BSTParaStEph (parallel BST). Inner StEph has 5 structural holes
(clone/usize::MAX) but is otherwise proved.

Same pattern as above. The 17 `external_body` stubs are Mt wrapper functions:
`new`, `insert`, `delete`, `find`, `size`, `union`, `intersect`, `difference`,
`previous_key`, `next_key`, `rank_key`, `collect`, `in_order`, `pre_order`, `clone`, etc.

## DO NOT TOUCH

- Chap43 (Agent 2)
- Chap42 or Chap41/MtPer (Agent 1)
- Chap37, Chap47, Chap45, Chap41/MtEph, Chap41/StEph (Agent 4)
- BSTParaStEph.rs (5 remaining holes are structural — clone and usize::MAX)
- Any Example files

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **NO accept().** NO assume→accept conversion.
- **DO NOT weaken ensures.** Prove the existing postconditions. Leave `external_body`
  if you cannot prove them. Never delete ensures to make proofs pass.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Push to `agent3/ready`. Write `plans/agent3-round16-report.md`.

## Target: -12 (stretch -25). Close Chap39.
