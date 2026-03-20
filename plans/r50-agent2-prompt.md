# Agent 2 — Round 50: Refactor BSTParaMtEph from Arc<RwLock> to plain RwLock

## Goal

Convert `src/Chap38/BSTParaMtEph.rs` from the Arc<RwLock> antipattern to the correct
plain RwLock pattern. This unblocks type_invariant and should eliminate external_body
holes on expose_internal, intersect_inner, difference_inner, and filter_inner.

## Why This Matters

BSTParaMtEph currently has 7 holes (2 assumes in lemma_cmp_order_axioms, 4 external_body,
1 clone_elem assume in StEph). The 4 external_body functions are all "blocked_by:
expose_internal" — and expose_internal is blocked because Arc<RwLock> prevents
type_invariant.

The struct uses `Arc<RwLock<...>>` as its field (line 71). This is the Role 4 antipattern
from `src/standards/arc_usage_standard.rs`.

The correct pattern is already used in **the St counterpart in the same chapter**:

**`src/Chap38/BSTParaStEph.rs`** — Uses plain RwLock (line 83), type_invariant (line 108),
use_type_invariant at 12 call sites. This is your primary reference. You already know this
file from R49.

## Current Holes in BSTParaMtEph (7 errors + 8 warnings)

### Errors (holes)
| # | Line | Type | Description |
|---|------|------|-------------|
| 1 | 193 | assume | obeys_cmp_spec (in lemma_cmp_order_axioms) |
| 2 | 194 | assume | view_ord_consistent (in lemma_cmp_order_axioms) |
| 3 | 516 | external_body | expose_internal |
| 4 | 782 | external_body | intersect_inner (blocked_by expose_internal) |
| 5 | 815 | external_body | difference_inner (blocked_by expose_internal) |
| 6 | 848 | external_body | filter_inner (blocked_by expose_internal) |

### Warnings (fix these too)
| # | Line | Warning | Fix |
|---|------|---------|-----|
| 1 | 544 | fn_missing_requires | join_mid |
| 2 | 569 | fn_missing_requires | split_inner |
| 3 | 689 | fn_missing_requires | join_m |
| 4 | 695 | fn_missing_requires | find_recursive |
| 5 | 738 | fn_missing_requires | join_pair_inner |
| 6 | 752 | fn_missing_requires | union_inner |
| 7 | 906 | fn_missing_ensures | reduce_inner |
| 8 | 943 | fn_missing_ensures | reduce_parallel |

## Standards to Read First

1. `src/standards/arc_usage_standard.rs` — Role 4 antipattern, why plain RwLock is better
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — the correct Layer 1/Layer 2
   pattern with ghost shadow and type_invariant

## Steps

1. **Read thoroughly**: BSTParaMtEph.rs (current) and BSTParaStEph.rs (reference).
   Note how StEph uses type_invariant to connect ghost_locked_root to the locked value,
   and how use_type_invariant provides existence witnesses for set membership.
2. **Refactor the struct**: Replace `Arc<RwLock<Option<Box<NodeInner<T>>>, Inv>>` with
   plain `RwLock<Option<Box<NodeInner<T>>>, Inv>`. Keep ghost_locked_root.
3. **Add type_invariant**: Model after BSTParaStEph.rs:108-112. The invariant should
   constrain ghost_locked_root (finite, equals pred contents, existence witnesses for
   all values in the set).
4. **Remove Arc bridge**: `new_param_bst_arc` and `clone_arc_rwlock` calls become
   direct `RwLock::new` calls.
5. **Add use_type_invariant**: At every operation entry point. BSTParaStEph has 12 call
   sites — follow the same pattern.
6. **Prove expose_internal**: With type_invariant available, the ghost decomposition
   (left set + right set + root = parent set) should be provable from the tree structure.
7. **Unblock intersect/difference/filter**: With expose_internal proved, the
   "blocked_by: expose_internal" cascade should resolve. Agent 2 in R49 said these need
   existence witnesses — type_invariant provides exactly that.
8. **Fix fn_missing_requires/ensures warnings**: Read each function, add real preconditions
   and postconditions. Do NOT add `requires true`.
9. **Validate incrementally**: `scripts/validate.sh` after each major change.
10. **RTT + PTT**: `scripts/rtt.sh && scripts/ptt.sh` at the end.

## Constraints

- Do NOT modify BSTParaStEph.rs (reference only)
- Do NOT add new assumes or external_body
- Do NOT weaken ensures clauses
- The 2 Ord consistency assumes in lemma_cmp_order_axioms may survive this round — they
  require MtKey to add PartialEqSpecImpl bound. Focus on the Arc<RwLock> refactor first.
- BSTParaStEph.rs has 1 hole (clone_elem assume, line 152) — irreducible per standards,
  leave it alone

## Success Criteria

- BSTParaMtEph uses plain RwLock as struct field (no Arc)
- type_invariant present and constraining ghost_locked_root
- expose_internal proved (no external_body)
- At least 2 of the 3 remaining external_body functions proved
- 8 fn_missing_requires/ensures warnings fixed
- 0 verification errors, 0 RTT failures, 0 PTT failures
