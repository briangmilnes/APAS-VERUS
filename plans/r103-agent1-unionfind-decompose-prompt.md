# R103 Agent 1 — UnionFind struct decomposition to avoid Z3 &mut blowup, STEP 20

## Objective

UnionFindStEph has 2 external_body functions (union_merge, union) blocked by
Z3 using 17GB+ on `&mut` of a 4-field struct with quantified ensures. Decompose
the mutation into field-level helpers that Z3 can handle.

## The Problem

```rust
pub struct UnionFindStEph<V: StT + Hash> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}
```

When `union_merge(uf: &mut UnionFindStEph<V>, ...)` has ensures that quantify
over `uf.parent@`, `uf.rank@`, `uf.size@`, Z3 generates a cross-product encoding
of all 4 fields' frame conditions. This blows up to 17GB+.

## The Fix: Field-level mutation helpers

Write a helper that takes individual `&mut` references:

```rust
fn union_merge_fields(
    parent: &mut Vec<usize>,
    rank: &mut Vec<usize>,
    size: &mut Vec<usize>,
    count: &mut usize,
    root_u: usize,
    root_v: usize,
) -> (info: Ghost<UnionMergeInfo<V>>)
    requires
        old(parent)@.len() == old(rank)@.len(),
        old(parent)@.len() == old(size)@.len(),
        root_u < old(parent)@.len(),
        root_v < old(parent)@.len(),
        root_u != root_v,
    ensures
        parent@.len() == old(parent)@.len(),
        rank@.len() == old(rank)@.len(),
        size@.len() == old(size)@.len(),
        // Each field's frame condition is INDEPENDENT
        forall|i: int| 0 <= i < parent@.len() && i != root_u as int
            ==> #[trigger] parent@[i] == old(parent)@[i],
        // ... rank and size similar
```

Each `&mut Vec<usize>` is a simple single-field mutation. Z3 encodes each
independently — no cross-product.

Then `union_merge` destructures and delegates:

```rust
fn union_merge<V: StT + Hash>(uf: &mut UnionFindStEph<V>, root_u: V, root_v: V)
    -> (info: Ghost<UnionMergeInfo<V>>)
{
    let info = union_merge_fields(
        &mut uf.parent, &mut uf.rank, &mut uf.size, &mut uf.count,
        root_u_idx, root_v_idx,
    );
    info
}
```

**WAIT**: Verus may not allow `&mut uf.parent` (field borrows on &mut self).
If not, use a different pattern:
- Take ownership: `let mut parent = std::mem::take(&mut uf.parent);`
- Mutate `parent` directly
- Put it back: `uf.parent = parent;`
- The ensures on the outer function use `old(uf)` and `uf` which Verus handles

## What to prove

The exec logic in `union_merge` is already written (inside external_body):
1. Compare ranks of root_u and root_v
2. Winner gets loser as child (parent[loser] = winner)
3. Update size[winner] += size[loser]
4. Maybe update rank[winner]
5. Decrement count

The proof delegates to `lemma_union_merge_wf` (already proved) which shows
the wf invariant is maintained. All the lemma infrastructure exists.

## Also prove `union`

`union` calls `find_root_loop` (proved) then `union_merge`. Once union_merge
verifies, union should follow — remove its external_body too.

Then Kruskal's `kruskal_process_edge` and `kruskal_mst` should cascade
(they're downstream of union).

## Read first

- `src/Chap65/UnionFindStEph.rs` — union_merge (line 1087), union (line 1315)
- `src/experiments/mut_struct_quantifier_limit.rs` — the minimal experiment confirming the blowup
- `src/Chap65/KruskalStEph.rs` — downstream kruskal_process_edge, kruskal_mst

## Isolation

```bash
scripts/validate.sh isolate Chap65
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept.
- If `&mut uf.parent` field borrow doesn't work in Verus, use take/put-back.
- If the decomposition STILL blows up Z3, try with `#[verifier::rlimit(20)]`
  and report the Z3 memory usage.
- Even proving union_merge alone (-1 hole) is a breakthrough.

## STEP 20

## Report

Write `plans/agent1-r103-unionfind-report.md`.
