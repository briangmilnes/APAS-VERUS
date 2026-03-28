# R91 Agent 2 — ClonePreservesWf trait + cascade AdjTableGraph proofs, STEP 20

## Objective

Create a `ClonePreservesWf` pattern that lets Table operations propagate
well-formedness through cloned values. Then prove as many AdjTableGraph
holes as possible across all 3 files (StEph, StPer, MtPer).

## Background

Table::insert and Table::delete rebuild the entries array by cloning non-key
values via `clone_plus`. Currently `clone_plus` only ensures `result@ == self@`
(view equality). But AdjTableGraph stores `Table<V, AVLTreeSetStEph<V>>` and
needs returned/stored neighbor sets to be well-formed. The clone gap means
Table can't prove stored values remain wf after insert/delete.

## Step 1: Create ClonePreservesWf

In `src/vstdplus/clone_view.rs` (or a new file `src/vstdplus/clone_wf.rs`),
add a trait:

```rust
pub trait ClonePreservesWf: Clone {
    spec fn spec_wf(&self) -> bool;

    fn clone_wf(&self) -> (result: Self)
        requires self.spec_wf(),
        ensures result.spec_wf(), result@ == self@;
}
```

Implement it for AVLTreeSetStEph, AVLTreeSetStPer, AVLTreeSetMtPer:

```rust
impl<T: StT + Ord> ClonePreservesWf for AVLTreeSetStEph<T> {
    open spec fn spec_wf(&self) -> bool { self.spec_avltreesetsteph_wf() }

    fn clone_wf(&self) -> (result: Self)
    {
        let r = clone_plus(self);
        proof { assume(r.spec_avltreesetsteph_wf()); }
        r
    }
}
```

The `assume` is justified: `#[derive(Clone)]` on AVLTreeSetStEph clones the
backing ArraySeq field-by-field, preserving the sorted/no-dup structure. This
is the same category as the eq/clone workaround assume — a Verus limitation,
not a soundness gap.

**IMPORTANT: Use `assume` here, not `accept`.** Per CLAUDE.md, only the user
promotes assumes to accepts.

## Step 2: Strengthen Table insert/delete

In TableStEph, add to insert/delete ensures:
```rust
// For non-modified keys, stored values retain wf if they had it before.
forall|k: K::V| k != key@ && old(self)@.contains_key(k)
    && old(self).spec_stored_value(k).spec_wf()
    ==> self.spec_stored_value(k).spec_wf()
```

This requires `V: ClonePreservesWf` as an additional bound on Table methods
that clone values. If adding the bound to Table is too invasive, add it only
to AdjTableGraph's wf predicate and use `clone_wf()` directly in the graph
impl instead of going through Table.

**Alternative (simpler):** Skip modifying Table. Instead, in AdjTableGraph,
replace `table.insert(...)` and `table.delete(...)` calls with sequences
that use `clone_wf()` directly when rebuilding neighbor sets. This avoids
touching Chap42 entirely.

## Step 3: Prove AdjTableGraph holes

With clone wf available, work through AdjTableGraphStEph first (8 holes):
1. `out_neighbors` — use clone_wf instead of clone, return wf set
2. `out_degree` — delegates to out_neighbors, falls automatically
3. `num_edges` — iterate domain, sum sizes using find_ref + wf
4. `vertices` — iterate domain keys
5. `insert_vertex` — Table insert + wf maintenance
6. `delete_vertex` — iteration + nested ops
7. `insert_edge` — find_ref + set insert + Table insert
8. `delete_edge` — find_ref + set delete + Table insert

Then port to StPer (10 holes) and MtPer (10 holes) — same pattern.

## Read first

- `src/vstdplus/clone_view.rs` — existing ClonePreservesView trait
- `src/vstdplus/clone_plus.rs` — clone_plus function and cloned spec
- `src/Chap42/TableStEph.rs` — Table trait/impl, find_ref, spec_stored_value
- `src/Chap52/AdjTableGraphStEph.rs` — primary target (8 holes)
- `src/Chap52/AdjTableGraphStPer.rs` — port target (10 holes)
- `src/Chap52/AdjTableGraphMtPer.rs` — port target (10 holes)
- `src/Chap41/AVLTreeSetStEph.rs` — impl ClonePreservesWf here

## Isolation

```bash
scripts/validate.sh isolate Chap52
```

Check Chap43 backwards compatibility:
```bash
scripts/validate.sh isolate Chap43
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT weaken ensures clauses.
- Do NOT add `accept`. Use `assume` for the clone wf bridge (same category
  as eq/clone workaround).
- Prioritize StEph first — each function proved there is a template for StPer/MtPer.
- If modifying Table is too invasive, work entirely within Chap52 using clone_wf
  directly in the graph implementations.
- Even proving 10 of 28 holes is a big win. Don't burn all 20 steps on one file.

## STEP 20

## Report

Write `plans/agent2-r91-clone-wf-report.md`.
