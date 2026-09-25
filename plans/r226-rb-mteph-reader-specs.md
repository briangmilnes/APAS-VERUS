# r226: strong specs for the BSTRBMtEph readers (Chap37)

Branch `r226/rb-reader-specs`, worktree `~/projects/APAS-VERUS-r226`, from
`r225/rb-parallel` (`cbe48973f`). Toolchain: Verus `0.2026.09.13.671956e`,
Z3 4.16.0. The user approved this round (r223 report, decision 2): readers may
use lock-boundary (RWLOCK_GHOST) assumes to obtain real specs.

## Problem

In `src/Chap37/BSTRBMtEph.rs`, these Layer-2 readers of `BSTRBMtEph` say
nothing about `self@`:

| # | Chap | Function | Current |
|---|---|---|---|
| 1 | 37 | minimum, maximum | `ensures true`, no requires |
| 2 | 37 | in_order, pre_order | `ensures true` |
| 3 | 37 | filter | `ensures true` |
| 4 | 37 | reduce | `ensures true` |
| 5 | 37 | iter | relates the iterator to itself only |

The Layer-1 functions they call now have exact specs (r225):
`in_order_parallel` and `pre_order_parallel` return exact sequences,
`filter_parallel` satisfies `spec_filtered`, `reduce_parallel` satisfies
`spec_reduced`, and `min_link` and `max_link` have ordering specs. The missing
link is the relation between the link borrowed from the read lock and the
ghost root.

## Design

1. **One assume per reader, in the writers' form.** Right after `borrow()`,
   add `proof { assume(self.ghost_root@ == *data); }`, with the same shape as
   the writers' `assume(self.ghost_root@ == current)`. Veracity classifies that
   shape as RWLOCK_GHOST. Prove everything else from the Layer-1 ensures and
   the bridge lemmas (`lemma_link_to_bbt_*`).
   - Do not assume result properties directly.
   - Do not change the existing readers `contains`, `size`, `is_empty`,
     `height`, and `find`. Their assumes stay exactly as they are; the user
     decides later whether to convert them to this form.
2. **Specs**, in terms of `self@` (the `BalBinTree` view). All require
   `self.spec_bstrbmteph_wf()`, plus what r225 already added.
   - minimum / maximum: none iff the size is 0; otherwise a key the tree
     contains that is ≤ (≥) every key. Model: StEph `minimum`/`maximum` (r223).
   - in_order: `seq@ == self@.spec_in_order()`, using the view of
     `ArraySeqStPerS`; pre_order: `seq@ == self@.spec_pre_order()`.
   - filter: the Layer-1 `spec_filtered` guarantee stated on `self@`, at least
     "every element is a key of `self@` and satisfies the predicate". Use the
     strongest form that proves.
   - reduce: the Layer-1 `spec_reduced` relation stated on the view.
   - iter: remaining elements `== self@.spec_in_order()`, as in BB/RB StEph
     `iter`.
   - Update the matching `BSTSetRBMtEph` wrappers only if a changed requires
     forces it. Do not strengthen the set's specs; that is a separate round.
3. Do not change exec bodies or costs; r225's bodies stay as they are.

## Steps

1. Read CLAUDE.md and every file in `src/standards/`.
2. Implement; run `scripts/validate.sh isolate Chap37` until 0 errors,
   0 warnings, 0 trigger notes.
3. Run the RB MtEph and set test binaries:
   `cargo nextest run --release -E 'binary(/TestBSTRBMtEph|TestBSTSetRBMtEph|TestBSTMtEph/)'`.
   If a PTT in `rust_verify_test/tests/Chap37/ProveBSTRBMtEph.rs` is affected
   by the new requires, update it. Do not run `scripts/ptt.sh`; the main
   session runs it.
4. Run seeds 1-8 on module `Chap37::BSTRBMtEph::BSTRBMtEph`.
5. Run `scripts/holes.sh src/Chap37/BSTRBMtEph.rs`. The only new holes allowed
   are RWLOCK_GHOST assumes of the form `self.ghost_root@ == *data`, at most
   one per strengthened reader (7 at most). List each in the report.
6. Report `plans/r226-rb-mteph-reader-specs-report.md`, with `#` and `Chap`
   columns: per function, the spec before and after, and the assume added.
7. Commit with `git add -A`; push `r226/rb-reader-specs`. Do not merge.

## Constraints

- Only the RWLOCK_GHOST assumes described above. No other assume, and no
  admit, accept, or external_body. No rlimit raised; no spec weakened; no
  `#![auto]`; no formatters; no `// veracity: no_requires`; no subagents.
- Do not run full `scripts/validate.sh`, full `scripts/rtt.sh`, or
  `scripts/ptt.sh`; use isolate validates and filtered test runs only.
- Work only in `~/projects/APAS-VERUS-r226`. Never pipe validate output; read
  the log.
- No Python, no Perl, and no `git clean`.
