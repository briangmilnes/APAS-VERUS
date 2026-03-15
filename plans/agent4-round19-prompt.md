# Agent 4 — Round 19: Tier 2 Spec Audit — Chap06 + Chap21 + Chap23 + Chap26

## Mission

Audit every exec fn's `requires`/`ensures` in Chap06 (Graphs), Chap21 (Trees),
Chap23 (Augmented Trees), and Chap26 (Priority Queues — basic) against APAS textbook
prose. These are structural ADTs used throughout the later algorithm chapters.

## Procedure

Same as Agent 3's audit procedure:

1. **Read** `prompts/ChapNN.txt` — the APAS textbook prose.
2. **Read** the StEph trait file(s).
3. **Read** `src/ChapNN/analyses/veracity-review-module-fn-impls.md`.
4. **Compare** each fn's ensures against the prose.
5. **Classify** as strong/partial/weak/missing.
6. **Write** `src/ChapNN/analyses/spec-audit.md`.
7. **Fix** weak/missing/partial specs.
8. Add `external_body` where proof breaks.
9. `scripts/validate.sh` — 0 errors.

## Required Reading

- `src/standards/total_order_standard.rs`
- `src/standards/using_closures_standard.rs`
- `src/standards/view_standard.rs`

## Files to Audit

### Chap06 (20 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | DirGraphStEph.rs | ADT 6.1 Directed Graphs | add_vertex, add_arc, out_neighbors, in_neighbors |
| 2 | UnDirGraphStEph.rs | ADT 6.2 Undirected Graphs | add_vertex, add_edge, neighbors |
| 3 | LabDirGraphStEph.rs | ADT 6.3 Labeled Graphs | add_vertex, add_arc with label |
| 4 | LabUnDirGraphStEph.rs | ADT 6.4 | Same, undirected |
| 5-8 | WeightedDirGraph variants | Weighted | Same + weight functions |
| 9-20 | Mt/StPer/MtPer variants | Wrappers | Delegating |

### Chap21 (12 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | TreeStEph.rs | ADT 21.1 Trees | root, children, parent, subtree |
| 2-12 | Variants | StPer, Mt, etc. | Same ops |

### Chap23 (2 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | AugTreeStEph.rs | ADT 23.1 Augmented Trees | Same + aug_value |
| 2 | AugTreeStPer.rs | Same, persistent | Same |

### Chap26 (8 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | PriorityQueueStEph.rs | ADT 26.1 PQs | insert, find_min, delete_min |
| 2-8 | Variants | StPer, Mt, etc. | Same ops |

## What "Strong" Means for Graph/Tree ADTs

```rust
// DirGraph::add_arc
ensures
    self@ == old(self)@.add_arc(u@, v@),
    // or equivalently:
    self@.vertices() == old(self)@.vertices(),
    self@.arcs() == old(self)@.arcs().insert((u@, v@)),

// DirGraph::out_neighbors
ensures
    result@ == self@.out_neighbors(v@),

// Tree::children
ensures
    result@ == self@.children(v@),

// PQ::find_min — should have TotalOrder minimality (see total_order_standard.rs)
```

## Important

- These are clean chapters. If specs look strong, say so in the audit — don't
  change them unnecessarily.
- Only fix genuinely weak/missing specs.
- The prose is the source of truth.
- Do NOT modify files in holed chapters (Chap37-47, Chap52).
- Skip Example files.

## Deliverables

- `src/Chap06/analyses/spec-audit.md`
- `src/Chap21/analyses/spec-audit.md`
- `src/Chap23/analyses/spec-audit.md`
- `src/Chap26/analyses/spec-audit.md`
- Strengthened ensures where needed.
- `plans/agent4-round19-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
