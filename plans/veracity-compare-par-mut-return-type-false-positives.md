# veracity-compare-par-mut: Fix false positive return type errors

## Bug

Phase 3 reports return type mismatches that are false positives. The tool does
literal string comparison on return types, but variant-specific types (iterators,
collection wrappers) will always differ by name across St/Mt/Eph/Per variants.

## Current false positives (4 of 6 total errors)

```
src/Chap37/AVLTreeSeqMtPer.rs:247: error: `iter` returns `AVLTreeSeqMtPerBorrowIter<'a, T>` but StPer returns `AVLTreeSeqStPerIter<'a, T>`
src/Chap37/BSTSplayMtEph.rs:1786: error: `insert` returns `Result<(), ()>` but StEph returns `()`
src/Chap43/OrderedSetMtEph.rs:164: error: `to_seq` returns `ArraySeqStPerS<T>` but StPer returns `AVLTreeSeqStPerS<T>`
src/Chap43/OrderedTableMtPer.rs:180: error: `domain` returns `OrderedSetMtEph<K>` but StPer returns `ArraySetStEph<K>`
```

### Error 1: `iter` — false positive

`AVLTreeSeqMtPerBorrowIter` is the MtPer variant's iterator. `AVLTreeSeqStPerIter`
is StPer's. Every variant defines its own iterator struct named after itself. These
are structurally equivalent — they wrap the same data and iterate the same way. The
name difference is the variant suffix, not a semantic difference.

### Error 2: `insert` returns `Result<(), ()>` vs `()` — real but expected

Mt variants wrap operations in lock acquire/release. When the lock can fail (capacity,
poisoning), Mt returns `Result`. This is a known Mt pattern, not a bug. Should be
downgraded to info, not error.

### Error 3: `to_seq` returns different seq types — real but expected

`ArraySeqStPerS` and `AVLTreeSeqStPerS` are both sequence implementations. The Mt
variant chose a different backing structure. Both implement the same trait and have
the same View type (`Seq<T::V>`). The semantic contract is the same.

### Error 4: `domain` returns different set types — real but expected

`OrderedSetMtEph` and `ArraySetStEph` are both set implementations. Same situation
as error 3 — different backing type, same View (`Set<T::V>`).

## Proposed fix

### For iterator types (error 1)

When comparing return types, recognize variant-name substitution. If the return type
contains the variant suffix of its own file (e.g., `MtPer`, `StEph`), check whether
substituting the reference variant's suffix produces the reference return type. If so,
it's a match — not an error.

Example: `AVLTreeSeqMtPerBorrowIter` with MtPer→StPer substitution would look for
`AVLTreeSeqStPerBorrowIter` or `AVLTreeSeqStPerIter`. The tool should try both the
direct substitution and common iterator naming patterns (`*BorrowIter` ↔ `*Iter`).

### For Result wrapping (error 2)

When the Mt variant returns `Result<T, E>` and the St variant returns `T`, downgrade
from error to info. Mt lock operations legitimately add a Result wrapper. The inner
type `T` should still match.

### For different collection backing types (errors 3, 4)

This is harder. The return types are genuinely different structs. But if both return
types implement the same base trait and have the same View type, the semantic contract
is preserved. Two options:

**Option A (simple):** Downgrade to warning instead of error when the return types
differ but both contain a recognized APAS collection name (`ArraySeq`, `AVLTreeSeq`,
`ArraySet`, `OrderedSet`, etc.).

**Option B (precise):** If both return types can be resolved to structs in the codebase,
check whether their View types match. If both have `View = Seq<T::V>` (or both have
`View = Set<T::V>`), downgrade to info.

Option A is simpler and sufficient for now.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
