# R173 Prompt — Agent 1: Fix lemma_rank_lt_elements matching loop. AFK.

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

Read every file in `src/standards/`. Then read this prompt again.

## The problem

`lemma_rank_lt_elements` in `src/Chap65/UnionFindStEph.rs` fails at rlimit 80.
It's a matching loop, not a budget issue. Your R172 work killed the 9M
`spec_elements_distinct` loop by changing the trigger to `spec_elem_at_neq`.
But the function still loops.

## Root cause analysis

I've read your code. Here's what's happening:

1. `spec_elem_at_neq` is **open**. Z3 sees through it to `elements@[i]@ != elements@[j]@`.

2. `lemma_element_index` ensures `uf.elements@[i]@ == v_view`. This puts
   `elements@[i]@` terms into the Z3 context.

3. `lemma_rank_lt_elements` calls `lemma_element_index` twice — once for `v_view`
   (returns `i_v`) and once for `w` (returns `i_w`). That gives Z3 two
   `elements@[_]@` terms.

4. Even though `spec_elements_distinct` is closed and not revealed in
   `lemma_rank_lt_elements`, the trigger `spec_elem_at_neq` is open. Z3 unfolds
   it and sees `elements@[i]@ != elements@[j]@`. The two terms from
   `lemma_element_index` match `i` and `j`, firing the quantifier. This cascades.

## The fix

**Make `spec_elem_at_neq` closed.** That's the whole point of a trigger wrapper —
Z3 should only fire `spec_elements_distinct` when `spec_elem_at_neq` appears
literally, not when `elements@[i]@` terms happen to exist.

```rust
pub closed spec fn spec_elem_at_neq<V: StT + Hash>(
    uf: &UnionFindStEph<V>, i: int, j: int
) -> bool {
    uf.elements@[i]@ != uf.elements@[j]@
}
```

After making it closed, any code that needs the actual inequality must call a
reveal lemma:

```rust
proof fn lemma_reveal_elem_at_neq<V: StT + Hash>(
    uf: &UnionFindStEph<V>, i: int, j: int,
)
    requires spec_elem_at_neq(uf, i, j),
    ensures uf.elements@[i]@ != uf.elements@[j]@,
{ reveal(spec_elem_at_neq); }
```

## Steps

1. Read all standards.
2. Read `src/Chap65/UnionFindStEph.rs` — understand the current state.
3. Make `spec_elem_at_neq` **closed**.
4. Validate: `scripts/validate.sh isolate Chap65`. Expect new errors from code
   that was relying on the open definition.
5. Fix each error by adding targeted `reveal(spec_elem_at_neq)` or calling
   the reveal lemma in the specific proof context that needs it.
6. Validate again. `lemma_rank_lt_elements` should now verify — the trigger
   wrapper is opaque and Z3 can't fire `spec_elements_distinct` from
   `elements@[i]@` terms alone.
7. If `lemma_rank_lt_elements` still fails, profile it:
   `scripts/validate.sh isolate Chap65 --profile` and read the instantiation
   counts. Report what's still firing.

## After rank_lt_elements

If it works, also try:
- `lemma_union_merge_wf`: the 19.8 GB / rlimit 80 monster. The `=~=` in its
  requires generates 100K instantiations. Try replacing `=~=` in the requires
  with a closed predicate `spec_dom_eq(a, b)` that wraps `a =~= b`. Same
  principle: don't let Z3 see extensional equality directly.
- `fn union`: blocked on merge_wf. Try after.

## Do NOT touch

- `KruskalStEph.rs` or `PrimStEph.rs`
- Any file outside Chap65

## Validation

```bash
scripts/validate.sh isolate Chap65
```

## Report

Write `plans/agent1-round173-report.md`.

## RCP

`git add -A && git commit -m "R173 Agent 1: close spec_elem_at_neq, prove lemma_rank_lt_elements (−N holes)"`, then `git push`.
