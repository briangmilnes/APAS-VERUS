# veracity-compare-par-mut: St/Mt × Eph/Per Variant Alignment

## Goal

Heuristic lint that checks whether the 4 variants of an ADT (StEph, StPer, MtEph, MtPer)
are consistent with each other. Not a formal subtyping proof — a structural comparison
that flags gaps and mismatches.

## Output format

Emacs compilation-mode format: `file:line: LEVEL: message`

```
src/Chap18/ArraySeqStEph.rs:0: info: file group ArraySeq — comparing StEph, StPer, MtEph, MtPer
src/Chap18/ArraySeqStEph.rs:43: info: struct ArraySeqStEphS<T> { seq: Vec<T> }
src/Chap18/ArraySeqStPer.rs:43: info: struct ArraySeqStPerS<T> { seq: Vec<T> }
src/Chap18/ArraySeqMtEph.rs:87: info: struct ArraySeqMtEphS<T> { locked: ... }
src/Chap18/ArraySeqStEph.rs:50: info: View = Seq<T>
src/Chap18/ArraySeqStPer.rs:50: info: View = Seq<T>
src/Chap37/BSTPlainStEph.rs:0: info: file group BSTPlain — comparing StEph, MtEph (no StPer, no MtPer)
src/Chap65/UnionFindStEph.rs:47: warning: ghost field `roots` has no counterpart in other variants
src/Chap43/OrderedSetStPer.rs:60: error: View = Set<T::V> but OrderedSetStEph View = Set<T::V> — wait, match
src/Chap52/AdjTableGraphStEph.rs:30: error: struct field `adj: OrderedTableStEph` but StPer has `adj: TableStPer`
```

Severity levels:
- `info:` — each file set being compared, each struct/view/wf identified
- `warning:` — one variant has ghost fields the others don't, missing wf, missing variant
- `error:` — view types don't match, struct field types diverge, wf names inconsistent

## Phase 1: Identify file groups

Each APAS ADT has up to 4 files following the naming convention:
```
src/ChapNN/FooStEph.rs
src/ChapNN/FooStPer.rs
src/ChapNN/FooMtEph.rs
src/ChapNN/FooMtPer.rs
```

Phase 1 scans `src/Chap*/` and groups files by their base name (everything before
`St`/`Mt` and `Eph`/`Per`). Output: a table of groups with which variants exist.

```
| Base        | Chap | StEph | StPer | MtEph | MtPer |
|-------------|------|-------|-------|-------|-------|
| ArraySeq    | 18   | ✓     | ✓     | ✓     | ✓     |
| ArraySeq    | 19   | ✓     | ✓     | ✓     | —     |
| BSTPlain    | 37   | ✓     | —     | ✓     | —     |
| UnionFind   | 65   | ✓     | —     | —     | —     |
```

Flags:
- Missing variants (e.g., StEph exists but no StPer — is that intentional?)
- Files that don't match the naming convention (standalone files like MCSSSpec.rs)
- Files in lib.rs vs files commented out vs files on disk but not in lib.rs

## Phase 2: Compare data types within each group

For each file group from Phase 1, compare the struct/enum definitions across variants.

For each variant file, extract:
- The primary struct (e.g., `ArraySeqStEphS<T>`)
- Its fields and their types
- Its View impl and view type
- Its wf predicate name and signature

Then compare across variants in the group:

**Struct field alignment**: Do StEph and StPer have analogous fields? Mt variants
typically wrap the St struct in RwLock — does the Mt struct contain the St struct
(directly or inside a lock)?

**View alignment**: Do all variants in a group map to the same logical view type?
(e.g., all ArraySeq variants should view as `Seq<T>`)

**wf alignment**: Does the Mt wf reference or imply the St wf? (e.g.,
`spec_arrayseqmteph_wf` should include the locked inner value satisfying
`spec_arrayseqsteph_wf`)

Output: a per-group report.

```
ArraySeq (Chap18):
  StEph: struct ArraySeqStEphS<T> { seq: Vec<T> }  View = Seq<T>  wf = spec_arrayseqsteph_wf
  StPer: struct ArraySeqStPerS<T> { seq: Vec<T> }  View = Seq<T>  wf = spec_arrayseqstper_wf
  MtEph: struct ArraySeqMtEphS<T> { locked: ... }  View = Seq<T>  wf = spec_arrayseqmteph_wf
  MtPer: struct ArraySeqMtPerS<T> { locked: ... }  View = Seq<T>  wf = spec_arrayseqmtper_wf
  ✓ Views aligned
  ✓ Mt wf references St wf
```

Flags:
- View type mismatch across variants
- Mt wf does not reference St wf
- Struct field count mismatch (may indicate different backing stores)
- Missing wf predicate

## Not in scope (future phases)

- Phase 3: Compare trait function signatures across variants
- Phase 4: Compare requires/ensures clauses for semantic alignment
- Phase 5: Compare spec function definitions across variants
