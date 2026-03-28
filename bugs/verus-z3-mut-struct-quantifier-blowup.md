# Z3 &mut encoding blowup: 17GB+ on 4-field struct with quantified maps

**Verus revision:** `c78aa4958372cfa69e6cb38fd31997c881473271`
**Date:** 2026-03-27
**Severity:** Blocks &mut proofs on UnionFind and similar multi-field structs

## Reproducer

```rust
use vstd::prelude::*;

verus! {

struct UF {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

fn union_merge(uf: &mut UF, rx: usize, ry: usize)
    requires
        old(uf).parent@.len() == old(uf).rank@.len(),
        old(uf).parent@.len() == old(uf).size@.len(),
        rx < old(uf).parent@.len(),
        ry < old(uf).parent@.len(),
    ensures
        uf.parent@.len() == old(uf).parent@.len(),
        forall|i: int| 0 <= i < uf.parent@.len() && i != rx as int && i != ry as int
            ==> #[trigger] uf.parent@[i] == old(uf).parent@[i],
{
    uf.parent.set(rx, ry);
}

} // verus!
```

## Expected

Z3 verifies within reasonable memory (< 4GB).

## Actual

Z3 uses 17.7GB RSS and exceeds rlimit. The &mut encoding for a 4-field struct
with quantified postconditions over multiple Vec fields causes exponential
SMT formula growth.

## Impact

Blocks proving `union_merge` and `union` in UnionFindStEph (Chap65).
All lemma infrastructure is proved — only the exec functions with &mut self
on the 4-field struct are blocked.

## Notes

Confirmed by minimal experiment: `src/experiments/mut_struct_quantifier_limit.rs`.
Reducing to 2 fields works. 3 fields is borderline. 4 fields blows up.
