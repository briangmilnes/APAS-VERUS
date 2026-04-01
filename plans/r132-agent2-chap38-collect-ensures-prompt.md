# R132 Agent 2 — Strengthen BSTParaMtEph collect_in_order ensures. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `spec_wf_standard.rs` — wf propagation through ensures

Report file: `plans/r132-agent2-chap38-collect-report.md`

## Problem

`src/Chap38/BSTParaMtEph.rs` — the `collect_in_order` function (or equivalent in-order
traversal) has ensures that are too weak to prove that the returned sequence's elements
correspond to the tree's set.

Chap41's `to_seq` had assumes like:
```
assume(seq@.to_set() =~= self@)
assume(forall|i| 0 <= i < seq@.len() ==> self@.contains(seq@[i]))
```

Agent 3 R131 proved these with ghost captures and map_values bridges, but the proof
was manual. If `collect_in_order` had stronger ensures, these would follow directly.

## What to do

1. Find the in-order traversal function in BSTParaMtEph (might be `collect_in_order`,
   `in_order`, or similar). Read its current ensures.

2. Strengthen the ensures to include:
   - `result@.len() == self@.len()` (sequence length = set cardinality)
   - `result@.to_set() =~= self@` (sequence elements = set elements, as a set)
   - Or equivalently: `forall|v| self@.contains(v) <==> result@.contains(v)`
   - `result@.no_duplicates()` (no repeated elements, since it's a set)

3. Prove the strengthened ensures. The in-order traversal visits each node once,
   so the length/membership/no-dup properties should follow from the BST invariant.

4. If the function is in a bare impl (not a trait), add the ensures directly.
   If it's in a trait, update both trait declaration and impl.

5. Check callers — strengthened ensures should not break anyone (it only adds
   information, doesn't change requirements).

## Also check BSTParaStEph

`src/Chap38/BSTParaStEph.rs` may have the same traversal function with the same
weak ensures. If so, strengthen it too — the Mt version may delegate to it.

## Validation

Run `scripts/validate.sh isolate Chap38`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken existing ensures — only add new ones.
- Strengthening ensures is safe: it gives callers more information.
