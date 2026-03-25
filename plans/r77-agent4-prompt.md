# R77 Agent 4 — Chap37 BST Mt holes + Chap64 remaining (11 holes)

## Objective

Prove holes in Chap37 BST Mt files and Chap64.

## Baseline

- 4869 verified, 0 errors, 0 warnings

## Holes

### BSTRBMtEph.rs (Chap37, 3 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 796 | filter_parallel | external_body | Fork-join with Arc closure |
| 2 | 821 | reduce_parallel | external_body | Fork-join with Arc closure |
| 3 | 1052 | height (Mt wrapper) | assume | `link_height < usize::MAX` |

**Strategy for height**: Agent 4 R76 found that the wf invariant bounds height as
`<= usize::MAX` (not strict `<`). Check if adding a `requires link_height(*data) < usize::MAX`
or proving from wf that `height < MAX` (since height < size, and size fits in usize).

**Strategy for filter/reduce**: These are genuine fork-join with `Arc<F>`. Narrow
external_body to just the thread boundary if possible. Read the function bodies.

### BSTSplayMtEph.rs (Chap37, 5 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 1453 | build_balanced | external_body | Blocked by clone |
| 2 | 1479 | filter_parallel | external_body | Blocked by clone |
| 3 | 1512 | reduce_parallel | external_body | Blocked by clone |
| 4 | 1729 | height (Mt wrapper) | assume | Same as RB |
| 5 | 1801 | clone | external_body | ROOT — recursive Clone |

**Strategy**: Root cause is `clone` (recursive Clone on Node). This is a known Verus
limitation. Try the `strictly_cloned` broadcasts from feq.rs (agent4 R76 work).
If clone can't be proved, try the height assume (same as RB).

### SpanTreeStEph.rs (Chap64, 1 hole)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 53 | spanning_tree_star_contraction | external_body | Closure interface |

**Strategy**: Uses closures passed to `star_contract`. Agent 4 R76 found the closure
requires `forall|inputs| expand.requires(inputs)` but the expand closure has specific wf
preconditions. Check if the closure interface can be redesigned per closures standard.

### TSPApproxStEph.rs (Chap64, 2 holes)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 120 | euler_tour_dfs | external_body | ROOT — mutable visited set |
| 2 | 96 | euler_tour | external_body | Blocked by euler_tour_dfs |

**Strategy**: `euler_tour_dfs` uses `HashSetWithViewPlus<(V,V)>` for visited tracking.
Agent 4 R76 found `obeys_key_model::<(V,V)>()` is uninterpreted for tuples. Check if
replacing with a verified `SetStEph` or `Vec<bool>` visited array enables verification.

## Priority

1. Height assumes (BSTRBMtEph, BSTSplayMtEph) — quickest wins
2. SpanTreeStEph closure interface
3. TSPApprox visited set rewrite
4. filter/reduce narrowing
5. BSTSplayMtEph clone — hardest, likely stays

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round77-report.md` with holes before/after (table with Chap column).
