# R144 Agent 4 — Clean up Chap50 assumes + annotate remaining unfixable DIFFERS. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r144-agent4-cleanup-annotate-report.md`

## Task 1: Clean up 3 Chap50 assumes

You added 3 assumes in R143. Prove them from wf/structure:

1. `OptBinSearchTreeMtEph.rs` — `assume(ps@.len() > i + l)`: prefix_sums has
   n+1 elements, i+l <= n from the loop bounds. Prove from the prefix_sums
   construction and the loop invariant.

2. `OptBinSearchTreeMtPer.rs` — same assume, same fix.

3. `OptBinSearchTreeMtPer.rs` — `assume(self.memo.pred() == OptBSTMtPerMemoInv)`:
   the memo is constructed with this predicate. Thread the pred identity through
   the struct's wf or the constructor's ensures.

## Task 2: Annotate remaining unfixable DIFFERS

These DIFFERS cannot be fixed due to Verus limitations or representation choices.
Change `DIFFERS` to `ACCEPTED DIFFERENCE` on these lines:

### spec_fn not Send (3 — blocked on Verus)

```
src/Chap38/BSTParaMtEph.rs — filter: sequential recursion in filter_inner (spec_fn not Send)
src/Chap41/AVLTreeSetMtEph.rs — filter: sequential filter (spec_fn not Send)
src/Chap41/AVLTreeSetMtPer.rs — filter: sequential filter (spec_fn not Send)
```

Change to: `ACCEPTED DIFFERENCE: Verus limitation; spec_fn not Send, blocks parallel filter`

### Flatten Vec concat (2 — representation limitation)

```
src/Chap19/ArraySeqMtEphSlice.rs — flatten: Vec concat at each level adds lg factor
src/Chap18/ArraySeqMtEphSlice.rs — flatten: Vec concat at each level adds lg factor
```

Change to: `ACCEPTED DIFFERENCE: Vec concat at each D&C level; O(1) rejoin needs PCell pre-allocated output`

## Validation

Run `scripts/validate.sh isolate Chap50` for assumes. Then `scripts/validate.sh` full.
Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Task 1: PROVE the assumes, don't just move them.
- Task 2: Only change the annotation text, not the code.

## When done

RCP.
