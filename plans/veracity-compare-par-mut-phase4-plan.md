# veracity-compare-par-mut Phase 4: Compare requires/ensures clause text (Round 5)

## Context

Phases 1-3b are done and working. The tool identifies file groups, compares structs/views/wf,
compares traits with function matching, and reports requires/ensures **presence**. 21 errors,
222 warnings, 823 info.

Do NOT re-run phases 1, 2, 3, or 3b. Build on the existing code.

## Phase 4: Compare requires/ensures clause text

For each matched function pair that BOTH have requires (or BOTH have ensures), extract
the clause text and compare structurally.

### What to extract

For each function's requires/ensures block, extract each clause as a separate string.
A clause is one comma-separated condition. Example:

```rust
fn insert(&mut self, x: T)
    requires
        self.spec_orderedsetsteph_wf(),
        self@.len() + 1 < usize::MAX,
    ensures
        self.spec_orderedsetsteph_wf(),
        self@ == old(self)@.insert(x@),
```

Produces:
- requires clauses: `["self.spec_orderedsetsteph_wf()", "self@.len() + 1 < usize::MAX"]`
- ensures clauses: `["self.spec_orderedsetsteph_wf()", "self@ == old(self)@.insert(x@)"]`

### Normalization before comparison

Before comparing clause text across variants, normalize:

1. **Variant suffix in wf names**: `spec_orderedsetsteph_wf` and `spec_orderedsetmteph_wf`
   are the same logical predicate for their respective variants. Normalize by stripping
   the variant suffix: `spec_orderedset_wf` for comparison purposes.

2. **self vs old(self)**: In Eph `&mut self` functions, `old(self)` refers to pre-state
   and `self` refers to post-state. In Per `&self -> Self` functions, `self` is the input
   and the return value is the output. Do NOT try to normalize this — just note it as
   `info:` when the clauses differ only in self/old(self)/result naming.

3. **Type aliases**: `<T as View>::V` and `T::V` are the same. Normalize to `T::V`.

### What to compare

For each matched function pair with ensures (or requires):

**Clause count**: same number of clauses? Different count is `warning:`.

**Clause matching**: for each clause in the reference variant, find the best match
in the other variant (after normalization). Use substring containment or key-term
matching — NOT exact string equality.

Key terms to match on:
- `self@` / `old(self)@` / `result@` — state references
- `.insert(` / `.remove(` / `.contains(` / `.len()` — operation names
- Spec function calls: `spec_is_bst`, `spec_sorted`, etc.

### Severity

- `info:` — clauses match (after normalization), or differ only in self/old(self) naming
- `warning:` — clause count differs, or a clause in reference has no match in other variant
- `error:` — reference has a strong ensures (e.g., `self@ == old(self)@.insert(x@)`) but
  the other variant has only wf ensures (e.g., `self.spec_wf()`) — this is spec weakening

### Output example

```
src/Chap41/AVLTreeSetMtEph.rs:120: info: fn insert: ensures clause count 3 == 3
src/Chap41/AVLTreeSetMtEph.rs:121: info: fn insert: ensures[0] wf match (variant suffix normalized)
src/Chap41/AVLTreeSetMtEph.rs:122: info: fn insert: ensures[1] `self@ == old(self)@.insert(x@)` matches
src/Chap43/OrderedSetMtEph.rs:150: warning: fn first: ensures clause count 2 vs 4 (StPer has 4)
src/Chap43/OrderedSetMtEph.rs:155: error: fn insert: StPer ensures `self@ == old(self)@.insert(x@)` but MtEph only ensures wf
```

### Scope control

This is expensive to run — many functions, many clauses. Add a `--phase4` flag to
run only this phase (skip 1-3). Also add `--chapter ChapNN` to limit to one chapter
for testing.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All parsing must be token-aware or AST-aware. Parse requires/ensures
blocks with brace/comma/semicolon awareness.
