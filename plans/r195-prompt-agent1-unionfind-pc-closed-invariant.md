# R195 Prompt — Agent 1: Close the loop invariant. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent1`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, or `accept`.**
6. **NEVER read `src/Chap65/UnionFindStEph.rs`.** Clean-room.
7. **NEVER modify `Cargo.toml`, `scripts/validate.sh`.**
8. **ONLY modify `src/UnionFind/UnionFindPCStEph.rs`.**

## Read all standards first

## Context

R190-R193: path compression built, 1 error remains. The compression while
loop in find() has a 35-line, 15-quantifier invariant. Z3 cross-fires every
quantifier at every iteration, causing rlimit timeout even at 500.

R194: broadcast proof fn didn't fit (9 context params + z, no single trigger).

## The problem

The loop invariant restates wf for TWO maps (current and original) as
raw quantifiers. That's ~15 foralls visible to Z3 on every iteration.
Z3 cross-fires them. This is the same problem we solved for the wf
decomposition in R172 — too many quantifiers in one Z3 context.

## The fix: closed predicates for the loop invariant

Bundle the invariant into 3-4 closed spec fns. Z3 sees opaque names,
not the quantifiers inside. Reveal only what each proof step needs,
inside isolated `assert ... by { reveal(...); }` blocks.

### Step 1: Define closed invariant predicates

```rust
/// Light wf: parent-in-domain, rank invariant, rank bounded, domain finite.
/// Does NOT include size_rank_inv.
pub closed spec fn spec_light_wf<V: Bounds>(
    parent: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
) -> bool {
    // all the foralls for parent-in-domain, rank invariant, rank bounded, etc.
}

/// Find preservation: all elements have the same root in current as in orig.
pub closed spec fn spec_find_preserved<V: Bounds>(
    parent_curr: Map<V::V, V>, rank: Map<V::V, usize>, n: nat,
    parent_orig: Map<V::V, V>,
) -> bool {
    forall|z: V::V| parent_orig.dom().contains(z) ==>
        spec_pure_find(parent_curr, rank, n, z)
            == spec_pure_find(parent_orig, rank, n, z)
}

/// Domain unchanged: current and orig have same domain.
pub closed spec fn spec_same_domain<V: Bounds>(
    parent_curr: Map<V::V, V>, parent_orig: Map<V::V, V>,
) -> bool {
    forall|k: V::V| parent_curr.dom().contains(k) <==> parent_orig.dom().contains(k)
}
```

### Step 2: Rewrite the loop invariant

```rust
while steps < n
    invariant
        // Closed predicates — Z3 can't see inside
        spec_light_wf::<V>(self.parent@, self.rank@, n),
        spec_light_wf::<V>(orig_parent, orig_rank, orig_n),
        spec_find_preserved::<V>(self.parent@, self.rank@, n, orig_parent),
        spec_same_domain::<V>(self.parent@, orig_parent),
        // Small open facts
        self.rank@ == orig_rank,
        n == orig_n,
        spec_is_root_map::<V>(self.parent@, root@),
        self.parent@.dom().contains(root@),
        self.parent@.dom().contains(curr@),
        root@ == spec_pure_find::<V>(orig_parent, orig_rank, orig_n, v@),
        steps <= n,
    decreases n - steps,
```

~12 lines, 0 visible quantifiers. Z3 carries the closed predicates
as opaque booleans.

### Step 3: In the loop body, reveal selectively

For each micro-lemma call, reveal only what it needs:

```rust
proof {
    // Need parent-in-domain for curr:
    assert(self.parent@.dom().contains(curr@)) by {
        reveal(spec_light_wf);
    };

    // Compress step wf:
    lemma_compress_parent_in_dom(...);
    lemma_compress_rank_inv(...);
    lemma_compress_basic(...);

    // Find preservation:
    lemma_compress_step_find(...);

    // Re-establish closed invariants for next iteration:
    // spec_light_wf for new parent — from micro-lemma postconditions
    assert(spec_light_wf::<V>(new_parent, self.rank@, n)) by {
        reveal(spec_light_wf);
        // Z3 has the postconditions from micro-lemmas
    };
}
```

Each `reveal` is inside an `assert ... by { }` block — isolated Z3
context. The main loop body context never sees the raw quantifiers.

### Step 4: Drop orig_wf from the invariant

The orig maps never change. The function's requires already gives
`spec_uf_wf(orig_parent, orig_rank, orig_n)`. Z3 should have this
from the function-level context. Try dropping `spec_light_wf(orig_parent, ...)`
from the invariant entirely. If Z3 loses it across iterations, add it back
as a closed predicate.

## Also: rename union_sets to union

## Validation

```bash
scripts/validate.sh isolate UnionFind
```

Target: **0 errors, 0 external_body.**

If the closed invariant approach gets find() to verify, remove the
`#[verifier::external_body]` that's currently on find(). If it still
times out, report the Z3 RSS and what's still cross-firing.

## Report

Write `plans/agent1-round195-report.md`.

## RCP

`git add -A && git commit -m "R195 Agent 1: closed loop invariant — find() proved"`, then `git push`.
