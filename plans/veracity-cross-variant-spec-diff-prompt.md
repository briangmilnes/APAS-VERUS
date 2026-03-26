# Veracity Tool: `veracity-review-cross-variant-specs`

## Goal

Build `veracity-review-cross-variant-specs` — a tool that compares trait function specs
(requires/ensures) across the St/Mt × Eph/Per variants of the same algorithm and flags
where contracts diverge.

## Motivation

Every APAS-VERUS algorithm has up to 4 implementations:
- `FooStEph.rs` — single-threaded ephemeral
- `FooStPer.rs` — single-threaded persistent
- `FooMtEph.rs` — multi-threaded ephemeral
- `FooMtPer.rs` — multi-threaded persistent

All variants of the same algorithm implement the same abstract data type. Their trait
declarations should have identical specs after accounting for variant-specific naming.
Differences indicate bugs — a spec weakened to make a proof easier, a variant that fell
behind during development, or a missing postcondition.

## Variant Groups in the Codebase

The codebase has 244 modules. Multi-variant groups come in three sizes:

**Full 4-variant groups** (13 algorithms): ArraySeq, OrderStatSelect, AVLTreeSet,
OrderedTable, MinEditDist, SubsetSum, MatrixChain, OptBinSearchTree, BottomUpDP,
TopDownDP, AdjMatrixGraph, AdjSeqGraph, BFS.

**3-variant groups** (7 algorithms): AVLTreeSeq, Table, AugOrderedTable, OrderedSet,
AdjTableGraph, EdgeSetGraph, GraphSearch.

**2-variant groups** (~25 algorithms): Most BST variants (BSTAVL, BSTPlain, BSTRB,
BSTSplay, BSTBBAlpha — all StEph+MtEph), graph algorithms (DirGraph, Boruvka,
Connectivity, SpanTree, etc.), and St-only Eph/Per pairs (DFS, SCC, TopoSort, etc.).

**Singletons** (~50 modules): BSTSet* (MtEph-only wrappers), hash tables, Kruskal,
Prim, TSPApprox, etc. No variants to compare.

The maximum variant count for any algorithm is **4** (St×Mt × Eph×Per). Struct names
within a group differ only by the variant suffix — `OrderedTableStEph`, `OrderedTableMtEph`,
etc. This makes mechanical substitution viable.

## The Core Problem: Variant-Aware Identifier Matching

Each variant uses its own names for everything:
```
// StEph:  spec_orderedtablesteph_wf()    OrderedTableStEph    OrderedTableStEphTrait
// MtEph:  spec_orderedtablemteph_wf()    OrderedTableMtEph    OrderedTableMtEphTrait
```

These are the *same* predicate/type/trait. The tool must canonicalize variant-specific
identifiers before comparing specs. The naming convention is mechanical:
- Strip variant suffix (`StEph`, `MtEph`, `StPer`, `MtPer`) from type/trait names
- Strip lowercase variant infix (`steph`, `mteph`, `stper`, `mtper`) from wf/spec fn names

## Algorithm — Phased Approach

Each phase produces checkable output. Run phases sequentially; early phases feed later ones.

### Phase 1: Grouping

Strip variant suffixes to find groups:
```
OrderedTableStEph.rs  \
OrderedTableStPer.rs   |-- group "OrderedTable"
OrderedTableMtEph.rs   |
OrderedTableMtPer.rs  /
```

Suffix patterns to strip (longest first):
`MtEphSlice`, `MtEph`, `MtPer`, `StEph`, `StPer`

**Output (checkable):** List of groups with their variants. Singletons listed separately.
```
Group "OrderedTable" (Chap43): StEph, StPer, MtEph, MtPer
Group "BSTAVL" (Chap37): StEph, MtEph
Singleton: BSTSetAVLMtEph (Chap37)
```

### Phase 2: Parse trait declarations

For each file in a group, parse the **trait declaration**. Extract per function:
- Name
- Parameter list (types and names)
- Return type and name
- `requires` clauses (list of predicates, split on commas at the clause level)
- `ensures` clauses (same)

Also extract:
- `spec fn` declarations in the trait (name, params, return type)
- `type V = ...` from the View impl (the abstract state type)

**Output (checkable):** Per-file function inventory.
```
OrderedTableStEph.rs — 18 trait functions, 3 spec fns, View = Map<K::V, V::V>
  fn new() -> Self  ensures wf
  fn find(&self, key: &K) -> Option<V>  requires wf  ensures ...
  ...
```

### Phase 3: Extract and compare wf predicate bodies

The wf predicate is the foundation. If wf bodies diverge, all downstream comparisons
are suspect.

Wf predicates appear in two forms:

**Form A — trait spec fn with body in impl:**
```rust
// In trait:
spec fn spec_orderedtablesteph_wf(&self) -> bool;
// In impl:
open spec fn spec_orderedtablesteph_wf(&self) -> bool {
    self.tree.spec_avltreesteph_wf() && self.size <= usize::MAX
}
```

