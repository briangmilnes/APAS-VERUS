# r225: fork-join traversals in Chap37 BSTRBMtEph

Branch `r225/rb-parallel`, worktree `~/projects/APAS-VERUS-r225`, from main
`ec039f497`. Toolchain: Verus `0.2026.09.13.671956e`, Z3 4.16.0. The user
approved this round (r223 report, decision 3).

## Problem

In `src/Chap37/BSTRBMtEph.rs`, the Layer-1 functions `in_order_parallel`,
`pre_order_parallel`, `filter_parallel`, and `reduce_parallel` have no
fork-join in them. Each is a sequential recursion (or a call to the sequential
`in_order_collect`/`pre_order_collect`), so its span equals its work, O(n). An
Mt module must be parallel (CLAUDE.md: "Never sequentialize parallel files").
All four currently `ensures true`.

## Design

1. Make each function a fork-join recursion over the two subtrees, with
   `join()` from the help-first scheduler (`src/Chap02/HFSchedulerMtEph.rs`).
   Follow `src/standards/hfscheduler_standard.rs` and
   `src/standards/using_closures_standard.rs`:
   - use named closures with explicit `ensures`;
   - put no `external_body` around `join`;
   - keep all of the code inside `verus!`.
   Existing uses to copy from: `src/Chap35/OrderStatSelectMtEph.rs`,
   `src/Chap36/QuickSortMtEphSlice.rs`, `src/Chap37/AVLTreeSeqMtPer.rs`,
   `src/Chap41/AVLTreeSetMtEph.rs`.
   - in_order: `join(left, right)`, then left ++ [key] ++ right.
   - pre_order: `join(left, right)`, then [key] ++ left ++ right.
   - filter: `join` over both subtrees, then concatenate, keeping the key if
     the predicate holds.
   - reduce: `join(reduce left, reduce right)`, then `op(op(l, key), r)`.
   Do not add a sequential cutoff threshold (CLAUDE.md: "No Thread Threshold
   Optimization").
2. Costs. Tree height is at most 2·lg(n+1) (proved). The per-node
   concatenation costs O(size), so Work O(n lg n) and Span O(n) for the
   Vec-append form. Reaching a lower span needs a sequence type with an
   O(lg n)-span append. Choose the design, state the resulting Work and Span
   honestly in each Alg Analysis annotation, and justify the choice in the
   report. Do not claim a span the code does not achieve. If Span O(lg² n)
   or better is reachable with an existing verified parallel sequence type
   (for example Chap18/Chap19 array sequences with a parallel append or
   flatten), prefer it.
3. Specs: strengthen, never weaken.
   - `in_order_parallel` ensures `elements@ == link_to_bbt(*self).spec_in_order()`;
     `pre_order_parallel` likewise with `spec_pre_order()`. StEph's
     `in_order_into`/`pre_order_into` are the model; the clone bridge needs
     `obeys_feq_clone::<T>()` in requires, via `clone_plus`.
   - Give `filter_parallel` and `reduce_parallel` the strongest ensures you can
     prove from the closure's `ensures` (see the closures standard). At minimum,
     every element of the filter result is a key of the tree and satisfies the
     predicate. If a stronger spec is not provable, say so in the report.
   - Strengthen `in_order_collect`/`pre_order_collect` to exact-sequence ensures
     too, or comment them out with `// BYPASSED:` if they become unused. Do
     not delete them.
   - Layer 2 (`in_order`, `pre_order`, `filter`, `reduce` on `BSTRBMtEph`)
     stays as it is. A later round (r226) strengthens those readers. Adjust
     Layer 2 only as much as the new Layer-1 requires force you to (for example,
     add `obeys_feq_clone::<T>()` to the Layer-2 requires).

## Steps

1. Read CLAUDE.md and every file in `src/standards/`.
2. Implement; run `scripts/validate.sh isolate Chap37` until 0 errors,
   0 warnings, 0 trigger notes.
3. RTTs: `scripts/rtt.sh` with the test-name filters that cover RB MtEph and
   BSTSetRBMtEph (rtt.sh filters on test names). Add tests showing
   in_order/pre_order/filter/reduce results equal a sequential model on
   n = 500 random, ascending, and empty trees. Threaded tests need timeouts.
4. Run seeds 1-8 on module `Chap37::BSTRBMtEph::BSTRBMtEph` with
   `VERUS_EXTRA_ARGS="--verify-only-module ... --smt-option smt.random_seed=N"`.
5. Run `scripts/holes.sh src/Chap37/BSTRBMtEph.rs`: no new holes.
6. Report `plans/r225-rb-mteph-parallel-traversals-report.md`, with `#` and
   `Chap` columns: per function, the body change, spec before and after,
   Work/Span before and after, seeds, holes, and tests.
7. Commit with `git add -A`; push `r225/rb-parallel`. Do not merge.

## Constraints

- Another round runs on this machine at the same time. Do NOT run full
  `scripts/validate.sh`, full `scripts/rtt.sh`, or `scripts/ptt.sh`; use
  isolate validates and filtered RTTs only.
- No assume, admit, accept, or external_body added; no rlimit raised; no spec
  weakened; no `#![auto]`; no formatters; no `// veracity: no_requires`; no
  subagents.
- Work only in `~/projects/APAS-VERUS-r225`.
- Never pipe or filter validate output; read the log.
- No Python, no Perl, and no `git clean`.
