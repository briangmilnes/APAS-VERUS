# R70 Agent 3: BSTTreapStEph — wf Propagation + reduce assume

## Goal

Eliminate the 1 proof hole in `src/Chap39/BSTTreapStEph.rs` and fix 28 fn_missing_wf
style errors. Target: 1 hole → 0, 28 style errors → 0.

## Current State

Run `scripts/holes.sh src/Chap39/BSTTreapStEph.rs` first to get exact current list.

### Proof Hole (1)

| # | Line | Type | Content |
|---|------|------|---------|
| 1 | 2652 | assume | `assume(left_base == identity)` in `reduce_inner_st` — eq/clone workaround |

### fn_missing_wf Style Errors (28)

These functions take or return `BSTTreapStEph<T>` but lack wf in requires/ensures:

| # | Line | Function | Missing |
|---|------|----------|---------|
| 1 | 147 | clone_with_view | requires tree.spec_bsttreapsteph_wf() |
| 2 | 147 | clone_with_view | ensures cloned.spec_bsttreapsteph_wf() |
| 3 | 1488 | make_node_treap_st | requires left.spec_bsttreapsteph_wf() |
| 4 | 1488 | make_node_treap_st | requires right.spec_bsttreapsteph_wf() |
| 5 | 1488 | make_node_treap_st | ensures node.spec_bsttreapsteph_wf() |
| 6 | 1529 | tree_priority_st | requires (fn_missing_requires) |
| 7 | 1539 | expose_to_parts_st | requires tree.spec_bsttreapsteph_wf() |
| 8 | 1574 | join_with_priority_st | requires left.spec_bsttreapsteph_wf() |
| 9 | 1574 | join_with_priority_st | requires right.spec_bsttreapsteph_wf() |
| 10 | 1574 | join_with_priority_st | ensures result.spec_bsttreapsteph_wf() |
| 11 | 1665 | split_inner_st | requires tree.spec_bsttreapsteph_wf() |
| 12 | 1845 | join_pair_inner_st | requires left.spec_bsttreapsteph_wf() |
| 13 | 1845 | join_pair_inner_st | requires right.spec_bsttreapsteph_wf() |
| 14 | 1845 | join_pair_inner_st | ensures joined.spec_bsttreapsteph_wf() |
| 15 | 2035 | union_inner_st | requires a.spec_bsttreapsteph_wf() |
| 16 | 2035 | union_inner_st | requires b.spec_bsttreapsteph_wf() |
| 17 | 2035 | union_inner_st | ensures combined.spec_bsttreapsteph_wf() |
| 18 | 2187 | intersect_inner_st | requires a.spec_bsttreapsteph_wf() |
| 19 | 2187 | intersect_inner_st | requires b.spec_bsttreapsteph_wf() |
| 20 | 2187 | intersect_inner_st | ensures common.spec_bsttreapsteph_wf() |
| 21 | 2338 | difference_inner_st | requires a.spec_bsttreapsteph_wf() |
| 22 | 2338 | difference_inner_st | requires b.spec_bsttreapsteph_wf() |
| 23 | 2338 | difference_inner_st | ensures remaining.spec_bsttreapsteph_wf() |
| 24 | 2499 | filter_inner_st | requires tree.spec_bsttreapsteph_wf() |
| 25 | 2499 | filter_inner_st | ensures result.spec_bsttreapsteph_wf() |
| 26 | 2631 | reduce_inner_st | requires tree.spec_bsttreapsteph_wf() |
| 27 | 2662 | collect_in_order_st | requires tree.spec_bsttreapsteph_wf() |
| 28 | 1529 | tree_priority_st | fn_missing_requires (not wf-specific) |

## Strategy

### wf Propagation (28 style fixes)

These are internal helper functions (`_st` suffix = standalone treap helpers). Each takes
or returns a `BSTTreapStEph` and needs the wf predicate propagated.

**Approach**: Work bottom-up through the call graph:

1. **Leaf functions first**: `tree_priority_st`, `expose_to_parts_st`, `clone_with_view`,
   `collect_in_order_st` — these don't call other _st functions
2. **Then composition functions**: `make_node_treap_st`, `join_with_priority_st`
3. **Then higher functions**: `split_inner_st`, `join_pair_inner_st`
4. **Then set operations**: `union_inner_st`, `intersect_inner_st`, `difference_inner_st`
5. **Finally**: `filter_inner_st`, `reduce_inner_st`

For each function:
1. Add `requires param.spec_bsttreapsteph_wf()` for each BSTTreapStEph input
2. Add `ensures result.spec_bsttreapsteph_wf()` for BSTTreapStEph outputs
3. Check that the function body verifies (the wf from inputs should propagate to outputs)
4. If verification fails, you may need intermediate `assert` steps to help Z3

For `tree_priority_st` (fn_missing_requires), read the function body to understand what
precondition it actually needs. It likely needs `tree.spec_bsttreapsteph_wf()` and possibly
that the tree is non-empty.

**CAUTION**: BSTTreap is a complex verified tree module. Adding requires/ensures may
cascade — callers of these _st functions (the trait impl methods) must already provide wf,
so cascade should be minimal. But verify after each batch of changes.

### reduce_inner_st assume (1 hole)

Line 2652: `assume(left_base == identity)` — this is the eq/clone workaround. The code
does `let left_base = identity.clone()` and needs to prove `left_base == identity`.

**Fix options**:
1. Use `clone_plus` instead of `clone` to get a view-preserving clone ensures, then
   prove equality from view equality. Check if the type has PartialEq — if `T: PartialEq`
   then the eq spec can bridge it.
2. Check the `partial_eq_eq_clone_standard.rs` for the standard pattern. The assume may
   be a legitimate eq/clone workaround that should become `accept` (but do NOT convert
   without trying to prove first).
3. If unprovable, leave the assume and report it.

## Steps

1. **Read** BSTTreapStEph.rs — understand spec_bsttreapsteph_wf, call graph of _st functions
2. **Read** the standard: `src/standards/partial_eq_eq_clone_standard.rs`
3. **Add** wf requires/ensures bottom-up through the helper call graph
4. **Validate** after each batch (leaf functions, then composition, etc.)
5. **Attempt** proving the reduce_inner_st clone assume
6. **Final validate**, **rtt**, **ptt** — run sequentially

## Constraints

- Modify only `src/Chap39/BSTTreapStEph.rs`.
- Do NOT modify any Chap43 files (Agents 1, 2, 4 own those).
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Do NOT add `requires true` or tautological requires.
- Do NOT add `// veracity: no_requires` annotations.
- For tree_priority_st, identify the REAL precondition — don't use `requires true`.
- Run validate, rtt, ptt sequentially, never in parallel.
- Write report to `plans/agent3-round70-report.md` when done.
