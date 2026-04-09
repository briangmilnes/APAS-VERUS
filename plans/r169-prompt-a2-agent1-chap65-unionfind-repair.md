# R169 Prompt A2 — Agent 1: Repair UnionFindStEph — decompose the wf predicate. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**
6. **DO NOT bump rlimit above 30.** Brute force budget is not the answer.
   If Z3 needs more than rlimit 30, the proof structure is wrong.

## Background

`src/Chap65/UnionFindStEph.rs` was fully proven at R130 (April 1) with
zero holes. Between then and now, vstdplus TotalOrder was restructured
and the proofs no longer verify. 5 errors: 1 postcondition + 4 rlimit.

The root cause is Z3 matching loops on the 13-quantifier wf predicate
`spec_unionfindsteph_wf`. When Z3 sees all 13 quantifiers at once, it
generates exponential trigger instantiations. The fix is structural
decomposition, not more budget.

## Strategy: DO NOT throw rlimit at this.

### Step 1 — Profile first

```bash
scripts/validate.sh isolate Chap65 --profile
ls -t logs/profile/SUMMARY-*.txt | head -1 | xargs cat
```

Read the profile. Find which quantifiers Z3 is stuck on. How many
instantiations? Which triggers are firing? This tells you exactly
what to fix.

### Step 2 — Decompose the wf predicate

`spec_unionfindsteph_wf` has 13 conjuncts. Z3 sees all 13 quantifiers
simultaneously and matching-loops between them. The fix:

**Option A — Assert conjuncts individually.** Before any line that needs
`spec_unionfindsteph_wf(uf)`, prove each conjunct separately:

```rust
assert(uf.parent_closed());      // conjunct 1
assert(uf.self_parent_is_root()); // conjunct 2
assert(uf.rank_increases());      // conjunct 3
// ... etc
assert(spec_unionfindsteph_wf(uf)); // now Z3 has all pieces, combines trivially
```

This is the R28 conjunction flakiness workaround. Z3 handles 13 small
queries in sequence, not 1 big query with 13 quantifiers cross-firing.

**Option B — Make wf opaque with per-conjunct reveal lemmas.**

```rust
#[verifier::opaque]
pub open spec fn spec_unionfindsteph_wf(...) -> bool { ... }

proof fn lemma_wf_parent_closed(uf: &UnionFindStEph<V>)
    requires spec_unionfindsteph_wf(uf),  // opaque, Z3 doesn't see inside
    ensures uf.parent_closed(),
{ reveal(spec_unionfindsteph_wf); }

proof fn lemma_wf_from_conjuncts(uf: &UnionFindStEph<V>)
    requires uf.parent_closed(), uf.self_parent_is_root(), ...
    ensures spec_unionfindsteph_wf(uf),
{ reveal(spec_unionfindsteph_wf); }
```

Callers never see all 13 quantifiers at once. They extract what they need
via reveal lemmas, prove their obligations conjunct-by-conjunct, then
reassemble via `lemma_wf_from_conjuncts`.

**Option C (if A and B fail) — Intermediate ghost state.** Add ghost
variables that cache individual conjuncts:

```rust
let ghost parent_closed = uf.parent_closed();
assert(parent_closed);
// ... Z3 only sees `parent_closed` as a bool, not the quantifier body
```

### Step 3 — Fix the postcondition failure first

The postcondition failure at line 286 is likely a changed ensures from
a dependency (TotalOrder restructuring). Read the error carefully. The
fix may be as simple as adding a call to a renamed lemma or asserting
a fact that a deleted bridge lemma used to provide.

### Step 4 — After fixes, validate Kruskal too

Kruskal depends on UnionFind. After UnionFind verifies:

```bash
scripts/validate.sh isolate Chap65
```

Must show 0 errors across all 3 files.

## Read all standards first.

## What changed in dependencies

- TotalOrderBridge merged into TotalOrder
- Bridge lemma assumes moved to TotalOrder default bodies
- ThreadShareablePlus deleted

Search for uses of TotalOrder, bridge lemmas, or ThreadShareablePlus
in UnionFindStEph.rs to find what broke.

## Report

Write `plans/agent1-round169-report.md`.

## RCP

`git add -A && git commit -m "R169 Agent 1: repair UnionFindStEph — decompose wf predicate (−5 errors)"`, then `git push`.
