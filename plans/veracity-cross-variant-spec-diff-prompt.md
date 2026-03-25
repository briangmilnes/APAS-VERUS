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

All variants implement the same abstract data type. Their trait declarations should have
identical specs after accounting for variant-specific naming. Differences indicate bugs —
a spec weakened to make a proof easier, a variant that fell behind during development, or
a missing postcondition.

## The Core Problem: Variant-Aware Type Matching

Naively diffing requires/ensures text will flag everything as different because each
variant uses its own type names:

```
// StEph variant:
fn find(&self, key: &K) -> (found: Option<V>)
    requires self.spec_orderedtablesteph_wf()
    ensures found matches Some(v) ==> self@.dom().contains(key@)

// MtEph variant:
fn find(&self, key: &K) -> (found: Option<V>)
    requires self.spec_orderedtablemteph_wf()
    ensures found matches Some(v) ==> self@.dom().contains(key@)
```

These are the *same contract*. The tool must recognize that `spec_orderedtablesteph_wf`
and `spec_orderedtablemteph_wf` are the same predicate applied to different variants.

## Algorithm

### Step 1: Group files by algorithm

Strip variant suffixes to find groups:
```
OrderedTableStEph.rs  \
OrderedTableStPer.rs   |-- group "OrderedTable"
OrderedTableMtEph.rs   |
OrderedTableMtPer.rs  /
```

Suffix patterns to strip (in order, longest first):
`MtEphSlice`, `MtEph`, `MtPer`, `StEph`, `StPer`

Files that don't match any suffix are singletons — list them as "no variants to compare."
Groups with only 1 variant are also singletons.

### Step 2: Parse trait declarations and wf bodies

For each file in a group, parse the **trait declaration**. Extract per function:
- Name
- Parameter list (types and names)
- Return type and name
- `requires` clauses (list of predicates, split on commas at the clause level)
- `ensures` clauses (same)

Also extract:
- `spec fn` declarations in the trait
- `type V = ...` from the View impl (the abstract state type)

### Step 2a: Extract wf predicate bodies

The wf predicate is the foundation of every spec. If wf bodies diverge between variants,
all downstream requires/ensures comparisons will be misleading. The tool must extract
and compare wf bodies *first*, before comparing function specs.

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

**Form B — free spec fn (not in a trait):**
```rust
pub open spec fn spec_orderedtablesteph_wf(table: &OrderedTableStEph<K, V>) -> bool {
    table.tree.spec_avltreesteph_wf() && table.size <= usize::MAX
}
```

For each file, find the wf predicate by name pattern (`spec_*_wf`). If it's a trait
spec fn, look up the body in the corresponding `impl` block. If it's a free function,
take the body directly.

### Step 2b: Compare wf bodies across variants

Before comparing function specs (Step 5), compare the canonicalized wf bodies.
Apply the substitution map (Step 3) to each wf body, then compare.

Report wf differences prominently — they indicate structural divergence:
```
src/Chap43/OrderedTableMtEph.rs:42: error: [xvar] wf body DIFF — missing `self.size <= usize::MAX` (present in StEph)
```

If wf bodies match after canonicalization, the group is "wf-consistent" and function
spec comparison (Step 5) is meaningful. If wf bodies diverge, flag it as an error and
still proceed with function comparison — but note in the output that wf divergence may
explain downstream differences.

### Step 3: Build the canonical substitution map

For each group, the naming convention mechanically generates the substitution map.
Given group name `G` and variants `StEph`, `MtEph`, `StPer`, `MtPer`:

| Variant-specific | Canonical |
|------------------|-----------|
| `GStEph`, `GMtEph`, `GStPer`, `GMtPer` | `G` |
| `spec_gsteph_wf`, `spec_gmteph_wf`, `spec_gstper_wf`, `spec_gmtper_wf` | `spec_g_wf` |
| `GStEphTrait`, `GMtEphTrait`, ... | `GTrait` |
| `GStEphIter`, `GMtEphIter`, ... | `GIter` |
| `GStEphGhostIter`, `GMtEphGhostIter`, ... | `GGhostIter` |
| `spec_gsteph_*`, `spec_gmteph_*`, ... (any spec fn) | `spec_g_*` |

The wf name follows the project convention: `spec_<module>_wf` where `<module>` is the
module name in lowercase with no internal underscores. So for `OrderedTable`:
- `spec_orderedtablesteph_wf` → `spec_orderedtable_wf`
- `spec_orderedtablemteph_wf` → `spec_orderedtable_wf`

Build the map by:
1. Collecting all identifiers that contain the variant suffix (case-insensitive)
2. Replacing the variant suffix with nothing to get the canonical form
3. Two identifiers that map to the same canonical form are associated

### Step 4: Canonicalize each clause

For each requires/ensures clause:
1. Apply the substitution map — replace all variant-specific identifiers with canonical
2. Normalize whitespace (collapse to single spaces, trim)
3. The result is a canonical clause string

### Step 5: Sort and compare

For each function that exists in 2+ variants:
1. Collect the canonical requires clauses for each variant. **Sort them alphabetically.**
2. Collect the canonical ensures clauses for each variant. **Sort them alphabetically.**
3. Compare the sorted canonical clause lists across variants.

