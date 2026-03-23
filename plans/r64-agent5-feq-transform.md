# R64: Run veracity-full-generic-feq transform on all algorithm chapters

## Context

The `veracity-full-generic-feq` transformer adds `obeys_feq_full::<T>()` to
`spec_*_wf` predicates and loop invariants throughout the codebase. This
strengthens well-formedness specs so that Z3 can use the functional-equality
axiom (`feq`) without broadcast hints at every call site.

Main is at `ac89aa742` — 4489 verified, 0 errors, 45 clean chapters.

## Task

1. Run the `veracity-full-generic-feq` transformer on the APAS-VERUS codebase,
   **excluding** `src/experiments/` and `src/vstdplus/`.
2. Run `scripts/validate.sh`. Fix any verification failures introduced by the
   transform. Known pattern: loops whose invariants rely on
   `seq.to_set().len() == seq.len()` may need an explicit
   `elements@.unique_seq_to_set()` proof hint before the loop (see
   `OrderedSetStPer.rs` line 914 and `OrderedSetStEph.rs` lines 998-1000
   for the pattern).
3. Run `scripts/rtt.sh` and `scripts/ptt.sh` to confirm no regressions.
4. Commit with: `R64: apply veracity-full-generic-feq transform to algorithm chapters`
5. Push to `agent5/ready`.

## Constraints

- Do NOT modify files in `src/experiments/` or `src/vstdplus/`.
- Do NOT add `assume`, `accept`, or `external_body` to fix failures.
- If a file fails verification after the transform and you cannot fix it
  within 3 iterations, revert that single file and report it.
- Run validate, rtt, ptt sequentially, not in parallel.
