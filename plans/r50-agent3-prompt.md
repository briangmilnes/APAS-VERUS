# Agent 3 — Round 50: Refactor BSTParaTreapMtEph from Arc<RwLock> to plain RwLock

## Goal

Convert `src/Chap39/BSTParaTreapMtEph.rs` from the Arc<RwLock> antipattern to the correct
plain RwLock pattern. This unblocks type_invariant and should eliminate RWLOCK_GHOST assumes.

## Why This Matters

BSTParaTreapMtEph currently has 9 assumes. The root cause for several is that the struct
uses `Arc<RwLock<...>>` as its field (line 63). This is the Role 4 antipattern from
`src/standards/arc_usage_standard.rs`. It prevents:
- `type_invariant` (Arc's opacity blocks it)
- `use_type_invariant` (which connects ghost shadow to locked value)
- Value-level specs through the ghost

The correct pattern (plain RwLock + ghost shadow + type_invariant) is already used in
**two sibling files in the same codebase**:

1. **`src/Chap39/BSTTreapMtEph.rs`** — same chapter, non-parametric treap. Uses plain
   RwLock (line 57), type_invariant (line 68), use_type_invariant throughout. **This is
   your primary reference.**
2. **`src/Chap38/BSTParaStEph.rs`** — St counterpart of parametric BST. Uses plain RwLock
   (line 83), type_invariant (line 108), use_type_invariant at 12 call sites.

## Current Assumes (9)

| # | Category | Count | Functions | What type_invariant fixes |
|---|----------|-------|-----------|--------------------------|
| 1 | RWLOCK_GHOST | 2 | expose_internal | Ghost decomposition — type_invariant links ghost to lock |
| 2 | Ord consistency | 3 | split_inner | May need MtKey + PartialEqSpecImpl |
| 3 | BST cross-disjointness | 2 | intersect_inner, difference_inner | type_invariant carries BST ordering |
| 4 | spec_fn not Send | 1 | filter_parallel | Language limitation — may remain |
| 5 | find loop invariant | 1 | find | type_invariant carries BST ordering |

Categories 1, 3, and 5 (5 assumes) are directly caused by the missing type_invariant.

## Standards to Read First

1. `src/standards/arc_usage_standard.rs` — Role 4 antipattern, why plain RwLock is better
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the correct Layer 1/Layer 2
   pattern with ghost shadow and type_invariant

## Steps

1. **Read thoroughly**: BSTParaTreapMtEph.rs (current), BSTTreapMtEph.rs (reference),
   and both standards.
2. **Refactor the struct**: Replace `Arc<RwLock<Option<Box<NodeInner<T>>>, Inv>>` with
   plain `RwLock<Option<Box<NodeInner<T>>>, Inv>`. Keep ghost_locked_root.
3. **Add type_invariant**: Model after BSTTreapMtEph.rs:68-70. The invariant should
   constrain ghost_locked_root (finite, equals pred contents, existence witnesses).
4. **Remove Arc bridge**: `new_param_treap_arc` and `clone_arc_rwlock` calls become
   direct `RwLock::new` calls.
5. **Add use_type_invariant**: At every operation entry point, call
   `use_type_invariant(&*self)` to get the invariant into scope.
6. **Eliminate RWLOCK_GHOST assumes**: The 2 in expose_internal should be provable via
   use_type_invariant + set algebra.
7. **Attack BST ordering assumes**: With type_invariant carrying `forall|v| set.contains(v)
   ==> exists|t: T| t@ == v`, the cross-disjointness and find assumes may fall.
8. **Validate incrementally**: `scripts/validate.sh` after each major change.
9. **RTT + PTT**: `scripts/rtt.sh && scripts/ptt.sh` at the end.

## Constraints

- Do NOT modify BSTTreapMtEph.rs or BSTParaStEph.rs (reference only)
- Do NOT add new assumes or external_body
- Do NOT weaken ensures clauses
- If expose_internal still needs external_body for the split-into-children pattern
  (creating child structs that own their own locks), document why and minimize scope
- The Ord consistency assumes (#2, 3 count) may require MtKey to add PartialEqSpecImpl
  bound — if so, document what's needed but do not modify the MtKey trait without approval

## Success Criteria

- BSTParaTreapMtEph uses plain RwLock as struct field (no Arc)
- type_invariant present and constraining ghost_locked_root
- Net assume reduction (target: -3 or more from the 9 current assumes)
- 0 verification errors, 0 RTT failures, 0 PTT failures
