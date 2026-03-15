# Agent 1 Round 20 Report — Prove Chap19 iterate/reduce/scan

## Summary

Proved all iterate, reduce, and scan functions across 3 Chap19 files, eliminating 24
external_body holes. Chap19 now has 0 proof holes. This unblocks 17 downstream chapters
that depend on Chap19.

## Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 19 | ArraySeqStEph.rs | 8 | 0 | -8 |
| 2 | 19 | ArraySeqStPer.rs | 8 | 0 | -8 |
| 3 | 19 | ArraySeqMtEph.rs | 8 | 0 | -8 |

**Total: 24 holes eliminated, 0 remaining in Chap19.**

## Functions Proved (5 per file x 3 files = 15)

| # | Function | Technique |
|---|----------|-----------|
| 1 | iterate_iter | While-loop with fold_left invariant, extensional seq equality |
| 2 | iterate | Delegates to iterate_iter (avoids recursive termination) |
| 3 | reduce_iter | While-loop with fold_left invariant (same pattern as iterate_iter) |
| 4 | reduce | Delegates to reduce_iter (avoids recursive D&C termination) |
| 5 | scan | While-loop, double-call f pattern (push + acc update), spec_monoid invariant |

## Proof Techniques

- **fold_left invariant**: `acc == a.seq@.take(i).fold_left(seed, spec_f)` with
  `reveal(Seq::fold_left)` to unfold the recursive spec definition.
- **Extensional seq equality**: `assert(a.seq@.take(i+1) =~= a.seq@.take(i).push(a.seq@[i]))`
  to connect iteration steps.
- **Delegation pattern**: Recursive `iterate`/`reduce` delegate to their iterative
  counterparts, which share the same postcondition. Avoids needing `decreases` on
  recursive calls through `subseq`.
- **Double-call f**: scan calls `f(&acc, &a.seq[i])` twice (once for push, once for acc)
  instead of cloning. Avoids needing `obeys_feq_clone` in scan's requires.
- **Direct `a.seq@` access**: Unlike Chap18 which uses ghost `s` + `lemma_spec_index`,
  Chap19 specs are directly in terms of `self.seq@`, so proofs use `a.seq@` directly.

## Approach Not Taken

- **Monoid shift lemma**: Attempted a standalone `proof fn lemma_fold_left_monoid_shift` to
  prove the recursive D&C reduce directly. Failed — `reveal_with_fuel(Seq::fold_left, N)`
  does not reliably unfold in standalone proof functions. Abandoned in favor of delegation.

## Verification

- 3964 verified, 0 errors (up from 3948, +16 new verifications)
- Chap19: 0 holes, 4 clean modules, 13 clean proof functions, 268 total functions

## Commit

- Branch: agent1/ready
