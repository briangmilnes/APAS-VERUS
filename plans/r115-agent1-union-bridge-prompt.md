# R115 Agent 1 — Prove union (tricky three). AFK. PBOGH.

## Context

Chap65 is commented out of lib.rs — the last 3 holes in the codebase.
`union_merge` is proved (R106). The tricky three remain:

1. `union` trait impl (UnionFindStEph.rs) — external_body
2. `uf_wf_opaque` (KruskalStEph.rs) — opaque wf wrapper, rename to `opaque_spec_unionfindsteph_wf`
3. `kruskal` (KruskalStEph.rs) — external_body, blocked by union

Your R106 report identified three blockers for union:
- Rank bounds not in wf
- Roots quantifier matching loop (spec_union_result × spec_roots_changed_by_merge × dom =~=)
- rlimit exhaustion calling infrastructure lemmas from &mut body

## Rename first

`uf_wf_opaque` is a terrible name. Rename it to `opaque_spec_unionfindsteph_wf`
in KruskalStEph.rs before doing anything else.

## Strategy A: Single bridge lemma (try this first)

Do ALL proof work in a single helper that takes `&` references (not `&mut`).
The `&mut` body calls this one lemma and nothing else.

```rust
proof fn lemma_union_bridge<V: StT + Hash>(
    uf_old: &UnionFindStEph<V>,  // before find_root_loop calls
    uf_mid: &UnionFindStEph<V>,  // after find_root_loop, before merge
    uf_new: &UnionFindStEph<V>,  // after union_merge
    u_view: V::V,
    v_view: V::V,
    root_u_view: V::V,
    root_v_view: V::V,
    info: UnionMergeInfo<V>,
)
    requires
        uf_old.spec_unionfindsteph_wf(),
        uf_mid.spec_unionfindsteph_wf(),
        uf_new.spec_unionfindsteph_wf(),
        // find_root_loop postconditions ...
        // union_merge postconditions (opaque form) ...
    ensures
        spec_union_result(uf_new.roots@, uf_old.roots@, u_view, v_view),
        uf_new.roots@.dom() =~= uf_old.roots@.dom(),
        uf_new.elements@ == uf_old.elements@,
{
    // Call infrastructure lemmas here, outside &mut encoding
}
```

Then union's body:
```rust
fn union(&mut self, u: &V, v: &V) {
    let ghost old_self = *self;
    let root_u = find_root_loop(self, u);
    let root_v = find_root_loop(self, v);
    let ghost mid_self = *self;
    if !feq(&root_u, &root_v) {
        let info = union_merge(self, root_u, root_v);
        proof { lemma_union_bridge(&old_self, &mid_self, self, ...); }
    }
}
```

The key: Z3 sees only the bridge lemma's requires/ensures in union's context,
not the revealed quantifier bodies.

## Strategy B: If bridge lemma still loops

If Strategy A still triggers matching loops, try decomposing wf into separate
predicates that are never all revealed simultaneously:

- Never reveal `spec_roots_changed_by_merge` and `spec_union_result` in the
  same context that has `dom =~=`
- Use intermediate `assert` checkpoints to let Z3 forget irrelevant facts
- Call sub-lemmas one at a time with `assert` between each

## Rank bounds

`union_merge` requires `rank < elements.len()`. This is a true invariant
(rank < log₂(n) < n) but not in the current 14-predicate wf. Options:
- Add `spec_rank_bounded` as a 15th sub-predicate
- Or prove it from existing predicates if possible

Try proving from existing first. If not, add the predicate and prove it's
preserved by `new`, `insert`, and `union_merge`.

## After union is proved

Once union compiles, tackle KruskalStEph:
1. Rename `uf_wf_opaque` → `opaque_spec_unionfindsteph_wf`
2. See if `kruskal` can now be proved through the stronger union ensures

## Read first

- `src/Chap65/UnionFindStEph.rs` — full file, especially the infrastructure lemmas
- `src/Chap65/KruskalStEph.rs` — the opaque wrapper and kruskal
- `plans/agent1-r106-unionfind-report.md` — your R106 report

## Uncomment Chap65

You need Chap65 in lib.rs to validate. Uncomment it:
```rust
#[cfg(all(not(feature = "experiments_only"), any(not(feature = "isolate"), feature = "Chap65")))]
pub mod Chap65 {
    pub mod UnionFindStEph;
    pub mod KruskalStEph;
    pub mod PrimStEph;
}
```

Use `scripts/validate.sh isolate Chap65` for all iterative work.

## Rules

- Do NOT increase rlimit above 50. If it needs more, the approach is wrong.
- Do NOT add assume or accept in algorithmic code.
- Do NOT weaken ensures.
- Commit working intermediate states.
- No subagents.

## STEP 50

## Report

Write `plans/agent1-r115-union-report.md`.
