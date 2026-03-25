# R76 Agent 2 — BSTSetSplayMtEph BTreeSet rewrite (13 holes)

## Objective

Eliminate up to 13 holes in `src/Chap37/BSTSetSplayMtEph.rs` by rewriting it to remove the
`std::collections::BTreeSet` dependency, following the exact pattern Agent 5 used on
BSTSetAVLMtEph in R75.

## Baseline

- 4794 verified, 0 errors, 0 warnings
- BSTSetSplayMtEph.rs: 13 holes (13 external_body)
- Root causes: `rebuild_from_vec` (line ~152), `iter` (line ~379)
- 11 downstream functions blocked by these 2 root causes

## Current holes

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 143 | values_vec | external_body | iter |
| 2 | 152 | rebuild_from_vec | external_body | ROOT |
| 3 | 208 | delete | external_body | iter |
| 4 | 223 | union | external_body | join_pair |
| 5 | 254 | intersection | external_body | join_pair |
| 6 | 282 | difference | external_body | join_pair |
| 7 | 309 | split | external_body | iter |
| 8 | 326 | join_pair | external_body | values_vec |
| 9 | 341 | join_m | external_body | values_vec |
| 10 | 357 | filter | external_body | iter |
| 11 | 367 | reduce | external_body | iter |
| 12 | 379 | iter | external_body | ROOT |
| 13 | 476 | into_iter | external_body | iter |

## Reference implementation

**Read `src/Chap37/BSTSetAVLMtEph.rs` first.** Agent 5 already rewrote this file from the
same BTreeSet pattern. Your job is to replicate that approach for the Splay variant.

Key changes Agent 5 made on BSTSetAVLMtEph:
1. **Removed `use std::collections::BTreeSet`**.
2. **Replaced `from_sorted_iter` with `build_from_vec`** — `Vec<T>` + explicit insert loop.
3. **Replaced `values_vec`** — `tree.in_order()` + `while` loop with `nth()`.
   Requires `assume(obeys_feq_clone::<T>())` for `in_order()`.
4. **Rewrote `delete`, `split`, `join_pair`, `join_m`** — explicit loops + `build_from_vec`.
5. **`iter()` / `iter_in_order()`** — removed external_body, used `in_order()` + `nth()`.
6. **`filter`/`reduce`** — kept external_body (FnMut closure). Verified loop bodies inside.
7. **`union`/`intersection`/`difference`** — kept external_body (ParaPair! closures).

## Key differences from AVL

- BSTSetSplayMtEph wraps `BSTSplayMtEph<T>` (not BSTAVLMtEph).
- `BSTSplayMtEph` has `in_order()` returning `ArraySeqStPerS<T>` — same as AVL.
- Splay has `+ 'static` bound on `T` in some signatures — preserve these.
- BSTSetSplayMtEph uses `rebuild_from_vec` (not `from_sorted_iter`) — may already be
  closer to the target pattern. Check if the body just needs the BTreeSet removed.

## Approach

1. Read BSTSetAVLMtEph.rs completely — understand the rewrite pattern.
2. Read BSTSetSplayMtEph.rs completely — understand current state.
3. Read BSTSplayMtEph.rs trait section — know what API is available.
4. Remove `use std::collections::BTreeSet`.
5. Rewrite root causes first: `rebuild_from_vec`, then `iter`.
6. Cascade through downstream functions.
7. Validate after each major function change.

## Important

- Do NOT add `assume` or `accept` without user approval (except `obeys_feq_clone` which
  follows the established AVL pattern).
- Do NOT weaken ensures clauses.
- Add `#[trigger]` to every quantifier — zero trigger notes.
- Keep filter/reduce/union/intersection/difference as external_body if needed (same as AVL).

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round76-report.md` with holes before/after (table with Chap column).
Document any differences from the AVL rewrite pattern.
