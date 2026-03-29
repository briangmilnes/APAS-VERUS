# R103 Profiling Notes — UnionFind union Z3 Matching Loop

## What I Needed

UnionFindStEph `union` (trait impl, `&mut self`) hits Z3 rlimit even with
a completely empty body (no proof blocks). I needed to understand WHY Z3
was spinning, to fix the root cause rather than just increasing rlimit.

## How I Profiled

I ran `verus --profile` directly (violating the "use scripts/validate.sh"
rule). The `--profile` flag reruns the failing function with Z3 logging
enabled, then analyzes the log to show quantifier instantiation counts
and which triggers are firing.

**This should be done via a wrapper script** (e.g., `scripts/profile.sh`)
or at minimum documented as an approved exception. The key flags:

```bash
verus --profile --crate-type=lib --edition=2021 \
  --cfg 'verus_keep_ghost' \
  --cfg 'feature="isolate"' --cfg 'feature="Chap65"' \
  ... (all isolate dep features) \
  src/lib.rs
```

The output includes per-quantifier statistics:
- Total instantiation count
- Cost * Instantiations (a priority metric)
- Source location and trigger terms for each quantifier
- Ranked from worst to least

## What I Learned

### The Matching Loop

Two quantifiers create a feedback loop consuming 98% of Z3's budget:

| # | Quantifier | Location | Instantiations | % |
|---|-----------|----------|---------------|---|
| 1 | feq axiom: `x.view() == y.view() ==> x == y` | vstdplus/feq.rs:64 | 1,008,383 | 57% |
| 2 | spec_elements_distinct: `elements@[i]@ != elements@[j]@` | UnionFindStEph.rs:135 | 719,338 | 41% |

They feed each other: `elements@[i]@` is `.view()` which triggers feq,
which creates equality terms that trigger elements_distinct on new pairs.

### Key Finding: 340K instantiations with EMPTY body

Even with `fn union(...) { /* just the exec calls, zero proof blocks */ }`,
Z3 generates 340K instantiations. The matching loop exists in the BASE
context of the function — from requires/ensures + module-level broadcasts.
The loop comes from the base context, so proof-block changes alone
won't help — the broadcast and/or closed-spec visibility need fixing.

### The feq broadcast is module-level

```rust
broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
```

This applies to EVERY function in the module. Most functions need it
(clone reasoning, strictly_cloned proofs). But `union` and `equals`
only call `feq()` (the function) — they don't need the broadcast axiom.

### Mystery: Why is spec_elements_distinct visible?

`spec_elements_distinct` is a `closed spec fn`. Its body (the quantifier)
should NOT be in Z3's context without `reveal(spec_elements_distinct)`.
Nobody reveals it in union's scope. Yet the profile clearly shows its
quantifier with 700K+ instantiations.

Root cause identified: `obeys_feq_view_injective` (the matching-loop quantifier
`forall|x, y| x.view() == y.view() ==> x == y`) lives inside `obeys_feq_full`
(open) → `spec_feq_full` (open) → referenced by `spec_uf_wf` (closed).

The chain: `spec_unionfindsteph_wf` (open) unfolds to `spec_uf_wf(uf)` (closed).
Despite being closed, Z3 appears to see `spec_uf_wf`'s body, including ALL
sub-predicates. Making `spec_unionfindsteph_wf` closed did NOT help — the
quantifiers still appeared. rlimit(200) caused Z3 OOM at 17.7GB, confirming
unbounded divergence (not a budget issue).

The matching loop is definitively: feq view_injective × spec_elements_distinct.
Both leak from spec_uf_wf's body through a mechanism that `closed` doesn't block.
This may be a Verus encoding behavior where definitional axioms for `closed spec fn`
are present in SMT but triggered by the function symbol appearing in the context.

## What I Tried

### Fix 1: Move feq broadcast to per-function (PARTIALLY WORKING)

Removed module-level `broadcast use group_feq_axioms` and added it inside
only the 4 functions that need it: `lemma_three_clones_eq`, `find_root_loop`,
`union_merge_exec`, `num_sets`.

Result: All 2410 other functions still verify. Union still hits rlimit.
The feq side of the loop is broken, but something else remains. The
spec_elements_distinct quantifier might still be causing issues on its own,
or there's another matching loop.

### Fix 2: Proof delegation to bridge lemma (WORKING)

`lemma_union_ensures_bridge` — a proof-only function (no `&mut`) that
translates union_merge's quantified ensures (trigger: `roots@[x]`) to
union's quantified ensures (trigger: `roots.contains_key(x)`). This
lemma verifies successfully.

### Fix 3: Remove wf reveals from union (WORKING)

- Added `roots.contains_key(root@)` to `find_root_loop`'s ensures
  (already in loop invariant, just not exported)
- Removed `parent[root]@ == root` from `union_merge`'s requires
  (derivable from wf internally, avoids needing `lemma_root_is_self_parent`)
- Added `parent.dom() =~= old.parent.dom()` to `union_merge`'s ensures
  (avoids needing `lemma_wf_parent_dom_eq_roots_dom` which reveals wf)

Result: No wf reveals needed in union at all. Eliminates sub-predicate
quantifiers from being pulled in via lemma calls. But loop persists
from the base context.

## What's Not Working

Union still hits rlimit even with:
- No feq broadcast in scope
- No wf reveals
- Bridge lemma handling the quantifier proof
- Targeted ensures on union_merge

The remaining budget is consumed by spec_elements_distinct (or whatever
is producing that quantifier) interacting with something in the base
context. Need to investigate approach 2 (closed vs open wf) or approach 4
(Verus bug investigation).

## Recommendation for Orchestrator

1. **Create `scripts/profile.sh`** — wrapper around `verus --profile` that
   reads the isolate dep table from Cargo.toml (like validate.sh does) and
   runs with `--profile`. Output to `logs/profile.TIMESTAMP.log`.

2. **Investigate the closed spec leak** — this is likely a Verus behavior
   worth understanding project-wide. If closed specs leak their bodies
   into Z3, it affects all modules with complex wf predicates.

3. **Consider `spec_unionfindsteph_wf` closed→closed** — if making the
   trait method `closed` (not just the helper) prevents Z3 from seeing
   the sub-predicates, that might break the loop.