**Form B — free spec fn:**
```rust
pub open spec fn spec_orderedtablesteph_wf(table: &OrderedTableStEph<K,V>) -> bool {
    table.tree.spec_avltreesteph_wf() && table.size <= usize::MAX
}
```

For each file, find the wf predicate by name pattern (`spec_*_wf`). If it's a trait
spec fn, look up the body in the `impl` block. If free function, take body directly.

**Output (checkable):** Per-group wf body comparison.
```
Group "OrderedTable" wf bodies:
  StEph: self.tree.spec_G_wf() && self.size <= usize::MAX
  StPer: self.tree.spec_G_wf() && self.size <= usize::MAX
  MtEph: self.tree.spec_G_wf() && self.size <= usize::MAX
  MtPer: self.tree.spec_G_wf()
  STATUS: DIFF — MtPer missing size bound
```

If wf bodies diverge, flag as error. Still proceed to Phase 5 but note that downstream
differences may be explained by wf divergence.

### Phase 4: Build the canonical substitution map

For each group, generate the map mechanically from the naming convention.
Given group name `G` and variants present:

| Variant-specific | Canonical |
|------------------|-----------|
| `GStEph`, `GMtEph`, `GStPer`, `GMtPer` | `G` |
| `spec_gsteph_wf`, `spec_gmteph_wf`, ... | `spec_g_wf` |
| `spec_gsteph_*`, `spec_gmteph_*`, ... | `spec_g_*` |
| `GStEphTrait`, `GMtEphTrait`, ... | `GTrait` |
| `GStEphIter`, `GMtEphIter`, ... | `GIter` |
| `GStEphGhostIter`, ... | `GGhostIter` |
| `GStEphInv`, `GMtEphInv`, ... | `GInv` |

The wf name convention: `spec_<module>_wf` where `<module>` is the module name in
lowercase with no internal underscores. So `OrderedTable` →
`spec_orderedtablesteph_wf` / `spec_orderedtablemteph_wf` → canonical `spec_orderedtable_wf`.

Build the map by:
1. Collecting all identifiers that contain a variant suffix (case-insensitive match)
2. Replacing the variant suffix with nothing to get canonical form
3. Two identifiers mapping to the same canonical form are associated

**Output (checkable):** The substitution map for each group.
```
Group "OrderedTable" substitution map:
  OrderedTableStEph → OrderedTable
  OrderedTableMtEph → OrderedTable
  spec_orderedtablesteph_wf → spec_orderedtable_wf
  spec_orderedtablemteph_wf → spec_orderedtable_wf
  OrderedTableStEphTrait → OrderedTableTrait
  ...
```

### Phase 5: Canonicalize and compare function specs

For each function that exists in 2+ variants:

1. Apply the substitution map to each requires/ensures clause
2. Normalize whitespace (collapse to single spaces, trim)
3. **Sort** the canonical clauses alphabetically (order doesn't matter semantically —
   `requires a, b` equals `requires b, a`)
4. Compare sorted canonical clause lists across variants

**Output (checkable):** Per-function comparison.
```
OrderedTable::find
  StEph requires (sorted): [spec_orderedtable_wf(&self)]
  MtEph requires (sorted): [spec_orderedtable_wf(&self)]
  StEph ensures (sorted):  [found matches Some(v) ==> self@.dom().contains(key@)]
  MtEph ensures (sorted):  [true]
  STATUS: WEAK — MtEph ensures weaker than StEph
```

### Phase 6: Classify differences

For each function across variants:

| Classification | Meaning |
|---------------|---------|
| **MATCH** | Same canonical clauses (after sort) in all variants |
| **WEAK** | One variant has fewer ensures clauses (weaker postcondition) |
| **STRONG** | One variant has more ensures clauses (stronger — notable but not bad) |
| **DIFF** | Clauses differ in substance after canonicalization |
| **MISS** | Function exists in some variants but not others |
| **EXTRA_REQ** | One variant has additional requires (stricter precondition) |
| **MISSING_REQ** | One variant has fewer requires (weaker precondition) |

### Phase 7: Filter expected Mt differences

Some St↔Mt differences are expected:

| Expected difference | Detection | Level |
|--------------------|-----------|-------|
| Mt adds `Send + Sync + 'static` bounds | Extra trait bounds | info |
| Mt uses `&self` through RwLock | Ownership pattern | info |
| Mt adds `Ghost(...)` parameters | Extra ghost params | info |
| Mt `ensures true` where St has real spec | Real ensures missing | **warning** |
| St has postcondition Mt lacks | Spec regression | **warning** |

## Output Formats

### Default: emacs compile format

```
src/Chap43/OrderedTableMtEph.rs:45: warning: [xvar] find: WEAK ensures — missing `found matches Some(v) ==> self@.dom().contains(key@)` (present in StEph)
src/Chap43/OrderedTableMtPer.rs:1: warning: [xvar] MISS — function `previous` missing (present in StEph, StPer, MtEph)
src/Chap43/OrderedTableMtEph.rs:62: info: [xvar] insert: EXTRA_REQ — Mt adds `Send + Sync` bounds (expected)
```

