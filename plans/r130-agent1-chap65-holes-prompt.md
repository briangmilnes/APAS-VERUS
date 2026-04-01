# R130 Agent 1 — Prove Chap65 UnionFindStEph holes. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- Standard 4 (`spec_wf_standard.rs`) — wf predicates
- Standard 22 (`capacity_bounds_standard.rs`) — propagating size bounds through requires

Report file: `plans/r130-agent1-chap65-report.md`

## Problem

`src/Chap65/UnionFindStEph.rs` has 11 holes — the largest single-file hole cluster.
`src/Chap65/KruskalStEph.rs` has 2 holes (1 opaque, 1 external_body).

Holes in UnionFindStEph:

```
:1058: fn_missing_wf_requires — fn union_merge_exec needs uf.spec_unionfindsteph_wf()
:1780: external_body on merge_roots
:1786: assume(self.rank@[root_u@] < self.elements@.len())
:1787: assume(self.rank@[root_v@] < self.elements@.len())
:1876: assume(root_u@ != root_v@)
:1877: assume(uf.rank@[root_u@] < uf.elements@.len())
:1878: assume(uf.rank@[root_v@] < uf.elements@.len())
:1904: assume(root_u@ != root_v@)
:1905: assume(uf.rank@[root_u@] < uf.elements@.len())
:1906: assume(uf.rank@[root_v@] < uf.elements@.len())
```

## Analysis

The assumes fall into two patterns:

1. **rank[root] < elements.len()** (6 assumes): The rank array should be bounded by
   the elements array length. This is a wf invariant: `forall|i| rank@[i] < elements@.len()`.
   If `spec_unionfindsteph_wf()` includes this, all 6 disappear.

2. **root_u != root_v** (2 assumes): After find(u) and find(v), if u and v are in
   different components, their roots differ. This should be provable from the union-find
   invariant (distinct components have distinct roots).

3. **external_body on merge_roots** (1 hole): Read the function, understand why it was
   external_body, and try to prove the body.

4. **fn_missing_wf_requires** (1 hole): Add `uf.spec_unionfindsteph_wf()` to the
   `requires` of `union_merge_exec`.

## Approach

1. Read the file thoroughly. Understand the wf predicate and what it currently includes.
2. Strengthen `spec_unionfindsteph_wf()` to include rank bounds if missing.
3. Add wf to requires of `union_merge_exec`.
4. Prove the rank bounds from wf — eliminate the 6 rank assumes.
5. Prove root_u != root_v from the union-find component invariant.
6. Try to prove merge_roots body (remove external_body).

Also look at the 2 holes in KruskalStEph.rs — they may be related.

## Validation

Run `scripts/validate.sh isolate Chap65`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add new assumes, accepts, or external_body.
- Do NOT weaken ensures.
- If you can't prove a hole, leave it and report what you tried.
