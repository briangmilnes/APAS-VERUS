# Agent 3 — Round 15

## Status: 149 holes, 4078 verified, 38 clean chapters.

Round 14: you proved 10 BSTParaStEph holes with the T::V witness breakthrough. Now apply
your tree proof expertise to Chap39 (treap variants).

## Your files

### Chap39/BSTTreapMtEph.rs (8 assume)

All 8 are `assume(...)` in the Mt wrapper methods that bridge ghost state after lock
operations. These follow the same pattern as the Chap41 MtEph assumes you've seen:

1. `assume(self@.contains(value_view))` — insert postcondition
2. `assume(!self@.contains(target@))` — delete postcondition
3. `assume(result.is_some() <==> self@.contains(target@) ...)` — find postcondition
4. `assume(result as nat == self@.len() && self@.finite())` — size postcondition
5. `assume(result.is_some() ==> self@.contains(...))` — previous_key postcondition
6. `assume(result.is_some() ==> self@.contains(...))` — next_key postcondition
7. `assume(ordered@.len() == self@.len())` — in_order postcondition
8. `assume(preordered@.len() == self@.len())` — pre_order postcondition

**How to fix**: After `acquire_read`/`acquire_write`, the lock invariant guarantees
`ghost_locked_root@ == locked_val@`. The inner StEph method's `ensures` gives the
property on `locked_val`. Bridge through the invariant to prove the assume on `self@`.

Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` for the pattern.

### Chap39/BSTParaTreapMtEph.rs (15 external_body)

These are `external_body` stubs on the parallel treap Mt wrapper. The inner type is
BSTParaTreapStEph (already at 0 holes — fully proved).

- 1 `external_body` on new (constructor)
- 14 `external_body` on operations: insert, delete, find, size, union, intersect,
  difference, previous_key, next_key, rank_key, collect, in_order, pre_order, clone

Pattern: remove `external_body`, acquire lock, call inner StEph method, bridge ghost
state, release lock. Same as BSTTreapMtEph but for the parallel variant.

### Stretch: Chap38/BSTParaMtEph.rs (17 external_body)

If you finish Chap39, tackle Chap38's Mt wrapper. Same pattern — the inner
BSTParaStEph is the one you proved in R14. You know the specs intimately.

## DO NOT

- Touch Chap43 (Agent 2)
- Touch Chap42 (Agent 1)
- Touch Chap41 (Agents 1 and 4)
- Touch Chap37, Chap47, Chap45 (Agent 4)
- Touch BSTParaStEph.rs (your R14 work is done; the 5 remaining are structural)

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversion.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Push to `agent3/ready`. Write `plans/agent3-round15-report.md`.

## Target: -12 (stretch -20, including Chap38). Close Chap39.
