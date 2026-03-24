# Veracity Tool: Cross-Variant Spec Diff

## Goal

Build a veracity tool `veracity-review-cross-variant-specs` that compares trait function
specs (requires/ensures) across the St/Mt x Eph/Per variants of the same algorithm and
flags differences.

## Motivation

Every APAS-VERUS algorithm has up to 4 implementations:
- `FooStEph.rs` — single-threaded ephemeral
- `FooStPer.rs` — single-threaded persistent
- `FooMtEph.rs` — multi-threaded ephemeral
- `FooMtPer.rs` — multi-threaded persistent

All variants implement the same abstract data type. Their trait declarations should have
identical specs: same `requires`, same `ensures`, same View type semantics. Differences
indicate bugs — a spec weakened to make a proof easier, a variant that fell behind during
development, or a missing postcondition.

## Input

A directory path (e.g., `src/Chap43/`) or the entire `src/` tree.

## Grouping

Files are grouped by stripping the variant suffix:
```
OrderedTableStEph.rs  \
OrderedTableStPer.rs   |-- group "OrderedTable"
OrderedTableMtEph.rs   |
OrderedTableMtPer.rs  /
```

Suffix patterns to strip: `StEph`, `StPer`, `MtEph`, `MtPer`, `MtEphSlice` (rare).

Some groups are incomplete (only 2-3 variants exist). That's fine — diff what exists.

Files that don't match any suffix pattern (e.g., `ArraySeq.rs`, `BSTParaStEph.rs` with
no Mt counterpart) are singletons — skip them or list them as "no variants to compare."

## Parsing

For each file in a group, parse the **trait declaration** (not the impl). Extract:

1. **Trait name** — e.g., `OrderedTableStEphTrait`
2. **Function signatures** — for each `fn` in the trait:
   - Name
   - Parameter types (normalize: `&self`, `&mut self`, key types, closure types)
   - Return type
   - `requires` clauses (list of predicates)
   - `ensures` clauses (list of predicates)
3. **Spec functions** — `spec fn` declarations in the trait
4. **View type** — the `type V = ...` in the trait's `View` supertype

## Normalization

Before diffing, normalize for expected variant differences:
- Trait name suffixes differ (`StEphTrait` vs `MtEphTrait`) — ignore
- Wf predicate names differ (`spec_orderedtablesteph_wf` vs `spec_orderedtablemteph_wf`) — treat as equivalent
- Mt variants may have additional `Send + Sync + 'static` bounds — flag but don't error
- Mt variants wrap in `Arc<RwLock<...>>` — the inner type's View should match
- Parameter ownership may differ (`self` vs `&self` vs `&Arc<...>`) — flag but don't error
- `Ghost(...)` parameters may differ between St and Mt — flag

## Diff Rules

For each function that appears in 2+ variants, compare:

### Must Match (error if different)
- `ensures` predicates (after normalization) — the postcondition contract
- `requires` predicates (after normalization) — the precondition contract
- Return type's View semantics

### Should Match (warning if different)
- Number of `ensures` clauses
- Number of `requires` clauses
- Function exists in one variant but not another

### Expected Differences (info only)
- Trait bound differences (Mt adds Send/Sync/Clone)
- Ownership differences (&self vs Arc<RwLock>)
- Wf predicate name differences
- Ghost parameter presence

## Output

### 1. Per-Group Report

For each algorithm group with differences:
```
## OrderedTable (Chap43) — 4 variants

| # | Function | StEph | StPer | MtEph | MtPer | Status |
|---|----------|-------|-------|-------|-------|--------|
| 1 | new      | match | match | match | match | OK     |
| 2 | find     | match | match | match | match | OK     |
| 3 | union    | match | WEAK  | match | -     | DIFF   |
```

Where:
- `match` = spec matches the reference (first variant found)
- `WEAK` = fewer ensures clauses than reference
- `STRONG` = more ensures clauses than reference (not necessarily bad)
- `MISS` = function missing from this variant
- `-` = variant file doesn't exist
- `DIFF` = requires or ensures differ in substance

### 2. Differences Detail

For each `DIFF` or `WEAK`:
```
### OrderedTable::union — StPer is WEAK

StEph ensures:
  self@ == old(self)@.union(other@)
  self.spec_orderedtablesteph_wf()

StPer ensures:
  result@ == self@.union(other@)
  (missing: result.spec_orderedtablestper_wf())

Assessment: StPer is missing wf ensures on result.
```

### 3. Summary

```
| # | Chap | Group | Variants | Fns | Match | Diff | Weak | Missing | Score |
```

Score = Match / (Match + Diff + Weak + Missing).

## Implementation Notes

### Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust
source. All parsing must be token-aware or AST-aware. Parse trait blocks with
brace/comma/semicolon awareness. Requires/ensures blocks can span multiple lines with
commas, contain nested expressions, and have trailing commas.

### Verus-Specific Parsing

- `requires` and `ensures` blocks are comma-separated predicate lists
- A predicate can span multiple lines (e.g., `ensures exposed matches Exposed::Node(l, k, r) ==> { ... }`)
- `spec fn` declarations may have `open` or `closed` visibility
- Trait functions may have `proof { }` blocks in their default implementations — ignore these
- `#[verifier::external_body]` marks functions whose body is not verified

### Matching Function Names

Functions in different variants usually have the same name. Exceptions:
- Mt may have `_locked` or `_inner` variants
- Per may use different verb forms (e.g., `insert` returning new value vs mutating)

Match by name first, then by signature similarity.

## CLI Interface

```bash
veracity-review-cross-variant-specs src/Chap43/
veracity-review-cross-variant-specs src/           # all chapters
veracity-review-cross-variant-specs -v src/Chap18/ # verbose: show all functions, not just diffs
```

Output to stdout (same as other veracity tools). Per-chapter logs written to
`src/ChapNN/analyses/veracity-review-cross-variant-specs.log`.
