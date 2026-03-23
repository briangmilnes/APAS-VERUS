# R59 Agent 1 — Chap47 Triangular Proofs + Chap26 ETSPMtEph

## Assignment

Close 3 actionable holes: 2 in `QuadProbFlatHashTableStEph.rs` (triangular number
proofs) and 1 in `ETSPMtEph.rs` (external_body).

**Do NOT touch** the other 2 Chap47 holes — they are working patterns:
- `clone_elem` assume (line 120) — intentional clone bridge per standard.
- `call_hash_fn` external_body (line 342) — correct closure boundary.

**Do NOT touch** the 8 fn_missing_wf warnings in ParaHashTableStEph.rs — they need a
multi-file refactor that is out of scope.

## Targets

### Target 1: `lemma_triangular_injective` (QuadProbFlatHashTableStEph.rs:142)

Prove: if `0 <= i < j < m` and `m` is a power of two, then
`(i*(i+1)/2) % m != (j*(j+1)/2) % m`.

The proof strategy (already in the TODO comment):
1. `(j*(j+1)/2 - i*(i+1)/2) = (j-i)*(j+i+1)/2`
2. For this to be 0 mod m = 2^k, need `(j-i)*(j+i+1) ≡ 0 (mod 2^(k+1))`
3. `(j-i) + (j+i+1) = 2j+1` is odd, so exactly one of `{j-i, j+i+1}` is even
4. The even one has fewer than k+1 factors of 2 (because both terms are < m = 2^k)
5. The odd one has 0 factors of 2. Product has < k+1 factors. Contradiction.

Use `vstd::arithmetic::power::pow` and `vstd::arithmetic::div_mod` lemmas.
You may need a helper `proof fn lemma_factors_of_two(n: int, k: nat)` that bounds
the number of times 2 divides a positive integer less than 2^k.

### Target 2: `lemma_empty_slot_reachable` (QuadProbFlatHashTableStEph.rs:160)

Prove: if all m probe positions are non-empty and none match the key, derive `false`.

Depends on Target 1. Strategy:
1. Instantiate `lemma_triangular_injective` for all pairs `(i, j)` with `0 <= i < j < m`
   to establish that the m probe positions are all distinct modulo m.
2. Since m distinct positions cover all indices `[0, m)` and `num_elements < m`
   (from wf), at least one slot must be Empty — contradiction.

May need a helper lemma that m distinct values in `[0, m)` cover all of `[0, m)`.

### Target 3: `ETSPMtEph.rs:612` — external_body

Read `src/Chap26/ETSPMtEph.rs` around line 612. Understand the function, its
requires/ensures, and attempt to prove the body. If the external_body wraps a
thread-spawn boundary, it is structural and should be left alone. If it wraps
algorithmic logic, prove it.

## Validation

After each target, run `scripts/validate.sh` and show full output. Fix any errors
before moving to the next target. Do NOT add `assume`, `accept`, or `external_body`.

## Report

Write `plans/agent1-round59-report.md` with:
- Holes before/after per file (table with # and Chap columns)
- Techniques used
- Verification count
