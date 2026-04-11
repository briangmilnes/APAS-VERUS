# R187 Prompt — Agent 1: Close the 7 gaps. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `src/lib.rs`, or `scripts/validate.sh`.**
   Use `scripts/validate.sh` as-is. If isolate UnionFind doesn't work,
   use full validate. Do NOT waste time fixing build infrastructure.

## Read all standards first

## Context

R186: 724 verified, 7 errors, fuel-based spec_pure_find. Two root causes:

1. **find() postcondition**: loop invariant gives `find(curr, remaining_fuel) == find(v, n)`.
   At loop exit, curr is a root, so `find(curr, any_fuel) == curr`. But Z3
   can't connect `curr == find(v, n)` without an explicit fuel-equivalence
   assertion at the return point.

2. **union_sets() postconditions**: find_after_link needs parent-in-domain
   for the new parent map. Z3 can't derive these from forall quantifiers
   without explicit assertions.

Both are the same problem: **Z3 needs explicit trigger instantiation for
Map domain quantifiers.** Every. Single. Call.

## The fix pattern (do this systematically)

For EVERY proof lemma call and EVERY recursive step:

```rust
// Before calling lemma_foo(parent, k):
assert(parent.dom().contains(k));  // trigger the domain quantifier
lemma_foo(parent, k);
```

For EVERY forall-implies about the new parent map after mutation:

```rust
// After parent_new = parent.insert(loser, winner):
// Bridge: old domain + insert = new domain
assert(parent_new.dom().contains(loser@));   // just inserted
assert(parent_new.dom().contains(winner@));  // was in old, still in new
assert(parent_new.dom().contains(z@));       // for each z in the proof
```

For the find() postcondition:

```rust
// At loop exit: curr is a root
assert(parent[curr as int] == curr);  // loop condition false
assert(spec_pure_find(parent_view, n, curr@, 1) == curr@);  // root in 1 step
// From loop invariant: find(curr, remaining) == find(v, n)
// Therefore: curr == find(v, n)
lemma_fuel_ge(parent_view, n, curr@, 1, remaining_fuel);
assert(curr@ == spec_pure_find(parent_view, n, v@, n));
```

## Do NOT restructure the architecture

The fuel-based approach is correct. The specs are correct. The proof
architecture is correct. Do NOT rewrite anything. Just add the missing
assertions. This is grunt work — there are ~20-30 assertions to add
across find() and the three union branches. Do them all.

## Strategy

1. Fix find() first — add fuel-equivalence assertion at both return points
   (early return for root, and loop exit).
2. Validate. Should drop from 7 to ~4 errors.
3. Fix union_sets() rx < ry branch — add all domain bridges for the new
   parent map. Call lemma_find_after_link with explicit domain assertions.
4. Validate. Check which errors remain.
5. Fix rx > ry branch (symmetric).
6. Fix rx == ry branch (same + rank_bounded).
7. Validate after each branch.

## Rank bounded

For the equal-rank case, if you haven't ported the counting machinery yet,
you can use a simpler argument for now:

Two distinct roots root_u, root_v in domain → domain.len() >= 2 → n >= 2.
Root_u has rank r, which means at least one non-root has rank < r and points
toward root_u. So root_u's tree has >= 2 elements. Same for root_v.
Disjoint trees: >= 4 elements. So n >= 4 when r >= 1.

More generally: a root of rank r needs a descendant of each rank 0..r-1
(by rank_invariant, each parent has strictly higher rank). That's r+1
elements per tree. Two trees: 2*(r+1) <= n. So r+1 <= n/2 < n.

If this is too hard to prove in one round, leave it as the last error
and report clearly what's needed.

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Or if that doesn't work:
```bash
scripts/validate.sh
```

## Report

Write `plans/agent1-round187-report.md`.

## RCP

`git add -A && git commit -m "R187 Agent 1: close HashMap UnionFind gaps"`, then `git push`.
