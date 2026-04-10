# R172 Prompt — Agent 1: Decompose UnionFind wf predicate, prove 5 external_body. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**
6. **NEVER use rlimit above 30.** The problem is structural, not budget.

## Read all standards first

Read every file in `src/standards/`. Then read this prompt again.

## The problem

`src/Chap65/UnionFindStEph.rs` has 5 `external_body` functions. All 5 share the
same root cause: Z3 matching loops from `spec_elements_distinct`'s two-trigger
quantifier cross-firing with `elements@[i]@` terms, and `=~=` extensional equality
generating pointwise axioms that cross-fire with `contains_key`.

Profile data (from R169):

| # | Instantiation Count | Quantifier |
|---|---------------------|------------|
| 1 | 6,554,456 | `spec_elements_distinct` (from `lemma_union_merge_wf`) |
| 2 | 1,083,406 | `spec_elements_distinct` (from `lemma_insert_preserves_wf`) |
| 3 | 811,320 | `spec_elements_distinct` (from `lemma_rank_lt_elements`) |

## The 5 external_body functions to prove

All in `src/Chap65/UnionFindStEph.rs`. All have correct specs. All have proof
bodies present but bypassed.

1. **`lemma_rank_lt_elements`** — proof fn. The `i_v != i_w` proof is propositional.
   Does NOT need `spec_elements_distinct`. Loops because `spec_elements_backward`
   (revealed for choose) shares Z3 context.

2. **`lemma_assemble_wf`** — proof fn. Body is `reveal(spec_unionfindsteph_wf)` with
   all 16 sub-predicates as opaque booleans in requires. Should be trivial. Loops
   because revealing the 16-conjunct wf puts all sub-predicates in scope together.

3. **`lemma_union_merge_wf`** — proof fn. Orchestrator calling 6 sub-lemmas. Uses
   `lemma_decompose_wf(mid)` which leaks `spec_elements_distinct(mid)` into scope.

4. **`lemma_parent_dom_eq_from_wf`** — proof fn. 3-line proof. Loops because
   `=~=` + `reveal(spec_roots_changed_by_merge)` coexist.

5. **`fn union`** — exec fn. Trait ensures `self@.parent.dom() =~= old(self)@.parent.dom()`.
   The `=~=` coexists with `contains_key` terms from `find_root_loop` ensures.

## The strategy: decompose wf into opaque sub-groups

The core fix is: **never let `spec_elements_distinct` and `=~=` appear in the same
Z3 context as `elements@[i]@` terms.**

### Step 1: Decompose `spec_unionfindsteph_wf`

The current wf predicate is a 16-conjunct conjunction. Split it into 3-4 opaque
sub-predicates, each grouping related conjuncts:

```rust
// Group A: structural (lengths, domains match)
pub closed spec fn spec_wf_structural(uf: &UnionFindStEphS) -> bool { ... }

// Group B: element properties (distinct, backward, forward)
pub closed spec fn spec_wf_elements(uf: &UnionFindStEphS) -> bool { ... }

// Group C: root/rank properties
pub closed spec fn spec_wf_roots(uf: &UnionFindStEphS) -> bool { ... }

// Group D: parent/rank relationship
pub closed spec fn spec_wf_parent_rank(uf: &UnionFindStEphS) -> bool { ... }

// Main wf = A && B && C && D
pub open spec fn spec_unionfindsteph_wf(uf: &UnionFindStEphS) -> bool {
    spec_wf_structural(uf) && spec_wf_elements(uf) && spec_wf_roots(uf) && spec_wf_parent_rank(uf)
}
```

### Step 2: Write per-group reveal lemmas

```rust
proof fn lemma_reveal_structural(uf: &UnionFindStEphS)
    requires spec_unionfindsteph_wf(uf)
    ensures spec_wf_structural(uf)  // and the individual conjuncts
{ reveal(spec_unionfindsteph_wf); reveal(spec_wf_structural); }
```

One for each group. These are cheap — they just open one layer.

### Step 3: Reprove each external_body using only the groups it needs

- `lemma_rank_lt_elements`: needs group C (roots) + maybe A (structural). Does NOT
  need group B (elements — the dangerous one).
- `lemma_assemble_wf`: reverse direction — given A && B && C && D, prove wf. Use
  the conjunction-building pattern (assert each group, then assert wf == conjunction).
- `lemma_union_merge_wf`: call targeted extractors instead of `lemma_decompose_wf`.
  Never reveal group B in the same context as group A.
- `lemma_parent_dom_eq_from_wf`: use a closed spec wrapper around `=~=` to prevent
  extensional equality from leaking.
- `fn union`: wrap the `=~=` ensures in a closed predicate, or split the postcondition
  proof into a helper that doesn't see `contains_key`.

### Step 4: Validate

```bash
scripts/validate.sh isolate Chap65
```

Must get 0 errors, 0 external_body on algorithmic logic (the 5 functions must all
have real proof bodies).

## What NOT to do

- Do NOT raise rlimit. If you need >10 on any function, the decomposition is wrong.
- Do NOT add `assume` or `accept` anywhere.
- Do NOT weaken any ensures clauses.
- Do NOT change the trait signatures or public API.
- Do NOT touch `KruskalStEph.rs` or `PrimStEph.rs` — only `UnionFindStEph.rs`.
- Do NOT try to fix all 5 at once. Fix them one at a time, validating after each.
  Start with `lemma_assemble_wf` (should be easiest after decomposition), then
  `lemma_parent_dom_eq_from_wf`, then `lemma_rank_lt_elements`, then
  `lemma_union_merge_wf`, then `fn union`.

## Uncommenting UnionFind in lib.rs

UnionFindStEph and KruskalStEph are currently commented out in lib.rs. You need to
uncomment them to validate. Do this first, then validate isolate Chap65 to confirm
agent1's R169 state (0 errors, 5 external_body) still holds.

## Validation

```bash
scripts/validate.sh isolate Chap65
```

## Report

Write `plans/agent1-round172-report.md`.

## RCP

`git add -A && git commit -m "R172 Agent 1: decompose UnionFind wf, prove N external_body (−N holes)"`, then `git push`.