### Markdown: `-m` / `--markdown`

Per-group function table:
```
## OrderedTable (Chap43) — 4 variants

| # | Function | StEph | StPer | MtEph | MtPer | Status |
|---|----------|-------|-------|-------|-------|--------|
| 1 | new      | match | match | match | match | OK     |
| 2 | find     | match | match | WEAK  | -     | DIFF   |
```

Difference detail:
```
### OrderedTable::find — MtEph is WEAK
StEph ensures (canonical, sorted):
  found matches Some(v) ==> self@.dom().contains(key@)
  result.spec_orderedtable_wf()
MtEph ensures (canonical, sorted):
  result.spec_orderedtable_wf()
  (missing: found matches Some(v) ==> self@.dom().contains(key@))
```

Summary:
```
| # | Chap | Group | Variants | Fns | Match | Diff | Weak | Miss | Score |
|---|------|-------|----------|-----|-------|------|------|------|-------|
| 1 | 43   | OrderedTable | 4 | 18 | 15   | 1    | 1    | 1    | 83%   |
```

## CLI Interface

```bash
veracity-review-cross-variant-specs [options] [path]
```

### Options

| Flag | Long | Description |
|------|------|-------------|
| `-c` | `--codebase DIR` | Project root. Default: cwd. |
| `-d` | `--dry-run` | Parse and report without writing logs. |
| `-e` | `--exclude DIR` | Exclude directory (repeatable). Standard: `experiments`, `standards`. |
| `-m` | `--markdown` | Markdown table output instead of emacs compile format. |
| `-v` | `--verbose` | Show all functions including matches (default: only differences). |
| | `--phase N` | Run only up to phase N (1-7). For incremental checking. |

### Examples

```bash
# Full review, one chapter
veracity-review-cross-variant-specs -c ~/projects/APAS-VERUS src/Chap43/

# Full review, all chapters, markdown
veracity-review-cross-variant-specs -m -c ~/projects/APAS-VERUS src/

# Just grouping (Phase 1) to verify groups are correct
veracity-review-cross-variant-specs --phase 1 -c ~/projects/APAS-VERUS src/

# Through wf comparison (Phase 3) to check wf consistency
veracity-review-cross-variant-specs --phase 3 -v -c ~/projects/APAS-VERUS src/Chap43/

# Through substitution map (Phase 4) to verify the map is correct
veracity-review-cross-variant-specs --phase 4 -v -c ~/projects/APAS-VERUS src/Chap18/
```

### Output files

Per-chapter logs: `src/ChapNN/analyses/veracity-review-cross-variant-specs.log`

Exit code: 0 if all specs match, 1 if warnings found, 2 on tool error.

## Parsing Notes

### Requires/ensures clause splitting

Requires and ensures blocks are comma-separated predicate lists that can span multiple
lines. A clause boundary is a comma at the top level (not inside parentheses, braces,
or angle brackets). Examples:

```rust
// Two clauses:
requires self.spec_wf(), key@ > 0

// One clause spanning multiple lines:
ensures found matches Some(v) ==> {
    &&& self@.dom().contains(key@)
    &&& v@ == self@[key@]
}

// Three clauses:
requires
    self.spec_wf(),
    other.spec_wf(),
    obeys_feq_clone::<T>(),
```

The parser must track brace/paren/angle-bracket depth to correctly split on commas.

### Trait vs impl

Parse the **trait declaration** for function signatures and specs (the abstract contract).
Parse the **impl** only for wf predicate bodies (Phase 3). The impl may have weaker specs
due to proof limitations — that's a separate concern.

### Spec functions

`spec fn` declarations in traits should also be compared across variants. Compare
signatures (parameter types, return type). Bodies may differ — compare only signatures
unless both are `open spec fn` with visible bodies.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust
source. All parsing must be token-aware or AST-aware. Parse requires/ensures blocks
with brace/comma/semicolon awareness. A string-hacking detector will flag and kill
tools that corrupt source syntax.

## Test Strategy

Test with `--phase N` on well-known chapters:

| Phase | Test on | Expected |
|-------|---------|----------|
| 1 | `src/` | 13 four-variant groups, 7 three-variant, ~25 two-variant |
| 2 | `src/Chap43/` | OrderedTable: 18 fns, OrderedSet: ~12 fns |
| 3 | `src/Chap18/` | ArraySeq wf bodies should match across all 4 |
| 4 | `src/Chap43/` | Map shows `spec_orderedtablesteph_wf` → `spec_orderedtable_wf` |
| 5 | `src/Chap18/` | ArraySeq specs should mostly MATCH |
| 6-7 | `src/Chap43/` | Expect some WEAK in MtPer (newer, less mature) |
