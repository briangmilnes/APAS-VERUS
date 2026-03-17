# Agent 2 — R31 Report

## Objective

Fix `fn_missing_requires` / `requires_true` warnings in Chap21, Chap28, Chap42, Chap45,
and Chap59. Add real preconditions where possible.

## Results

| # | Chap | File | Function | Before | After | Action |
|---|------|------|----------|--------|-------|--------|
| 1 | 45 | BinaryHeapPQ.rs | `parent` | fn_missing_requires | fixed | Added `requires i > 0` (real precondition) |
| 2 | 45 | HeapsortExample.rs | `is_vec_sorted_exec` | requires_true | clean (ℹ) | Removed vacuous `requires true, ensures true` |

## Unchanged (requires_true remains)

These functions are genuinely total — no real precondition exists. They have meaningful
`ensures` clauses, so removing `requires true` would create a worse `fn_missing_requires`
error. They need `// veracity: no_requires` annotations (user-only).

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 21 | Exercise21_7.rs | `is_even` | Total: `*x % 2 == 0` works for any usize |
| 2 | 21 | Exercise21_7.rs | `is_vowel` | Total: pattern match on any char |
| 3 | 21 | Exercise21_8.rs | `is_prime` | Total: handles all cases including n < 2 |
| 4-11 | 28 | 8 MaxContigSubSum files | `max_with_neginf` | Total: pattern match on Option\<i32\> |
| 12 | 42 | TableStEph.rs | `from_sorted_entries` | Needs cross-chapter coordination (callers in Chap43 + macros outside verus!) |
| 13 | 42 | TableStPer.rs | `from_sorted_entries` | Same as above |
| 14 | 42 | TableMtEph.rs | `from_sorted_entries` | Same as above |
| 15 | 59 | JohnsonStEphI64.rs | `adjust_distance` | Total: i128 arithmetic, no overflow |
| 16 | 59 | JohnsonStEphI64.rs | `reweight_edge` | Total: i128 arithmetic with clamping |
| 17 | 59 | JohnsonStEphI64.rs | `create_negative_cycle_result` | Total: AllPairsResult::new(n) has no requires |

## Why most functions couldn't be fixed

The plan characterized all 18 targets as "mechanical fn_missing_requires fixes". In
practice, 16 of 18 are genuinely total functions: they operate on primitive types or
Option\<i32\> with no structural/bounds constraints. Their `requires true` is technically
correct (the precondition IS `true`) but veracity flags it as vacuous.

The CLAUDE.md rule "DO NOT ADD requires true" is about not adding NEW `requires true` to
suppress fn_missing_requires. For existing `requires true` on total functions, the only
resolution is `// veracity: no_requires` (user annotation). Removing `requires true`
creates a worse `fn_missing_requires` error.

For Chap42 `from_sorted_entries`, a real precondition (`spec_keys_no_dups` on entries)
was attempted but:
1. The assertion bridging Vec\<Pair\> view to ArraySeq view couldn't verify
2. Callers in Chap43 couldn't prove the new precondition
3. Macros in Chap42 (outside verus!) can't prove anything

## Verification

- `scripts/validate.sh`: 4116 verified, 0 errors
- `scripts/rtt.sh`: 2613 tests passed
- `scripts/ptt.sh`: 147 tests passed

## Chapters status

- Chap21: no change (2 requires_true on total functions)
- Chap28: no change (8 requires_true on total functions)
- Chap42: no change (3 requires_true + 3 fn_missing_wf_ensures)
- Chap45: **HeapsortExample ❌→ℹ**, BinaryHeapPQ fn_missing_requires fixed (still ❌ due to assume)
- Chap59: no change (external_body + 3 requires_true)

## Recommendation

The user should add `// veracity: no_requires` annotations to the 16 genuinely total
functions listed above to close Chap21 and Chap28. For Chap42, the `from_sorted_entries`
functions need a coordinated fix with Chap43 (Agent 4) to add `spec_keys_no_dups` precondition.
