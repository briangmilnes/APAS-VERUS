# veracity-compare-par-mut: Generic bounds warnings are false positives

## Bug

29 warnings about generic bounds mismatches between St and Mt variants. Every
single one follows the same pattern: Mt variants use thread-safe bounds
(`StTInMtT`, `MtKey`, `MtVal`, `+ 'static`) where St variants use `StT`.
This is by design — Mt types must be `Send + Sync + 'static` for thread safety.

## The trait hierarchy

```
StT                    — View + Sized + PartialEq + Eq + Clone
StTInMtT : StT         — adds Send + Sync + 'static
MtKey    : StTInMtT     — adds Ord (for keys in Mt collections)
MtVal    : StTInMtT     — (values in Mt collections)
HashOrd  : StT          — adds Hash + Ord
```

These are supertraits: `StTInMtT` implies `StT`, `MtKey` implies `StTInMtT`
implies `StT`. The Mt bounds are strictly stronger than St bounds. An Mt
variant with `<T: StTInMtT>` is compatible with an St variant with `<T: StT>`.

## All 29 warnings

Every warning matches one of these substitution patterns:

| St bound | Mt bound | Relationship |
|----------|----------|-------------|
| `StT` | `StTInMtT` | subtrait |
| `StT` | `MtVal` | subtrait (via StTInMtT) |
| `StT + Ord` | `StTInMtT + Ord` | subtrait + same |
| `StT + Ord` | `MtKey` | subtrait (MtKey : StTInMtT + Ord) |
| `StT + Ord` | `MtKey + TotalOrder` | subtrait + extra |
| `StT + Hash` | `StTInMtT + Hash` | subtrait + same |
| `TotalOrder` | `StTInMtT + Ord + TotalOrder` | extra bounds (Mt needs Send/Sync) |
| any | any `+ 'static` | Mt requires 'static for thread spawning |
| `F` (bare) | `MtReduceFn<V>` | Mt closure bound |

None of these are real mismatches. The Mt variant always has the same or stronger
bounds. The `'static` bound is universally required by Mt for `Arc`/thread safety.

## Fix

The tool should recognize known supertrait relationships and suppress the warning
when the Mt bound is a known supertrait of the St bound. Specifically:

1. Build a supertrait map from the codebase (or hardcode the APAS-VERUS hierarchy):
   - `StTInMtT` supersedes `StT`
   - `MtKey` supersedes `StT + Ord` and `StTInMtT + Ord`
   - `MtVal` supersedes `StT` and `StTInMtT`
   - `HashOrd` supersedes `StT + Hash + Ord`

2. When comparing generic bounds, normalize each type parameter's bounds through
   the supertrait map. If the Mt bounds are a superset of the St bounds after
   normalization, suppress the warning.

3. Always ignore `'static` in the comparison — Mt variants universally add it
   for thread safety.

4. Downgrade to info (not suppress entirely) when the Mt bound adds extra trait
   bounds beyond the supertrait substitution (e.g., `TotalOrder` added in Mt but
   not in St). These are worth noting but not warnings.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
