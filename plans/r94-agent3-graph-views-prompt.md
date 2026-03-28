# R94 Agent 3 — Fix 4 Chap06 graph View discrepancies, STEP 20

## Objective

4 graph StEph files have `View = Seq<V>` but their MtEph counterparts use
`GraphView<V::V>` or `LabGraphView<V::V, L::V>`. Change StEph to match MtEph.

## The 4 Errors

| # | File | Current StEph View | Correct View (from MtEph) |
|---|------|--------------------|--------------------------|
| 1 | DirGraphStEph.rs | `Seq<V>` | `GraphView<V::V>` |
| 2 | UnDirGraphStEph.rs | `Seq<V>` | `GraphView<V::V>` |
| 3 | LabDirGraphStEph.rs | `Seq<V>` | `LabGraphView<V::V, L::V>` |
| 4 | LabUnDirGraphStEph.rs | `Seq<V>` | `LabGraphView<V::V, L::V>` |

## What to do

For each file:

### 1. Change `impl View`

```rust
// Before:
impl<V: StT + Hash> View for DirGraphStEph<V> {
    type V = Seq<V>;  // WRONG
    open spec fn view(&self) -> Seq<V> { ... }
}

// After:
impl<V: StT + Hash> View for DirGraphStEph<V> {
    type V = GraphView<V::V>;
    open spec fn view(&self) -> GraphView<V::V> {
        GraphView { V: self.V@, A: self.A@ }  // or however the graph fields map
    }
}
```

Check how MtEph implements its View — mirror exactly.

### 2. Fix all `self@` usage

Every place that says `self@[i]` (indexing into the old Seq view) needs to change.
The new view is `GraphView` which has `.V` (Set of vertices) and `.A` (Set of arcs).
So `self@` becomes `self@.V` for vertex access, `self@.A` for arc access.

Common patterns to fix:
- `self@.len()` → `self@.V.len()` or remove (graph view doesn't have a single length)
- `self@.contains(v)` → `self@.V.contains(v@)`
- `self@[i]` → must be rewritten (no indexing on GraphView)

### 3. Fix callers in other chapters

Grep for files that use `DirGraphStEph@`, `UnDirGraphStEph@`, etc:
```bash
grep -r "DirGraphStEph\|UnDirGraphStEph\|LabDirGraphStEph\|LabUnDirGraphStEph" \
  src/Chap52/ src/Chap53/ src/Chap54/ src/Chap55/ --include="*.rs" | grep "@"
```

Most callers probably already use `spec_graphview_wf(graph@)` and `graph@.V` /
`graph@.A` — which work with `GraphView`. The StEph callers that use `graph@[i]`
need to be updated.

### 4. Update trait supertrait

If the trait says `View<V = Seq<V>>`, change to `View<V = GraphView<V::V>>`.

## Read first

- `src/Chap06/DirGraphMtEph.rs` — **reference for correct View impl**
- `src/Chap06/DirGraphStEph.rs` — primary target
- `src/Chap06/UnDirGraphMtEph.rs` — reference
- `src/Chap06/UnDirGraphStEph.rs` — target
- Same for LabDirGraph and LabUnDirGraph

## Isolation

```bash
scripts/validate.sh isolate Chap55
```

(Pulls in Chap06, 52, 53, 54 transitively — covers most callers.)

If that's too heavy:
```bash
scripts/validate.sh isolate Chap06
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify the MtEph files.
- Do NOT modify Chap05 (Set) — that's a separate, larger refactor.
- If a caller chapter has too many `self@[i]` references to fix within budget,
  report it and skip that caller. Focus on getting the View impl right and
  fixing the direct Chap06 code.
- The graph StEph files may already use `spec_graphview_wf(graph@)` internally,
  which means they already treat self@ as GraphView in many places. The actual
  diff may be smaller than expected.

## STEP 20

## Report

Write `plans/agent3-r94-graph-views-report.md`.
