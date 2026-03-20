# R47 Agent 2: Chap39 BSTParaTreapMtEph (8 holes)

## Assignment

Prove remaining external_body functions in BSTParaTreapMtEph.rs. Agent 4 has
closed 19 holes across R43-R45 using ghost field migration, helper ensures,
and the ParaPair direct-reference technique. You're taking over Chap39 to
bring fresh eyes.

## Baseline

47 holes total. 4413 verified. Your chapter: Chap39 (8 holes).

## REQUIRED READING

1. `plans/agent4-round44-report.md` — Ghost field migration + cyclic self-reference fix
2. `plans/agent4-round45-report.md` — ParaPair direct x@ technique, remaining blockers
3. `plans/parapair-is-not-a-blocker.md` — Named closure pattern for ParaPair
4. `plans/veracity-root-cause-vs-downstream-holes.md` — Root-cause analysis

## Current State

Run `scripts/holes.sh src/Chap39/` to see current holes with root-cause annotations.

Agent 4's R45 report identified these remaining blockers:

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 39 | BSTParaTreapMtEph.rs | expose_internal | external_body | ROOT CAUSE: RwLock boundary |
| 2 | 39 | BSTParaTreapMtEph.rs | tree_priority | external_body | ROOT CAUSE: RwLock read |
| 3 | 39 | BSTParaTreapMtEph.rs | tree_size | external_body | ROOT CAUSE: RwLock read |
| 4 | 39 | BSTParaTreapMtEph.rs | split_inner | external_body | BST ordering not in ghost |
| 5 | 39 | BSTParaTreapMtEph.rs | intersect_inner | external_body | Downstream of split_inner |
| 6 | 39 | BSTParaTreapMtEph.rs | difference_inner | external_body | Downstream of split_inner |
| 7 | 39 | BSTParaTreapMtEph.rs | filter_inner | external_body | Arc::clone missing spec |
| 8 | 39 | BSTParaTreapMtEph.rs | find | assume | BST search correctness |

Run holes.sh yourself to verify — line numbers may have shifted.

## Strategy

### Root causes first

`expose_internal`, `tree_priority`, `tree_size` all do `acquire_read` on
`Arc<RwLock<...>>`. The bodies must stay external_body, but you can
**strengthen their ensures** so downstream functions can use the results.

Agent 4 already added ensures to expose_internal in R44. Check what's there
and see if the ensures are strong enough for the downstream callers. If not,
strengthen them.

For `tree_priority` and `tree_size` — these are simple read-through-lock
functions. Their ensures should state what the returned value equals in terms
of the ghost field.

### Arc::clone issue

`filter_inner` and potentially others need `Arc::clone` which Verus can't
spec. Agent 4's approach was the ParaPair direct x@ technique (reference x@
in closure ensures without ghost capture). Check if this can be extended.

Alternative: factor the Arc::clone into a tiny external_body helper with
tight ensures, then prove the rest of the function body.

### BST ordering

`split_inner` needs BST ordering information that isn't in the ghost Set.
The ghost field carries `Set<T::V>` (membership only). If you can add
ordering information to the ghost field or prove ordering from the BST
structure, `split_inner` unlocks `intersect_inner` and `difference_inner`.

### Key technique from Agent 4 R45

**ParaPair without ghost captures**: Reference `x@` directly in closure
ensures instead of `let ghost view = x@`. This avoids the Send bound issue
with `Set<T::V>` containing FnSpec. This technique already proved
`union_inner`, `reduce_parallel`, `collect_in_order` in R45.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel implementations — keep ParaPair! calls.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap39/`.
Write your report to `plans/agent2-round47-report.md`.
