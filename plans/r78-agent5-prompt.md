# R78 Agent 5 — SpanTreeStEph spanning_tree_star_contraction (Chap64, 1 hole)

## Objective

Prove or narrow 1 external_body hole in SpanTreeStEph.rs.

## Baseline

- 4898 verified, 0 errors, 0 warnings
- SpanTreeStEph.rs: 1 hole

## Hole

| # | Chap | File | Line | Function | Type |
|---|------|------|------|----------|------|
| 1 | 64 | SpanTreeStEph.rs | 53 | spanning_tree_star_contraction | external_body |

## Context

`spanning_tree_star_contraction` passes closures to `star_contract` (from Chap62).
Agent 4 R76 and R77 found the closure interface is the blocker: `star_contract` requires
`forall|inputs| expand.requires(inputs)` but the expand closure has specific wf
preconditions (`spec_setsteph_wf`) that aren't universally satisfiable.

## Strategy

1. **Read `src/Chap64/SpanTreeStEph.rs`** fully — understand what closures are passed.
2. **Read `src/Chap62/StarContractionStEph.rs`** — understand `star_contract` API and
   what it requires of its closure arguments.
3. **Read `src/standards/using_closures_standard.rs`** — the closure verification pattern.

**Possible approaches**:
- Restructure the closures with explicit `requires`/`ensures` that satisfy star_contract's
  universal quantification. The closure's requires may need to be `true` (total function)
  or the star_contract API may need to accept closures with specific preconditions.
- If star_contract's API genuinely can't accept closures with preconditions, check if the
  function body can be rewritten to not use star_contract (inline the algorithm).
- Narrow external_body to just the closure interface gap.

## Key resources

- `src/Chap64/SpanTreeStEph.rs` — Read fully
- `src/Chap62/StarContractionStEph.rs` — star_contract API
- `src/standards/using_closures_standard.rs`

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent5/ready`.

## Report

Write `plans/agent5-round78-report.md` with holes before/after (table with Chap column).
