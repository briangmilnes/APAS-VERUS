# veracity-fix-redundant-finites — Tool Prompt

A veracity transformer that removes redundant `.finite()` and `.dom().finite()` from
`ensures` clauses where the same function already ensures `spec_*_wf()` and wf
includes finite.

## CLI Interface

```
veracity-fix-redundant-finites [OPTIONS] --codebase <CODEBASE>

Options:
  -c, --codebase <CODEBASE>    Root of the APAS-VERUS codebase
  -d, --directory <DIRECTORY>  Process only this chapter directory
  -f, --file <FILE>            Process only this file
  -n, --dry-run                Show what would change, don't write
  -h, --help                   Print help
```

## Fixture

TOML file built into the binary. Maps each module's wf predicate to the finite
property it guarantees.

```toml
# veracity-fix-redundant-finites-fixture.toml

[[module]]
wf = "spec_avltreesetsteph_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_avltreesetstper_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_avltreesetmteph_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_avltreesetmtper_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_arraysetsteph_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_arraysetenummteph_wf"
finite = "self@.finite()"
chapter = 41

[[module]]
wf = "spec_orderedsetsteph_wf"
finite = "self@.finite()"
chapter = 43

[[module]]
wf = "spec_orderedsetstper_wf"
finite = "self@.finite()"
chapter = 43

[[module]]
wf = "spec_orderedsetmteph_wf"
finite = "self@.finite()"
chapter = 43

[[module]]
wf = "spec_tablesteph_wf"
finite = "self@.dom().finite()"
chapter = 42

[[module]]
wf = "spec_tablestper_wf"
finite = "self@.dom().finite()"
chapter = 42

[[module]]
wf = "spec_tablemteph_wf"
finite = "self@.dom().finite()"
chapter = 42

[[module]]
wf = "spec_orderedtablesteph_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_orderedtablestper_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_orderedtablemteph_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_orderedtablemtper_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_augorderedtablesteph_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_augorderedtablestper_wf"
finite = "self@.dom().finite()"
chapter = 43

[[module]]
wf = "spec_augorderedtablemteph_wf"
finite = "self@.dom().finite()"
chapter = 43
```

## Algorithm

For each Rust file in scope:

1. **Parse ensures blocks.** For each function, collect the ensures clause tokens.

2. **Check wf coverage.** If the ensures contains `EXPR.spec_foo_wf()` for some wf
   in the fixture, note which finite pattern (`self@.finite()` or
   `self@.dom().finite()`) it covers.

3. **Match finite clauses.** Check if the ensures also contains the redundant finite
   pattern for the SAME expression root. The expression root is the text before
   `.spec_*_wf()` — typically `self`, `old(self)`, `split.0`, `range`, etc.

   Examples of matches:
   - `ensures self.spec_orderedtablesteph_wf()` covers `self@.dom().finite()`
   - `ensures split.0.spec_orderedtablesteph_wf()` covers `split.0@.dom().finite()`
   - `ensures old(self).spec_avltreesetsteph_wf()` covers `old(self)@.finite()`

   The tool must handle the `old(self)` → `old(self)@` mapping and bare `self` → `self@`.

4. **Remove redundant finite.** Delete the finite clause from ensures. Handle comma
   separation: if the finite was the only clause on a line, remove the line. If it
   was comma-separated with other clauses, remove it and fix commas.

5. **Emit info line.** `FILE:LINE:info: REMOVED redundant FINITE_PATTERN (covered by WF_NAME)`

## Output

All output to stdout. Emacs compilation-mode format:

```
src/Chap41/ArraySetStEph.rs:142:info: REMOVED redundant self@.finite() (covered by spec_arraysetsteph_wf)
src/Chap43/OrderedTableStEph.rs:120:info: REMOVED redundant self@.dom().finite() (covered by spec_orderedtablesteph_wf)
src/Chap43/OrderedTableStEph.rs:128:info: REMOVED redundant self@.dom().finite() (covered by spec_orderedtablesteph_wf)
...
```

## Logging

Per-file log written to `src/ChapNN/analyses/veracity-fix-redundant-finites.log`.

Summary table on stdout:

```
Summary:
|   # | Chap | File                        | Removed |
|-----|------|-----------------------------|---------|
|   1 |   41 | ArraySetStEph.rs            |       2 |
|   2 |   41 | AVLTreeSetMtEph.rs          |       1 |
|   3 |   41 | AVLTreeSetMtPer.rs          |       8 |
|   4 |   43 | OrderedSetStEph.rs          |       4 |
|   5 |   43 | OrderedSetStPer.rs          |      22 |
|   6 |   43 | OrderedSetMtEph.rs          |      14 |
|   7 |   43 | OrderedTableStEph.rs        |      30 |
|   8 |   43 | OrderedTableStPer.rs        |       5 |
|   9 |   43 | OrderedTableMtEph.rs        |      21 |
|  10 |   43 | OrderedTableMtPer.rs        |      12 |
|  11 |   43 | AugOrderedTableStEph.rs     |      19 |
|  12 |   43 | AugOrderedTableStPer.rs     |      14 |
|  13 |   43 | AugOrderedTableMtEph.rs     |      47 |
|     | TOTAL|                             |     199 |
```

## What NOT to Remove

- `.finite()` in `ensures` when there is NO corresponding `spec_*_wf()` in the same
  ensures block. The finite is the only guarantee — keep it.
- `.finite()` in `requires` clauses. This tool only touches ensures.
- `.finite()` on types not in the fixture (e.g., bare `Set` or `Map` returned by a
  helper that has no wf predicate).
- `.finite()` in spec function bodies or proof blocks — only trait/impl function
  ensures.

## Prerequisite

This tool assumes the wf predicates already contain finite. If a wf predicate is
missing finite, this tool will incorrectly remove ensures that are the sole source
of the finite guarantee. The caller must first add finite to wf predicates
(see `plans/finite-in-wf-plan.md` Phase 1) before running this tool for those modules.

For modules where wf already has finite (AVLTreeSetStEph, AVLTreeSetStPer,
AVLTreeSetMtEph, AVLTreeSetMtPer, ArraySetStEph, OrderedSetMtEph,
OrderedTableMtEph, OrderedTableMtPer, AugOrderedTableMtEph), the tool is safe
to run immediately.

## Error Cases

- File not found → `error: FILE not found`
- No ensures block parseable → skip function silently
- Fixture wf not found in file → skip file (no removals)

## DOT

Don't overthink. Parse ensures, match patterns, remove redundant finites, emit
diagnostics. One pass through the fixture, validate at the end.
