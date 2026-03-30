# veracity-review-verus-proof-holes: Three wf detection false positives

## Bug 1: Free function wf calls not recognized

Veracity flags `fn_missing_wf_requires` when a function uses the free function form
`spec_<mod>_wf(x)` instead of the method form `x.spec_<mod>_wf()`.

**Example:**

```
src/Chap44/DocumentIndex.rs:438: error: fn_missing_wf_requires - fn new
    — requires should include index.spec_documentindex_wf() for input type DocumentIndex
```

But `new` already has `requires spec_documentindex_wf(index)` — the free function
form. The free function body is `spec_documentindex_wf(&self) -> bool { spec_documentindex_wf(self) }`.
They're semantically identical.

**Fix:** When checking for wf in requires/ensures, recognize both forms:
- Method: `x.spec_<mod>_wf()` or `self.spec_<mod>_wf()`
- Free function: `spec_<mod>_wf(x)` or `spec_<mod>_wf(&x)` or `spec_<mod>_wf(self)` or `spec_<mod>_wf(&self)`

Both should satisfy the `fn_missing_wf_requires` / `fn_missing_wf_ensures` check.

## Bug 2: `Self::spec_<mod>_wf` polymorphic dispatch not recognized

Veracity flags 7 false positives in `ParaHashTableStEph.rs` because the trait uses
`Self::spec_parahashtablesteph_wf(table)` (polymorphic dispatch through the trait)
instead of `spec_hashtable_wf(table)` directly.

**Note:** The function is being renamed from `spec_impl_wf` to
`spec_parahashtablesteph_wf` in a concurrent change. After that rename, the pattern
will be `Self::spec_parahashtablesteph_wf(table)`.

**Example (post-rename):**

```rust
trait ParaHashTableStEphTrait {
    open spec fn spec_parahashtablesteph_wf(table: &HashTable<...>) -> bool {
        spec_hashtable_wf(table)  // default impl
    }

    fn insert(table: &mut HashTable<...>)
        requires Self::spec_parahashtablesteph_wf(old(table)),
        ensures Self::spec_parahashtablesteph_wf(table),
    ;
}
```

Veracity sees `Self::spec_parahashtablesteph_wf(table)` but doesn't recognize it as
a wf predicate because of the `Self::` prefix.

**Fix:** When checking for wf in requires/ensures, also match `Self::spec_<mod>_wf(x)`.
The `Self::` prefix is trait polymorphic dispatch — the spec fn is still a wf predicate.

## Bug 3: Pure functions flagged for missing requires

Functions with no wf-bearing input types get flagged as `fn_missing_requires`:

```
src/Chap26/ETSPMtEph.rs:629: error: fn_missing_requires - fn point_distance
src/Chap44/DocumentIndex.rs:557: error: fn_missing_requires - fn tokens
```

`point_distance` takes `(&Point, &Point)` — f64 coordinates, no wf predicate exists.
`tokens` takes `(&String)` — no wf predicate exists.

**Fix:** Only flag `fn_missing_requires` when at least one parameter type has a known
`spec_<mod>_wf` predicate in the codebase. If no parameter type has a wf predicate,
downgrade to info or suppress entirely. Primitive types (`f64`, `String`, `usize`, etc.)
and types without a wf predicate are not missing requires — they genuinely have no
precondition.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
