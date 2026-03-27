# Agent 2 — Round 85 Report

## Objective

Remove `external_body` from `kruskal_greedy_phase` in `src/Chap65/KruskalStEph.rs`.

## Result

**Partial success.** The greedy loop is now fully proved. The `external_body` moved from
`kruskal_greedy_phase` (complex loop + body) down to `kruskal_process_edge` (simple
if-branch with 3 calls).

## What Changed

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 65 | KruskalStEph.rs | Removed `external_body` from `kruskal_greedy_phase` |
| 2 | 65 | KruskalStEph.rs | Added nested module `uf_opaque_wrappers` with opaque UF wf |
| 3 | 65 | KruskalStEph.rs | Added `kruskal_process_edge` helper with `external_body` |

## Technique: Opaque WF + Module Isolation

The core challenge: `spec_unionfindsteph_wf()` is `open` with 13 quantified conjuncts.
When Z3 unfolds it in any function, the quantifiers interact with broadcast axioms
(`group_float_finite_total_order`, `group_LabEdge_axioms`) and cause divergence.

- **At rlimit 50 with full wf in loop invariant**: Z3 peaked at 10.5GB, rlimit exceeded.
- **Opaque wf wrapper**: wraps `spec_unionfindsteph_wf` in `#[verifier::opaque]` spec fn
  so Z3 never sees the 13 quantifiers in the loop or process_edge.
- **Nested module**: the opaque spec and wrappers live in a nested module WITHOUT the
  float/LabEdge broadcast groups, preventing broadcast × wf quantifier interaction.
- **Scoped reveal**: `assert(uf_wf_opaque(uf)) by { reveal(uf_wf_opaque); }` bridges
  from raw wf (in `kruskal_greedy_phase` requires) to opaque wf (in loop invariant).

The greedy loop invariant uses `uf_wf_opaque(uf)` instead of `uf.spec_unionfindsteph_wf()`.
Z3 verifies the loop cleanly with 0 wf quantifiers in its context.

## Why `kruskal_process_edge` Remains `external_body`

Inside `kruskal_process_edge`, Z3 must unfold wf for `uf.equals` and `uf.union` calls.
Even with the nested module (no broadcasts), the 13 quantifiers from `equals` ensures +
13 from `union` ensures = 26+ quantifiers cause Z3 divergence.

Attempts that failed:
- Fold/unfold lemmas (rlimit 30, 11.4GB, diverging)
- Raw wf wrappers (verified at rlimit 25 individually, but chaining them in process_edge diverged)
- Direct reveal (same divergence)

The spec of `kruskal_process_edge` is obviously correct: `equals` preserves wf + dom,
`insert` preserves setsteph_wf, `union` preserves wf + dom.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 65 | KruskalStEph.rs | external_body on greedy loop | external_body on process_edge + opaque spec | net 0 |

The external_body moved from a complex loop function to a simple 5-line if-body. The
greedy loop (the algorithmic core) is now fully verified.

## Verification

```
scripts/validate.sh isolate Chap65
verification results:: 2406 verified, 0 errors
Elapsed: 71s
```

## What Blocks Full Proof

Proving `kruskal_process_edge` requires Z3 to carry `spec_unionfindsteph_wf` (13 quantifiers)
through an if-branch with equals + union. This causes E-matching divergence. Possible future
approaches:
- Close `spec_unionfindsteph_wf` in UnionFindStEph.rs (would fix everything)
- Split into fewer quantifiers (reduce the wf predicate)
- Use Verus's `--profile` to identify specific matching loops
