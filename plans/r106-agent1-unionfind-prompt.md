# R106 Agent 1 — Prove UnionFind union_merge, STEP 20

## Objective

The last 2 external_body functions in APAS-VERUS: `union_merge` (line 1167) and
`union` (line 1404) in `src/Chap65/UnionFindStEph.rs`. `union` just calls
`union_merge`, so proving `union_merge` is the real target.

## The problem: spec_elements_distinct self-loop

Profiling shows `spec_elements_distinct` at 3.7M instantiations — a self-feeding
matching loop. The quantifier:

```rust
pub closed spec fn spec_elements_distinct<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
    forall|i: int, j: int|
        0 <= i < uf.elements@.len() as int &&
        0 <= j < uf.elements@.len() as int &&
        i != j ==>
        #[trigger] uf.elements@[i]@ != #[trigger] uf.elements@[j]@
}
```

The triggers `elements[i]@` and `elements[j]@` are symmetric — each instantiation
produces new `elements[k]@` terms that trigger more instantiations. This fires
3.7M times even though the functions that `reveal` it are proof lemmas, not exec.

Previously this was a feq × spec_elements_distinct feedback loop. Now feq is
`closed` (R105), so the feq side is dead (136 instantiations). But
spec_elements_distinct loops with itself — 3.7M instantiations from the
symmetric index-pair triggers.

## Strategy: avoid revealing spec_elements_distinct in union_merge

The key insight: `union_merge` does NOT modify `elements`. It only modifies
`parent`, `rank`, and `roots`. The elements array is unchanged:
```
uf.elements@ == old(uf).elements@
```

Therefore `spec_elements_distinct` is trivially preserved — if elements didn't
change, distinctness didn't change. The proof should NOT need to reveal the
quantifier body. Instead, use a frame lemma.

### Existing infrastructure

`lemma_union_wf_frame` (line 894) already does this:
```rust
proof fn lemma_union_wf_frame<V: StT + Hash>(
    uf: &UnionFindStEph<V>,
    mid: &UnionFindStEph<V>,
)
    requires
        spec_elements_forward(mid),
        spec_elements_backward(mid),
        spec_elements_distinct(mid),
        uf.elements@ =~= mid.elements@,
        uf.parent@.dom() =~= mid.parent@.dom(),
    ensures
        spec_elements_forward(uf),
        spec_elements_backward(uf),
        spec_elements_distinct(uf),
{
    reveal(spec_elements_forward);
    reveal(spec_elements_backward);
    reveal(spec_elements_distinct);
}
```

BUT — this still reveals spec_elements_distinct inside the frame lemma body,
which triggers the loop. The fix: **don't reveal spec_elements_distinct in the
frame lemma**. The proof that elements didn't change means distinctness transfers
without seeing the quantifier body.

### Proposed frame lemma fix

```rust
proof fn lemma_elements_distinct_frame<V: StT + Hash>(
    uf: &UnionFindStEph<V>,
    mid: &UnionFindStEph<V>,
)
    requires
        spec_elements_distinct(mid),
        uf.elements@ =~= mid.elements@,
    ensures
        spec_elements_distinct(uf),
{
    // Do NOT reveal spec_elements_distinct.
    // The elements are extensionally equal, so the predicate transfers
    // by substitution. Verus should handle this without the quantifier body.
    assert(uf.elements@ =~= mid.elements@);
}
```

If Verus can't prove this without reveal (because `closed` hides the body),
try making spec_elements_distinct `#[verifier::opaque]` instead of `closed`,
and use a targeted reveal ONLY in this frame lemma with a tight rlimit.

### Decomposing union_merge proof

The `union_merge` proof needs to establish `spec_unionfindsteph_wf(uf)` after mutations.
Instead of revealing all sub-predicates at once (which triggers the loop),
decompose into individual sub-predicate lemmas:

1. `lemma_elements_distinct_frame` — frame, no reveal needed
2. `lemma_elements_forward_frame` — frame, reveal only forward
3. `lemma_elements_backward_frame` — frame, reveal only backward
4. Individual lemmas for parent/rank/roots predicates — these don't involve
   elements, so no loop risk

Then `union_merge` calls each lemma separately and assembles wf from the parts,
never revealing spec_elements_distinct in a context where other quantifiers
can trigger the loop.

## What to prove

Remove `#[verifier::external_body]` from `union_merge` (line 1167). The body is
already written — it calls `union_merge_exec` for mutations, then needs proof
blocks to establish each sub-predicate of wf.

After `union_merge` verifies, remove `#[verifier::external_body]` from `union`
(line 1404). It calls `find_root_loop` twice, then `union_merge`. The ensures
should follow from `union_merge`'s ensures.

## Read first

- `src/Chap65/UnionFindStEph.rs` — the entire file, especially:
  - `spec_unionfindsteph_wf` and all sub-predicates (lines 114-185)
  - `lemma_insert_preserves_wf` (line 205) — working proof that reveals everything
  - `lemma_union_wf_frame` (line 894) — existing frame lemma
  - `union_merge_exec` (line 917) — the exec mutations, already verified
  - `union_merge` (line 1167) — TARGET
  - `union` (line 1404) — TARGET
  - `lemma_union_ensures_bridge` (line 680) — existing infrastructure
- `plans/agent1-r103-report.md` — previous UnionFind analysis

## Isolation

```bash
scripts/validate.sh isolate Chap65
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT increase rlimit above 30 for union_merge. If it needs more, the
  approach is wrong — the matching loop is running.
- Do NOT reveal spec_elements_distinct in the same proof context as other
  wf sub-predicates. That's what triggers the loop.
- The key proof insight: elements don't change during union, so elements
  predicates transfer by frame. Only parent/rank/roots predicates need
  real proof work.
- If the frame approach doesn't work (Verus can't transfer closed/opaque
  predicates without reveal), report what you tried and stop. Don't brute-force
  rlimit.

## STEP 20

## Report

Write `plans/agent1-r106-unionfind-report.md`.
