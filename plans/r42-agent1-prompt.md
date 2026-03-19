# R42 Agent 1: OrderedTableStEph Remaining 11 Methods

## Baseline
- Main at `e83db19f`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4320 verified, 153 holes, 30 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.

## Context

In R41, you proved 7 methods (reduce, from_sorted_entries, map, delete, difference,
restrict, subtract) + fixed the RTT ordering bug + proved MtEph from_sorted_entries.
11 external_body methods remain in OrderedTableStEph.rs.

## Assignment

Your R41 report identified blockers for each method. Here's how to attack them:

### Tier 1: Tractable with Effort

| # | Method | Your R41 Blocker | Strategy |
|---|--------|-----------------|----------|
| 1 | insert | usize::MAX edge (new key → len+1) | Check if `from_vec` can take `<=` instead of `<`. Or add `requires self@.dom().len() < usize::MAX - 1` to the trait method. |
| 2 | domain | feq_clone vs feq_full mismatch | Can you construct ArraySetStEph differently? Or add `obeys_feq_full::<K>()` to domain's requires? |
| 3 | tabulate | usize::MAX edge (keys.len ≤ max) | Same as insert — trait requires may need tightening |
| 4 | filter | Z3 completeness invariant | Agent 2 proved filter for StPer using bidirectional ghost tracking. Read Agent 2's R41 code in OrderedTableStPer.rs and adapt the technique. |
| 5 | intersection | Closure value-correctness existential | Agent 2 proved intersection for StPer using closure witness tracking (result_v1/v2/r). Read and adapt. |
| 6 | union | Same closure existential | Agent 2 proved union using two-phase loop. Read and adapt. |

### Tier 2: TotalOrder Operations

| # | Method | Notes |
|---|--------|-------|
| 7 | get_key_range | Agent 2 proved this for StPer. Read and adapt. |
| 8 | split_key | Agent 2 proved this for StPer. Read and adapt. |
| 9 | rank_key | Complex TotalOrder counting |
| 10 | select_key | Complex TotalOrder indexed access |

### Tier 3: Helpers (Low Priority)

| # | Method | Notes |
|---|--------|-------|
| 11 | avl_seq_length | Intentional wrapper, may need wf added to requires |
| 12 | avl_seq_nth | Intentional wrapper, may need wf added to requires |

### Key Strategy

**Read Agent 2's OrderedTableStPer.rs code.** Agent 2 proved filter, intersection, union,
split_key, get_key_range, tabulate, restrict, subtract using bidirectional ghost tracking
and closure witness patterns. Your StEph versions need the same techniques but adapted
for `&mut self` (ephemeral) vs `&self -> Self` (persistent). The core proof logic is the
same.

### Priority

1. filter, intersection, union (adapt from Agent 2's StPer proofs) — 3 methods
2. get_key_range, split_key (adapt from Agent 2) — 2 methods
3. insert, domain, tabulate (requires changes) — 3 methods
4. rank_key, select_key (hardest) — 2 methods

### Expected Results

Conservative: 3-5 methods proved.
Optimistic: 7-10 methods proved.

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent1-r42-report.md`.
