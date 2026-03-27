# R85 Agent 1 — Close spec_unionfindsteph_wf + prove union_merge + union, STEP 20

## Objective

Make `spec_unionfindsteph_wf` a `closed spec fn`. Refactor all internal functions
to use pointwise `reveal` and lemma calls instead of relying on the open wf
quantifiers. Then prove `union_merge` and `union`.

## Isolation — use ONLY this command for validation

```bash
scripts/validate.sh isolate Chap65
```

Do NOT run full `scripts/validate.sh`, `scripts/rtt.sh`, or `scripts/ptt.sh`.
Push to `agent1/ready` when your isolated validate is clean.

## No Subagents

Do NOT use the Agent tool to spawn subagents. Do all work yourself, sequentially.

## PLAN FIRST, THEN EDIT

This is a refactoring task. Do NOT start by removing external_body and validating.

### The problem

`spec_unionfindsteph_wf` is an `open spec fn` with 13 quantified conjuncts. Any
function with wf in requires + ensures gives Z3 26+ quantifiers. Z3 peaks at 17 GB.

### The fix

1. Change `spec_unionfindsteph_wf` from `open spec fn` to `closed spec fn`.
   This prevents Z3 from automatically unfolding the 13 quantifiers.

2. Add `lemma_decompose_wf` and `lemma_assemble_wf` (already exist from R84) to
   convert between monolithic wf and individual sub-predicates.

3. Refactor `find_root_loop`: its loop invariant currently relies on wf being open
   so Z3 can access individual conjuncts. With closed wf, the function must call
   `lemma_decompose_wf` at entry, then carry only the specific sub-predicates it
   needs in the loop invariant. Use `reveal` on individual sub-predicates as needed.

4. Refactor all other internal functions similarly: `insert`, `find`, `equals`,
   `num_sets`, `lemma_root_is_self_parent`, `lemma_non_root_rank_lt_root`.

5. The TRAIT ensures still say `self.spec_unionfindsteph_wf()` — callers (Kruskal)
   use the monolithic wf. Inside the impl, functions call `lemma_assemble_wf` at
   the end to rebuild the monolithic wf for the ensures.

6. With closed wf, `union_merge` and `union` should verify under rlimit(80) because
   Z3 never sees 26 quantifiers simultaneously.

### The existing infrastructure (from R84)

- 10 closed sub-predicates (spec_roots_idempotent, spec_parent_closed, etc.)
- `lemma_decompose_wf` — monolithic wf → individual sub-predicates
- `lemma_assemble_wf` — individual sub-predicates → monolithic wf
- `union_merge_exec` — exec mutations isolated from proof
- `lemma_establish_union_pre` — assembles spec_union_lemma_pre
- 3 verified sub-lemmas: wf_roots, wf_parent, wf_ordering
- `UnionMergeInfo` ghost struct

### The admit for rank overflow

Line 718 has `admit()` for `rank_u + 1 <= usize::MAX`. Leave it.

## Important

- Do NOT modify KrustalStEph.rs or PrimStEph.rs — other agents work on Kruskal.
- Do NOT add new `assume` or `accept`.
- Do NOT weaken ensures clauses.
- The trait's public interface keeps `spec_unionfindsteph_wf()` — only close it, don't remove it.

## STEP 20

## Report

Write `plans/agent1-round85-report.md`.