Ordering doesn't matter semantically — `requires a, b` equals `requires b, a`. Sorting
makes comparison order-independent.

### Step 6: Classify differences

For each function across variants:

| Classification | Meaning |
|---------------|---------|
| **MATCH** | Same canonical clauses (after sort) in all variants |
| **WEAK** | One variant has fewer ensures clauses than another |
| **STRONG** | One variant has more ensures clauses (not bad, but notable) |
| **DIFF** | Clauses differ in substance after canonicalization |
| **MISS** | Function exists in some variants but not others |
| **EXTRA_REQ** | One variant has additional requires (stricter precondition) |
| **MISSING_REQ** | One variant has fewer requires (weaker precondition) |

### Step 7: Handle expected Mt differences

Some differences between St and Mt are expected and should be flagged as info, not
warnings:

| Expected difference | How to detect | Level |
|--------------------|---------------|-------|
| Mt adds `Send + Sync + 'static` bounds | Extra trait bounds on type parameter | info |
| Mt uses `&self` through RwLock | Ownership pattern difference | info |
| Mt adds `Ghost(...)` parameters | Extra ghost parameters | info |
| Mt `ensures true` where St has real spec | Real ensures missing | **warning** |
| St has postcondition Mt lacks | Spec regression | **warning** |

## Output

### Default: emacs compile format

```
src/Chap43/OrderedTableMtEph.rs:45: warning: [xvar] find: WEAK ensures — missing `found matches Some(v) ==> self@.dom().contains(key@)` (present in StEph)
src/Chap43/OrderedTableMtPer.rs:1: warning: [xvar] MISS — function `previous` missing (present in StEph, StPer, MtEph)
src/Chap43/OrderedTableMtEph.rs:62: info: [xvar] insert: EXTRA_REQ — Mt adds `Send + Sync` bounds (expected)
```

### Markdown: `-m` / `--markdown`

#### Per-group function table

```
## OrderedTable (Chap43) — 4 variants

| # | Function | StEph | StPer | MtEph | MtPer | Status |
|---|----------|-------|-------|-------|-------|--------|
| 1 | new      | match | match | match | match | OK     |
| 2 | find     | match | match | WEAK  | -     | DIFF   |
| 3 | insert   | match | match | match | -     | OK     |
| 4 | previous | match | match | match | MISS  | MISS   |
```

#### Difference detail

For each DIFF/WEAK/MISS:
```
### OrderedTable::find — MtEph is WEAK

StEph ensures (canonical, sorted):
  found matches Some(v) ==> self@.dom().contains(key@)
  result.spec_orderedtable_wf()

MtEph ensures (canonical, sorted):
  result.spec_orderedtable_wf()
  (missing: found matches Some(v) ==> self@.dom().contains(key@))
```

#### Summary table

```
| # | Chap | Group | Variants | Fns | Match | Diff | Weak | Miss | Score |
|---|------|-------|----------|-----|-------|------|------|------|-------|
| 1 | 43   | OrderedTable | 4 | 18 | 15   | 1    | 1    | 1    | 83%   |
```

Score = Match / Total comparisons.

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
| `-f` | `--file FILE` | Not applicable (needs groups). Use path to a chapter directory instead. |
| `-m` | `--markdown` | Markdown table output instead of emacs compile format. |
| `-v` | `--verbose` | Show all functions including matches (default: only differences). |

### Examples

```bash
# Review one chapter
veracity-review-cross-variant-specs -c ~/projects/APAS-VERUS src/Chap43/

# Review all chapters
veracity-review-cross-variant-specs -c ~/projects/APAS-VERUS src/

# Markdown report for all chapters
veracity-review-cross-variant-specs -m -c ~/projects/APAS-VERUS src/

# Verbose: show every function in every group
veracity-review-cross-variant-specs -v -c ~/projects/APAS-VERUS src/Chap18/
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

Parse the **trait declaration** only (the abstract contract). The impl may have weaker
or different specs due to proof limitations — that's a separate concern. The trait is
the source of truth for what the API promises.

### Spec functions

`spec fn` declarations in traits should also be compared. Their signatures (parameter
types, return type) should match across variants. Their bodies may differ (open vs closed,
different View types) — compare only the signature.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust
source. All parsing must be token-aware or AST-aware. Parse requires/ensures blocks
with brace/comma/semicolon awareness. A string-hacking detector will flag and kill
tools that corrupt source syntax.

## Test Strategy

Test on chapters with known-good variant coverage:
- `src/Chap18/` — ArraySeq (4 variants, reference implementation)
- `src/Chap43/` — OrderedTable (4 variants, well-maintained)
- `src/Chap37/` — BSTSet (5 BST variants × Mt, complex)
- `src/Chap05/` — Set (2 variants)

Verify:
- Groups are correctly identified
- Substitution map correctly associates wf names
- Clause sorting produces stable canonical forms
- Known-matching specs produce MATCH
- Known-divergent specs (if any) produce correct DIFF/WEAK classification
