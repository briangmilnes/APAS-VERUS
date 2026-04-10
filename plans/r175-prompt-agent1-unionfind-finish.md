# R175 Prompt — Agent 1: Finish UnionFind. Prove Big or Go Home. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `external_body`, `assume`, `admit`, or `accept`.**
6. **NEVER use rlimit above 30.**

## Read all standards first

## Context

R172: decomposed wf into 5 groups. Proved 3 of 5 original external_body.
R173: closed spec_elem_at_neq. Killed 9M matching loop.
R174: replaced =~= in wf with directional predicates. Proved lemma_rank_lt_elements.
  But regressed num_sets (directional predicates fire via wf in loop invariant).

Current state: 5 external_body. 3 real (lemma_union_merge_wf, fn union, num_sets).
2 experiments (don't care). 0 errors. 48s, 549MB.

## The 3 remaining problems

### 1. lemma_union_merge_wf (lines 1403-1434)

The function's own `requires` at lines 1431-1432 still has raw `=~=`:
```rust
uf.parent@.dom() =~= mid.parent@.dom(),
uf.rank@.dom() =~= mid.rank@.dom(),
```

You changed the wf sub-predicates to directional closed predicates, but you
didn't change this function's requires. The =~= in the requires puts the
extensional equality axiom directly into this function's Z3 context, where it
cross-fires with every contains_key.

**Fix:** Replace the =~= requires with directional pointwise facts:
```rust
forall|k: V::V| mid.parent@.contains_key(k) ==> uf.parent@.contains_key(k),
forall|k: V::V| uf.parent@.contains_key(k) ==> mid.parent@.contains_key(k),
forall|k: V::V| mid.rank@.contains_key(k) ==> uf.rank@.contains_key(k),
forall|k: V::V| uf.rank@.contains_key(k) ==> mid.rank@.contains_key(k),
```

Then update the call site in `fn union` to prove these directional facts instead
of the =~= (which it already has via the parent_insert ensures).

### 2. num_sets (lines 2255-2281)

The while loop invariant uses `self.spec_unionfindsteph_wf()`. When Z3 opens
the wf definition axiom, the directional domain predicates inside leak into
the loop context, cross-firing with contains_key from find_root_loop.

**Fix:** Don't use `spec_unionfindsteph_wf()` in the loop invariant. Use only
the specific group predicates the loop body actually needs:
```rust
invariant
    spec_wf_type_dom(self),     // for clone/feq axioms
    spec_wf_elem_map(self),     // for elements@[i]@ == parent key
    // NOT spec_wf_roots, NOT spec_wf_rank, NOT spec_wf_elem_distinct
    self@ == old(self)@,
    0 <= i <= self.elements@.len(),
```

The loop body calls `find_root_loop(self, &v)` which requires
`self.spec_unionfindsteph_wf()`. So you need to either:
- (a) Change find_root_loop's requires to accept group predicates, or
- (b) Prove `spec_unionfindsteph_wf(self)` inside the loop body from the
  group invariants using `lemma_assemble_wf_from_groups`, keeping it out of
  the loop invariant's Z3 context.

Option (b) is simpler — call the assembly lemma inside a proof block in the
loop body, then call find_root_loop. The assembly lemma runs in its own Z3
context (it just joins the groups), and find_root_loop gets the full wf.

### 3. fn union (line ~2200)

Blocked on lemma_union_merge_wf. Once #1 is fixed, tackle this. The trait
ensures has =~= on parent.dom(). If needed, wrap it in a closed predicate
or prove it from directional facts in a helper that isolates the =~= from
the rest of the proof context.

## Order

1. Fix lemma_union_merge_wf: replace =~= requires with directional facts
2. Fix num_sets: use group predicates in loop invariant + assembly in body
3. Fix fn union: whatever remains after #1
4. Validate after each: `scripts/validate.sh isolate Chap65`

## The experiments

`union_experiment_merge_no_wf_ensures` and `union_experiment_merge_with_wf`
are experiments. If they regress, mark them with `#[verifier::external_body]`
and move on. Do not spend time fixing experiments.

## Validation

```bash
scripts/validate.sh isolate Chap65
```

## Report

Write `plans/agent1-round175-report.md`.

## RCP

`git add -A && git commit -m "R175 Agent 1: finish UnionFind — prove remaining external_body (−N holes)"`, then `git push`.
