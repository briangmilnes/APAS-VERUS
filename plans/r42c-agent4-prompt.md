# R42c Agent 4: Chap47 QuadProb + Chap59 Johnson + Chap63 Connectivity

## Baseline
- Main at `c010cf2a`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4333 verified, 146 holes, 30 clean chapters
- (Your R42 resize work is not yet merged — work from main)

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**
**DO NOT move code outside `verus!{}` or add `#[cfg(not(verus_keep_ghost))]` to
dodge verification.** All algorithm implementations belong inside `verus!{}`.
If you can't prove it, leave the `external_body` and report what you tried.

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Assignment

### Part A: Chap47 QuadProb diverge fix (1 hole)

`src/Chap47/QuadProbFlatHashTableStEph.rs` line 376: `assume(false)` without
`diverge()`. Add `diverge();` after the `assume(false)` — this is the established
pattern for unreachable table-full branches with load factor < 1.

### Part B: Chap59 JohnsonStEphI64 (1 hole + 2 warnings)

`src/Chap59/JohnsonStEphI64.rs`:

1. **Line 438**: `assume(reweighted@.A.len() * 2 + 2 <= usize::MAX)` — this is a
   usize overflow guard before calling Dijkstra. The reweighted graph has the same
   vertices as the input graph plus one virtual source. If the input graph satisfies
   a reasonable size bound, this should be provable. Read the function to understand
   the graph construction and add a `requires` to `johnson_apsp` that bounds the
   input graph size, then prove the assume from it.

2. **Lines 72, 88**: `fn_missing_requires` on `adjust_distance` and `reweight_edge`.
   Read the function bodies. These are arithmetic helpers — figure out the real
   preconditions (e.g., no overflow) and add them as requires.

### Part C: Chap63 ConnectivityStEph (5 holes)

`src/Chap63/ConnectivityStEph.rs` has 5 external_body holes:

| # | Function | Line | Notes |
|---|----------|------|-------|
| 1 | count_components | 80 | Calls star_contract with base/expand closures |
| 2 | connected_components | 109 | Same pattern, returns component map |
| 3 | build_quotient_edges | 141 | Helper: builds edges for quotient graph |
| 4 | count_components_hof | 173 | Higher-order version using star_contract |
| 5 | connected_components_hof | 189 | Higher-order version using star_contract |

These functions call `star_contract` and `sequential_star_partition` from Chap62,
which themselves have external_body. This means you CAN'T prove these by removing
external_body — the callee bodies aren't available to Verus.

**What you CAN do:**
- Add real `requires` and `ensures` to the trait methods (currently the trait
  has requires but weak/missing ensures).
- Strengthen the ensures to match what the algorithm actually guarantees:
  - `count_components` ensures result > 0 for non-empty graphs
  - `connected_components` ensures partition map covers all vertices
  - `build_quotient_edges` ensures returned edges connect different components
- If you find functions where the cfg-gated body can be replaced with a verifiable
  body (some may be simple enough), do so.

### Part D: Chap63 ConnectivityMtEph (7 holes, stretch goal)

Same analysis as Part C but for the Mt version. If you finish Part C, apply the
same ensures-strengthening approach here.

### Priority

1. Part A: QuadProb diverge (trivial, 1 hole)
2. Part B: JohnsonStEphI64 assume + warnings (1 hole + 2 warnings)
3. Part C: ConnectivityStEph ensures strengthening (spec work, may not close holes)
4. Part D: ConnectivityMtEph (stretch)

### Expected Results

Conservative: 2 holes closed (QuadProb + Johnson assume) + 2 warnings fixed.
Optimistic: 2 holes closed + 2 warnings fixed + spec strengthening across Chap63.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent4-r42c-report.md`.

## Continue

Commit early, commit often. Push after each successful validation.
