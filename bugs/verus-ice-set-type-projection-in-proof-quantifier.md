# Verus ICE: Set<V::V> in proof-mode quantifier crashes sst_to_air

**Verus revision:** `c78aa4958372cfa69e6cb38fd31997c881473271`
**Date:** 2026-03-27
**Severity:** Blocks proof work on generic graph modules

## Reproducer

```rust
use vstd::prelude::*;

verus! {

struct Foo<V: View> {
    data: Map<V::V, Set<V::V>>,
}

proof fn lemma_foo<V: View>(foo: &Foo<V>)
    requires foo.data.dom().finite(),
{
    assert forall|k: V::V| foo.data.dom().contains(k) implies
        #[trigger] foo.data[k].len() >= 0
    by {}
}

} // verus!
```

## Expected

Verus verifies or reports a verification error.

## Actual

Verus panics with:
```
sst_to_air.rs: assertion failed: abstract datatype should be boxed
```

## Impact

Cannot write loop invariants or `assert forall` over `Map<K::V, Set<V::V>>`
in proof mode. This blocks proving any function that iterates over a table
whose values are sets — specifically AdjTableGraph's `num_edges`, `vertices`,
and `delete_vertex` (8 holes across 3 files in APAS-VERUS Chap52).

## Workaround

Use `assume(...)` or `#[verifier::external_body]` to bypass the proof
obligation. The underlying logic is sound — only the Verus compilation crashes.

## Notes

A related bug (`Set<V::V>` inside tuple inside Seq causing sst_to_air crash)
was fixed between Verus revisions `76e69b81` and `c78aa4958`. This variant
with `Set<V::V>` as a Map value in proof-mode quantifiers still crashes.
