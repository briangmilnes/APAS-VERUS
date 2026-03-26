# R79 Agent 4 — UnionFind find + cascade (Chap65, 5 holes)

## Objective

Prove `find` in UnionFindStEph.rs, which unblocks `union`, `equals`, `num_sets` (3
downstream) and `kruskal_mst` in KruskalStEph.rs (1 downstream). 5 holes total.

## Baseline

- 4908 verified, 0 errors, 0 warnings
- UnionFindStEph.rs: 4 holes (find ROOT, union/equals/num_sets blocked)
- KruskalStEph.rs: 1 hole (kruskal_mst blocked by find/union)

## Holes

| # | Chap | File | Line | Function | Type | Blocked by |
|---|------|------|------|----------|------|------------|
| 1 | 65 | UnionFindStEph.rs | 352 | find | external_body | ROOT |
| 2 | 65 | UnionFindStEph.rs | 377 | union | external_body | find |
| 3 | 65 | UnionFindStEph.rs | 398 | equals | external_body | find |
| 4 | 65 | UnionFindStEph.rs | 406 | num_sets | external_body | find |
| 5 | 65 | KruskalStEph.rs | 178 | kruskal_mst | external_body | find/union |

## The wf predicate

The wf predicate (`spec_unionfindsteph_wf`, line ~271) has 14 conjuncts. **Do not try
to prove all 14 simultaneously.** Z3 will exhaust its rlimit. The proven approach:

### Step 1: Name the wf properties as sub-predicates

Factor the 14 conjuncts into named `spec fn` predicates. Examples:
```rust
open spec fn spec_key_model_ok<V: StT + Hash>() -> bool {
    obeys_key_model::<V>() && obeys_feq_full::<V>()
}

open spec fn spec_domains_consistent<V: StT + Hash>(&self) -> bool {
    self.parent@.dom() =~= self.rank@.dom()
    && self.roots@.dom() =~= self.parent@.dom()
}

open spec fn spec_roots_idempotent<V: StT + Hash>(&self) -> bool {
    forall|v: <V as View>::V| #[trigger] self.roots@.contains_key(v) ==> {
        self.roots@.contains_key(self.roots@[v])
        && self.roots@[self.roots@[v]] == self.roots@[v]
    }
}

open spec fn spec_rank_ordering<V: StT + Hash>(&self) -> bool {
    forall|v: <V as View>::V| self.parent@.contains_key(v) && self.parent@[v]@ != v ==>
        self.rank@[v] < #[trigger] self.rank@[self.parent@[v]@]
}
// ... etc for each logical property
```

Note: conjunct 3 (`forall|k1, k2| k1@ == k2@ ==> k1 == k2`) is redundant with
`obeys_feq_full::<V>()`. Remove it.

Then wf becomes:
```rust
open spec fn spec_unionfindsteph_wf(&self) -> bool {
    spec_key_model_ok::<V>()
    && self.spec_domains_consistent()
    && self.spec_roots_idempotent()
    && self.spec_parent_closed()
    && self.spec_roots_in_domain()
    && self.spec_elements_cover_domain()
    && self.spec_elements_no_duplicates()
    && self.spec_self_parent_is_root()
    && self.spec_parent_preserves_root()
    && self.spec_rank_ordering()
    && self.spec_rank_bounded_by_root()
}
```

### Step 2: Write isolated proof lemmas per property

For `find` (path compression), write a `proof fn lemma_find_preserves_wf` with
`#[verifier::rlimit(50)]` following the pattern at line 91 (`lemma_insert_preserves_wf`).

The key insight from `insert`'s proof: use **one frame `assert forall`** to establish
that for all existing keys `w != current`, the maps are unchanged. Then prove each
named sub-predicate individually. Z3 can handle one property at a time.

### Step 3: find algorithm

`find` has two phases:
1. **Walk to root**: follow parent pointers until `parent[v]@ == v`. Decreases by rank
   (the `spec_rank_ordering` conjunct guarantees strict decrease for non-root nodes).
2. **Path compression**: walk the path again, pointing every node directly to root.
   This only changes parent pointers — rank and roots are unchanged.

For path compression, the frame lemma needs to show that changing `parent[v] = root`
preserves all wf properties. The critical ones:
- `spec_parent_closed`: root is in domain (already known)
- `spec_parent_preserves_root`: `roots[root@] == roots[v]` (root IS the root)
- `spec_rank_ordering`: `rank[v] < rank[root@]` (follows from rank transitivity along path)

### Step 4: Cascade

Once `find` is proved:
- `union`: calls `find` twice, updates parent + rank. Frame lemma on the union operation.
- `equals`: calls `find` twice, compares results. Trivial after `find`.
- `num_sets`: loop calling `find`, counting distinct roots.
- `kruskal_mst`: loop calling insert/find/union/equals.

## Key reference: lemma_insert_preserves_wf

Read `src/Chap65/UnionFindStEph.rs:91-180`. This is the proven pattern:
- Isolated `proof fn` with `#[verifier::rlimit(50)]`
- Requires: old state wf + specific field updates
- Ensures: new state wf
- Body: frame `assert forall` for existing keys, then per-property assertions

## Key resources

- `src/Chap65/UnionFindStEph.rs` — Read fully, especially lines 91-180 and 271-315
- `plans/agent1-round78-report.md` — Agent1's R78 progress (if exists)

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round79-report.md` with holes before/after (table with Chap column).
