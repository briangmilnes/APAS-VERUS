# veracity-compare-par-mut fix: reduce false positives (Round 4)

## Context

Phase 3b is done. The tool reports 46 errors. Several are false positives.
Do NOT re-run phases 1, 2, 3, or 3b. Fix the comparison logic only.

## Fix 1: Associated type syntax normalization

Before comparing types, normalize:
- `< T as View > :: V` → `T :: V`
- `< V as View > :: V` → `V :: V`

Strip the `< ... as Trait >` wrapper when the result is just `:: AssocType`.
These are the same type written two ways. Currently reported as errors.

## Fix 2: Reclassify known Mt/St return type differences

These are real differences but expected patterns, not errors. Reclassify from
`error:` to `info:`:

**Owned vs borrowed returns** (Mt can't return references through RwLock):
- `Option<T>` vs `Option<&T>` — Mt returns owned, St returns borrowed
- `Vec<T>` vs `&Vec<T>` — same pattern
- `Arc<Vec<T>>` vs `&Vec<T>` — Mt wraps in Arc

**Iterator return types**: each variant has its own iterator type. Different
iterator struct names across variants is expected, not an error:
- `FooStEphIter` vs `FooMtEphIter` — expected

Classify as `info: fn foo: Mt returns owned T, St returns &T (RwLock pattern)`.

## Fix 3: Empty supertrait parse failures

Several errors show empty supertrait on one side:
```
BSTSplayMtEph: supertrait `Sized + View<V = Link<T>>` but StEph has ``
AdjTableGraphMtPer: supertrait `` but StPer has `Sized`
```

The parser is failing to extract the supertrait from one variant. Fix the
parser to handle these cases. If it can't parse, report `warning:` (parse
failure), not `error:` (mismatch).

## What stays as error

- View type mismatches (e.g., `Seq<T>` vs `Set<T::V>`) — real divergence
- Return type differs and it's NOT an owned/borrowed or iterator pattern
- Supertrait mismatches (after both sides successfully parsed)

## Output format

Same emacs compilation-mode format.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All parsing must be token-aware or AST-aware.
