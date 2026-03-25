# R76 Agent 1 — BSTSetRBMtEph BTreeSet rewrite (16 holes)

## Objective

Eliminate up to 16 holes in `src/Chap37/BSTSetRBMtEph.rs` by rewriting it to remove the
`std::collections::BTreeSet` dependency, following the exact pattern Agent 5 used on
BSTSetAVLMtEph in R75.

## Baseline

- 4794 verified, 0 errors, 0 warnings
- BSTSetRBMtEph.rs: 16 holes (14 external_body + 2 external)
- Root causes: `from_sorted_iter` (line ~143), `iter` (line ~366), `iter_in_order` (line ~361)
- 11 downstream functions blocked by these 3 root causes

## Current holes

| # | Line | Function | Type | Blocked by |
|---|------|----------|------|------------|
| 1 | 125 | values_vec | external_body | iter |
| 2 | 133 | copy_set | external_body | from_sorted_iter |
| 3 | 143 | from_sorted_iter | external_body | ROOT |
| 4 | 185 | delete | external_body | filter |
| 5 | 200 | union | external_body | join_m |
| 6 | 236 | intersection | external_body | join_m |
| 7 | 264 | difference | external_body | join_m |
| 8 | 291 | split | external_body | from_sorted_iter |
| 9 | 308 | join_pair | external_body | from_sorted_iter |
| 10 | 323 | join_m | external_body | from_sorted_iter |
| 11 | 339 | filter | external_body | from_sorted_iter |
| 12 | 351 | reduce | external_body | iter |
| 13 | 361 | iter_in_order | external_body | ROOT |
| 14 | 366 | iter | external_body | ROOT |
| 15 | 449 | IntoIterator &ref | external | — |
| 16 | 458 | IntoIterator owned | external | — |

## Reference implementation

**Read `src/Chap37/BSTSetAVLMtEph.rs` first.** Agent 5 already rewrote this file from the
same BTreeSet pattern. Your job is to replicate that approach for the RB variant. Key
changes Agent 5 made:

1. **Removed `use std::collections::BTreeSet`** — replaced with verified types only.
2. **Replaced `from_sorted_iter` with `build_from_vec`** — takes `Vec<T>` instead of
   generic `IntoIterator`. Uses explicit loop to build the tree via `insert()`.
3. **Replaced `values_vec`** — uses `tree.in_order()` + explicit `while` loop with `nth()`
   instead of std iterator chain. Requires `assume(obeys_feq_clone::<T>())` for `in_order()`.
4. **Rewrote `delete`, `split`, `join_pair`, `join_m`** — all use `values_vec` + explicit
   loops + `build_from_vec` instead of BTreeSet operations.
5. **Removed ParaPair** from `join_pair`/`join_m` — sequential insert loops instead.
6. **`iter()` / `iter_in_order()`** — already had iterator infrastructure (ghost struct,
   view impls). Just needed to remove external_body and write verified bodies using
   `in_order()` + `nth()`.
7. **`filter`/`reduce`** — kept external_body (FnMut closure requires not provable in Verus).
   But wrote verified loop bodies inside.
8. **`union`/`intersection`/`difference`** — kept external_body (recursive ParaPair! closures).

**Expected outcome**: ~10-11 holes eliminated (same as AVL: 17→13, minus the 4 already-clean
downstream functions). The remaining ~5-6 holes will be structural: `obeys_feq_clone` assumes,
filter/reduce external_body, union/intersection/difference external_body.

## Key differences from AVL

- BSTSetRBMtEph wraps `BSTRBMtEph<T>` (not BSTAVLMtEph).
- `BSTRBMtEph` has `in_order()` returning `ArraySeqStPerS<T>` — same pattern as AVL.
- `BSTRBMtEph` has `insert()`, `from_sorted_slice()`.
- Check what `BSTRBMtEph` exposes — it may have slightly different API than BSTAVLMtEph.

## Approach

1. Read BSTSetAVLMtEph.rs completely — understand every change Agent 5 made.
2. Read BSTSetRBMtEph.rs completely — understand current state.
3. Read BSTRBMtEph.rs trait section — know what API is available.
4. Remove `use std::collections::BTreeSet`.
5. Rewrite functions following AVL pattern, bottom-up from root causes.
6. Validate after each major function change.
7. Run RTT and PTT before committing.

## Important

- Do NOT add `assume` or `accept` without user approval (except `obeys_feq_clone` which
  follows the established AVL pattern).
- Do NOT weaken ensures clauses.
- Add `#[trigger]` to every quantifier — zero trigger notes.
- Keep filter/reduce/union/intersection/difference as external_body if needed (same as AVL).

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent1/ready`.

## Report

Write `plans/agent1-round76-report.md` with holes before/after (table with Chap column).
Document any differences from the AVL rewrite pattern.
