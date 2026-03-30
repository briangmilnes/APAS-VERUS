# R109 Agent 1 — Prove `union`. STEP 20.

## Context

You proved `union_merge` in R106 — excellent work. The infrastructure lemmas for
`union` are all written and verified individually. The problem is composing them
in `union`'s `&mut` body without triggering the matching loop.

Your R106 report identified three blockers:

1. **Rank bounds**: `union_merge` requires `rank < elements.len()` but this isn't
   derivable from the current 14-predicate wf.
2. **Roots quantifier matching loop**: proving `spec_union_result` from
   `spec_roots_changed_by_merge` in any context with Set `=~=` triggers a loop.
3. **rlimit exhaustion**: calling infrastructure lemmas from `union`'s `&mut` body
   adds opaque booleans + dom `=~=` to the Z3 context, exceeding rlimit 30.

## Strategy: single bridge lemma outside &mut

The key insight: do ALL the proof work in a single helper that takes `&` references
(not `&mut`). The `&mut` body calls this one lemma and nothing else.

### Step 1: Add rank bounds to wf (or prove from wf)

`rank[v] < elements.len()` for all v is a true invariant of union-by-rank. Two options:

**Option A**: Add `spec_rank_bounded` as a 15th sub-predicate of wf. Then prove it's
preserved by `insert`, `union_merge`, and `new`. This is the clean approach — rank
bounds are a real structural invariant.

**Option B**: Prove `rank[v] < elements.len()` from existing wf predicates. The chain:
`rank[v] <= rank[roots[v]]` (from `spec_rank_bounded_by_root`) and root ranks are
bounded by... what? If there's no existing bound, option A is needed.

### Step 2: Single bridge lemma

Write one proof function that takes immutable snapshots and produces everything
`union` needs:

```rust
proof fn lemma_union_bridge<V: StT + Hash>(
    uf_old: &UnionFindStEph<V>,  // snapshot before find_root_loop calls
    uf_mid: &UnionFindStEph<V>,  // snapshot after find_root_loop, before merge
    uf_new: &UnionFindStEph<V>,  // snapshot after union_merge
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
        // find_root_loop postconditions
        uf_old@.parent.contains_key(u_view),
        uf_old@.parent.contains_key(v_view),
        uf_mid.roots@.contains_key(root_u_view),
        uf_mid.roots@.contains_key(root_v_view),
        uf_mid.roots@[root_u_view] == root_u_view,
        uf_mid.roots@[root_v_view] == root_v_view,
        root_u_view != root_v_view,
        // union_merge postconditions (opaque form)
        spec_roots_changed_by_merge(uf_new, uf_mid, root_u_view, root_v_view, info.winner_view),
        uf_new.roots@.dom() =~= uf_mid.roots@.dom(),
        uf_new.elements@ == uf_mid.elements@,
        // ... other union_merge ensures as needed
    ensures
        // The ensures union needs for its own postcondition:
        spec_union_result(uf_new.roots@, uf_old.roots@, u_view, v_view),
        uf_new.roots@.dom() =~= uf_old.roots@.dom(),
        uf_new.elements@ == uf_old.elements@,
{
    // Call lemma_prove_union_result and other infrastructure HERE,
    // outside &mut encoding.
}
```

Then `union`'s body becomes:

```rust
fn union(&mut self, u: &V, v: &V) {
    let ghost old_self = *self;
    let root_u = find_root_loop(self, u);
    let ghost mid_self = *self;
    let root_v = find_root_loop(self, v);
    // mid_self updated after second find
    let ghost mid_self2 = *self;

    if !feq(&root_u, &root_v) {
        let info = union_merge(self, root_u, root_v);
        proof {
            lemma_union_bridge(&old_self, &mid_self2, self, u@, v@, ...);
        }
    }
}
```

The bridge lemma runs in pure proof mode — no `&mut` encoding, no old/new aliasing.
Z3 sees only the lemma's requires/ensures in union's context, not the revealed
quantifier bodies.

### Step 3: rlimit discipline

The bridge lemma itself may need careful rlimit management. If it loops internally:
- Use `reveal` only one predicate at a time
- Call sub-lemmas sequentially, each with its own `assert` checkpoint
- Keep rlimit at 30; if it exceeds, the approach is wrong

## Read first

- Your R106 report: `plans/agent1-r106-unionfind-report.md`
- The full file: `src/Chap65/UnionFindStEph.rs` (your branch has the latest)
- Your infrastructure lemmas: `lemma_prove_union_result`, `lemma_union_roots_bridge`,
  `lemma_union_ensures_bridge`, `lemma_wf_type_axioms`, `lemma_union_merge_exec_pre`

## Isolation

```bash
scripts/validate.sh isolate Chap65
```

## Rules

- Do NOT add `assume` or `accept` (the existing rank bound assumes are OK to keep
  temporarily while you work on step 1, but the goal is to eliminate them).
- Do NOT increase rlimit above 30 for `union`.
- Do NOT reveal `spec_roots_changed_by_merge` or `spec_union_result` in `union`'s
  body or in any context that also has dom `=~=`.
- Do NOT spawn subagents.
- Commit working intermediate states (e.g., rank bounds added to wf) before
  attempting the final `union` proof.

## STEP 20

## Report

Write `plans/agent1-r109-union-report.md`.
