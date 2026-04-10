# R171 Prompt A — Agent 8: Prove ETSPMtEph lemma_combined_cycle. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent8`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**

## The hole

`src/Chap26/ETSPMtEph.rs:206` — `lemma_combined_cycle` has `#[verifier::external_body]`.
The proof body is empty.

## The answer is in the StEph version

`src/Chap26/ETSPStEph.rs` has the SAME lemma, fully proven, 92 lines. Same
signature, same requires, same ensures. Read it. Adapt it to the MtEph file.

## Steps

1. **Read all standards.**
2. **Read `src/Chap26/ETSPStEph.rs`** — find `lemma_combined_cycle`. Study the
   92-line proof. Understand how it proves `spec_edges_form_cycle(combined)`.
3. **Read `src/Chap26/ETSPMtEph.rs`** — find the `external_body` version.
   Compare the signatures. Note any differences in types or bounds.
4. **Copy the StEph proof body** into the MtEph version. Adapt type parameters
   if needed (StT → MtKey or similar).
5. **Remove `#[verifier::external_body]`.**
6. **Validate:** `scripts/validate.sh isolate Chap26`
7. **Fix any errors.** The proof may need minor adjustments for Mt type bounds.

## Do NOT touch the f64 sort/split external_body

There is a second `external_body` in ETSPMtEph.rs around line 519 — the f64
sort/split function. Leave it alone. That's an accepted structural hole for
float arithmetic. Your job is ONLY `lemma_combined_cycle`.

## Validation

```bash
scripts/validate.sh isolate Chap26
```

## Report

Write `plans/agent8-round171-report.md`.

## RCP

`git add -A && git commit -m "R171 Agent 8: prove ETSPMtEph lemma_combined_cycle (−1 hole)"`, then `git push`.
