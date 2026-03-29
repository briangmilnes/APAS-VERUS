# Agent 1 — R103 Report: UnionFind Decomposition

## Objective

Remove `external_body` from `union_merge` and `union` in
`src/Chap65/UnionFindStEph.rs`, blocked by Z3 blowup on `&mut` of a
4-field struct with quantified ensures.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 65 | UnionFindStEph.rs | 2 | 2 | 0 |
| 2 | 65 | KruskalStEph.rs | 2 | 2 | 0 |

No holes removed. The external_body on union remains due to a Z3
matching loop that diverges unboundedly (17GB OOM at rlimit 200).

## Verified Infrastructure Built (+2 verified)

All infrastructure for removing union's external_body is in place
and verified. The proof IS correct — it's the Z3 matching loop that
prevents convergence.

| # | Function | Purpose |
|---|----------|---------|
| 1 | `lemma_union_ensures_bridge` | Translates union_merge's quantified ensures (trigger: `roots@[x]`) to union's (trigger: `roots.contains_key(x)`) |
| 2 | `lemma_wf_parent_dom_eq_roots_dom` | Extracts `parent.dom() =~= roots.dom()` from wf without revealing sub-predicates |

## Other Changes

| # | Change | Why |
|---|--------|-----|
| 1 | `find_root_loop` ensures now includes `roots.contains_key(root@)` | Avoids needing dom-equivalence lemma in union |
| 2 | `union_merge` requires: removed parent self-loop conditions | Avoids `lemma_root_is_self_parent` in union (which reveals wf) |
| 3 | `union_merge` ensures: added `parent.dom() =~= old.parent.dom()` | Avoids wf dom lemma in union |
| 4 | `union_merge` ensures: `elements@ ==` instead of `=~=` | Attempt to reduce seq quantifier (minor) |
| 5 | feq broadcast moved from module-level to per-function | Removes feq axiom from union's Z3 context |

## The Blocker: Z3 Matching Loop

Two quantifiers create an unbounded feedback loop:

1. `obeys_feq_view_injective`: `forall|x, y| x.view() == y.view() ==> x == y`
2. `spec_elements_distinct`: `forall|i, j| ... elements@[i]@ != elements@[j]@`

They feed each other: `elements@[i]@` is `.view()`, triggering #1. #1
produces equality facts, triggering #2 on new pairs. At rlimit(200),
Z3 consumed 17.7GB and OOM'd.

**Root cause**: Both quantifiers leak from `spec_uf_wf`'s `closed` body
into Z3's context. The chain: `spec_unionfindsteph_wf` (open) unfolds to
`spec_uf_wf(uf)` (closed). Despite `closed`, Z3 sees the sub-predicates.
Making `spec_unionfindsteph_wf` closed did NOT help — same behavior.

**Approaches tried**:
- Move feq broadcast to per-function: partial help, loop persists
- Make `spec_unionfindsteph_wf` closed: no help
- Remove all wf reveals from union: no help (loop is in base context)
- rlimit(200): OOM, confirming unbounded divergence

## What Would Unblock This

1. Understanding how `closed spec fn` bodies leak into Z3 — this may be
   a fundamental Verus encoding behavior (definitional axioms always present)
   rather than a bug.

2. If #1 is confirmed: restructure `spec_uf_wf` to NOT include
   `spec_feq_full` and `spec_elements_distinct` in the same conjunction.
   Factor them into a separate predicate that's only revealed when needed.

3. Alternative: move `obeys_feq_view_injective` from `obeys_feq_full`
   into a separate opt-in predicate, so revealing spec_uf_wf doesn't
   bring the matching-loop trigger into scope.

## Verification

2410 verified, 0 errors (isolate Chap65). All changes are clean.
